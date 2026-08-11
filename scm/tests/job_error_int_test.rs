//! Integration tests for JobError.

use edge_application_handler::HandlerError;
use edge_proxy::{JobError, RoutingError};

/// @covers: JobError::Routing
#[test]
fn test_job_error_wraps_routing_error() {
    let err: JobError = RoutingError::NoMatch.into();
    assert!(matches!(err, JobError::Routing(RoutingError::NoMatch)));
}

/// @covers: JobError::Handler
#[test]
fn test_job_error_wraps_handler_error() {
    let err: JobError = HandlerError::Unhealthy.into();
    match err {
        // `Handler` now carries the `HandlerError` itself, not a formatted
        // string — so ingress adapters can map its variant to a transport
        // status. The wrapped error is preserved verbatim.
        JobError::Handler(HandlerError::Unhealthy) => {}
        JobError::Handler(other) => panic!("expected HandlerError::Unhealthy, got {other:?}"),
        other => panic!("expected JobError::Handler, got {other:?}"),
    }
    // Display still renders the wrapped error's message.
    assert!(JobError::Handler(HandlerError::Unhealthy)
        .to_string()
        .contains("handler unhealthy"));
}

/// @covers: JobError::HandlerUnavailable
#[test]
fn test_job_error_handler_unavailable_display() {
    let err = JobError::HandlerUnavailable("missing".into());
    assert!(err.to_string().contains("missing"));
}
