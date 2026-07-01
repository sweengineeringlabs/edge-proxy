//! Integration tests for the NullJob type alias.
//! @covers: api/job/null_job.rs
#![allow(clippy::expect_used)]

use edge_domain_observer::StdObserveFactory;
use edge_domain_security::{SecurityBootstrap, SecurityServices};
use edge_proxy::{ExecutionRequest, HandlerContext, JobError, NullJob, ProxySvc, SecurityContext};
use futures::future::BoxFuture;

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio")
}

struct NullBus;
impl edge_proxy::CommandBus for NullBus {
    fn dispatch(
        &self,
        _: Box<dyn edge_domain_command::Command>,
    ) -> BoxFuture<'_, Result<(), edge_domain_command::CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

fn anon_ctx_parts() -> (SecurityContext, NullBus) {
    (SecurityServices::unauthenticated(), NullBus)
}

/// NullJob — happy: a concrete Job impl can be used through the NullJob alias.
#[test]
fn test_null_job_type_alias_accepts_null_job_impl_happy() {
    let arc_job = ProxySvc::new_null_job::<String, String>();
    let null_job_ref: &NullJob = &*arc_job;
    let (s, b) = anon_ctx_parts();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &s,
        commands: &b,
        observer: observer.as_ref(),
    };
    let result = rt().block_on(null_job_ref.run(ExecutionRequest {
        req: "x".into(),
        ctx: &ctx,
    }));
    assert!(matches!(result, Err(JobError::Cancelled)));
}

/// NullJob — error: the erased job returns Cancelled for empty input too.
#[test]
fn test_null_job_via_alias_returns_cancelled_on_empty_input_error() {
    let arc_job = ProxySvc::new_null_job::<String, String>();
    let null_job_ref: &NullJob = &*arc_job;
    let (s, b) = anon_ctx_parts();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &s,
        commands: &b,
        observer: observer.as_ref(),
    };
    let result = rt().block_on(null_job_ref.run(ExecutionRequest {
        req: String::new(),
        ctx: &ctx,
    }));
    assert!(matches!(result, Err(JobError::Cancelled)));
}

/// NullJob — edge: the aliased dyn type is Send and Sync.
#[test]
fn test_null_job_type_alias_is_send_and_sync_edge() {
    fn _check<T: Send + Sync + ?Sized>() {}
    _check::<NullJob>();
}
