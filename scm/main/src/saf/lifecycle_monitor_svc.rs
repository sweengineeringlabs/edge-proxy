//! LifecycleMonitor SAF facade — re-exports for the Lifecycle concern.

pub use crate::api::lifecycle::{
    ComponentHealth, HealthReport, HealthStatus, LifecycleError, LifecycleMonitor,
};

/// Identifies the LifecycleMonitor concern within the proxy crate.
pub const LIFECYCLE_MONITOR_CONCERN: &str = "lifecycle_monitor";
