//! Integration tests for the Router trait.

use edge_proxy::Router;

/// @covers: Router
#[test]
fn test_router_trait_is_object_safe() {
    fn _accept(_r: &dyn Router<String>) {}
}
