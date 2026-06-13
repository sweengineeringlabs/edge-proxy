//! `NullRouterMarker` — marker type for null-object router implementations.

/// Marker for null-object implementations of [`crate::api::router::traits::router::Router`].
///
/// Null routers always return `RoutingError::NoMatch`. Include this in the
/// type signature when signalling an intentionally inert router.
pub struct NullRouterMarker;
