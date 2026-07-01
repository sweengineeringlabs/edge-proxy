//! `NoopLifecycleMonitor` — a no-op `LifecycleMonitor` useful for tests and
//! for bootstrapping a Controller that does not (yet) need real lifecycle
//! management.
//!
//! Always reports `Healthy`. `start_background_tasks` does nothing.
//! `shutdown` returns `Ok(())` on the first call and `AlreadyShutDown` on
//! subsequent calls.

use futures::future::BoxFuture;
use parking_lot::Mutex;

use crate::api::{
    ComponentRequest, ComponentResponse, HealthRequest, HealthResponse, HealthStatus,
    LifecycleError, LifecycleMonitor, Monitor, ShutdownRequest, StartBackgroundTasksRequest,
    StatusRequest, StatusResponse,
};

/// No-op lifecycle monitor suitable for tests and early bring-up.
///
/// `pub(crate)` — consumers obtain an instance through
/// [`crate::saf::ProxySvc::new_null_lifecycle_monitor`] rather than naming the
/// type directly, keeping the impl detail behind the trait contract.
pub(crate) struct NoopLifecycleMonitor {
    shut_down: Mutex<bool>,
}

impl NoopLifecycleMonitor {
    /// Construct a fresh no-op monitor in the "running" state.
    pub(crate) fn new() -> Self {
        Self {
            shut_down: Mutex::new(false),
        }
    }
}

impl Default for NoopLifecycleMonitor {
    fn default() -> Self {
        Self::new()
    }
}

// Use full path to avoid shadowing the local struct with a same-named trait import.
impl crate::api::NullLifecycleMonitor for NoopLifecycleMonitor {}
impl Monitor for NoopLifecycleMonitor {}

impl LifecycleMonitor for NoopLifecycleMonitor {
    fn health(&self, _req: HealthRequest) -> BoxFuture<'_, Result<HealthResponse, LifecycleError>> {
        Box::pin(async move {
            let status = if *self.shut_down.lock() {
                HealthStatus::Unhealthy
            } else {
                HealthStatus::Healthy
            };
            Ok(HealthResponse {
                overall: status,
                components: Vec::new(),
            })
        })
    }

    fn start_background_tasks(
        &self,
        _req: StartBackgroundTasksRequest,
    ) -> BoxFuture<'_, Result<(), LifecycleError>> {
        Box::pin(async {
            // Intentionally empty — nothing to start.
            Ok(())
        })
    }

    fn shutdown(&self, _req: ShutdownRequest) -> BoxFuture<'_, Result<(), LifecycleError>> {
        Box::pin(async move {
            let mut flag = self.shut_down.lock();
            if *flag {
                return Err(LifecycleError::AlreadyShutDown);
            }
            *flag = true;
            Ok(())
        })
    }

    fn status(&self, _req: StatusRequest) -> BoxFuture<'_, Result<StatusResponse, LifecycleError>> {
        Box::pin(async move {
            let status = if *self.shut_down.lock() {
                HealthStatus::Unhealthy
            } else {
                HealthStatus::Healthy
            };
            Ok(StatusResponse { status })
        })
    }

    fn component<'a>(
        &'a self,
        _req: ComponentRequest<'_>,
    ) -> BoxFuture<'a, Result<ComponentResponse, LifecycleError>> {
        Box::pin(async move { Ok(ComponentResponse { health: None }) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_monitor_in_running_state() {
        let m = NoopLifecycleMonitor::new();
        assert!(!*m.shut_down.lock());
    }

    #[tokio::test]
    async fn test_starts_healthy() {
        let m = NoopLifecycleMonitor::new();
        assert_eq!(
            m.health(HealthRequest).await.unwrap().overall,
            HealthStatus::Healthy
        );
    }

    #[tokio::test]
    async fn test_shutdown_flips_to_unhealthy() {
        let m = NoopLifecycleMonitor::new();
        m.shutdown(ShutdownRequest)
            .await
            .expect("first shutdown ok");
        assert_eq!(
            m.health(HealthRequest).await.unwrap().overall,
            HealthStatus::Unhealthy
        );
    }

    #[tokio::test]
    async fn test_shutdown_is_single_use() {
        let m = NoopLifecycleMonitor::new();
        assert!(m.shutdown(ShutdownRequest).await.is_ok());
        match m.shutdown(ShutdownRequest).await {
            Err(LifecycleError::AlreadyShutDown) => {}
            other => panic!("expected AlreadyShutDown, got {:?}", other),
        }
    }
}
