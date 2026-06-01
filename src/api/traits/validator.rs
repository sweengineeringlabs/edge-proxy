//! Shared trait contracts required for all non-orchestrator proxy crates.

/// Validates a value before it enters the dispatch pipeline.
pub trait Validator: Send + Sync {
    /// The type being validated.
    type Target: ?Sized;
    /// The error type returned on validation failure.
    type Error;

    /// Validate `value`, returning `Ok(())` if it passes or `Err` with a reason.
    fn validate(&self, value: &Self::Target) -> Result<(), Self::Error>;
}
