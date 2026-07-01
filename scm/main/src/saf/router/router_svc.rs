//! Router SAF facade — re-export of the Router trait contract.

pub use crate::api::Router;

/// Identifies the Router concern within the proxy crate.
pub const ROUTER_CONCERN: &str = "router";
