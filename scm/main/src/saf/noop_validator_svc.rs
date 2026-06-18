//! NoopValidator SAF facade — re-export of the always-passing validator marker.

pub use crate::api::NoopValidator;

/// Identifies the NoopValidator concern within the proxy crate.
pub const NOOP_VALIDATOR_CONCERN: &str = "noop_validator";
