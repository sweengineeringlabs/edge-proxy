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
        JobError::Handler(message) => {
            assert!(message.contains("handler unhealthy"));
        }
        other => panic!("expected JobError::Handler, got {other:?}"),
    }
}

/// @covers: JobError::HandlerUnavailable
#[test]
fn test_job_error_handler_unavailable_display() {
    let err = JobError::HandlerUnavailable("missing".into());
    assert!(err.to_string().contains("missing"));
}
