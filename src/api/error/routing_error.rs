//! RoutingError — errors raised by [`Router::route`](crate::Router::route).

use thiserror::Error;

/// Errors raised by [`Router::route`](crate::Router::route).
#[derive(Debug, Error)]
pub enum RoutingError {
    /// Input was empty or otherwise unusable.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// Input was valid but did not match any registered intent.
    #[error("no intent matched")]
    NoMatch,

    /// An unexpected error occurred inside the classifier.
    #[error("routing internal error: {0}")]
    Internal(String),
}
