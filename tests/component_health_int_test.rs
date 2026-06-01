//! Integration tests for ComponentHealth.

use edge_proxy::{ComponentHealth, HealthStatus};

/// @covers: ComponentHealth::healthy
#[test]
fn test_component_health_healthy_creates_healthy_component() {
    let c = ComponentHealth::healthy("svc");
    assert_eq!(c.status, HealthStatus::Healthy);
    assert!(c.message.is_none());
    assert_eq!(c.id, "svc");
}

/// @covers: ComponentHealth::with_status
#[test]
fn test_component_health_with_status_stores_message_and_status() {
    let c = ComponentHealth::with_status("svc", HealthStatus::Degraded, "slow response");
    assert_eq!(c.status, HealthStatus::Degraded);
    assert_eq!(c.message.as_deref(), Some("slow response"));
}
