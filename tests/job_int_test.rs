//! Integration tests for the Job trait.

use edge_proxy::Job;

/// @covers: Job
#[test]
fn test_job_trait_is_object_safe() {
    fn _accept(_j: &dyn Job<String, String>) {}
}
