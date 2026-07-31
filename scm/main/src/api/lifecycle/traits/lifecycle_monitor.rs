//! LifecycleMonitor trait — **Lifecycle** concern of the 5-Concern Controller
//! pattern.
//!
//! Runtime state management: aggregate health, background tasks, and graceful
//! shutdown. Kept separate from `Job` so the operator surface (health, shutdown)
//! can be exposed without granting execution privileges.

use async_trait::async_trait;

use crate::api::lifecycle::dto::{
    ComponentRequest, HealthRequest, HealthResponse, ShutdownRequest, StartBackgroundTasksRequest,
    StatusRequest,
};
use crate::api::lifecycle::errors::LifecycleError;
use crate::api::lifecycle::vo::{ComponentHealth, HealthStatus};
use crate::api::types::EmptyResponse;

/// Runtime lifecycle management for a Controller instance.
#[async_trait]
pub trait LifecycleMonitor: Send + Sync {
    /// Aggregate health across all handlers and subsystems.
    async fn health(&self, req: HealthRequest) -> Result<HealthResponse, LifecycleError>;

    /// Start any long-running background tasks (watchdogs, pollers).
    ///
    /// Idempotent: implementations must tolerate repeated calls by no-op'ing
    /// after the first successful start.
    async fn start_background_tasks(
        &self,
        req: StartBackgroundTasksRequest,
    ) -> Result<(), LifecycleError>;

    /// Gracefully shut down: stop accepting new work, drain in-flight work,
    /// release resources. After `shutdown` returns, `health` is expected to
    /// report a non-healthy status.
    async fn shutdown(&self, req: ShutdownRequest) -> Result<(), LifecycleError>;

    /// Return the overall health status without the full component breakdown.
    async fn status(
        &self,
        req: StatusRequest,
    ) -> Result<EmptyResponse<HealthStatus>, LifecycleError>;

    /// Return the health snapshot for a named component, if tracked.
    async fn component(
        &self,
        req: ComponentRequest<'_>,
    ) -> Result<EmptyResponse<Option<ComponentHealth>>, LifecycleError>;
}
