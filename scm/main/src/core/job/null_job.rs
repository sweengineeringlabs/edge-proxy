//! `NullJob` — a no-op `Job` that always cancels, useful as a placeholder.

use async_trait::async_trait;

use crate::api::{ExecutionRequest, Job, JobError, JobResponse};

/// No-op job that returns `JobError::Cancelled` for every request.
///
/// `pub(crate)` — consumers obtain jobs through their own `Job` implementations.
pub(crate) struct NullJob;

#[async_trait]
impl<Req, Resp> Job<Req, Resp> for NullJob
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    async fn run(&self, _req: ExecutionRequest<'_, Req>) -> Result<JobResponse<Resp>, JobError> {
        Err(JobError::Cancelled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_application_handler::HandlerContext;
    use edge_security_runtime::SecurityContext;
    use futures::future::BoxFuture;

    struct NullBus;
    impl edge_application_command::CommandBus for NullBus {
        fn dispatch(
            &self,
            _: edge_application_command::CommandDispatchRequest,
        ) -> BoxFuture<'_, Result<(), edge_application_command::CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[tokio::test]
    async fn test_null_job_always_returns_cancelled() {
        use edge_application_observer::StdObserveFactory;

        let s: SecurityContext = SecurityContext::unauthenticated();
        let b = NullBus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &s,
            commands: &b,
            observer: observer.as_ref(),
        };
        let result: Result<JobResponse<()>, _> =
            NullJob.run(ExecutionRequest { req: (), ctx: &ctx }).await;
        assert!(matches!(result, Err(JobError::Cancelled)));
    }
}
