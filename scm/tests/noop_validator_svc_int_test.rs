//! Integration tests verifying noop_validator_svc re-exports are reachable.

use edge_proxy::{NoopValidator, ValidationRequest, Validator};

struct NoopValidatorDouble;

impl Validator for NoopValidatorDouble {
    type Target = ();
    type Error = std::convert::Infallible;
    fn validate(&self, _req: ValidationRequest<'_, ()>) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl NoopValidator for NoopValidatorDouble {}

/// Verifies `NoopValidator` trait is exported at the crate level.
#[test]
fn test_noop_validator_type_is_reachable_from_crate_boundary() {
    fn accepts_noop_validator<T: NoopValidator>(_: &T) -> bool {
        true
    }
    assert!(accepts_noop_validator(&NoopValidatorDouble));
}
