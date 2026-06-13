//! API layer — proxy dispatch concerns, organised by theme.
//!
//! | Theme | Concern |
//! |-------|---------|
//! | [`job`] | Job — top-level entry called by the runtime |
//! | [`router`] | Routing — classify input into an intent |
//! | [`lifecycle`] | Lifecycle — health, background tasks, shutdown |
//! | [`proxy`] | Proxy — facade handle, config builder, pattern marker |
//! | [`validator`] | Validation — pre-dispatch input checks |

pub mod job;
pub mod lifecycle;
pub mod proxy;
pub mod router;
pub mod validator;
