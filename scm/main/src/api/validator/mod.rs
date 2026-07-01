//! Validator theme — pre-dispatch validation contracts.

pub(crate) mod traits;
pub mod types;

pub use traits::{NoopValidator, Validator};
pub use types::ValidationRequest;
