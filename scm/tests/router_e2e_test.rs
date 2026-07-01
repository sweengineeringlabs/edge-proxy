//! End-to-end contract tests for the `Router` trait, exercised through a
//! test-double implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_proxy::{
    AsNullRouterMarkerRequest, AsNullRouterRequest, ProxySvc, RouteRequest, RouteResponse, Router,
    RoutingError,
};
use futures::future::BoxFuture;

struct RouterDouble;
impl Router<String> for RouterDouble {
    fn route<'a>(
        &'a self,
        req: RouteRequest<'a>,
    ) -> BoxFuture<'a, Result<RouteResponse<String>, RoutingError>> {
        let intent = req.input.to_string();
        Box::pin(async move { Ok(RouteResponse { intent }) })
    }
}

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio")
}

/// @covers: Router::route
#[test]
fn test_route_classifies_input_happy() {
    let result = rt().block_on(RouterDouble.route(RouteRequest { input: "echo" }));
    assert_eq!(result.unwrap().intent, "echo");
}

/// @covers: Router::route
#[test]
fn test_route_null_router_returns_no_match_error() {
    let router = ProxySvc::new_null_router();
    let result = rt().block_on(router.route(RouteRequest { input: "anything" }));
    assert!(matches!(result, Err(RoutingError::NoMatch)));
}

/// @covers: Router::as_null_router
#[test]
fn test_as_null_router_default_returns_none_edge() {
    assert!(RouterDouble
        .as_null_router(AsNullRouterRequest)
        .unwrap()
        .router
        .is_none());
}

/// @covers: Router::as_null_router_marker
#[test]
fn test_as_null_router_marker_default_returns_none_edge() {
    assert!(RouterDouble
        .as_null_router_marker(AsNullRouterMarkerRequest)
        .unwrap()
        .marker
        .is_none());
}
