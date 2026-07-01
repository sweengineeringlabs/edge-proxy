//! LifecycleMonitor SAF sub-facade.

mod lifecycle_monitor_svc;
mod lifecycle_monitor_svc_factory;

pub use lifecycle_monitor_svc::{LifecycleMonitor, LIFECYCLE_MONITOR_CONCERN};
pub use lifecycle_monitor_svc_factory::LIFECYCLE_MONITOR_SVC_FACTORY;
