//! `NoopValidator` — always-valid implementation of [`Validator`].

use crate::api::{ValidationRequest, Validator};

/// A no-op validator that accepts every input without inspection.
pub(crate) struct NoopValidator;

impl crate::api::NoopValidator for NoopValidator {}

impl Validator for NoopValidator {
    type Target = ();
    type Error = std::convert::Infallible;

    fn validate(&self, _req: ValidationRequest<'_, ()>) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_validator_always_returns_ok() {
        assert_eq!(
            NoopValidator.validate(ValidationRequest { value: &() }),
            Ok(())
        );
    }
}
