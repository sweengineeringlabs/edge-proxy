//! `NullLifecycleMonitorApi` — marker supertrait for no-op lifecycle monitors.

use crate::api::traits::lifecycle_monitor::LifecycleMonitor;

/// Marker supertrait for no-op [`LifecycleMonitor`] implementations.
pub trait NullLifecycleMonitorApi: LifecycleMonitor {}
