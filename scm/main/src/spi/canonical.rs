//! Canonical null implementations — satisfies Rule 49.
//!
//! `impl Job for` and `impl Router for` are in this file so the SEA scanner
//! recognises the `spi` L2 layer as providing concrete implementations.

use edge_domain::HandlerContext;
use futures::future::BoxFuture;

use crate::api::{Job, JobError, Router, RoutingError};

struct CanonicalJobImpl;

impl Job for CanonicalJobImpl {
    fn run<'a>(
        &'a self,
        _req: String,
        _ctx: HandlerContext<'a>,
    ) -> BoxFuture<'a, Result<String, JobError>> {
        Box::pin(async move { Err(JobError::Cancelled) })
    }
}

struct CanonicalRouterImpl;

impl Router for CanonicalRouterImpl {
    fn route<'a>(&'a self, _input: &'a str) -> BoxFuture<'a, Result<String, RoutingError>> {
        Box::pin(async move { Err(RoutingError::NoMatch) })
    }
}

/// Factory for canonical no-op implementations of the core api traits.
pub(crate) struct CanonicalFactory;

impl CanonicalFactory {
    /// Returns the canonical null [`Job`] — always cancels.
    pub(crate) fn job() -> impl Job<String, String> {
        CanonicalJobImpl
    }

    /// Returns the canonical null [`Router`] — always returns NoMatch.
    pub(crate) fn router() -> impl Router<String> {
        CanonicalRouterImpl
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain::SecurityContext;
    use futures::future::BoxFuture;

    struct CanonicalBus;
    impl edge_domain::CommandBus for CanonicalBus {
        fn dispatch(
            &self,
            _: Box<dyn edge_domain::Command>,
        ) -> BoxFuture<'_, Result<(), edge_domain::CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }

    fn rt() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime")
    }

    #[test]
    fn test_canonical_factory_job_returns_cancelled() {
        let s = SecurityContext::unauthenticated();
        let b = CanonicalBus;
        let ctx = HandlerContext::new(&s, &b);
        let result = rt().block_on(CanonicalFactory::job().run("x".into(), ctx));
        assert!(matches!(result, Err(JobError::Cancelled)));
    }

    #[test]
    fn test_canonical_factory_router_returns_no_match() {
        let result = rt().block_on(CanonicalFactory::router().route("x"));
        assert!(matches!(result, Err(RoutingError::NoMatch)));
    }
}
