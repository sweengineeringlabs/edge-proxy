//! Shared trait contracts required for all non-orchestrator proxy crates.

/// Validates a value before it enters the dispatch pipeline.
pub trait Validator: Send + Sync {
    /// The type being validated.
    type Target;
    /// The error type returned on validation failure.
    type Error;

    /// Validate `value`, returning `Ok(())` if it passes or `Err` with a reason.
    fn validate(&self, value: &Self::Target) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysOk;
    impl Validator for AlwaysOk {
        type Target = ();
        type Error  = String;
        fn validate(&self, _: &()) -> Result<(), String> { Ok(()) }
    }

    #[test]
    fn test_validator_ok_impl_returns_ok() {
        assert!(AlwaysOk.validate(&()).is_ok());
    }
}
