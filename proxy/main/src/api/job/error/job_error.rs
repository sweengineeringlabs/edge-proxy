//! JobError — errors raised by [`Job::run`](crate::Job::run).

use thiserror::Error;

pub use edge_domain::HandlerError;

use crate::api::router::error::routing_error::RoutingError;

/// Errors raised by [`Job::run`](crate::Job::run).
#[derive(Debug, Error)]
pub enum JobError {
    /// The requested handler was not registered or not available.
    #[error("handler not available: {0}")]
    HandlerUnavailable(String),

    /// Routing failed before a handler could be chosen.
    #[error("routing failed: {0}")]
    Routing(#[from] RoutingError),

    /// The chosen handler failed during execution.
    #[error("handler failed: {0}")]
    Handler(#[from] HandlerError),

    /// The job was cancelled by a lifecycle event (e.g. shutdown).
    #[error("job cancelled")]
    Cancelled,
}
