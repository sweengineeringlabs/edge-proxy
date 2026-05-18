//! `NullLifecycleMonitor` — a no-op `LifecycleMonitor` useful for tests and
//! for bootstrapping a Controller that does not (yet) need real lifecycle
//! management.
//!
//! Always reports `Healthy`. `start_background_tasks` does nothing.
//! `shutdown` returns `Ok(())` on the first call and `AlreadyShutDown` on
//! subsequent calls.

use futures::future::BoxFuture;
use parking_lot::Mutex;

use crate::api::error::LifecycleError;
use crate::api::health::{HealthReport, HealthStatus};
use crate::api::lifecycle_monitor::LifecycleMonitor;
use crate::api::null_lifecycle_monitor::NullLifecycleMonitorApi;

/// No-op lifecycle monitor suitable for tests and early bring-up.
///
/// `pub(crate)` — consumers obtain an instance through
/// [`crate::saf::new_null_lifecycle_monitor`] rather than naming the type
/// directly, keeping the impl detail behind the trait contract.
pub(crate) struct NullLifecycleMonitor {
    shut_down: Mutex<bool>,
}

impl NullLifecycleMonitor {
    /// Construct a fresh no-op monitor in the "running" state.
    pub(crate) fn new() -> Self {
        Self {
            shut_down: Mutex::new(false),
        }
    }
}

impl Default for NullLifecycleMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl NullLifecycleMonitorApi for NullLifecycleMonitor {}
impl crate::api::null_lifecycle_monitor::Monitor for NullLifecycleMonitor {}

impl LifecycleMonitor for NullLifecycleMonitor {
    fn health(&self) -> BoxFuture<'_, HealthReport> {
        Box::pin(async move {
            let status = if *self.shut_down.lock() {
                HealthStatus::Unhealthy
            } else {
                HealthStatus::Healthy
            };
            HealthReport {
                overall: status,
                components: Vec::new(),
            }
        })
    }

    fn start_background_tasks(&self) -> BoxFuture<'_, ()> {
        Box::pin(async {
            // Intentionally empty — nothing to start.
        })
    }

    fn shutdown(&self) -> BoxFuture<'_, Result<(), LifecycleError>> {
        Box::pin(async move {
            let mut flag = self.shut_down.lock();
            if *flag {
                return Err(LifecycleError::AlreadyShutDown);
            }
            *flag = true;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_monitor_in_running_state() {
        let m = NullLifecycleMonitor::new();
        // New monitor starts in "not shut down" state
        assert!(!*m.shut_down.lock());
    }

    #[tokio::test]
    async fn test_starts_healthy() {
        let m = NullLifecycleMonitor::new();
        assert_eq!(m.health().await.overall, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_shutdown_flips_to_unhealthy() {
        let m = NullLifecycleMonitor::new();
        m.shutdown().await.unwrap();
        assert_eq!(m.health().await.overall, HealthStatus::Unhealthy);
    }

    #[tokio::test]
    async fn test_shutdown_is_single_use() {
        let m = NullLifecycleMonitor::new();
        assert!(m.shutdown().await.is_ok());
        match m.shutdown().await {
            Err(LifecycleError::AlreadyShutDown) => {}
            other => panic!("expected AlreadyShutDown, got {:?}", other),
        }
    }
}
