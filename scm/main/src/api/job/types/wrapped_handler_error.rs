//! [`WrappedHandlerError`] — local newtype wrapping the foreign
//! `edge_domain_handler::HandlerError` so `api/` never references the foreign
//! crate's error type directly in a type position (SEA `no_foreign_type`).

use thiserror::Error;

/// Wraps [`edge_domain_handler::HandlerError`].
///
/// Construct via `From`; read the wrapped value via the `inner()` accessor
/// (implemented in `core/`, since the field itself stays `pub(crate)`).
#[derive(Debug, Error)]
#[error("{inner}")]
pub struct WrappedHandlerError {
    pub(crate) inner: edge_domain_handler::HandlerError,
}
