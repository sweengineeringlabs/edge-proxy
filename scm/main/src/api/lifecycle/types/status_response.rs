//! [`StatusResponse`] — response for [`LifecycleMonitor::status`](crate::api::lifecycle::traits::LifecycleMonitor::status).

use crate::api::lifecycle::types::HealthStatus;

/// The overall health status without the full component breakdown.
pub struct StatusResponse {
    /// The overall health status.
    pub status: HealthStatus,
}
