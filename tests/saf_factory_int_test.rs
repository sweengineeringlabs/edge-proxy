//! Public-API integration tests for saf/factory.rs.

use edge_proxy::{new_null_lifecycle_monitor, HealthStatus};

/// @covers: new_null_lifecycle_monitor
#[tokio::test]
async fn test_new_null_lifecycle_monitor_reports_healthy() {
    let m = new_null_lifecycle_monitor();
    assert_eq!(m.health().await.overall, HealthStatus::Healthy);
}

/// @covers: new_null_lifecycle_monitor
#[tokio::test]
async fn test_new_null_lifecycle_monitor_shuts_down_cleanly() {
    let m = new_null_lifecycle_monitor();
    m.shutdown().await.expect("first shutdown must succeed");
}
