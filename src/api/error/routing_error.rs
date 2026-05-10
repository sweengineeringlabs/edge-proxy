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

    /// Domain-specific failure raised while classifying.
    #[error("routing error: {0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_match_display() {
        assert!(RoutingError::NoMatch.to_string().contains("matched"));
    }

    #[test]
    fn test_invalid_input_display() {
        let e = RoutingError::InvalidInput("empty body".into());
        assert!(e.to_string().contains("empty body"));
    }

    #[test]
    fn test_other_display() {
        let e = RoutingError::Other("classifier timeout".into());
        assert!(e.to_string().contains("classifier timeout"));
    }
}
