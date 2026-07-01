//! SAF layer — proxy public facade.

mod job;
mod lifecycle;
mod monitor;
mod noop_validator_svc;
mod noop_validator_svc_factory;
mod null;
mod proxy;
mod proxy_svc;
mod router;
mod validator;

// Context types (from edge-domain, not via api/)
pub use edge_domain_command::CommandBus;
pub use edge_domain_handler::HandlerContext;
pub use edge_domain_security::SecurityContext;

pub use job::{Job, JOB_CONCERN, JOB_SVC_FACTORY};
pub use lifecycle::{LifecycleMonitor, LIFECYCLE_MONITOR_CONCERN, LIFECYCLE_MONITOR_SVC_FACTORY};
pub use monitor::{NullMonitor, MONITOR_CONCERN, MONITOR_SVC_FACTORY};
pub use noop_validator_svc::{NoopValidator, NOOP_VALIDATOR_CONCERN};
pub use noop_validator_svc_factory::NOOP_VALIDATOR_SVC_FACTORY;
pub use null::{
    NullLifecycleMonitor, NULL_LIFECYCLE_MONITOR_CONCERN, NULL_LIFECYCLE_MONITOR_SVC_FACTORY,
};
pub use proxy::{PROXY_COMPOSER_CONCERN, PROXY_COMPOSER_SVC_FACTORY};
pub use proxy_svc::ProxyComposer;
pub use router::{Router, ROUTER_CONCERN, ROUTER_SVC_FACTORY};
pub use validator::{Validator, VALIDATOR_CONCERN, VALIDATOR_SVC_FACTORY};
