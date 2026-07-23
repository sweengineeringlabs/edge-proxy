//! JobError — errors raised by [`Job::run`](crate::Job::run).

use thiserror::Error;

pub use edge_application_handler::HandlerError;

use crate::api::router::errors::routing_error::RoutingError;

/// Errors raised by [`Job::run`](crate::Job::run).
#[derive(Debug, Error)]
pub enum JobError {
    /// The requested handler was not registered or not available.
    #[error("handler not available: {0}")]
    HandlerUnavailable(String),

    /// Routing failed before a handler could be chosen.
    #[error("routing failed: {0}")]
    Routing(#[from] RoutingError),

    /// The chosen handler failed during execution. Carries the upstream
    /// [`edge_application_handler::HandlerError`]'s formatted message —
    /// not the error itself, so `api/` never depends on its exact shape.
    #[error("handler failed: {0}")]
    Handler(String),

    /// The job was cancelled by a lifecycle event (e.g. shutdown).
    #[error("job cancelled")]
    Cancelled,
}
