//! [`RouterResponse`] — response for [`Job::router`](crate::api::job::traits::Job::router).

/// This `Job`'s wired [`Router`](crate::api::Router) — a compile-time witness that a real
/// router is behind every `Job`, not just documented (SEA#8).
pub struct RouterResponse<'a, R> {
    /// The wired router.
    pub router: &'a R,
}
