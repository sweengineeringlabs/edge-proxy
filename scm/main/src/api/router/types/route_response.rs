//! [`RouteResponse`] — response for [`Router::route`](crate::api::router::traits::Router::route).

/// The resolved intent for a classified input.
pub struct RouteResponse<Intent> {
    /// The resolved intent.
    pub intent: Intent,
}
