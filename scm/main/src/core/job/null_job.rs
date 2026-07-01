//! `NullJob` — a no-op `Job` that always cancels, useful as a placeholder.

use futures::future::BoxFuture;

use crate::api::{ExecutionRequest, Job, JobError};

/// No-op job that returns `JobError::Cancelled` for every request.
///
/// `pub(crate)` — consumers obtain jobs through their own `Job` implementations.
pub(crate) struct NullJob;

impl<Req, Resp> Job<Req, Resp> for NullJob
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn run<'a>(&'a self, _req: ExecutionRequest<'a, Req>) -> BoxFuture<'a, Result<Resp, JobError>> {
        Box::pin(async move { Err(JobError::Cancelled) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_handler::HandlerContext;
    use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};
    use futures::future::BoxFuture;

    struct NullBus;
    impl edge_domain_command::CommandBus for NullBus {
        fn dispatch(
            &self,
            _: Box<dyn edge_domain_command::Command>,
        ) -> BoxFuture<'_, Result<(), edge_domain_command::CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[tokio::test]
    async fn test_null_job_always_returns_cancelled() {
        use edge_domain_observer::StdObserveFactory;

        let s: SecurityContext = SecurityServices::unauthenticated();
        let b = NullBus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &s,
            commands: &b,
            observer: observer.as_ref(),
        };
        let result: Result<(), _> = NullJob.run(ExecutionRequest { req: (), ctx: &ctx }).await;
        assert!(matches!(result, Err(JobError::Cancelled)));
    }
}
