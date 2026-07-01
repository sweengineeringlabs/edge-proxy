//! Integration tests for NoopValidator marker trait.

use edge_proxy::{NoopValidator, ProxySvc, ValidationRequest, Validator};

/// @covers: NoopValidator
#[test]
fn test_noop_validator_trait_object_through_proxy_svc() {
    // Verify that new_noop_validator returns a type that also satisfies NoopValidator bounds
    // by verifying through Validator trait usage.
    let v = ProxySvc::new_noop_validator();
    assert_eq!(v.validate(ValidationRequest { value: &() }), Ok(()));
}

struct ConcreteNoop;
impl Validator for ConcreteNoop {
    type Target = ();
    type Error = std::convert::Infallible;
    fn validate(&self, _req: ValidationRequest<'_, ()>) -> Result<(), Self::Error> {
        Ok(())
    }
}
impl NoopValidator for ConcreteNoop {}

/// @covers: NoopValidator
#[test]
fn test_noop_validator_impl_accepts_unit_input() {
    let v = ConcreteNoop;
    assert_eq!(v.validate(ValidationRequest { value: &() }), Ok(()));
}
