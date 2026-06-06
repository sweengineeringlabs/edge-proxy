//! Integration tests for LifecycleError.

use edge_proxy::LifecycleError;

/// @covers: LifecycleError::DrainFailed
#[test]
fn test_lifecycle_error_drain_failed_display() {
    let err = LifecycleError::DrainFailed("worker A".to_string());
    assert!(err.to_string().contains("worker A"));
}

/// @covers: LifecycleError::AlreadyShutDown
#[test]
fn test_lifecycle_error_already_shut_down_display() {
    let err = LifecycleError::AlreadyShutDown;
    assert!(err.to_string().contains("shut down"));
}

/// @covers: LifecycleError::Internal
#[test]
fn test_lifecycle_error_internal_display() {
    let err = LifecycleError::Internal("timeout".to_string());
    assert!(err.to_string().contains("timeout"));
}
