//! Proxy theme — facade handle, pattern marker, and build-up traits.

pub(crate) mod errors;
pub(crate) mod proxy_svc;
pub(crate) mod traits;
pub(crate) mod types;

pub use errors::ProxyComposerError;
pub use traits::ProxyComposer;
pub use types::{
    ApplicationConfigBuilder, BootstrapNameRequest, BootstrapNameResponse, ProxyPattern, ProxySvc,
};
