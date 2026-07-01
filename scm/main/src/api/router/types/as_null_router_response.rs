//! [`AsNullRouterResponse`] — response for [`Router::as_null_router`](crate::api::router::traits::Router::as_null_router).

use crate::api::router::NullRouter;

/// The erased null-router form, if this implementation is a null object.
pub struct AsNullRouterResponse<'a> {
    /// The null-router reference, or `None` for real implementations.
    pub router: Option<&'a NullRouter>,
}
