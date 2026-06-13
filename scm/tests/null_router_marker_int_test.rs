//! Integration tests for the NullRouterMarker type.
//! @covers: api/router/null_router.rs

use edge_proxy::NullRouterMarker;

#[test]
fn test_null_router_marker_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<NullRouterMarker>(), 0);
}

#[test]
fn test_null_router_marker_can_be_constructed_happy() {
    let _m = NullRouterMarker;
}

#[test]
fn test_null_router_marker_is_in_public_api_edge() {
    fn _accept(_: NullRouterMarker) {}
    _accept(NullRouterMarker);
}
