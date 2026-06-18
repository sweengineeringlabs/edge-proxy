//! Monitor SAF facade — re-export of the null-object marker trait.

pub use crate::api::Monitor as NullMonitor;

/// Identifies the Monitor concern within the proxy crate.
pub const MONITOR_CONCERN: &str = "monitor";
