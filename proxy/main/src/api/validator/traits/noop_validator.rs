//! `NoopValidator` — marker supertrait for no-op validator implementations.

use crate::api::validator::traits::validator::Validator;

/// Marker supertrait for always-passing validator implementations.
pub trait NoopValidator: Validator<Target = (), Error = std::convert::Infallible> {}
