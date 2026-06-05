//! SAF layer — proxy public facade.

mod proxy_svc;

// Facade handle and application config builder
pub use crate::api::types::ApplicationConfigBuilder;
pub use crate::api::types::ProxySvc;

// Concern traits (re-exported from api/ surface)
pub use crate::api::Job;
pub use crate::api::LifecycleMonitor;
pub use crate::api::Router;
pub use crate::api::Validator;

// Null lifecycle monitor extension traits (for downstream impls)
pub use crate::api::null::lifecycle::Monitor as NullMonitor;
pub use crate::api::null::lifecycle::NullLifecycleMonitorApi;

// Validator extension traits (for downstream impls)
pub use crate::api::validator::noop_validator::NoopValidator;

// Marker types
pub use crate::api::types::ProxyPattern;

// Error types
pub use crate::api::error::{HandlerError, JobError, LifecycleError, RoutingError};

// Health types
pub use crate::api::health::{ComponentHealth, HealthReport, HealthStatus};
