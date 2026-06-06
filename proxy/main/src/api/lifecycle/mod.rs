//! Lifecycle theme — runtime state management contracts and errors.

pub(crate) mod error;
pub(crate) mod traits;

pub use error::LifecycleError;
pub use traits::{LifecycleMonitor, Monitor, NullLifecycleMonitorApi};
