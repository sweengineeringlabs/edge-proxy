//! Public factory and validation entry points for `edge-proxy`.

use std::sync::Arc;

use crate::api::job::traits::job::Job;
use crate::api::lifecycle::traits::lifecycle_monitor::LifecycleMonitor;
use crate::api::router::traits::router::Router;
use crate::api::validator::traits::validator::Validator;
use crate::core::job::null_job::NullJob;
use crate::core::lifecycle::NullLifecycleMonitor;
use crate::core::router::null_router::NullRouter;
use crate::core::validator::noop_validator::NoopValidator as CoreNoopValidator;

pub use crate::api::proxy::{ApplicationConfigBuilder, ProxyComposer, ProxyPattern, ProxySvc};

impl ProxySvc {
    /// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
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
        Arc::new(NullLifecycleMonitor::new())
    }

    /// Construct a no-op validator that accepts every `()` input without inspection.
    ///
    /// Useful for bring-up and tests where validation is not yet implemented.
    pub fn new_noop_validator() -> Arc<dyn Validator<Target = (), Error = std::convert::Infallible>>
    {
        Arc::new(CoreNoopValidator)
    }

    /// Apply `validator` to `value`, returning any validation error.
    pub fn validate<V: Validator>(validator: &V, value: &V::Target) -> Result<(), V::Error> {
        validator.validate(value)
    }

    /// Construct a no-op [`Job`] that always returns `JobError::Cancelled`.
    ///
    /// Useful as a placeholder during bring-up before a real job is wired.
    pub fn new_null_job<Req, Resp>() -> Arc<dyn Job<Req, Resp>>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        Arc::new(NullJob)
    }

    /// Construct a no-op [`Router`] that always returns `RoutingError::NoMatch`.
    ///
    /// Useful as a placeholder during bring-up before a real router is wired.
    pub fn new_null_router() -> Arc<dyn Router<String>> {
        Arc::new(NullRouter)
    }

    /// Construct the canonical [`Job`] — returns `JobError::Cancelled` for every
    /// `String` request.  Useful as a default-type reference implementation.
    pub fn new_canonical_job() -> Arc<dyn Job<String, String>> {
        Arc::new(crate::spi::CanonicalFactory::job())
    }

    /// Construct the canonical [`Router`] — returns `RoutingError::NoMatch` for
    /// every input.  Useful as a default-type reference implementation.
    pub fn new_canonical_router() -> Arc<dyn Router<String>> {
        Arc::new(crate::spi::CanonicalFactory::router())
    }
}
