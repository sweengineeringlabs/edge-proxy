//! [`AsNullJobResponse`] — response for [`Job::as_null_job`](crate::api::job::traits::Job::as_null_job).

use crate::api::job::NullJob;

/// The erased null-job form, if this implementation is a null object.
pub struct AsNullJobResponse<'a> {
    /// The null-job reference, or `None` for real implementations.
    pub job: Option<&'a NullJob>,
}
