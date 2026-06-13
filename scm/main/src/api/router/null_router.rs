//! `NullRouter` — type alias for the dynamic dispatch form of [`Router`].

use super::Router;

/// Convenience alias for `dyn Router<String>`.
///
/// Erases the concrete null-router type for callers that need only trait-object
/// dispatch without naming the implementor.
pub type NullRouter = dyn Router<String>;
