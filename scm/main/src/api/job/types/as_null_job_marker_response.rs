//! [`AsNullJobMarkerResponse`] — response for [`Job::as_null_job_marker`](crate::api::job::traits::Job::as_null_job_marker).

use crate::api::job::types::NullJobMarker;

/// The null-job marker token, if this implementation is a null object.
pub struct AsNullJobMarkerResponse {
    /// The marker token, or `None` for real implementations.
    pub marker: Option<NullJobMarker>,
}
