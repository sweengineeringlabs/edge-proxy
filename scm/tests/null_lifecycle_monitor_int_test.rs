//! Integration tests for null lifecycle monitor — exercises edge-application dep.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::HandlerError;
use edge_proxy::{HealthRequest, HealthStatus, JobError, ProxySvc};

#[tokio::test]
async fn test_null_lifecycle_monitor_health_is_healthy() {
    let monitor = ProxySvc::new_null_lifecycle_monitor();
    let response = monitor.health(HealthRequest).await.unwrap();
    assert_eq!(response.overall, HealthStatus::Healthy);
}

#[test]
fn test_handler_error_unhealthy_wraps_into_job_error() {
    let err: JobError = HandlerError::Unhealthy.into();
    // `Handler` carries the `HandlerError` verbatim, not a formatted string.
    assert!(matches!(err, JobError::Handler(HandlerError::Unhealthy)));
}

#[test]
fn test_handler_error_invalid_request_wraps_into_job_error() {
    let err: JobError = HandlerError::InvalidRequest("bad input".into()).into();
    match err {
        JobError::Handler(HandlerError::InvalidRequest(msg)) => assert_eq!(msg, "bad input"),
        other => panic!("expected JobError::Handler(InvalidRequest), got {other:?}"),
    }
}
