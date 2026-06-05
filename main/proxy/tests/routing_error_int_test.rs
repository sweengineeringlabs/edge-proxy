//! Integration tests for RoutingError.

use edge_proxy::RoutingError;

/// @covers: RoutingError::NoMatch
#[test]
fn test_routing_error_no_match_display() {
    let err = RoutingError::NoMatch;
    assert!(err.to_string().contains("matched"));
}

/// @covers: RoutingError::InvalidInput
#[test]
fn test_routing_error_invalid_input_display() {
    let err = RoutingError::InvalidInput("empty body".into());
    assert!(err.to_string().contains("empty body"));
}

/// @covers: RoutingError::Internal
#[test]
fn test_routing_error_internal_display() {
    let err = RoutingError::Internal("classifier timeout".into());
    assert!(err.to_string().contains("classifier timeout"));
}
