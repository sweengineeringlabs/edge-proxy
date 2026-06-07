//! SAF layer — proxy public facade.

mod proxy_svc;

// Facade handle and application config builder
pub use crate::api::types::ApplicationConfigBuilder;
pub use crate::api::types::ProxySvc;

// Concern traits (re-exported from api/ themes)
pub use crate::api::job::Job;
pub use crate::api::lifecycle::LifecycleMonitor;
pub use crate::api::router::Router;
pub use crate::api::validator::Validator;

// Null lifecycle monitor extension traits (for downstream impls)
pub use crate::api::lifecycle::Monitor as NullMonitor;
pub use crate::api::lifecycle::NullLifecycleMonitorApi;

// Validator extension traits (for downstream impls)
pub use crate::api::validator::NoopValidator;

// Marker types
pub use crate::api::types::ProxyPattern;

// Error types
pub use crate::api::job::{HandlerError, JobError};
pub use crate::api::lifecycle::LifecycleError;
pub use crate::api::router::RoutingError;

// Health types
pub use crate::api::health::{ComponentHealth, HealthReport, HealthStatus};
