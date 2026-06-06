//! Job theme — top-level dispatch entry contract and errors.

pub(crate) mod error;
pub(crate) mod traits;

pub use error::{HandlerError, JobError};
pub use traits::Job;
