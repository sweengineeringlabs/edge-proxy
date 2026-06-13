//! Integration tests for the Router trait.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use futures::future::BoxFuture;

use std::sync::Arc;

use edge_proxy::{NullRouterMarker, ProxySvc, Router, RoutingError};

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

#[test]
fn test_router_trait_is_object_safe() {
    fn _accept(_r: &dyn Router<String>) {}
}

struct FixedRouter {
    intent: &'static str,
}

impl Router<String> for FixedRouter {
    fn route<'a>(&'a self, _input: &'a str) -> BoxFuture<'a, Result<String, RoutingError>> {
        let v = self.intent.to_string();
        Box::pin(async move { Ok(v) })
    }
}

struct EmptyRejectRouter;

impl Router<String> for EmptyRejectRouter {
    fn route<'a>(&'a self, input: &'a str) -> BoxFuture<'a, Result<String, RoutingError>> {
        let result = if input.is_empty() {
            Err(RoutingError::InvalidInput("empty input".into()))
        } else if input == "unknown" {
            Err(RoutingError::NoMatch)
        } else {
            Ok(input.to_string())
        };
        Box::pin(async move { result })
    }
}

// Rule 222 scenario coverage for Router::route ────────────────────────────────

/// route — happy: known command returns the correct handler id.
#[test]
fn test_route_known_command_returns_handler_id_happy() {
    let r = FixedRouter { intent: "echo" };
    let result = rt().block_on(r.route("echo"));
    assert_eq!(result.unwrap(), "echo");
}

/// route — error: unknown command returns NoMatch.
#[test]
fn test_route_unknown_command_returns_no_match_error() {
    let r = EmptyRejectRouter;
    let result = rt().block_on(r.route("unknown"));
    assert!(matches!(result, Err(RoutingError::NoMatch)));
}

/// route — edge: empty input returns InvalidInput.
#[test]
fn test_route_empty_input_returns_invalid_input_edge() {
    let r = EmptyRejectRouter;
    let result = rt().block_on(r.route(""));
    assert!(matches!(result, Err(RoutingError::InvalidInput(_))));
}

// Rule 222 scenario coverage for Router::as_null_router ───────────────────────

/// as_null_router — happy: default implementation returns None on a concrete impl.
#[test]
fn test_as_null_router_default_returns_none_happy() {
    let r = FixedRouter { intent: "echo" };
    assert!(r.as_null_router().is_none());
}

/// as_null_router — error: null-router impl also returns None (no override needed).
#[test]
fn test_as_null_router_on_null_router_impl_returns_none_error() {
    let router: Arc<dyn Router<String>> = ProxySvc::new_null_router();
    assert!((*router).as_null_router().is_none());
}

/// as_null_router — edge: method is callable on a dyn Router trait object.
#[test]
fn test_as_null_router_accessible_on_dyn_trait_object_edge() {
    let r = FixedRouter { intent: "echo" };
    let dyn_ref: &dyn Router<String> = &r;
    assert!(dyn_ref.as_null_router().is_none());
}

// Rule 222 scenario coverage for Router::as_null_router_marker ────────────────

/// as_null_router_marker — happy: default impl returns None on a real router implementation.
#[test]
fn test_as_null_router_marker_default_returns_none_happy() {
    let r = FixedRouter { intent: "echo" };
    let result: Option<NullRouterMarker> = r.as_null_router_marker();
    assert!(result.is_none());
}

/// as_null_router_marker — error: returns None even when routing yields an error result.
#[test]
fn test_as_null_router_marker_on_reject_router_returns_none_error() {
    let r = EmptyRejectRouter;
    let result: Option<NullRouterMarker> = r.as_null_router_marker();
    assert!(result.is_none());
}

/// as_null_router_marker — edge: method is accessible through a dyn Router trait object.
#[test]
fn test_as_null_router_marker_accessible_on_dyn_trait_object_edge() {
    let r = FixedRouter { intent: "echo" };
    let dyn_ref: &dyn Router<String> = &r;
    let result: Option<NullRouterMarker> = dyn_ref.as_null_router_marker();
    assert!(result.is_none());
}
