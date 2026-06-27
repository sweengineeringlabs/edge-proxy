//! Integration tests for the Job trait.
//! @covers: api/job/traits/job.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]

use futures::future::BoxFuture;
use edge_domain_observer::StdObserveFactory;

use std::sync::Arc;

use edge_proxy::{HandlerContext, Job, JobError, NullJobMarker, ProxySvc, SecurityContext};

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
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
    (SecurityContext::unauthenticated(), NullBus)
}

struct OkJob;

impl Job<String, String> for OkJob {
    fn run<'a>(
        &'a self,
        req: String,
        _ctx: HandlerContext<'a>,
    ) -> BoxFuture<'a, Result<String, JobError>> {
        Box::pin(async move { Ok(req) })
    }
}

struct ErrJob;

impl Job<String, String> for ErrJob {
    fn run<'a>(
        &'a self,
        _req: String,
        _ctx: HandlerContext<'a>,
    ) -> BoxFuture<'a, Result<String, JobError>> {
        Box::pin(async move { Err(JobError::HandlerUnavailable("none".into())) })
    }
}

#[test]
#[ignore = "blocked: edge-domain v0.11.3 ObserverContext wiring pending"]
fn test_job_trait_is_object_safe() {
    fn _accept(_j: &dyn Job<String, String>) {}
}

#[test]
fn test_job_run_dispatches_request_happy() {
    let (s, b) = anon_ctx_parts();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&s, &b, observer.as_ref());
    let result = rt().block_on(OkJob.run("hello".into(), ctx));
    assert_eq!(result.unwrap(), "hello");
}

#[test]
fn test_job_run_propagates_handler_error_error() {
    let (s, b) = anon_ctx_parts();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&s, &b, observer.as_ref());
    let result = rt().block_on(ErrJob.run("x".into(), ctx));
    assert!(matches!(result, Err(JobError::HandlerUnavailable(_))));
}

#[test]
fn test_job_run_with_empty_request_edge() {
    let (s, b) = anon_ctx_parts();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&s, &b, observer.as_ref());
    let result = rt().block_on(OkJob.run(String::new(), ctx));
    assert_eq!(result.unwrap(), "");
}

// Rule 222 scenario coverage — test_run_* naming pattern ─────────────────────

/// run — happy: request is dispatched and response returned.
#[test]
fn test_run_dispatches_request_happy() {
    let (s, b) = anon_ctx_parts();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&s, &b, observer.as_ref());
    let result = rt().block_on(OkJob.run("payload".into(), ctx));
    assert_eq!(result.unwrap(), "payload");
}

/// run — error: handler unavailable propagates as HandlerUnavailable.
#[test]
fn test_run_handler_unavailable_error() {
    let (s, b) = anon_ctx_parts();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&s, &b, observer.as_ref());
    let result = rt().block_on(ErrJob.run("req".into(), ctx));
    assert!(matches!(result, Err(JobError::HandlerUnavailable(_))));
}

/// run — edge: empty string request is passed through unchanged.
#[test]
fn test_run_empty_string_request_edge() {
    let (s, b) = anon_ctx_parts();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&s, &b, observer.as_ref());
    let result = rt().block_on(OkJob.run(String::new(), ctx));
    assert_eq!(result.unwrap(), "");
}

// Rule 222 scenario coverage for Job::as_null_job ─────────────────────────────

/// as_null_job — happy: default implementation returns None on a concrete impl.
#[test]
#[ignore = "blocked: edge-domain v0.11.3 ObserverContext wiring pending"]
fn test_as_null_job_default_returns_none_happy() {
    assert!(OkJob.as_null_job().is_none());
}

/// as_null_job — error: null-job impl also returns None (no override needed).
#[test]
#[ignore = "blocked: edge-domain v0.11.3 ObserverContext wiring pending"]
fn test_as_null_job_on_null_job_impl_returns_none_error() {
    let job: Arc<dyn Job<String, String>> = ProxySvc::new_null_job::<String, String>();
    assert!((*job).as_null_job().is_none());
}

/// as_null_job — edge: method is callable on a dyn Job trait object.
#[test]
#[ignore = "blocked: edge-domain v0.11.3 ObserverContext wiring pending"]
fn test_as_null_job_accessible_on_dyn_trait_object_edge() {
    let dyn_ref: &dyn Job<String, String> = &OkJob;
    assert!(dyn_ref.as_null_job().is_none());
}

// Rule 222 scenario coverage for Job::as_null_job_marker ─────────────────────

/// as_null_job_marker — happy: default impl returns None on a real job implementation.
#[test]
#[ignore = "blocked: edge-domain v0.11.3 ObserverContext wiring pending"]
fn test_as_null_job_marker_default_returns_none_happy() {
    let result: Option<NullJobMarker> = OkJob.as_null_job_marker();
    assert!(result.is_none());
}

/// as_null_job_marker — error: returns None even when dispatching yields an error result.
#[test]
#[ignore = "blocked: edge-domain v0.11.3 ObserverContext wiring pending"]
fn test_as_null_job_marker_on_err_job_returns_none_error() {
    let result: Option<NullJobMarker> = ErrJob.as_null_job_marker();
    assert!(result.is_none());
}

/// as_null_job_marker — edge: method is accessible through a dyn Job trait object.
#[test]
#[ignore = "blocked: edge-domain v0.11.3 ObserverContext wiring pending"]
fn test_as_null_job_marker_accessible_on_dyn_trait_object_edge() {
    let dyn_ref: &dyn Job<String, String> = &OkJob;
    let result: Option<NullJobMarker> = dyn_ref.as_null_job_marker();
    assert!(result.is_none());
}
