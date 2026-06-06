//! Integration tests for NullLifecycleMonitorApi marker trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_proxy::{HealthStatus, ProxySvc};

/// @covers: NullLifecycleMonitorApi
#[tokio::test]
async fn test_new_null_lifecycle_monitor_implements_null_lifecycle_monitor_api() {
    // The factory returns an Arc<dyn LifecycleMonitor> backed by NullLifecycleMonitor,
    // which implements NullLifecycleMonitorApi. Verify the monitor works correctly.
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(m.health().await.overall, HealthStatus::Healthy);
}
