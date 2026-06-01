//! Job trait — **Job** concern of the 5-Concern Controller pattern.
//!
//! The single entry point the gateway calls. Each Controller implementation
//! provides one `Job` impl that orchestrates its full request→response flow.

use futures::future::BoxFuture;

use crate::api::error::JobError;

/// The single entry point for proxy dispatch.
///
/// Runtime holds `Arc<dyn Job<Request, Response>>` and calls `run` for each request.
pub trait Job<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Dispatch the request and return the response.
    fn run(&self, req: Request) -> BoxFuture<'_, Result<Response, JobError>>;
}
