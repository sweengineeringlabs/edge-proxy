//! [`ExecutionRequest`] — request for [`Job::run`](crate::api::job::traits::Job::run).

use edge_domain_handler::HandlerContext;

/// Request to dispatch a job's request payload with its request-scoped context.
pub struct ExecutionRequest<'a, Request> {
    /// The job-specific request payload.
    pub req: Request,
    /// The request-scoped execution context.
    pub ctx: &'a HandlerContext<'a>,
}
