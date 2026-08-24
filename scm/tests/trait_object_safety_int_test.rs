//! Integration tests verifying trait object safety for proxy concern traits.

use std::sync::Arc;

use edge_application_proxy::{Job, LifecycleMonitor, Router};

/// @covers: Job
#[test]
fn test_job_trait_is_object_safe() {
    // object-safe with concrete types, including the erased-router form every
    // `Arc<dyn Job<..>>` factory in this crate returns (SEA#8).
    fn _accept(_j: &dyn Job<String, String, Intent = String, Router = Arc<dyn Router<String>>>) {}
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
