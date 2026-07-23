//! Lifecycle theme — runtime state management contracts, types, and errors.

pub(crate) mod errors;
pub(crate) mod traits;
pub(crate) mod types;

pub use errors::LifecycleError;
pub use traits::{LifecycleMonitor, Monitor, NullLifecycleMonitor};
pub use types::{
    ComponentHealth, ComponentRequest, HealthRequest, HealthResponse, HealthStatus,
    ShutdownRequest, StartBackgroundTasksRequest, StatusRequest,
};
