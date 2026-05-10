//! Interface declarations for validator implementations.
pub mod noop_validator;
pub use noop_validator::NoopValidator;
pub use crate::api::traits::Validator;
