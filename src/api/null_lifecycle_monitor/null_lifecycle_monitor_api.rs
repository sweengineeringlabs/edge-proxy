//! `NullLifecycleMonitorApi` — marker supertrait for no-op lifecycle monitors.

use crate::api::lifecycle_monitor::LifecycleMonitor;

/// Marker supertrait for no-op [`LifecycleMonitor`] implementations.
#[allow(dead_code)]
pub trait NullLifecycleMonitorApi: LifecycleMonitor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_lifecycle_monitor_api_is_object_safe() {
        fn _assert(_: &dyn NullLifecycleMonitorApi) {}
    }
}
