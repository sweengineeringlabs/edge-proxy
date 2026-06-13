//! Default `ProxyComposer` implementation for `ProxySvc`.

use crate::api::proxy::traits::proxy_composer::ProxyComposer;
use crate::api::proxy::types::{ApplicationConfigBuilder, ProxyPattern, ProxySvc};

impl ProxyComposer for ProxySvc {
    fn compose() -> ProxySvc {
        ProxySvc
    }

    fn pattern() -> ProxyPattern {
        ProxyPattern
    }

    fn builder() -> ApplicationConfigBuilder {
        ApplicationConfigBuilder::new()
    }
}
