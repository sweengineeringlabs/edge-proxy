//! LifecycleMonitor trait — **Lifecycle** concern of the 5-Concern Controller
//! pattern.
//!
//! Runtime state management: aggregate health, background tasks, and graceful
//! shutdown. Kept separate from `Job` so the operator surface (health, shutdown)
//! can be exposed without granting execution privileges.

use futures::future::BoxFuture;

use crate::api::error::LifecycleError;
use crate::api::health::HealthReport;

/// Runtime lifecycle management for a Controller instance.
pub trait LifecycleMonitor: Send + Sync {
    /// Aggregate health across all handlers and subsystems.
    fn health(&self) -> BoxFuture<'_, HealthReport>;

    /// Start any long-running background tasks (watchdogs, pollers).
    ///
    /// Idempotent: implementations must tolerate repeated calls by no-op'ing
    /// after the first successful start.
    fn start_background_tasks(&self) -> BoxFuture<'_, ()>;

    /// Gracefully shut down: stop accepting new work, drain in-flight work,
    /// release resources. After `shutdown` returns, `health` is expected to
    /// report a non-healthy status.
    fn shutdown(&self) -> BoxFuture<'_, Result<(), LifecycleError>>;
}
