//! Job theme — top-level dispatch entry contract and errors.

pub(crate) mod errors;
pub mod null_job;
pub(crate) mod traits;
pub mod types;

pub use errors::{HandlerError, JobError};
pub use null_job::NullJob;
pub use traits::Job;
pub use types::{
    AsNullJobMarkerRequest, AsNullJobMarkerResponse, AsNullJobRequest, AsNullJobResponse,
    ExecutionRequest, NullJobMarker,
};
