//! Integration tests for the NullRouterMarker type.
//! @covers: api/router/null_router.rs

use edge_proxy::NullRouterMarker;

#[test]
fn test_null_router_marker_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<NullRouterMarker>(), 0);
}

#[test]
fn test_null_router_marker_can_be_constructed_happy() {
    let m = NullRouterMarker;
    assert_eq!(std::mem::size_of_val(&m), 0);
}

#[test]
fn test_null_router_marker_is_in_public_api_edge() {
    fn accept(v: NullRouterMarker) -> NullRouterMarker {
        v
    }
    let m = accept(NullRouterMarker);
    assert_eq!(std::mem::size_of_val(&m), 0);
}
