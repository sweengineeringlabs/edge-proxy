//! NullLifecycleMonitor SAF sub-facade.

mod null_lifecycle_monitor_svc;
mod null_lifecycle_monitor_svc_factory;

pub use null_lifecycle_monitor_svc::{NullLifecycleMonitor, NULL_LIFECYCLE_MONITOR_CONCERN};
pub use null_lifecycle_monitor_svc_factory::NULL_LIFECYCLE_MONITOR_SVC_FACTORY;
