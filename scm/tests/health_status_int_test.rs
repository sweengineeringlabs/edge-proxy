//! Integration tests for HealthStatus.

use edge_proxy::{ComponentHealth, HealthStatus};

/// @covers: HealthStatus
#[test]
fn test_health_status_equality() {
    let healthy = ComponentHealth::healthy("svc").status;
    assert_eq!(healthy, HealthStatus::Healthy);
    assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
    assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
}

/// @covers: HealthStatus
#[test]
fn test_health_status_is_copy() {
    let s = HealthStatus::Degraded;
    let s2 = s;
    assert_eq!(s, s2);
}
