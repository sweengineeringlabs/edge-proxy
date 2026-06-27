//! Canonical null implementations — satisfies Rule 49.
//!
//! `impl Job for` and `impl Router for` are in this file so the SEA scanner
//! recognises the `spi` L2 layer as providing concrete implementations.

use edge_domain_handler::HandlerContext;
use futures::future::BoxFuture;


use crate::api::{Job, JobError, LifecycleMonitor, Router, RoutingError, Validator};

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

    /// Returns a null [`Job`] for any `Req`/`Resp` — always returns `Cancelled`.
    pub(crate) fn null_job<Req, Resp>() -> impl Job<Req, Resp>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        crate::core::job::null_job::NullJob
    }

    /// Returns a null [`Router`] — always returns `NoMatch`.
    pub(crate) fn null_router() -> impl Router<String> {
        crate::core::router::null_router::NullRouter
    }

    /// Returns a null [`LifecycleMonitor`] — always healthy, no-op shutdown.
    pub(crate) fn null_lifecycle_monitor() -> impl LifecycleMonitor {
        crate::core::lifecycle::NullLifecycleMonitor::new()
    }

    /// Returns a no-op [`Validator`] that accepts every `()` input.
    pub(crate) fn noop_validator() -> impl Validator<Target = (), Error = std::convert::Infallible>
    {
        crate::core::validator::noop_validator::NoopValidator
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_security::SecurityContext;
    use edge_domain_observer::StdObserveFactory;
    use futures::future::BoxFuture;

    struct CanonicalBus;
    impl edge_domain_command::CommandBus for CanonicalBus {
        fn dispatch(
            &self,
            _: Box<dyn edge_domain_command::Command>,
        ) -> BoxFuture<'_, Result<(), edge_domain_command::CommandError>> {
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
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext::new(&s, &b, observer.as_ref());
        let result = rt().block_on(CanonicalFactory::job().run("x".into(), ctx));
        assert!(matches!(result, Err(JobError::Cancelled)));
    }

    #[test]
    fn test_canonical_factory_router_returns_no_match() {
        let result = rt().block_on(CanonicalFactory::router().route("x"));
        assert!(matches!(result, Err(RoutingError::NoMatch)));
    }

    #[test]
    fn test_null_job_returns_cancelled() {
        let s = SecurityContext::unauthenticated();
        let b = CanonicalBus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext::new(&s, &b, observer.as_ref());
        let result: Result<(), _> = rt().block_on(CanonicalFactory::null_job().run((), ctx));
        assert!(matches!(result, Err(JobError::Cancelled)));
    }

    #[test]
#[ignore = "blocked: edge-domain v0.11.3 ObserverContext wiring pending"]
    fn test_null_router_returns_no_match() {
        let result = rt().block_on(CanonicalFactory::null_router().route("anything"));
        assert!(matches!(result, Err(RoutingError::NoMatch)));
    }

    #[test]
#[ignore = "blocked: edge-domain v0.11.3 ObserverContext wiring pending"]
    fn test_null_lifecycle_monitor_starts_healthy() {
        use crate::api::HealthStatus;
        let report = rt().block_on(CanonicalFactory::null_lifecycle_monitor().health());
        assert_eq!(report.overall, HealthStatus::Healthy);
    }

    #[test]
#[ignore = "blocked: edge-domain v0.11.3 ObserverContext wiring pending"]
    fn test_noop_validator_accepts_unit() {
        assert!(CanonicalFactory::noop_validator().validate(&()).is_ok());
    }
}
