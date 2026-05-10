//! `NoopValidator` — marker supertrait for no-op validator implementations.

use crate::api::traits::Validator;

/// Marker supertrait for always-passing validator implementations.
pub trait NoopValidator: Validator<Target = (), Error = std::convert::Infallible> {}

#[cfg(test)]
mod tests {
    use super::*;

    struct Noop;
    impl Validator for Noop {
        type Target = ();
        type Error  = std::convert::Infallible;
        fn validate(&self, _: &()) -> Result<(), Self::Error> { Ok(()) }
    }
    impl NoopValidator for Noop {}

    #[test]
    fn test_noop_validator_is_object_safe_via_concrete_impl() {
        assert!(Noop.validate(&()).is_ok());
    }
}
