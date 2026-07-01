//! End-to-end contract tests for the `Job` trait, exercised through a
//! test-double implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_observer::StdObserveFactory;
use edge_domain_security::{SecurityBootstrap, SecurityServices};
use edge_proxy::{
    AsNullJobMarkerRequest, AsNullJobRequest, ExecutionRequest, HandlerContext, Job, JobError,
    ProxySvc,
};
use futures::future::BoxFuture;

struct NullBus;
impl edge_proxy::CommandBus for NullBus {
    fn dispatch(
        &self,
        _: Box<dyn edge_domain_command::Command>,
    ) -> BoxFuture<'_, Result<(), edge_domain_command::CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct JobDouble;
impl Job<String, String> for JobDouble {
    fn run<'a>(
        &'a self,
        req: ExecutionRequest<'a, String>,
    ) -> BoxFuture<'a, Result<String, JobError>> {
        Box::pin(async move { Ok(req.req) })
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
    let security = SecurityServices::unauthenticated();
    let bus = NullBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let result = rt().block_on(JobDouble.run(ExecutionRequest {
        req: "hi".into(),
        ctx: &ctx,
    }));
    assert_eq!(result.unwrap(), "hi");
}

/// @covers: Job::run
#[test]
fn test_run_null_job_returns_cancelled_error() {
    let job = ProxySvc::new_null_job::<String, String>();
    let security = SecurityServices::unauthenticated();
    let bus = NullBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
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
        .marker
        .is_none());
}
