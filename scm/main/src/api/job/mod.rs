//! Job theme — top-level dispatch entry contract and errors.

pub(crate) mod dto;
pub(crate) mod errors;
pub mod null_job;
pub(crate) mod traits;
pub(crate) mod vo;

pub use dto::{
    AsNullJobMarkerRequest, AsNullJobRequest, AsNullJobResponse, ExecutionRequest, JobResponse,
};
pub use errors::{HandlerError, JobError};
pub use null_job::NullJob;
pub use traits::Job;
pub use vo::NullJobMarker;
