//! Lifecycle request/response envelope types.

pub mod component_request;
pub mod health_request;
pub mod health_response;
pub mod shutdown_request;
pub mod start_background_tasks_request;
pub mod status_request;

pub use component_request::ComponentRequest;
pub use health_request::HealthRequest;
pub use health_response::HealthResponse;
pub use shutdown_request::ShutdownRequest;
pub use start_background_tasks_request::StartBackgroundTasksRequest;
pub use status_request::StatusRequest;
