//! Integration tests for the NullRouter type alias.
//! @covers: api/router/null_router.rs
#![allow(clippy::expect_used)]

use edge_proxy::{NullRouter, ProxySvc, RoutingError};

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio")
}

/// NullRouter — happy: a concrete Router impl can be used through the NullRouter alias.
#[test]
fn test_null_router_type_alias_accepts_null_router_impl_happy() {
    let arc_router = ProxySvc::new_null_router();
    let null_router_ref: &NullRouter = &*arc_router;
    let result = rt().block_on(null_router_ref.route("any"));
    assert!(matches!(result, Err(RoutingError::NoMatch)));
}

/// NullRouter — error: the erased router returns NoMatch for empty input too.
#[test]
fn test_null_router_via_alias_returns_no_match_on_empty_input_error() {
    let arc_router = ProxySvc::new_null_router();
    let null_router_ref: &NullRouter = &*arc_router;
    let result = rt().block_on(null_router_ref.route(""));
    assert!(matches!(result, Err(RoutingError::NoMatch)));
}

/// NullRouter — edge: the aliased dyn type is Send and Sync.
#[test]
fn test_null_router_type_alias_is_send_and_sync_edge() {
    fn _check<T: Send + Sync + ?Sized>() {}
    _check::<NullRouter>();
}
