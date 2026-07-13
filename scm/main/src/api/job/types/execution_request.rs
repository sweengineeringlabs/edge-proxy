//! [`ExecutionRequest`] — request for [`Job::run`](crate::api::job::traits::Job::run).
//!
//! Identical in shape to [`edge_domain_handler::ExecutionRequest`] (payload + request-scoped
//! [`HandlerContext`](edge_domain_handler::HandlerContext)) — re-exported wholesale rather than
//! locally re-declared so `api/` never names the foreign `HandlerContext` type in a type
//! position (SEA `no_foreign_type`).

pub use edge_domain_handler::ExecutionRequest;
