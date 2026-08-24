//! Blanket [`Router`] impl for `Arc<dyn Router<Intent>>` — lets a `Job` pin its
//! `Router` associated type to an erased, runtime-swappable router without
//! naming any concrete router type.

use std::sync::Arc;

use async_trait::async_trait;

use crate::api::{RouteRequest, RouteResponse, Router, RoutingError};

#[async_trait]
impl<Intent> Router<Intent> for Arc<dyn Router<Intent>>
where
    Intent: Send + 'static,
{
    async fn route(&self, req: RouteRequest<'_>) -> Result<RouteResponse<Intent>, RoutingError> {
        (**self).route(req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysMatch;
    #[async_trait]
    impl Router<String> for AlwaysMatch {
        async fn route(
            &self,
            _req: RouteRequest<'_>,
        ) -> Result<RouteResponse<String>, RoutingError> {
            Ok(RouteResponse {
                intent: "matched".to_string(),
            })
        }
    }

    fn rt() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime")
    }

    #[test]
    fn test_arc_dyn_router_route_delegates_to_inner_happy() {
        let router: Arc<dyn Router<String>> = Arc::new(AlwaysMatch);
        let result = rt().block_on(router.route(RouteRequest { input: "x" }));
        assert_eq!(result.unwrap().intent, "matched");
    }

    #[test]
    fn test_arc_dyn_router_route_propagates_error_error() {
        let router: Arc<dyn Router<String>> = Arc::new(crate::core::router::null_router::NullRouter);
        let result = rt().block_on(router.route(RouteRequest { input: "x" }));
        assert!(matches!(result, Err(RoutingError::NoMatch)));
    }
}
