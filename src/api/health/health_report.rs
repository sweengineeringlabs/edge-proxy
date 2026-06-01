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
