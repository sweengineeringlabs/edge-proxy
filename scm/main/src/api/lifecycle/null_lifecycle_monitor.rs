//! `NullLifecycleMonitor` — extension interface for null-object lifecycle monitors.

use crate::api::lifecycle::traits::lifecycle_monitor::LifecycleMonitor;

/// Marker supertrait for null-object [`LifecycleMonitor`] implementations.
///
/// Downstream crates implement this to signal that their monitor is a no-op
/// suitable for tests and early bring-up, without exposing the concrete type.
pub trait NullLifecycleMonitor: LifecycleMonitor {}
