//! Integration tests verifying router_svc re-exports are reachable.

use edge_proxy::{Router, RoutingError};

/// Verifies `Router` trait is exported at the crate level.
#[test]
fn test_router_type_is_reachable_from_crate_boundary() {
    fn _accept<T: Router<String>>() {}
}

/// Verifies `RoutingError::NoMatch` variant is accessible.
#[test]
fn test_routing_error_no_match_variant_is_accessible() {
    let e = RoutingError::NoMatch;
    assert!(matches!(e, RoutingError::NoMatch));
}

/// Verifies `RoutingError::InvalidInput` carries a message.
#[test]
fn test_routing_error_invalid_input_carries_message() {
    let e = RoutingError::InvalidInput("bad".into());
    let msg = format!("{e}");
    assert!(msg.contains("bad"));
}
