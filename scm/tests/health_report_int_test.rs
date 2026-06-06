//! Integration tests for HealthReport.

use edge_proxy::{ComponentHealth, HealthReport, HealthStatus};

/// @covers: HealthReport::from_components
#[test]
fn test_health_report_all_healthy_yields_healthy() {
    let report = HealthReport::from_components(vec![
        ComponentHealth::healthy("a"),
        ComponentHealth::healthy("b"),
    ]);
    assert_eq!(report.overall, HealthStatus::Healthy);
}

/// @covers: HealthReport::from_components
#[test]
fn test_health_report_any_unhealthy_wins() {
    let report = HealthReport::from_components(vec![
        ComponentHealth::healthy("a"),
        ComponentHealth::with_status("b", HealthStatus::Unhealthy, "dead"),
    ]);
    assert_eq!(report.overall, HealthStatus::Unhealthy);
}

/// @covers: HealthReport::from_components
#[test]
fn test_health_report_degraded_without_unhealthy() {
    let report = HealthReport::from_components(vec![
        ComponentHealth::healthy("a"),
        ComponentHealth::with_status("b", HealthStatus::Degraded, "slow"),
    ]);
    assert_eq!(report.overall, HealthStatus::Degraded);
}

/// @covers: HealthReport::from_components
#[test]
fn test_health_report_empty_components_is_healthy() {
    let report = HealthReport::from_components(vec![]);
    assert_eq!(report.overall, HealthStatus::Healthy);
}
