//! Integration tests for proxy health types.

use edge_proxy::{ComponentHealth, HealthResponse, HealthStatus};

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
    assert_eq!(c.id, "svc");
}

/// @covers: HealthResponse::from_components
#[test]
fn test_health_report_all_healthy_yields_healthy() {
    let report = HealthResponse::from_components(vec![
        ComponentHealth::healthy("a"),
        ComponentHealth::healthy("b"),
    ]);
    assert_eq!(report.overall, HealthStatus::Healthy);
}

/// @covers: HealthResponse::from_components
#[test]
fn test_health_report_any_unhealthy_wins() {
    let report = HealthResponse::from_components(vec![
        ComponentHealth::healthy("a"),
        ComponentHealth::with_status("b", HealthStatus::Degraded, "slow"),
        ComponentHealth::with_status("c", HealthStatus::Unhealthy, "dead"),
    ]);
    assert_eq!(report.overall, HealthStatus::Unhealthy);
}

/// @covers: HealthResponse::from_components
#[test]
fn test_health_report_degraded_without_unhealthy() {
    let report = HealthResponse::from_components(vec![
        ComponentHealth::healthy("a"),
        ComponentHealth::with_status("b", HealthStatus::Degraded, "slow"),
    ]);
    assert_eq!(report.overall, HealthStatus::Degraded);
}

/// @covers: HealthResponse::from_components
#[test]
fn test_health_report_empty_components_is_healthy() {
    let report = HealthResponse::from_components(vec![]);
    assert_eq!(report.overall, HealthStatus::Healthy);
}

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
