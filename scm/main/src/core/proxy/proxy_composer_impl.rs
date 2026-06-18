//! Default `ProxyComposer` implementation for `ProxySvc`.

use crate::api::{ApplicationConfigBuilder, ProxyComposer, ProxyPattern, ProxySvc};

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
