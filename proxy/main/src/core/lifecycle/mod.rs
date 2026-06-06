//! Lifecycle theme implementations — no-op monitor for bring-up and tests.

pub(crate) mod null_lifecycle_monitor;

pub(crate) use null_lifecycle_monitor::NullLifecycleMonitor;
