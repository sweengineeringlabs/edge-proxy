//! Job trait — **Job** concern of the 5-Concern Controller pattern.
//!
//! The single entry point the gateway calls. Each Controller implementation
//! provides one `Job` impl that orchestrates its full request→response flow.

use async_trait::async_trait;

use crate::api::job::dto::{
    AsNullJobMarkerRequest, AsNullJobRequest, AsNullJobResponse, ExecutionRequest, JobResponse,
    RouterRequest, RouterResponse,
};
use crate::api::job::errors::JobError;
use crate::api::job::vo::NullJobMarker;
use crate::api::types::EmptyResponse;
use crate::api::Router;

/// The single entry point for proxy dispatch.
///
/// Runtime holds `Arc<dyn Job<Request, Response>>` and calls `run` for each request.
/// Implement this to wire routing + handler lookup + lifecycle into a single entrypoint.
///
/// `Job` requires a `Router` (SEA#8): every implementation must name a `Router` for its
/// `Intent`, and expose it via [`Job::router`] — a compile-time witness that a real router
/// is wired in, not just documented. A `Job` that wants its router swappable at runtime
/// pins `Router = Arc<dyn Router<Self::Intent>>`; one that doesn't need that can name its
/// own router type directly.
///
/// # Examples
///
/// ```rust,no_run
/// use async_trait::async_trait;
/// use edge_application_proxy::{Job, JobError, JobResponse, ExecutionRequest, Router, RouterRequest, RouterResponse};
/// use std::sync::Arc;
///
/// struct EchoJob {
///     router: Arc<dyn Router<String>>,
/// }
///
/// #[async_trait]
/// impl Job<String, String> for EchoJob {
///     type Intent = String;
///     type Router = Arc<dyn Router<String>>;
///
///     fn router(&self, _req: RouterRequest) -> Result<RouterResponse<'_, Self::Router>, JobError> {
///         Ok(RouterResponse { router: &self.router })
///     }
///
///     async fn run(&self, req: ExecutionRequest<'_, String>) -> Result<JobResponse<String>, JobError> {
///         Ok(JobResponse { payload: req.req })
///     }
/// }
/// ```
#[async_trait]
pub trait Job<Request = String, Response = String>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// The intent this `Job`'s router classifies input into.
    type Intent: Send + 'static;

    /// The router this `Job` dispatches through. A compile-time witness that a real
    /// `Router` is wired in, not just documented (SEA#8).
    type Router: Router<Self::Intent>;

    /// Return the router this `Job` dispatches through.
    fn router(&self, req: RouterRequest) -> Result<RouterResponse<'_, Self::Router>, JobError>;

    /// Dispatch the request and return the response.
    ///
    /// [`ExecutionRequest::ctx`] carries the authenticated principal, tenant, claims,
    /// and command bus for the current request. Construct it at the inbound
    /// boundary and thread it through to [`Handler::execute`](edge_application::Handler::execute).
    async fn run(
        &self,
        req: ExecutionRequest<'_, Request>,
    ) -> Result<JobResponse<Response>, JobError>;

    /// Return a reference to the erased null-job form, if this implementation
    /// is a null object.  Returns `None` by default.
    fn as_null_job(&self, _req: AsNullJobRequest) -> Result<AsNullJobResponse<'_>, JobError> {
        Ok(AsNullJobResponse { job: None })
    }

    /// Return a [`NullJobMarker`](crate::api::job::vo::NullJobMarker) token if this
    /// implementation is a null-object job, or `None` for real implementations.  Used to
    /// identify inert jobs in bring-up and testing contexts without downcasting.
    fn as_null_job_marker(
        &self,
        _req: AsNullJobMarkerRequest,
    ) -> Result<EmptyResponse<Option<NullJobMarker>>, JobError> {
        Ok(EmptyResponse { value: None })
    }
}
