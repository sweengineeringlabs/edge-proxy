//! Job SAF facade — re-export of the Job trait contract.

pub use crate::api::Job;

/// Identifies the Job concern within the proxy crate.
pub const JOB_CONCERN: &str = "job";
