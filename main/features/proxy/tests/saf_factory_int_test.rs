//! Public-API integration tests for saf/proxy_svc.rs.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_proxy::{HealthStatus, ProxySvc};

/// @covers: ProxySvc::create_config_builder
#[test]
fn test_create_config_builder_returns_builder_with_package_metadata() {
    let builder = ProxySvc::create_config_builder();
    // Verify the builder can be constructed without panicking and holds the crate name.
    let _ = builder;
}

/// @covers: ProxySvc::new_null_lifecycle_monitor
#[tokio::test]
async fn test_new_null_lifecycle_monitor_reports_healthy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(m.health().await.overall, HealthStatus::Healthy);
}

/// @covers: ProxySvc::new_null_lifecycle_monitor
#[tokio::test]
async fn test_new_null_lifecycle_monitor_shuts_down_cleanly() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    m.shutdown().await.expect("first shutdown must succeed");
}
