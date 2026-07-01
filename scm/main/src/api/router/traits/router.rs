//! Router trait — **Routing** concern of the 5-Concern Controller pattern.
//!
//! Classifies an inbound input and returns a domain-defined `Intent` that
//! downstream logic (typically a `HandlerRegistry`) uses to pick the right
//! `Handler`.

use futures::future::BoxFuture;

use crate::api::router::errors::RoutingError;
use crate::api::router::types::{
    AsNullRouterMarkerRequest, AsNullRouterMarkerResponse, AsNullRouterRequest,
    AsNullRouterResponse, RouteRequest, RouteResponse,
};

/// Classifies input into a domain-specific intent.
///
/// | Domain | Intent |
/// |--------|--------|
/// | llmboot | `AgentIntent` (naming target agent / pattern) |
/// | security/iam | `ServiceIntent` (auth vs authz vs iam) |
/// | vmisolate | *(absent — VM ops are direct, no routing)* |
pub trait Router<Intent = String>: Send + Sync
where
    Intent: Send + 'static,
{
    /// Classify the input string and return the resolved intent.
    fn route<'a>(
        &'a self,
        req: RouteRequest<'a>,
    ) -> BoxFuture<'a, Result<RouteResponse<Intent>, RoutingError>>;

    /// Return a reference to the erased null-router form, if this implementation
    /// is a null object.  Returns `None` by default.
    fn as_null_router(
        &self,
        _req: AsNullRouterRequest,
    ) -> Result<AsNullRouterResponse<'_>, RoutingError> {
        Ok(AsNullRouterResponse { router: None })
    }

    /// Return a [`NullRouterMarker`](crate::api::router::types::NullRouterMarker) token if
    /// this implementation is a null-object router, or `None` for real implementations.  Used
    /// to identify inert routers in bring-up and testing contexts without downcasting.
    fn as_null_router_marker(
        &self,
        _req: AsNullRouterMarkerRequest,
    ) -> Result<AsNullRouterMarkerResponse, RoutingError> {
        Ok(AsNullRouterMarkerResponse { marker: None })
    }
}
