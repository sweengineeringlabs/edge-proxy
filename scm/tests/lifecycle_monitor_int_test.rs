//! Integration tests for the LifecycleMonitor trait.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use edge_proxy::{
    ComponentRequest, HealthRequest, HealthStatus, LifecycleMonitor, ProxySvc, ShutdownRequest,
    StartBackgroundTasksRequest, StatusRequest,
};

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

#[test]
fn test_lifecycle_monitor_trait_is_object_safe() {
    fn _accept(_l: &dyn LifecycleMonitor) {}
}

#[test]
fn test_lifecycle_monitor_health_returns_report() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    let response = rt().block_on(m.health(HealthRequest)).unwrap();
    assert_eq!(response.overall, HealthStatus::Healthy);
}

#[test]
fn test_lifecycle_monitor_shutdown_completes() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(rt().block_on(m.shutdown(ShutdownRequest)), Ok(()));
}

// Rule 222 scenario coverage for LifecycleMonitor trait fns ───────────────────

/// health — happy: fresh monitor reports Healthy.
#[test]
fn test_health_fresh_monitor_reports_healthy_happy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(
        rt().block_on(m.health(HealthRequest)).unwrap().overall,
        HealthStatus::Healthy
    );
}

/// health — error: after shutdown the monitor reports Unhealthy.
#[test]
fn test_health_after_shutdown_reports_unhealthy_error() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m.shutdown(ShutdownRequest))
        .expect("shutdown");
    assert_eq!(
        rt().block_on(m.health(HealthRequest)).unwrap().overall,
        HealthStatus::Unhealthy
    );
}

/// health — edge: components list is empty for the null monitor.
#[test]
fn test_health_with_no_components_returns_empty_list_edge() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert!(rt()
        .block_on(m.health(HealthRequest))
        .unwrap()
        .components
        .is_empty());
}

/// start_background_tasks — happy: completes without error.
#[test]
fn test_start_background_tasks_completes_without_error_happy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m.start_background_tasks(StartBackgroundTasksRequest))
        .unwrap();
    assert_eq!(
        rt().block_on(m.health(HealthRequest)).unwrap().overall,
        HealthStatus::Healthy
    );
}

/// start_background_tasks — error: calling after shutdown still completes.
#[test]
fn test_start_background_tasks_after_shutdown_still_completes_error() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m.shutdown(ShutdownRequest))
        .expect("shutdown");
    rt().block_on(m.start_background_tasks(StartBackgroundTasksRequest))
        .unwrap();
    assert_eq!(
        rt().block_on(m.health(HealthRequest)).unwrap().overall,
        HealthStatus::Unhealthy
    );
}

/// start_background_tasks — edge: idempotent — calling twice has no effect.
#[test]
fn test_start_background_tasks_called_twice_is_idempotent_edge() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m.start_background_tasks(StartBackgroundTasksRequest))
        .unwrap();
    rt().block_on(m.start_background_tasks(StartBackgroundTasksRequest))
        .unwrap();
    assert_eq!(
        rt().block_on(m.health(HealthRequest)).unwrap().overall,
        HealthStatus::Healthy
    );
}

/// shutdown — happy: first call returns Ok.
#[test]
fn test_shutdown_first_call_succeeds_happy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(rt().block_on(m.shutdown(ShutdownRequest)), Ok(()));
}

/// shutdown — error: second call returns AlreadyShutDown.
#[test]
fn test_shutdown_second_call_returns_already_shut_down_error() {
    use edge_proxy::LifecycleError;
    let m = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m.shutdown(ShutdownRequest))
        .expect("first ok");
    assert!(matches!(
        rt().block_on(m.shutdown(ShutdownRequest)),
        Err(LifecycleError::AlreadyShutDown)
    ));
}

/// shutdown — edge: shutdown followed by health shows Unhealthy.
#[test]
fn test_shutdown_then_health_reports_unhealthy_edge() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m.shutdown(ShutdownRequest))
        .expect("shutdown");
    assert_eq!(
        rt().block_on(m.health(HealthRequest)).unwrap().overall,
        HealthStatus::Unhealthy
    );
}

/// status — happy: fresh monitor status is Healthy.
#[test]
fn test_status_fresh_monitor_is_healthy_happy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(
        rt().block_on(m.status(StatusRequest)).unwrap().status,
        HealthStatus::Healthy
    );
}

/// status — error: after shutdown status is Unhealthy.
#[test]
fn test_status_after_shutdown_is_unhealthy_error() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m.shutdown(ShutdownRequest))
        .expect("shutdown");
    assert_eq!(
        rt().block_on(m.status(StatusRequest)).unwrap().status,
        HealthStatus::Unhealthy
    );
}

/// status — edge: status matches health.overall for the null monitor.
#[test]
fn test_status_matches_health_overall_edge() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    let report = rt().block_on(m.health(HealthRequest)).unwrap();
    let status = rt().block_on(m.status(StatusRequest)).unwrap().status;
    assert_eq!(status, report.overall);
}

/// component — happy: unknown id returns None.
#[test]
fn test_component_unknown_id_returns_none_happy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert!(rt()
        .block_on(m.component(ComponentRequest { id: "any-id" }))
        .unwrap()
        .health
        .is_none());
}

/// component — error: after shutdown, unknown id still returns None.
#[test]
fn test_component_after_shutdown_returns_none_error() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m.shutdown(ShutdownRequest))
        .expect("shutdown");
    assert!(rt()
        .block_on(m.component(ComponentRequest { id: "any-id" }))
        .unwrap()
        .health
        .is_none());
}

/// component — edge: empty id returns None.
#[test]
fn test_component_with_empty_id_returns_none_edge() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert!(rt()
        .block_on(m.component(ComponentRequest { id: "" }))
        .unwrap()
        .health
        .is_none());
}
