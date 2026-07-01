//! LifecycleMonitor SAF facade — re-export of the LifecycleMonitor trait contract.

pub use crate::api::LifecycleMonitor;

/// Identifies the LifecycleMonitor concern within the proxy crate.
pub const LIFECYCLE_MONITOR_CONCERN: &str = "lifecycle_monitor";
