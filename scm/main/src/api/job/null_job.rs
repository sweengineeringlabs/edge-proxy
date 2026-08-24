//! `NullJob` — type alias for the dynamic dispatch form of [`Job`].

use std::sync::Arc;

use super::Job;
use crate::api::Router;

/// Convenience alias for `dyn Job<String, String>`, pinned to an erased,
/// runtime-swappable router (`Arc<dyn Router<String>>`).
///
/// Erases the concrete null-job type and unifies callers on a single
/// trait-object pointer without naming the implementor or its router.
pub type NullJob = dyn Job<
    String,
    String,
    Intent = String,
    Router = Arc<dyn Router<String>>,
>;
