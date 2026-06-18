//! `NullJob` — a no-op `Job` that always cancels, useful as a placeholder.

use edge_domain::HandlerContext;
use futures::future::BoxFuture;

use crate::api::{Job, JobError};

/// No-op job that returns `JobError::Cancelled` for every request.
///
/// `pub(crate)` — consumers obtain jobs through their own `Job` implementations.
pub(crate) struct NullJob;

impl<Req, Resp> Job<Req, Resp> for NullJob
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn run<'a>(
        &'a self,
        _req: Req,
        _ctx: HandlerContext<'a>,
    ) -> BoxFuture<'a, Result<Resp, JobError>> {
        Box::pin(async move { Err(JobError::Cancelled) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain::SecurityContext;
    use futures::future::BoxFuture;

    struct NullBus;
    impl edge_domain::CommandBus for NullBus {
        fn dispatch(
            &self,
            _: Box<dyn edge_domain::Command>,
        ) -> BoxFuture<'_, Result<(), edge_domain::CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[tokio::test]
    async fn test_null_job_always_returns_cancelled() {
        let s = SecurityContext::unauthenticated();
        let b = NullBus;
        let ctx = HandlerContext::new(&s, &b);
        let result: Result<(), _> = NullJob.run((), ctx).await;
        assert!(matches!(result, Err(JobError::Cancelled)));
    }
}
