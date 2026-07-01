//! Integration tests verifying lifecycle_monitor_svc re-exports are reachable.

use edge_proxy::{ComponentHealth, HealthResponse, HealthStatus, LifecycleMonitor};

/// Verifies `LifecycleMonitor` is exported at the crate level.
#[test]
fn test_lifecycle_monitor_type_is_reachable_from_crate_boundary() {
    fn _accept(_l: &dyn LifecycleMonitor) {}
}

/// Verifies `HealthStatus` variants are accessible.
#[test]
fn test_health_status_variants_are_accessible() {
    assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
    assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
}

/// Verifies `HealthResponse` can be constructed with components.
#[test]
fn test_health_report_from_components_aggregates_status() {
    let components = vec![ComponentHealth::healthy("svc-a")];
    let report = HealthResponse::from_components(components);
    assert_eq!(report.overall, HealthStatus::Healthy);
}
