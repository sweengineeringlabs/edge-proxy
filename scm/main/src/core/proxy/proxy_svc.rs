//! Public factory and validation entry points for `edge-proxy`.

use std::sync::Arc;

use crate::api::{Job, LifecycleMonitor, ProxySvc, Router, ValidationRequest, Validator};

/// Compile-time witness that the SEA Rule 121 mirror shim at
/// `api/proxy/proxy_svc.rs` keeps aliasing this file's `impl` target.
const _: fn(crate::api::proxy_svc::ProxySvc) = |_: ProxySvc| {};

impl ProxySvc {
    /// Return a [`ConfigBuilder`](swe_edge_configbuilder::ConfigBuilder) pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Construct a no-op [`LifecycleMonitor`] useful for tests or early bring-up.
    ///
    /// Returned as `Arc<dyn LifecycleMonitor>` so the concrete impl type stays
    /// private.
    pub fn new_null_lifecycle_monitor() -> Arc<dyn LifecycleMonitor> {
        Self::to_arc(crate::spi::CanonicalFactory::null_lifecycle_monitor())
    }

    /// Construct a no-op validator that accepts every `()` input without inspection.
    ///
    /// Useful for bring-up and tests where validation is not yet implemented.
    pub fn new_noop_validator() -> Arc<dyn Validator<Target = (), Error = std::convert::Infallible>>
    {
        Self::to_arc(crate::spi::CanonicalFactory::noop_validator())
    }

    /// Apply `validator` to `value`, returning any validation error.
    pub fn validate<V: Validator>(validator: &V, value: &V::Target) -> Result<(), V::Error> {
        validator.validate(ValidationRequest { value })
    }

    /// Construct a no-op [`Job`] that always returns `JobError::Cancelled`.
    ///
    /// Useful as a placeholder during bring-up before a real job is wired.
    pub fn new_null_job<Req, Resp>() -> Arc<dyn Job<Req, Resp>>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        Self::to_arc(crate::spi::CanonicalFactory::null_job::<Req, Resp>())
    }

    /// Construct a no-op [`Router`] that always returns `RoutingError::NoMatch`.
    ///
    /// Useful as a placeholder during bring-up before a real router is wired.
    pub fn new_null_router() -> Arc<dyn Router<String>> {
        Self::to_arc(crate::spi::CanonicalFactory::null_router())
    }

    /// Construct the canonical [`Job`] — returns `JobError::Cancelled` for every
    /// `String` request.  Useful as a default-type reference implementation.
    pub fn new_canonical_job() -> Arc<dyn Job<String, String>> {
        Self::to_arc(crate::spi::CanonicalFactory::job())
    }

    /// Construct the canonical [`Router`] — returns `RoutingError::NoMatch` for
    /// every input.  Useful as a default-type reference implementation.
    pub fn new_canonical_router() -> Arc<dyn Router<String>> {
        Self::to_arc(crate::spi::CanonicalFactory::router())
    }

    /// Lift a concrete implementation into a shared, type-erasable handle.
    fn to_arc<T>(value: T) -> Arc<T> {
        Arc::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        ExecutionRequest, HealthRequest, HealthStatus, JobError, RouteRequest, RoutingError,
    };
    use edge_domain_handler::HandlerContext;
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};
    use futures::future::BoxFuture;

    struct ProxySvcNullBus;
    impl edge_domain_command::CommandBus for ProxySvcNullBus {
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

    /// @covers: to_arc
    #[test]
    fn test_to_arc_preserves_value() {
        let arc = ProxySvc::to_arc(42_u32);
        assert_eq!(*arc, 42);
    }

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_seeds_crate_name() {
        let b = ProxySvc::create_config_builder();
        assert_eq!(b.name(), env!("CARGO_PKG_NAME"));
    }

    /// @covers: new_null_lifecycle_monitor
    #[test]
    fn test_new_null_lifecycle_monitor_starts_healthy() {
        let m = ProxySvc::new_null_lifecycle_monitor();
        assert_eq!(
            rt().block_on(m.health(HealthRequest)).unwrap().overall,
            HealthStatus::Healthy
        );
    }

    /// @covers: new_noop_validator
    #[test]
    fn test_new_noop_validator_accepts_unit() {
        let v = ProxySvc::new_noop_validator();
        assert_eq!(v.validate(ValidationRequest { value: &() }), Ok(()));
    }

    /// @covers: validate
    #[test]
    fn test_validate_delegates_to_validator() {
        struct ProxySvcAlwaysOk;
        impl Validator for ProxySvcAlwaysOk {
            type Target = ();
            type Error = String;
            fn validate(&self, _req: ValidationRequest<'_, ()>) -> Result<(), String> {
                Ok(())
            }
        }
        assert_eq!(ProxySvc::validate(&ProxySvcAlwaysOk, &()), Ok(()));
    }

    /// @covers: new_null_job
    #[test]
    fn test_new_null_job_returns_cancelled() {
        let job = ProxySvc::new_null_job::<String, String>();
        let s: SecurityContext = SecurityServices::unauthenticated();
        let b = ProxySvcNullBus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &s,
            commands: &b,
            observer: observer.as_ref(),
        };
        let result = rt().block_on(job.run(ExecutionRequest {
            req: "x".into(),
            ctx: &ctx,
        }));
        assert!(matches!(result, Err(JobError::Cancelled)));
    }

    /// @covers: new_null_router
    #[test]
    fn test_new_null_router_returns_no_match() {
        let router = ProxySvc::new_null_router();
        let result = rt().block_on(router.route(RouteRequest { input: "x" }));
        assert!(matches!(result, Err(RoutingError::NoMatch)));
    }

    /// @covers: new_canonical_job
    #[test]
    fn test_new_canonical_job_returns_cancelled() {
        let job = ProxySvc::new_canonical_job();
        let s: SecurityContext = SecurityServices::unauthenticated();
        let b = ProxySvcNullBus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &s,
            commands: &b,
            observer: observer.as_ref(),
        };
        let result = rt().block_on(job.run(ExecutionRequest {
            req: "x".into(),
            ctx: &ctx,
        }));
        assert!(matches!(result, Err(JobError::Cancelled)));
    }

    /// @covers: new_canonical_router
    #[test]
    fn test_new_canonical_router_returns_no_match() {
        let router = ProxySvc::new_canonical_router();
        let result = rt().block_on(router.route(RouteRequest { input: "x" }));
        assert!(matches!(result, Err(RoutingError::NoMatch)));
    }
}
