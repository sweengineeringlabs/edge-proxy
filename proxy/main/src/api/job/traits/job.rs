//! Job trait — **Job** concern of the 5-Concern Controller pattern.
//!
//! The single entry point the gateway calls. Each Controller implementation
//! provides one `Job` impl that orchestrates its full request→response flow.

use futures::future::BoxFuture;

use crate::api::job::error::JobError;

/// The single entry point for proxy dispatch.
///
/// Runtime holds `Arc<dyn Job<Request, Response>>` and calls `run` for each request.
/// Implement this to wire routing + handler lookup + lifecycle into a single entrypoint.
///
/// # Examples
///
/// ```rust,no_run
/// use futures::future::BoxFuture;
/// use edge_proxy::{Job, JobError};
///
/// struct EchoJob;
///
/// impl Job<String, String> for EchoJob {
///     fn run(&self, req: String) -> BoxFuture<'_, Result<String, JobError>> {
///         Box::pin(async move { Ok(req) })
///     }
/// }
/// ```
pub trait Job<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Dispatch the request and return the response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// // Typical impl: route the request, look up a handler, execute it.
    /// // let intent = self.router.route(&req).await?;
    /// // let handler = self.registry.get(&intent)?;
    /// // handler.execute(req).await.map_err(JobError::from)
    /// ```
    fn run(&self, req: Request) -> BoxFuture<'_, Result<Response, JobError>>;
}
