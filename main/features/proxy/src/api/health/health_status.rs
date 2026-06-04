//! HealthStatus — terminal health classification.

use serde::{Deserialize, Serialize};

/// Terminal health classification.
///
/// Returned by health-check endpoints and lifecycle monitors. Used to decide
/// whether to route traffic to a pod: `Healthy` → route, `Degraded` → route
/// with caution, `Unhealthy` → remove from load balancer.
///
/// # Examples
///
/// ```rust
/// use edge_proxy::HealthStatus;
///
/// assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
/// assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
///
/// let status = HealthStatus::Degraded;
/// let should_route = !matches!(status, HealthStatus::Unhealthy);
/// assert!(should_route);
/// ```
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
