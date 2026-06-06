//! `Monitor` — interface counterpart for the null lifecycle monitor impl.

use crate::api::lifecycle::traits::lifecycle_monitor::LifecycleMonitor;

/// Marker trait for no-op lifecycle monitor implementations.
pub trait Monitor: LifecycleMonitor + Send + Sync {}
