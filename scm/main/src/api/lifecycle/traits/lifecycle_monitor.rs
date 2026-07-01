//! LifecycleMonitor trait — **Lifecycle** concern of the 5-Concern Controller
//! pattern.
//!
//! Runtime state management: aggregate health, background tasks, and graceful
//! shutdown. Kept separate from `Job` so the operator surface (health, shutdown)
//! can be exposed without granting execution privileges.

use futures::future::BoxFuture;

use crate::api::lifecycle::errors::LifecycleError;
use crate::api::lifecycle::types::{
    ComponentRequest, ComponentResponse, HealthRequest, HealthResponse, ShutdownRequest,
    StartBackgroundTasksRequest, StatusRequest, StatusResponse,
};

/// Runtime lifecycle management for a Controller instance.
pub trait LifecycleMonitor: Send + Sync {
    /// Aggregate health across all handlers and subsystems.
    fn health(&self, req: HealthRequest) -> BoxFuture<'_, Result<HealthResponse, LifecycleError>>;

    /// Start any long-running background tasks (watchdogs, pollers).
    ///
    /// Idempotent: implementations must tolerate repeated calls by no-op'ing
    /// after the first successful start.
    fn start_background_tasks(
        &self,
        req: StartBackgroundTasksRequest,
    ) -> BoxFuture<'_, Result<(), LifecycleError>>;

    /// Gracefully shut down: stop accepting new work, drain in-flight work,
    /// release resources. After `shutdown` returns, `health` is expected to
    /// report a non-healthy status.
    fn shutdown(&self, req: ShutdownRequest) -> BoxFuture<'_, Result<(), LifecycleError>>;

    /// Return the overall health status without the full component breakdown.
    fn status(&self, req: StatusRequest) -> BoxFuture<'_, Result<StatusResponse, LifecycleError>>;

    /// Return the health snapshot for a named component, if tracked.
    fn component<'a>(
        &'a self,
        req: ComponentRequest<'_>,
    ) -> BoxFuture<'a, Result<ComponentResponse, LifecycleError>>;
}
