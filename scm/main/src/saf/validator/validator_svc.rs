//! Validator SAF facade — re-export of the Validator trait contract.

pub use crate::api::Validator;

/// Identifies the Validator concern within the proxy crate.
pub const VALIDATOR_CONCERN: &str = "validator";
