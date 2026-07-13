//! Proxy dispatch — Router classifies input, Job runs the handler, LifecycleMonitor reports health.
//!
//! Run:
//!     cargo run -p edge-proxy --example dispatch
//!
//! Demonstrates the full proxy dispatch path:
//!   request → Job::run → Router::route → HandlerRegistry::get → Handler::execute
//!
//! SEA layer boundaries kept explicit:
//!   - `edge_domain_handler::` — Handler + HandlerRegistry contracts
//!   - `edge_proxy::` — Job + Router + LifecycleMonitor contracts and their SAF factory
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_command::{CommandBus, CommandDispatchRequest, CommandError};
use edge_domain_handler::{
    ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext, HandlerError,
    HandlerLookupRequest, HandlerRegistry, IdRequest, IdResponse, InProcessHandlerRegistry,
    PatternRequest, PatternResponse, RegisterHandlerRequest,
};
use edge_domain_observer::StdObserveFactory;
use edge_proxy::{
    ExecutionRequest, HealthRequest, Job, JobError, JobResponse, ProxySvc, RouteRequest,
    RouteResponse, Router, RoutingError,
};
use edge_security_runtime::SecurityContext;
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
    fn dispatch(&self, _: CommandDispatchRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

// ── Handler ───────────────────────────────────────────────────────────────────

struct EchoHandlerImpl;

#[async_trait::async_trait]
impl Handler for EchoHandlerImpl {
    type Request = Request;
    type Response = Response;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "echo".to_string(),
        })
    }
    fn pattern(&self, _req: PatternRequest) -> Result<PatternResponse, HandlerError> {
        Ok(PatternResponse {
            pattern: "direct".to_string(),
        })
    }

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, Request>,
    ) -> Result<Response, HandlerError> {
        let id = self.id(IdRequest)?.id;
        Ok(Response {
            handler: id,
            output: req.req.payload,
        })
    }
}

// ── Router ────────────────────────────────────────────────────────────────────

struct CommandRouter;

#[async_trait::async_trait]
impl Router<String> for CommandRouter {
    async fn route(&self, req: RouteRequest<'_>) -> Result<RouteResponse<String>, RoutingError> {
        match req.input {
            "echo" | "ping" => Ok(RouteResponse {
                intent: "echo".into(),
            }),
            _ => Err(RoutingError::NoMatch),
        }
    }
}

// ── Job ───────────────────────────────────────────────────────────────────────

struct DispatchJob {
    router: Arc<dyn Router<String>>,
    registry: Arc<dyn HandlerRegistry<Request = Request, Response = Response>>,
}

#[async_trait::async_trait]
impl Job<Request, Response> for DispatchJob {
    async fn run(
        &self,
        req: ExecutionRequest<'_, Request>,
    ) -> Result<JobResponse<Response>, JobError> {
        let intent = self
            .router
            .route(RouteRequest {
                input: &req.req.command,
            })
            .await?
            .intent;
        let err = JobError::HandlerUnavailable(intent.clone());
        let handler = self
            .registry
            .get(HandlerLookupRequest { id: intent })?
            .handler
            .ok_or(err)?;
        let payload = handler
            .execute(HandlerExecutionRequest {
                req: req.req,
                ctx: req.ctx,
            })
            .await?;
        Ok(JobResponse { payload })
    }
}

// ── main ──────────────────────────────────────────────────────────────────────

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Handler registry: populate it directly.
    let registry = InProcessHandlerRegistry::<Request, Response>::default();
    registry.register(RegisterHandlerRequest::new(Arc::new(EchoHandlerImpl)))?;
    let registry: Arc<dyn HandlerRegistry<Request = Request, Response = Response>> =
        Arc::new(registry);

    // 2. Proxy: wire router + registry into a Job.
    let job: Arc<dyn Job<Request, Response>> = Arc::new(DispatchJob {
        router: Arc::new(CommandRouter),
        registry,
    });

    // 3. Build the request context at the inbound boundary.
    let security: SecurityContext = SecurityContext::unauthenticated();
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
    println!(
        "echo  → handler={} output={}",
        resp.payload.handler, resp.payload.output
    );

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
