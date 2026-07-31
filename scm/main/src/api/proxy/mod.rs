//! Proxy theme — facade handle, pattern marker, and build-up traits.

pub(crate) mod dto;
pub(crate) mod errors;
pub(crate) mod proxy_svc;
pub(crate) mod traits;
pub(crate) mod vo;

pub use dto::{BootstrapNameRequest, BootstrapNameResponse};
pub use errors::ProxyComposerError;
pub use traits::ProxyComposer;
pub use vo::{ApplicationConfigBuilder, ProxyPattern, ProxySvc};
