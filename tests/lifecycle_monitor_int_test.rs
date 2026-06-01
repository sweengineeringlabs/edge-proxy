//! Integration tests for the LifecycleMonitor trait.

use edge_proxy::{LifecycleMonitor, ProxySvc, HealthStatus};

/// @covers: LifecycleMonitor
#[test]
fn test_lifecycle_monitor_trait_is_object_safe() {
    fn _accept(_l: &dyn LifecycleMonitor) {}
}

/// @covers: LifecycleMonitor::health
#[tokio::test]
async fn test_lifecycle_monitor_health_returns_report() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    let report = m.health().await;
    assert_eq!(report.overall, HealthStatus::Healthy);
}

/// @covers: LifecycleMonitor::shutdown
#[tokio::test]
async fn test_lifecycle_monitor_shutdown_completes() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert!(m.shutdown().await.is_ok());
}
