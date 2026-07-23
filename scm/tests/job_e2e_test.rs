//! End-to-end contract tests for the `Job` trait, exercised through a
//! test-double implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::ObserverContextAdapter;
use edge_application_observer::StdObserveFactory;
use edge_proxy::{
    AsNullJobMarkerRequest, AsNullJobRequest, ExecutionRequest, HandlerContext, Job, JobError,
    JobResponse, ProxySvc,
};
use edge_security_runtime::SecurityContext;
use futures::future::BoxFuture;

struct NullBus;
impl edge_proxy::CommandBus for NullBus {
    fn dispatch(
        &self,
        _: edge_application_command::CommandDispatchRequest,
    ) -> BoxFuture<'_, Result<(), edge_application_command::CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct JobDouble;
#[async_trait::async_trait]
impl Job<String, String> for JobDouble {
    async fn run(
        &self,
        req: ExecutionRequest<'_, String>,
    ) -> Result<JobResponse<String>, JobError> {
        Ok(JobResponse { payload: req.req })
    }
}

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio")
}

/// @covers: Job::run
#[test]
fn test_run_dispatches_request_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = NullBus;
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: &observer_adapter,
    };
    let result = rt().block_on(JobDouble.run(ExecutionRequest {
        req: "hi".into(),
        ctx: &ctx,
    }));
    assert_eq!(result.unwrap().payload, "hi");
}

/// @covers: Job::run
#[test]
fn test_run_null_job_returns_cancelled_error() {
    let job = ProxySvc::new_null_job::<String, String>();
    let security = SecurityContext::unauthenticated();
    let bus = NullBus;
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: &observer_adapter,
    };
    let result = rt().block_on(job.run(ExecutionRequest {
        req: "hi".into(),
        ctx: &ctx,
    }));
    assert!(matches!(result, Err(JobError::Cancelled)));
}

/// @covers: Job::as_null_job
#[test]
fn test_as_null_job_default_returns_none_edge() {
    assert!(JobDouble
        .as_null_job(AsNullJobRequest)
        .unwrap()
        .job
        .is_none());
}

/// @covers: Job::as_null_job_marker
#[test]
fn test_as_null_job_marker_default_returns_none_edge() {
    assert!(JobDouble
        .as_null_job_marker(AsNullJobMarkerRequest)
        .unwrap()
        .value
        .is_none());
}
