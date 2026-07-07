//! Integration tests for HealthResponse::from_components.

use edge_proxy::{ComponentHealth, HealthResponse, HealthStatus};

/// @covers: from_components
#[test]
fn test_from_components_all_healthy_happy() {
    let r = HealthResponse::from_components(vec![
        ComponentHealth::healthy("a"),
        ComponentHealth::healthy("b"),
    ]);
    assert_eq!(r.overall, HealthStatus::Healthy);
    assert_eq!(r.components.len(), 2);
}

/// @covers: from_components
#[test]
fn test_from_components_any_unhealthy_wins_error() {
    let r = HealthResponse::from_components(vec![
        ComponentHealth::healthy("a"),
        ComponentHealth::with_status("b", HealthStatus::Degraded, "slow"),
        ComponentHealth::with_status("c", HealthStatus::Unhealthy, "down"),
    ]);
    assert_eq!(r.overall, HealthStatus::Unhealthy);
}

/// @covers: from_components
#[test]
fn test_from_components_empty_list_is_healthy_edge() {
    let r = HealthResponse::from_components(vec![]);
    assert_eq!(r.overall, HealthStatus::Healthy);
    assert!(r.components.is_empty());
}
