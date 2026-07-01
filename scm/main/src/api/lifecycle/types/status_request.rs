//! [`StatusRequest`] — request for [`LifecycleMonitor::status`](crate::api::lifecycle::traits::LifecycleMonitor::status).

/// Request to return the overall health status without the full component breakdown.
pub struct StatusRequest;
