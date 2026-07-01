//! Shared trait contracts required for all non-orchestrator proxy crates.

use crate::api::validator::types::ValidationRequest;

/// Validates a value before it enters the dispatch pipeline.
pub trait Validator: Send + Sync {
    /// The type being validated.
    type Target: ?Sized;
    /// The error type returned on validation failure.
    type Error;

    /// Validate the request's value, returning `Ok(())` if it passes or `Err` with a reason.
    fn validate(&self, req: ValidationRequest<'_, Self::Target>) -> Result<(), Self::Error>;
}
