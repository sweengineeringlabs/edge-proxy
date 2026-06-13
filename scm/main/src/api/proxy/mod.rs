//! Proxy theme — facade handle, pattern marker, and build-up traits.

pub(crate) mod traits;
pub(crate) mod types;

pub use traits::ProxyComposer;
pub use types::{ApplicationConfigBuilder, ProxyPattern, ProxySvc};
