//! Integration tests for proxy error types.

use edge_proxy::{JobError, LifecycleError, RoutingError};

/// @covers: LifecycleError::DrainFailed
#[test]
fn test_lifecycle_error_drain_failed_display() {
    let err = LifecycleError::DrainFailed("worker A".to_string());
    assert!(err.to_string().contains("worker A"));
}

/// @covers: RoutingError::NoMatch
#[test]
fn test_routing_error_no_match_display() {
    assert!(RoutingError::NoMatch.to_string().contains("matched"));
}

/// @covers: RoutingError::InvalidInput
#[test]
fn test_routing_error_invalid_input_display() {
    let e = RoutingError::InvalidInput("empty body".into());
    assert!(e.to_string().contains("empty body"));
}

/// @covers: RoutingError::Internal
#[test]
fn test_routing_error_internal_display() {
    let e = RoutingError::Internal("classifier timeout".into());
    assert!(e.to_string().contains("classifier timeout"));
}

/// @covers: JobError::Routing
#[test]
fn test_job_error_wraps_routing_error() {
    let r: JobError = RoutingError::NoMatch.into();
    assert!(matches!(r, JobError::Routing(RoutingError::NoMatch)));
}
