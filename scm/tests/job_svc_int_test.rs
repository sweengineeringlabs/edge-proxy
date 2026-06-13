//! Integration tests verifying job_svc re-exports are reachable from the crate boundary.

use edge_proxy::{Job, JobError};

/// Verifies `Job` is exported at the crate level (compile-time check).
#[test]
fn test_job_type_is_reachable_from_crate_boundary() {
    fn _assert_job_import<T: Job<String, String>>() {}
}

/// Verifies `JobError::Cancelled` variant is accessible.
#[test]
fn test_job_error_cancelled_variant_is_accessible() {
    let e = JobError::Cancelled;
    assert!(matches!(e, JobError::Cancelled));
}

/// Verifies `JobError::HandlerUnavailable` carries a message.
#[test]
fn test_job_error_handler_unavailable_carries_message() {
    let e = JobError::HandlerUnavailable("test-handler".into());
    let msg = format!("{e}");
    assert!(msg.contains("test-handler"));
}
