//! Integration tests for NullLifecycleMonitor marker trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_proxy::{HealthStatus, ProxySvc};

/// @covers: NullLifecycleMonitor
#[tokio::test]
async fn test_new_null_lifecycle_monitor_implements_null_lifecycle_monitor() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(m.health().await.overall, HealthStatus::Healthy);
}
