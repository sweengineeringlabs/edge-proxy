//! SPI — extension hooks for downstream consumers.

mod canonical;
pub(crate) mod job;
pub(crate) mod lifecycle;
pub(crate) mod proxy;
pub(crate) mod router;
#[cfg(test)]
mod tests;
pub(crate) mod validator;

pub(crate) use canonical::CanonicalFactory;
