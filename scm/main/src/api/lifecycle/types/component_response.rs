//! [`ComponentResponse`] — response for [`LifecycleMonitor::component`](crate::api::lifecycle::traits::LifecycleMonitor::component).

use crate::api::lifecycle::types::ComponentHealth;

/// The health snapshot for the requested component, if tracked.
pub struct ComponentResponse {
    /// The component's health snapshot, or `None` if not tracked.
    pub health: Option<ComponentHealth>,
}
