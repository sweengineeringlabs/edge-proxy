//! Monitor SAF sub-facade.

mod monitor_svc;
mod monitor_svc_factory;

pub use monitor_svc::{NullMonitor, MONITOR_CONCERN};
pub use monitor_svc_factory::MONITOR_SVC_FACTORY;
