//! `ProxyComposer` — build-up entry point for proxy application configuration.

use crate::api::proxy::errors::ProxyComposerError;
use crate::api::proxy::types::{
    ApplicationConfigBuilder, BootstrapNameRequest, BootstrapNameResponse, ProxyPattern, ProxySvc,
};

/// Build and configure a proxy service instance.
///
/// Implement this to provide domain-specific factory variants.
/// The default implementation is on [`ProxySvc`] itself in `core/proxy/`.
pub trait ProxyComposer {
    /// Stable identity name for this composer variant.
    fn bootstrap_name(
        &self,
        _req: BootstrapNameRequest,
    ) -> Result<BootstrapNameResponse, ProxyComposerError> {
        Ok(BootstrapNameResponse {
            name: "proxy_composer",
        })
    }

    /// Create a new proxy service facade handle.
    fn compose() -> ProxySvc
    where
        Self: Sized;

    /// Return the proxy pattern marker for rustdoc discoverability.
    fn pattern() -> ProxyPattern
    where
        Self: Sized;

    /// Create a pre-seeded application configuration builder.
    fn builder() -> ApplicationConfigBuilder
    where
        Self: Sized;
}
