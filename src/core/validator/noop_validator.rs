//! `NoopValidator` — always-valid implementation of [`Validator`].

use crate::api::traits::Validator;

/// A no-op validator that accepts every input without inspection.
#[allow(dead_code)]
pub(crate) struct NoopValidator;

impl crate::api::validator::noop_validator::NoopValidator for NoopValidator {}

impl Validator for NoopValidator {
    type Target = ();
    type Error  = std::convert::Infallible;

    fn validate(&self, _value: &()) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_validator_always_returns_ok() {
        assert!(NoopValidator.validate(&()).is_ok());
    }
}
