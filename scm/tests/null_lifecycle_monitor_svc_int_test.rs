//! Integration tests verifying null_lifecycle_monitor_svc re-exports are reachable.

use async_trait::async_trait;
use edge_proxy::{
    ComponentHealth, ComponentRequest, EmptyResponse, HealthRequest, HealthResponse, HealthStatus,
    LifecycleError, LifecycleMonitor, NullLifecycleMonitor, ShutdownRequest,
    StartBackgroundTasksRequest, StatusRequest,
};

struct LifecycleMonitorDouble;

#[async_trait]
impl LifecycleMonitor for LifecycleMonitorDouble {
    async fn health(&self, _req: HealthRequest) -> Result<HealthResponse, LifecycleError> {
        Ok(HealthResponse {
            overall: HealthStatus::Healthy,
            components: Vec::<ComponentHealth>::new(),
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

impl NullLifecycleMonitor for LifecycleMonitorDouble {}

/// Verifies `NullLifecycleMonitor` trait is exported at the crate level.
#[test]
fn test_null_lifecycle_monitor_trait_is_reachable_from_crate_boundary() {
    fn accepts_null_lifecycle_monitor<T: NullLifecycleMonitor>(_: &T) -> bool {
        true
    }
    assert!(accepts_null_lifecycle_monitor(&LifecycleMonitorDouble));
}
