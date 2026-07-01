//! [`HealthResponse`] — response for [`LifecycleMonitor::health`](crate::api::lifecycle::traits::LifecycleMonitor::health).

use crate::api::lifecycle::types::{ComponentHealth, HealthStatus};

/// The aggregated health report.
///
/// `overall` summarizes the per-component results. Convention: if any
/// component is `Unhealthy`, overall is `Unhealthy`; else if any is
/// `Degraded`, overall is `Degraded`; else `Healthy`. Construct with
/// `HealthResponse::from_components`.
pub struct HealthResponse {
    /// Overall aggregated health status.
    pub overall: HealthStatus,
    /// Per-component health snapshots.
    pub components: Vec<ComponentHealth>,
}
