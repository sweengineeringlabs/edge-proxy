//! NullLifecycleMonitor SAF facade — re-export of the null-object extension trait.

pub use crate::api::NullLifecycleMonitor;

/// Identifies the NullLifecycleMonitor concern within the proxy crate.
pub const NULL_LIFECYCLE_MONITOR_CONCERN: &str = "null_lifecycle_monitor";
