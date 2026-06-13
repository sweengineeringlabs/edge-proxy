//! Core implementation layer.
//!
//! Holds concrete, domain-agnostic in-house implementations of the `api/`
//! traits, mirroring the api/ theme structure. Downstream Controller crates
//! add domain-specific impls alongside (or replace) these defaults.

pub mod job;
pub mod lifecycle;
pub mod proxy;
pub mod router;
pub mod validator;
