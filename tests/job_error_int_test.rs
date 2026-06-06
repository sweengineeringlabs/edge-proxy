//! Integration tests for JobError.

use edge_domain::HandlerError;
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
    assert!(matches!(err, JobError::Handler(HandlerError::Unhealthy)));
}

/// @covers: JobError::HandlerUnavailable
#[test]
fn test_job_error_handler_unavailable_display() {
    let err = JobError::HandlerUnavailable("missing".into());
    assert!(err.to_string().contains("missing"));
}
