//! Proxy dispatch — Router classifies input, Job runs the handler, LifecycleMonitor reports health.
//!
//! Run:
//!     cargo run -p edge-proxy --example dispatch
//!
//! Demonstrates the full proxy dispatch path:
//!   request → Job::run → Router::route → HandlerRegistry::get → Handler::execute
//!
//! SEA layer boundaries kept explicit:
//!   - `edge_domain_handler::` — Handler + HandlerRegistry contracts and their provider factory
//!   - `edge_proxy::` — Job + Router + LifecycleMonitor contracts and their SAF factory
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_command::{Command, CommandBus, CommandError};
use edge_domain_handler::{
    Handler, HandlerContext, HandlerError, HandlerProvider, HandlerRegistry,
};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};
use edge_proxy::{
    ExecutionRequest, HealthRequest, Job, JobError, ProxySvc, RouteRequest, RouteResponse, Router,
    RoutingError,
};
use futures::future::BoxFuture;

// ── request / response types ──────────────────────────────────────────────────

#[derive(Debug)]
struct Request {
    command: String,
    payload: String,
}

#[derive(Debug)]
struct Response {
    handler: String,
    output: String,
}

// ── Noop command bus for example wiring ──────────────────────────────────────

struct NoopBus;
impl CommandBus for NoopBus {
    fn dispatch(&self, _: Box<dyn Command>) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

// ── Handler provider — factory namespace for standard handler constructs ─────

struct DispatchHandlerProvider;
impl HandlerProvider for DispatchHandlerProvider {}

// ── Handler ───────────────────────────────────────────────────────────────────

struct EchoHandlerImpl;

#[async_trait::async_trait]
impl Handler for EchoHandlerImpl {
    type Request = Request;
    type Response = Response;

    fn id(&self) -> &str {
        "echo"
    }
    fn pattern(&self) -> &str {
        "direct"
    }

    async fn execute(
        &self,
        req: Request,
        _ctx: HandlerContext<'_>,
    ) -> Result<Response, HandlerError> {
        let id = self.id().to_string();
        Ok(Response {
            handler: id,
            output: req.payload,
        })
    }
}

// ── Router ────────────────────────────────────────────────────────────────────

struct CommandRouter;

impl Router<String> for CommandRouter {
    fn route<'a>(
        &'a self,
        req: RouteRequest<'a>,
    ) -> BoxFuture<'a, Result<RouteResponse<String>, RoutingError>> {
        Box::pin(async move {
            match req.input {
                "echo" | "ping" => Ok(RouteResponse {
                    intent: "echo".into(),
                }),
                _ => Err(RoutingError::NoMatch),
            }
        })
    }
}

// ── Job ───────────────────────────────────────────────────────────────────────

struct DispatchJob {
    router: Arc<dyn Router<String>>,
    registry: Arc<dyn HandlerRegistry<Request = Request, Response = Response>>,
}

impl Job<Request, Response> for DispatchJob {
    fn run<'a>(
        &'a self,
        req: ExecutionRequest<'a, Request>,
    ) -> BoxFuture<'a, Result<Response, JobError>> {
        Box::pin(async move {
            let intent = self
                .router
                .route(RouteRequest {
                    input: &req.req.command,
                })
                .await?
                .intent;
            let err = JobError::HandlerUnavailable(intent.clone());
            let handler = self.registry.get(&intent).ok_or(err)?;
            Ok(handler.execute(req.req, *req.ctx).await?)
        })
    }
}

// ── main ──────────────────────────────────────────────────────────────────────

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Handler provider: populate the handler registry.
    let registry = DispatchHandlerProvider::in_process_registry::<Request, Response>();
    registry.register(Arc::new(EchoHandlerImpl));
    let registry: Arc<dyn HandlerRegistry<Request = Request, Response = Response>> =
        Arc::new(registry);

    // 2. Proxy: wire router + registry into a Job.
    let job: Arc<dyn Job<Request, Response>> = Arc::new(DispatchJob {
        router: Arc::new(CommandRouter),
        registry,
    });

    // 3. Build the request context at the inbound boundary.
    let security: SecurityContext = SecurityServices::unauthenticated();
    let bus = NoopBus;
    let observer = StdObserveFactory::noop_observer_context();

    // 4. Dispatch — known command routes to the echo handler.
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let resp = job
        .run(ExecutionRequest {
            req: Request {
                command: "echo".into(),
                payload: "hello proxy".into(),
            },
            ctx: &ctx,
        })
        .await?;
    println!("echo  → handler={} output={}", resp.handler, resp.output);

    // 5. Dispatch — routing miss is surfaced as a JobError.
    let ctx2 = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let result = job
        .run(ExecutionRequest {
            req: Request {
                command: "unknown".into(),
                payload: "".into(),
            },
            ctx: &ctx2,
        })
        .await;
    let err = result.unwrap_err();
    println!("unknown → {err}");

    // 6. Lifecycle: null monitor reports Healthy out of the box.
    let lifecycle = ProxySvc::new_null_lifecycle_monitor();
    let report = lifecycle.health(HealthRequest).await?;
    println!("lifecycle overall → {:?}", report.overall);

    Ok(())
}
