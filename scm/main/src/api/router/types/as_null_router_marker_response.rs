//! [`AsNullRouterMarkerResponse`] — response for [`Router::as_null_router_marker`](crate::api::router::traits::Router::as_null_router_marker).

use crate::api::router::types::NullRouterMarker;

/// The null-router marker token, if this implementation is a null object.
pub struct AsNullRouterMarkerResponse {
    /// The marker token, or `None` for real implementations.
    pub marker: Option<NullRouterMarker>,
}
