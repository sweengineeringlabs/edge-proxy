//! Integration tests for null lifecycle monitor — exercises edge-domain dep.

use edge_domain::HandlerError;
use edge_proxy::{new_null_lifecycle_monitor, HealthStatus, JobError};

#[tokio::test]
async fn test_null_lifecycle_monitor_health_is_healthy() {
    let monitor = new_null_lifecycle_monitor();
    let report = monitor.health().await;
    assert_eq!(report.overall, HealthStatus::Healthy);
}

#[test]
fn test_handler_error_unhealthy_wraps_into_job_error() {
    let err: JobError = HandlerError::Unhealthy.into();
    assert!(matches!(err, JobError::Handler(HandlerError::Unhealthy)));
}

#[test]
fn test_handler_error_invalid_request_wraps_into_job_error() {
    let err: JobError = HandlerError::InvalidRequest("bad input".into()).into();
    assert!(matches!(
        err,
        JobError::Handler(HandlerError::InvalidRequest(_))
    ));
}
