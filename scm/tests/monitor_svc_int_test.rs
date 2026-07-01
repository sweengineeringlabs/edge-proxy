//! Integration tests verifying monitor_svc re-exports are reachable.

use edge_proxy::{
    ComponentHealth, ComponentRequest, ComponentResponse, HealthRequest, HealthResponse,
    HealthStatus, LifecycleError, LifecycleMonitor, NullMonitor, ShutdownRequest,
    StartBackgroundTasksRequest, StatusRequest, StatusResponse,
};
use futures::future::BoxFuture;

struct MonitorDouble;

impl LifecycleMonitor for MonitorDouble {
    fn health(&self, _req: HealthRequest) -> BoxFuture<'_, Result<HealthResponse, LifecycleError>> {
        Box::pin(async {
            Ok(HealthResponse {
                overall: HealthStatus::Healthy,
                components: Vec::<ComponentHealth>::new(),
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

/// Verifies `NullMonitor` (alias for Monitor trait) is exported at the crate level.
#[test]
fn test_null_monitor_type_is_reachable_from_crate_boundary() {
    fn accepts_null_monitor<T: NullMonitor>(_: &T) -> bool {
        true
    }
    assert!(accepts_null_monitor(&MonitorDouble));
}
