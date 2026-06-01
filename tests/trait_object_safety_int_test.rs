//! Integration tests verifying trait object safety for proxy concern traits.

use edge_proxy::{Job, LifecycleMonitor, Router};

/// @covers: Job
#[test]
fn test_job_trait_is_object_safe() {
    fn _accept(_j: &dyn Job<String, String>) {} // object-safe with concrete types
}

/// @covers: LifecycleMonitor
#[test]
fn test_lifecycle_monitor_trait_is_object_safe() {
    fn _accept(_l: &dyn LifecycleMonitor) {}
}

/// @covers: Router
#[test]
fn test_router_trait_is_object_safe() {
    fn _accept(_r: &dyn Router<String>) {}
}
