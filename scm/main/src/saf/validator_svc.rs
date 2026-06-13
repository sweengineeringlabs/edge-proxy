//! Validator SAF facade — re-export of the validation contract.

pub use crate::api::validator::Validator;

/// Identifies the Validator concern within the proxy crate.
pub const VALIDATOR_CONCERN: &str = "validator";
