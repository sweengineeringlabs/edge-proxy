//! Job SAF facade — re-exports for the Job concern.

pub use crate::api::job::{HandlerError, Job, JobError, NullJob, NullJobMarker};

/// Identifies the Job concern within the proxy crate.
pub const JOB_CONCERN: &str = "job";
