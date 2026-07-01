//! Router SAF sub-facade.

mod router_svc;
mod router_svc_factory;

pub use router_svc::{Router, ROUTER_CONCERN};
pub use router_svc_factory::ROUTER_SVC_FACTORY;
