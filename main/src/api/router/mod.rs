//! Router theme — input classification contract and errors.

pub(crate) mod error;
pub(crate) mod traits;

pub use error::RoutingError;
pub use traits::Router;
