//! `ProxyComposerError` — errors raised by [`ProxyComposer`](crate::api::proxy::traits::ProxyComposer) operations.

use thiserror::Error;

/// Errors raised by [`ProxyComposer`](crate::api::proxy::traits::ProxyComposer) operations.
#[derive(Debug, PartialEq, Error)]
pub enum ProxyComposerError {
    /// An unexpected internal error occurred.
    #[error("proxy composer internal error: {0}")]
    Internal(String),
}
