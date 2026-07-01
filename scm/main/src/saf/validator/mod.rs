//! Validator SAF sub-facade.

mod validator_svc;
mod validator_svc_factory;

pub use validator_svc::{Validator, VALIDATOR_CONCERN};
pub use validator_svc_factory::VALIDATOR_SVC_FACTORY;
