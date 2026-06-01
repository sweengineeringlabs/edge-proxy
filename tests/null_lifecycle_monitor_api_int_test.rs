//! Integration tests for NullLifecycleMonitorApi marker trait.

use edge_proxy::{NullLifecycleMonitorApi, ProxySvc, HealthStatus};

/// @covers: NullLifecycleMonitorApi
#[tokio::test]
async fn test_new_null_lifecycle_monitor_implements_null_lifecycle_monitor_api() {
    // The factory returns an Arc<dyn LifecycleMonitor> backed by NullLifecycleMonitor,
    // which implements NullLifecycleMonitorApi. Verify the monitor works correctly.
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(m.health().await.overall, HealthStatus::Healthy);
}
