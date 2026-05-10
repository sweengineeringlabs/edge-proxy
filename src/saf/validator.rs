//! SAF wrapper for the Validator trait concern.

use crate::api::traits::Validator;

/// Apply `validator` to `value`, returning any validation error.
pub fn validate<V: Validator>(validator: &V, value: &V::Target) -> Result<(), V::Error> {
    validator.validate(value)
}
