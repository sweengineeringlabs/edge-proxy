//! Public factory and validation entry points for `edge-proxy`.

use std::sync::Arc;

use crate::api::traits::lifecycle_monitor::LifecycleMonitor;
use crate::api::traits::validator::Validator;
use crate::api::types::proxy::proxy_svc::ProxySvc;
use crate::core::null::NullLifecycleMonitor;
use crate::core::validator::noop_validator::NoopValidator as CoreNoopValidator;

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
    pub fn new_noop_validator(
    ) -> Arc<dyn Validator<Target = (), Error = std::convert::Infallible>> {
        Arc::new(CoreNoopValidator)
    }

    /// Apply `validator` to `value`, returning any validation error.
    pub fn validate<V: Validator>(validator: &V, value: &V::Target) -> Result<(), V::Error> {
        validator.validate(value)
    }
}
