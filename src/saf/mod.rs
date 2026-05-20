//! SAF layer — proxy public facade.

mod factory;
mod validator;

// Factory functions
pub use factory::new_null_lifecycle_monitor;

// Validator wrapper
pub use validator::validate;

pub use crate::api::application_config_builder::ApplicationConfigBuilder;
pub use crate::api::architecture_config_builder::ArchitectureConfigBuilder;

// Concern traits
pub use crate::api::job::Job;
pub use crate::api::lifecycle_monitor::LifecycleMonitor;
pub use crate::api::router::Router;

// Error types
pub use crate::api::error::{HandlerError, JobError, LifecycleError, RoutingError};

// Health types
pub use crate::api::health::{ComponentHealth, HealthReport, HealthStatus};
