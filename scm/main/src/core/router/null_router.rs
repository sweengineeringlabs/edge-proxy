//! `NullRouter` — a no-op `Router` that always returns `NoMatch`.

use futures::future::BoxFuture;

use crate::api::{RouteRequest, RouteResponse, Router, RoutingError};

/// No-op router that returns `RoutingError::NoMatch` for every input.
///
/// `pub(crate)` — consumers provide their own `Router` implementations.
pub(crate) struct NullRouter;

impl Router<String> for NullRouter {
    fn route<'a>(
        &'a self,
        _req: RouteRequest<'a>,
    ) -> BoxFuture<'a, Result<RouteResponse<String>, RoutingError>> {
        Box::pin(async move { Err(RoutingError::NoMatch) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_null_router_always_returns_no_match() {
        let result = NullRouter.route(RouteRequest { input: "anything" }).await;
        assert!(matches!(result, Err(RoutingError::NoMatch)));
    }
}
