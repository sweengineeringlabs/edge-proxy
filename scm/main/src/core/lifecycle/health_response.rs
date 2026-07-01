//! Constructors for [`HealthResponse`].

use crate::api::{ComponentHealth, HealthResponse, HealthStatus};

impl HealthResponse {
    /// Derive `overall` from the component list per the aggregation rule.
    pub fn from_components(components: Vec<ComponentHealth>) -> Self {
        let overall = Self::aggregate_status(&components);
        Self {
            overall,
            components,
        }
    }

    /// Worst-status-wins aggregation: any `Unhealthy` beats any `Degraded` beats `Healthy`.
    fn aggregate_status(components: &[ComponentHealth]) -> HealthStatus {
        if components
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: from_components
    #[test]
    fn test_from_components_any_unhealthy_wins() {
        let r = HealthResponse::from_components(vec![
            ComponentHealth::healthy("a"),
            ComponentHealth::with_status("b", HealthStatus::Unhealthy, "dead"),
        ]);
        assert_eq!(r.overall, HealthStatus::Unhealthy);
    }

    /// @covers: aggregate_status
    #[test]
    fn test_aggregate_status_any_unhealthy_wins() {
        let components = vec![
            ComponentHealth::healthy("a"),
            ComponentHealth::with_status("b", HealthStatus::Unhealthy, "dead"),
            ComponentHealth::with_status("c", HealthStatus::Degraded, "slow"),
        ];
        assert_eq!(
            HealthResponse::aggregate_status(&components),
            HealthStatus::Unhealthy
        );
    }
}
