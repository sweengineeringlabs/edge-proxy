//! Lifecycle theme — runtime state management contracts, types, and errors.

pub(crate) mod dto;
pub(crate) mod errors;
pub(crate) mod traits;
pub(crate) mod vo;

pub use dto::{
    ComponentRequest, HealthRequest, HealthResponse, ShutdownRequest, StartBackgroundTasksRequest,
    StatusRequest,
};
pub use errors::LifecycleError;
pub use traits::{LifecycleMonitor, Monitor, NullLifecycleMonitor};
pub use vo::{ComponentHealth, HealthStatus};
