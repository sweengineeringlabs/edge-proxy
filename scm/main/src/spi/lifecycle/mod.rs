//! SPI extension point for the `lifecycle` domain.
//!
//! No external-lib-backed `LifecycleMonitor` variant exists yet — every
//! implementation in this crate lives in `core/lifecycle/`. This module marks
//! the extension point for a future external-lib-backed strategy without
//! providing one prematurely.
