//! Lifecycle theme implementations — no-op monitor for bring-up and tests.

pub(crate) mod component_health;
pub(crate) mod health_response;
pub(crate) mod noop_lifecycle_monitor;

pub(crate) use noop_lifecycle_monitor::NoopLifecycleMonitor;
