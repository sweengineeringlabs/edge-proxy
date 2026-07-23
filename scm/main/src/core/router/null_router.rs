//! `NullRouter` — a no-op `Router` that always returns `NoMatch`.

use async_trait::async_trait;

use crate::api::{RouteRequest, RouteResponse, Router, RoutingError};

/// No-op router that returns `RoutingError::NoMatch` for every input.
///
/// `pub(crate)` — consumers provide their own `Router` implementations.
pub(crate) struct NullRouter;

#[async_trait]
impl Router<String> for NullRouter {
    async fn route(&self, _req: RouteRequest<'_>) -> Result<RouteResponse<String>, RoutingError> {
        Err(RoutingError::NoMatch)
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
