//! API layer — proxy dispatch concerns.
//!
//! | Module | Concern |
//! |--------|---------|
//! | [`traits::job`] | Job — top-level entry called by the runtime |
//! | [`traits::router`] | Routing — classify input into an intent |
//! | [`traits::lifecycle_monitor`] | Lifecycle — health, background tasks, shutdown |

pub mod error;
pub mod health;
pub mod null;
pub mod traits;
pub mod types;
pub mod validator;

// Promote core traits to the api/ surface for convenient import in saf/
pub use traits::job::Job;
pub use traits::lifecycle_monitor::LifecycleMonitor;
pub use traits::router::Router;
pub use traits::validator::Validator;
