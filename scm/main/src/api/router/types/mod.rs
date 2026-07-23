//! Router public types — marker and value types for the router concern.

pub mod ass;
pub mod null_router_marker;
pub mod route_request;
pub mod route_response;

pub use ass::{AsNullRouterMarkerRequest, AsNullRouterRequest, AsNullRouterResponse};
pub use null_router_marker::NullRouterMarker;
pub use route_request::RouteRequest;
pub use route_response::RouteResponse;
