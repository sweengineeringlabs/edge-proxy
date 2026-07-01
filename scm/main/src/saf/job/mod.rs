//! Job SAF sub-facade.

mod job_svc;
mod job_svc_factory;

pub use job_svc::{Job, JOB_CONCERN};
pub use job_svc_factory::JOB_SVC_FACTORY;
