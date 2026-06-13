//! Integration tests verifying null_lifecycle_monitor_svc re-exports are reachable.

use edge_proxy::NullLifecycleMonitor;

/// Verifies `NullLifecycleMonitor` trait is exported at the crate level.
#[test]
fn test_null_lifecycle_monitor_trait_is_reachable_from_crate_boundary() {
    fn _accept<T: NullLifecycleMonitor>() {}
}
