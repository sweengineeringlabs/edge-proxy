//! `ProxyPattern` — marker type surfacing the 5 proxy concerns in rustdoc.

/// Marker type naming the five concerns for discoverability in docs.
///
/// Traits implementing each concern:
/// 1. **Job** — [`crate::Job`]
/// 2. **Routing** — [`crate::Router`]
/// 3. **Handlers** — [`crate::Handler`]
/// 4. **Lifecycle** — [`crate::LifecycleMonitor`]
/// 5. **Gateway (boundary types)** — `crate::gateway` module (internal)
#[allow(dead_code)]
pub struct ProxyPattern;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_pattern_is_constructible() {
        let _ = ProxyPattern;
    }
}
