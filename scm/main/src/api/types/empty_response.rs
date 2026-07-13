//! [`EmptyResponse`] — shared thin response envelope for accessors whose entire
//! output is a single, already-named domain value.

/// A response envelope wrapping a single value.
///
/// Used across themes by trait methods whose only output is one already-named
/// domain value (e.g. a health classification, a marker token) — avoids declaring
/// a dedicated single-field `*Response` struct per accessor. Flat (no theme
/// prefix): intentionally shared and referenceable from any theme's traits.
#[derive(Debug)]
pub struct EmptyResponse<T> {
    /// The wrapped value.
    pub value: T,
}
