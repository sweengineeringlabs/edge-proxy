//! HealthReport — aggregated health report returned by `LifecycleMonitor::health`.

use serde::{Deserialize, Serialize};

use super::component_health::ComponentHealth;
use super::health_status::HealthStatus;

/// Aggregated health report returned by `LifecycleMonitor::health`.
///
/// `overall` summarizes the per-component results. Convention: if any
/// component is `Unhealthy`, overall is `Unhealthy`; else if any is
/// `Degraded`, overall is `Degraded`; else `Healthy`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub overall: HealthStatus,
    pub components: Vec<ComponentHealth>,
}

impl HealthReport {
    /// Derive `overall` from the component list per the aggregation rule.
    pub fn from_components(components: Vec<ComponentHealth>) -> Self {
        let overall = if components
            .iter()
            .any(|c| c.status == HealthStatus::Unhealthy)
        {
            HealthStatus::Unhealthy
        } else if components
            .iter()
            .any(|c| c.status == HealthStatus::Degraded)
        {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };
        Self {
            overall,
            components,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: from_components
    #[test]
    fn test_from_components_all_healthy_yields_healthy() {
        let report = HealthReport::from_components(vec![
            ComponentHealth::healthy("a"),
            ComponentHealth::healthy("b"),
        ]);
        assert_eq!(report.overall, HealthStatus::Healthy);
    }

    /// @covers: from_components
    #[test]
    fn test_from_components_any_unhealthy_wins() {
        let report = HealthReport::from_components(vec![
            ComponentHealth::healthy("a"),
            ComponentHealth::with_status("b", HealthStatus::Degraded, "slow"),
            ComponentHealth::with_status("c", HealthStatus::Unhealthy, "dead"),
        ]);
        assert_eq!(report.overall, HealthStatus::Unhealthy);
    }

    /// @covers: from_components
    #[test]
    fn test_from_components_degraded_without_unhealthy() {
        let report = HealthReport::from_components(vec![
            ComponentHealth::healthy("a"),
            ComponentHealth::with_status("b", HealthStatus::Degraded, "slow"),
        ]);
        assert_eq!(report.overall, HealthStatus::Degraded);
    }

    /// @covers: from_components
    #[test]
    fn test_from_components_empty_is_healthy() {
        let report = HealthReport::from_components(vec![]);
        assert_eq!(report.overall, HealthStatus::Healthy);
    }
}
