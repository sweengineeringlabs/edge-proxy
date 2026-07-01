//! Lifecycle theme — runtime state management contracts, types, and errors.

pub(crate) mod errors;
pub mod null_lifecycle_monitor;
pub(crate) mod traits;
pub(crate) mod types;

pub use errors::LifecycleError;
pub use null_lifecycle_monitor::NullLifecycleMonitor;
pub use traits::{LifecycleMonitor, Monitor};
pub use types::{
    ComponentHealth, ComponentRequest, ComponentResponse, HealthRequest, HealthResponse,
    HealthStatus, ShutdownRequest, StartBackgroundTasksRequest, StatusRequest, StatusResponse,
};
