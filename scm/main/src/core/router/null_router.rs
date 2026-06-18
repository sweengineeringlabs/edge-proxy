//! `NullRouter` — a no-op `Router` that always returns `NoMatch`.

use futures::future::BoxFuture;

use crate::api::{Router, RoutingError};

/// No-op router that returns `RoutingError::NoMatch` for every input.
///
/// `pub(crate)` — consumers provide their own `Router` implementations.
pub(crate) struct NullRouter;

impl Router<String> for NullRouter {
    fn route<'a>(&'a self, _input: &'a str) -> BoxFuture<'a, Result<String, RoutingError>> {
        Box::pin(async move { Err(RoutingError::NoMatch) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_null_router_always_returns_no_match() {
        let result = NullRouter.route("anything").await;
        assert!(matches!(result, Err(RoutingError::NoMatch)));
    }
}
