//! SAF layer — proxy public facade.

mod job_svc;
mod lifecycle_monitor_svc;
mod monitor_svc;
mod noop_validator_svc;
mod null_lifecycle_monitor_svc;
mod proxy;
mod proxy_svc;
mod router_svc;
mod validator_svc;

// Context types (from edge-domain, not via api/)
pub use edge_domain::CommandBus;
pub use edge_domain::HandlerContext;
pub use edge_domain::SecurityContext;

pub use job_svc::*;
pub use lifecycle_monitor_svc::*;
pub use monitor_svc::*;
pub use noop_validator_svc::*;
pub use null_lifecycle_monitor_svc::*;
pub use proxy::*;
pub use proxy_svc::*;
pub use router_svc::*;
pub use validator_svc::*;
