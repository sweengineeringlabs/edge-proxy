//! API layer — proxy dispatch concerns.
//!
//! | File | Concern |
//! |------|---------|
//! | [`job`] | Job — top-level entry called by the runtime |
//! | [`router`] | Routing — classify input into an intent |
//! | [`lifecycle_monitor`] | Lifecycle — health, background tasks, shutdown |

pub mod application_config_builder;
pub mod architecture_config_builder;
pub mod error;
pub mod health;

pub mod job;
pub mod lifecycle_monitor;
pub mod router;

pub mod proxy_pattern;
pub mod traits;

pub mod null_lifecycle_monitor;
pub mod validator;
