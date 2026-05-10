//! `Monitor` — interface counterpart for the null lifecycle monitor impl.

use crate::api::lifecycle_monitor::LifecycleMonitor;

/// Marker trait for no-op lifecycle monitor implementations.
pub trait Monitor: LifecycleMonitor + Send + Sync {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_is_object_safe() {
        fn _assert(_: &dyn Monitor) {}
    }
}
