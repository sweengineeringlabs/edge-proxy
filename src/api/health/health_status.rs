//! HealthStatus — terminal health classification.

use serde::{Deserialize, Serialize};

/// Terminal health classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// All subsystems responsive and within operating parameters.
    Healthy,
    /// One or more subsystems impaired but the Controller can still serve.
    Degraded,
    /// Cannot serve requests; operator intervention expected.
    Unhealthy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
    }

    #[test]
    fn test_health_status_is_copy() {
        let s = HealthStatus::Degraded;
        let s2 = s;
        assert_eq!(s, s2);
    }
}
