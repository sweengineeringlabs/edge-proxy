//! [`ComponentRequest`] — request for [`LifecycleMonitor::component`](crate::api::lifecycle::traits::LifecycleMonitor::component).

/// Request for the health snapshot of a named component.
pub struct ComponentRequest<'a> {
    /// The component id to look up.
    pub id: &'a str,
}
