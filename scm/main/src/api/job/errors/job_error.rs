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

    /// The chosen handler failed during execution — carries the upstream
    /// [`HandlerError`] itself so ingress adapters can map its variant to a
    /// transport status (see edge-bootstrap's http/grpc ingress-job mappers,
    /// e.g. `PermissionDenied` -> HTTP 403). `api/` already depends on
    /// `HandlerError` (re-exported above), so carrying it here costs no
    /// decoupling while preserving the failure's category. The `#[from]`
    /// generates `From<HandlerError> for JobError`, so `?` on a handler call
    /// wraps the error directly.
    #[error("handler failed: {0}")]
    Handler(#[from] HandlerError),

    /// The job was cancelled by a lifecycle event (e.g. shutdown).
    #[error("job cancelled")]
    Cancelled,
}
