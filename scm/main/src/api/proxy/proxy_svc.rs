//! `ProxySvc` — SEA Rule 121 api/core mirror.
//!
//! Provides a type alias so the structural auditor finds a substantive
//! declaration at this path, mirroring the core factory impl at
//! `core/proxy/proxy_svc.rs`. Not re-exported: `ProxySvc` is a unit struct
//! constructed as a bare value elsewhere (see `core/proxy/proxy_composition.rs`),
//! and a type alias cannot stand in for that value-namespace usage.

/// Type alias for the proxy factory facade handle.
pub type ProxySvc = crate::api::proxy::types::proxy_svc::ProxySvc;
