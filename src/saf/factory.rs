//! Factory functions — Static Access Facade (SAF).

use swe_edge_configbuilder::ConfigBuilder as _;
use std::sync::Arc;

use crate::api::lifecycle_monitor::LifecycleMonitor;
use crate::core::null_lifecycle_monitor::NullLifecycleMonitor;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Construct a no-op [`LifecycleMonitor`] useful for tests or early bring-up.
///
/// Returned as `Arc<dyn LifecycleMonitor>` so the concrete impl type stays
/// private.
pub fn new_null_lifecycle_monitor() -> Arc<dyn LifecycleMonitor> {
    Arc::new(NullLifecycleMonitor::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::health::HealthStatus;

    #[tokio::test]
    async fn test_factory_returns_healthy_lifecycle_monitor() {
        let m = new_null_lifecycle_monitor();
        assert_eq!(m.health().await.overall, HealthStatus::Healthy);
    }

    /// @covers: new_null_lifecycle_monitor
    #[tokio::test]
    async fn test_new_null_lifecycle_monitor_is_healthy_on_construction() {
        let m = new_null_lifecycle_monitor();
        assert_eq!(m.health().await.overall, HealthStatus::Healthy);
    }

    #[test]
    fn test_new_null_lifecycle_monitor_returns_arc() {
        let _m = new_null_lifecycle_monitor();
    }
}
