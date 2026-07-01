//! [`BootstrapNameResponse`] — response for [`ProxyComposer::bootstrap_name`](crate::api::proxy::traits::ProxyComposer::bootstrap_name).

/// A composer implementation's stable identifier.
pub struct BootstrapNameResponse {
    /// The stable, non-empty identifier.
    pub name: &'static str,
}
