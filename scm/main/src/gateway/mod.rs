//! Gateway layer — boundary types wiring ingress and egress adapters.

pub(crate) mod egress;
pub(crate) mod ingress;

pub use crate::saf::ApplicationConfigBuilder;
pub use crate::saf::ComponentHealth;
pub use crate::saf::HandlerError;
pub use crate::saf::HealthReport;
pub use crate::saf::HealthStatus;
pub use crate::saf::Job;
pub use crate::saf::JobError;
pub use crate::saf::LifecycleError;
pub use crate::saf::LifecycleMonitor;
pub use crate::saf::NoopValidator;
pub use crate::saf::NullJob;
pub use crate::saf::NullJobMarker;
pub use crate::saf::NullLifecycleMonitor;
pub use crate::saf::NullMonitor;
pub use crate::saf::NullRouter;
pub use crate::saf::NullRouterMarker;
pub use crate::saf::ProxyComposer;
pub use crate::saf::ProxyPattern;
pub use crate::saf::ProxySvc;
pub use crate::saf::Router;
pub use crate::saf::RoutingError;
pub use crate::saf::SecurityContext;
pub use crate::saf::Validator;
pub use crate::saf::JOB_CONCERN;
pub use crate::saf::LIFECYCLE_MONITOR_CONCERN;
pub use crate::saf::MONITOR_CONCERN;
pub use crate::saf::NOOP_VALIDATOR_CONCERN;
pub use crate::saf::NULL_LIFECYCLE_MONITOR_CONCERN;
pub use crate::saf::PROXY_COMPOSER_CONCERN;
pub use crate::saf::ROUTER_CONCERN;
pub use crate::saf::VALIDATOR_CONCERN;
