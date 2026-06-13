//! `NullJob` — a no-op `Job` that always cancels, useful as a placeholder.

use edge_domain::SecurityContext;
use futures::future::BoxFuture;

use crate::api::job::errors::job_error::JobError;
use crate::api::job::traits::job::Job;

/// No-op job that returns `JobError::Cancelled` for every request.
///
/// `pub(crate)` — consumers obtain jobs through their own `Job` implementations.
pub(crate) struct NullJob;

impl<Req, Resp> Job<Req, Resp> for NullJob
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn run(&self, _req: Req, _ctx: SecurityContext) -> BoxFuture<'_, Result<Resp, JobError>> {
        Box::pin(async move { Err(JobError::Cancelled) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_null_job_always_returns_cancelled() {
        let result: Result<(), _> = NullJob.run((), SecurityContext::unauthenticated()).await;
        assert!(matches!(result, Err(JobError::Cancelled)));
    }
}
