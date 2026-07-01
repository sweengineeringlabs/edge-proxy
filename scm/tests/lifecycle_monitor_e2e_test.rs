//! End-to-end contract tests for the `LifecycleMonitor` trait, exercised through the
//! crate's canonical null implementation via the public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_proxy::{
    ComponentRequest, HealthRequest, HealthStatus, ProxySvc, ShutdownRequest,
    StartBackgroundTasksRequest, StatusRequest,
};

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio")
}

/// @covers: LifecycleMonitor::health
#[test]
fn test_health_fresh_monitor_is_healthy_happy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(
        rt().block_on(m.health(HealthRequest)).unwrap().overall,
        HealthStatus::Healthy
    );
}

/// @covers: LifecycleMonitor::health
#[test]
fn test_health_after_shutdown_is_unhealthy_error() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m.shutdown(ShutdownRequest))
        .expect("shutdown");
    assert_eq!(
        rt().block_on(m.health(HealthRequest)).unwrap().overall,
        HealthStatus::Unhealthy
    );
}

/// @covers: LifecycleMonitor::start_background_tasks
#[test]
fn test_start_background_tasks_completes_edge() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(
        rt().block_on(m.start_background_tasks(StartBackgroundTasksRequest)),
        Ok(())
    );
}

/// @covers: LifecycleMonitor::shutdown
#[test]
fn test_shutdown_first_call_ok_happy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(rt().block_on(m.shutdown(ShutdownRequest)), Ok(()));
}

/// @covers: LifecycleMonitor::shutdown
#[test]
fn test_shutdown_second_call_already_shut_down_error() {
    use edge_proxy::LifecycleError;
    let m = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m.shutdown(ShutdownRequest))
        .expect("first ok");
    assert!(matches!(
        rt().block_on(m.shutdown(ShutdownRequest)),
        Err(LifecycleError::AlreadyShutDown)
    ));
}

/// @covers: LifecycleMonitor::status
#[test]
fn test_status_fresh_monitor_is_healthy_happy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(
        rt().block_on(m.status(StatusRequest)).unwrap().status,
        HealthStatus::Healthy
    );
}

/// @covers: LifecycleMonitor::component
#[test]
fn test_component_unknown_id_returns_none_edge() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert!(rt()
        .block_on(m.component(ComponentRequest { id: "unknown" }))
        .unwrap()
        .health
        .is_none());
}
