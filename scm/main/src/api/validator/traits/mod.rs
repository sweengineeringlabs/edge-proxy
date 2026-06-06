//! Validator theme port contracts.

pub mod noop_validator;
#[allow(clippy::module_inception)]
pub mod validator;

pub use noop_validator::NoopValidator;
pub use validator::Validator;
