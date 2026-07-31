//! [`RouteRequest`] — request for [`Router::route`](crate::api::router::traits::Router::route).

/// Request to classify an input string into a domain-specific intent.
pub struct RouteRequest<'a> {
    /// The input to classify.
    pub input: &'a str,
}
