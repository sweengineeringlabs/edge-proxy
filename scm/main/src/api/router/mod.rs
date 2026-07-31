//! Router theme — input classification contract and errors.

pub(crate) mod dto;
pub(crate) mod errors;
pub mod null_router;
pub(crate) mod traits;
pub(crate) mod vo;

pub use dto::{
    AsNullRouterMarkerRequest, AsNullRouterRequest, AsNullRouterResponse, RouteRequest,
    RouteResponse,
};
pub use errors::RoutingError;
pub use null_router::NullRouter;
pub use traits::Router;
pub use vo::NullRouterMarker;
