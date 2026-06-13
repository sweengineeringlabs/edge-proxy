//! Router SAF facade — re-exports for the Routing concern.

pub use crate::api::router::{NullRouter, NullRouterMarker, Router, RoutingError};

/// Identifies the Router concern within the proxy crate.
pub const ROUTER_CONCERN: &str = "router";
