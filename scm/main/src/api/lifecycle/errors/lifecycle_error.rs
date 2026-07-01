//! LifecycleError — errors raised by [`LifecycleMonitor::shutdown`](crate::LifecycleMonitor::shutdown).

use thiserror::Error;

/// Errors raised by [`LifecycleMonitor::shutdown`](crate::LifecycleMonitor::shutdown).
#[derive(Debug, PartialEq, Error)]
pub enum LifecycleError {
    /// Shutdown was called twice or on an already-stopped instance.
    #[error("already shut down")]
    AlreadyShutDown,

    /// A background task failed to stop cleanly.
    #[error("background task did not drain: {0}")]
    DrainFailed(String),

    /// An unexpected internal error occurred during lifecycle management.
    #[error("lifecycle internal error: {0}")]
    Internal(String),
}
