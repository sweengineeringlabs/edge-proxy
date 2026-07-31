//! Validator theme — pre-dispatch validation contracts.

pub(crate) mod dto;
pub(crate) mod traits;

pub use dto::ValidationRequest;
pub use traits::{NoopValidator, Validator};
