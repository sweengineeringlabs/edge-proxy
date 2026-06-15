//! Job trait — **Job** concern of the 5-Concern Controller pattern.
//!
//! The single entry point the gateway calls. Each Controller implementation
//! provides one `Job` impl that orchestrates its full request→response flow.

use edge_domain::HandlerContext;
use futures::future::BoxFuture;

use crate::api::job::errors::JobError;
use crate::api::job::types::null_job_marker::NullJobMarker;
use crate::api::job::NullJob;

/// The single entry point for proxy dispatch.
///
/// Runtime holds `Arc<dyn Job<Request, Response>>` and calls `run` for each request.
/// Implement this to wire routing + handler lookup + lifecycle into a single entrypoint.
///
/// # Examples
///
/// ```rust,no_run
/// use futures::future::BoxFuture;
/// use edge_proxy::{HandlerContext, Job, JobError};
///
/// struct EchoJob;
///
/// impl Job<String, String> for EchoJob {
///     fn run<'a>(&'a self, req: String, _ctx: HandlerContext<'a>) -> BoxFuture<'a, Result<String, JobError>> {
///         Box::pin(async move { Ok(req) })
///     }
/// }
/// ```
pub trait Job<Request = String, Response = String>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Dispatch the request and return the response.
    ///
    /// [`HandlerContext`] carries the authenticated principal, tenant, claims,
    /// and command bus for the current request. Construct it at the inbound
    /// boundary and thread it through to [`Handler::execute`].
    fn run<'a>(
        &'a self,
        req: Request,
        ctx: HandlerContext<'a>,
    ) -> BoxFuture<'a, Result<Response, JobError>>;

    /// Return a reference to the erased null-job form, if this implementation
    /// is a null object.  Returns `None` by default.
    fn as_null_job(&self) -> Option<&NullJob> {
        None
    }

    /// Return a [`NullJobMarker`] token if this implementation is a null-object
    /// job, or `None` for real implementations.  Used to identify inert jobs
    /// in bring-up and testing contexts without downcasting.
    fn as_null_job_marker(&self) -> Option<NullJobMarker> {
        None
    }
}
