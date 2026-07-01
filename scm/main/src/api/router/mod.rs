//! Router theme — input classification contract and errors.

pub(crate) mod errors;
pub mod null_router;
pub(crate) mod traits;
pub mod types;

pub use errors::RoutingError;
pub use null_router::NullRouter;
pub use traits::Router;
pub use types::{
    AsNullRouterMarkerRequest, AsNullRouterMarkerResponse, AsNullRouterRequest,
    AsNullRouterResponse, NullRouterMarker, RouteRequest, RouteResponse,
};
