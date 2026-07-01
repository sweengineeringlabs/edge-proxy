//! End-to-end contract tests for the `NullMonitor` marker trait, exercised through a
//! test-double implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_proxy::{
    ComponentRequest, ComponentResponse, HealthRequest, HealthResponse, HealthStatus,
    LifecycleError, LifecycleMonitor, NullMonitor, ShutdownRequest, StartBackgroundTasksRequest,
    StatusRequest, StatusResponse,
};
use futures::future::BoxFuture;

struct MonitorDouble;
impl LifecycleMonitor for MonitorDouble {
    fn health(&self, _req: HealthRequest) -> BoxFuture<'_, Result<HealthResponse, LifecycleError>> {
        Box::pin(async {
            Ok(HealthResponse {
                overall: HealthStatus::Healthy,
                components: vec![],
            })
        })
    }
    fn start_background_tasks(
        &self,
        _req: StartBackgroundTasksRequest,
    ) -> BoxFuture<'_, Result<(), LifecycleError>> {
        Box::pin(async { Ok(()) })
    }
    fn shutdown(&self, _req: ShutdownRequest) -> BoxFuture<'_, Result<(), LifecycleError>> {
        Box::pin(async { Ok(()) })
    }
    fn status(&self, _req: StatusRequest) -> BoxFuture<'_, Result<StatusResponse, LifecycleError>> {
        Box::pin(async {
            Ok(StatusResponse {
                status: HealthStatus::Healthy,
            })
        })
    }
    fn component<'a>(
        &'a self,
        _req: ComponentRequest<'_>,
    ) -> BoxFuture<'a, Result<ComponentResponse, LifecycleError>> {
        Box::pin(async { Ok(ComponentResponse { health: None }) })
    }
}
impl NullMonitor for MonitorDouble {}

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio")
}

/// @covers: NullMonitor
#[test]
fn test_monitor_double_reports_healthy_happy() {
    let m = MonitorDouble;
    assert_eq!(
        rt().block_on(m.health(HealthRequest)).unwrap().overall,
        HealthStatus::Healthy
    );
}

/// @covers: NullMonitor
#[test]
fn test_monitor_double_satisfies_bound_edge() {
    fn accepts<T: NullMonitor>(_: &T) -> bool {
        true
    }
    assert!(accepts(&MonitorDouble));
}
