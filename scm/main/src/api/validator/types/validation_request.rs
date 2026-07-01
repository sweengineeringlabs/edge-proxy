//! [`ValidationRequest`] — request for [`Validator::validate`](crate::api::validator::traits::Validator::validate).

/// Request to validate a value before it enters the dispatch pipeline.
pub struct ValidationRequest<'a, T: ?Sized> {
    /// The value to validate.
    pub value: &'a T,
}
