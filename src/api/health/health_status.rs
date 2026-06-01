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
