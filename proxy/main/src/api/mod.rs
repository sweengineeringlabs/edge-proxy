//! API layer — proxy dispatch concerns, organised by theme.
//!
//! | Theme | Concern |
//! |-------|---------|
//! | [`job`] | Job — top-level entry called by the runtime |
//! | [`router`] | Routing — classify input into an intent |
//! | [`lifecycle`] | Lifecycle — health, background tasks, shutdown |
//! | [`health`] | Health — report types consumed by lifecycle and handlers |
//! | [`validator`] | Validation — pre-dispatch input checks |

pub mod health;
pub mod job;
pub mod lifecycle;
pub mod router;
pub mod types;
pub mod validator;
