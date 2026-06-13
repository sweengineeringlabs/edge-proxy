//! `NullJob` — type alias for the dynamic dispatch form of [`Job`].

use super::Job;

/// Convenience alias for `dyn Job<String, String>`.
///
/// Erases the concrete null-job type and unifies callers on a single
/// trait-object pointer without naming the implementor.
pub type NullJob = dyn Job<String, String>;
