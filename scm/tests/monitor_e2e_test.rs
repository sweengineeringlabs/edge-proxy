//! End-to-end contract tests for the `NullMonitor` marker trait, exercised through a
//! test-double implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use async_trait::async_trait;
use edge_proxy::{
    ComponentHealth, ComponentRequest, EmptyResponse, HealthRequest, HealthResponse, HealthStatus,
    LifecycleError, LifecycleMonitor, NullMonitor, ShutdownRequest, StartBackgroundTasksRequest,
    StatusRequest,
};

struct MonitorDouble;

#[async_trait]
impl LifecycleMonitor for MonitorDouble {
    async fn health(&self, _req: HealthRequest) -> Result<HealthResponse, LifecycleError> {
        Ok(HealthResponse {
            overall: HealthStatus::Healthy,
            components: vec![],
        })
    }
    async fn start_background_tasks(
        &self,
        _req: StartBackgroundTasksRequest,
    ) -> Result<(), LifecycleError> {
        Ok(())
    }
    async fn shutdown(&self, _req: ShutdownRequest) -> Result<(), LifecycleError> {
        Ok(())
    }
    async fn status(
        &self,
        _req: StatusRequest,
    ) -> Result<EmptyResponse<HealthStatus>, LifecycleError> {
        Ok(EmptyResponse {
            value: HealthStatus::Healthy,
        })
    }
    async fn component(
        &self,
        _req: ComponentRequest<'_>,
    ) -> Result<EmptyResponse<Option<ComponentHealth>>, LifecycleError> {
        Ok(EmptyResponse { value: None })
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
