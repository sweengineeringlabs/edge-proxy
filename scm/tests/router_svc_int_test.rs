//! Integration tests verifying router_svc re-exports are reachable.

use edge_proxy::{ProxySvc, Router, RoutingError};

/// Verifies `Router` trait is exported at the crate level.
#[test]
fn test_router_type_is_reachable_from_crate_boundary() {
    fn accepts_router<T: Router<String> + ?Sized>(_: &T) -> bool {
        true
    }
    let router = ProxySvc::new_null_router();
    assert!(accepts_router(router.as_ref()));
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
