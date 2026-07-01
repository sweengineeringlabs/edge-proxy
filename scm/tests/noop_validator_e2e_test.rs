//! End-to-end contract tests for the `NoopValidator` marker trait, exercised through
//! `ProxySvc::new_noop_validator` via the crate's public API.

use edge_proxy::{NoopValidator, ProxySvc, ValidationRequest, Validator};

/// @covers: NoopValidator
#[test]
fn test_noop_validator_accepts_unit_happy() {
    let v = ProxySvc::new_noop_validator();
    assert_eq!(v.validate(ValidationRequest { value: &() }), Ok(()));
}

struct NoopValidatorDouble;
impl Validator for NoopValidatorDouble {
    type Target = ();
    type Error = std::convert::Infallible;
    fn validate(&self, _req: ValidationRequest<'_, ()>) -> Result<(), Self::Error> {
        Ok(())
    }
}
impl NoopValidator for NoopValidatorDouble {}

/// @covers: NoopValidator
#[test]
fn test_noop_validator_double_satisfies_bound_edge() {
    fn accepts<T: NoopValidator>(_: &T) -> bool {
        true
    }
    assert!(accepts(&NoopValidatorDouble));
}
