//! `ProxyPattern` — marker type surfacing the 5 proxy concerns in rustdoc.

/// Marker type naming the five concerns for discoverability in docs.
///
/// Traits implementing each concern:
/// 1. **Job** — [`crate::Job`]
/// 2. **Routing** — [`crate::Router`]
/// 3. **Handlers** — domain-defined handler types
/// 4. **Lifecycle** — [`crate::LifecycleMonitor`]
/// 5. **Gateway (boundary types)** — `crate::gateway` module (internal)
pub struct ProxyPattern;
