//! [`JobResponse`] — response for [`Job::run`](crate::api::job::traits::Job::run).

/// The response produced by dispatching a request through [`Job::run`](crate::api::job::traits::Job::run).
#[derive(Debug)]
pub struct JobResponse<T> {
    /// The caller-defined response payload.
    pub payload: T,
}
