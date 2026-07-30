//! `NullJobMarker` — marker type for null-object job implementations.

/// Marker for null-object implementations of [`crate::api::job::traits::job::Job`].
///
/// Null jobs always cancel or no-op. Include this in the type signature when
/// you want to signal that the job is intentionally inert (bring-up, testing).
pub struct NullJobMarker;
