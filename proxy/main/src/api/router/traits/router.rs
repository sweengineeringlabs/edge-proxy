//! Router trait — **Routing** concern of the 5-Concern Controller pattern.
//!
//! Classifies an inbound input and returns a domain-defined `Intent` that
//! downstream logic (typically a `HandlerRegistry`) uses to pick the right
//! `Handler`.

use futures::future::BoxFuture;

use crate::api::router::error::RoutingError;

/// Classifies input into a domain-specific intent.
///
/// | Domain | Intent |
/// |--------|--------|
/// | llmboot | `AgentIntent` (naming target agent / pattern) |
/// | security/iam | `ServiceIntent` (auth vs authz vs iam) |
/// | vmisolate | *(absent — VM ops are direct, no routing)* |
pub trait Router<Intent>: Send + Sync
where
    Intent: Send + 'static,
{
    /// Classify the input string and return the resolved intent.
    fn route<'a>(&'a self, input: &'a str) -> BoxFuture<'a, Result<Intent, RoutingError>>;
}
