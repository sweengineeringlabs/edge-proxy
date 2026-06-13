//! Integration tests verifying monitor_svc re-exports are reachable.

use edge_proxy::NullMonitor;

/// Verifies `NullMonitor` (alias for Monitor trait) is exported at the crate level.
#[test]
fn test_null_monitor_type_is_reachable_from_crate_boundary() {
    fn _accept<T: NullMonitor>() {}
}
