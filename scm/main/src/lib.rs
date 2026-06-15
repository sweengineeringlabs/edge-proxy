//! # edge-proxy
//!
//! The L2 Proxy — dispatch facade sitting between ingress and domain.
//!
//! ## Concerns
//!
//! 1. **Job** — top-level entry called by the runtime.
//! 2. **Routing** — classify input into an intent.
//! 3. **Lifecycle** — health, background tasks, shutdown.
//!
//! ## Canonical usage shape
//!
//! ```ignore
//! use std::sync::Arc;
//! use edge_proxy::{Job, JobError, Router, ProxySvc};
//! use edge_domain::{Handler, HandlerRegistry, new_handler_registry};
//!
//! // 1. Build a registry and register domain handlers.
//! let registry: Arc<HandlerRegistry<MyReq, MyResponse>> = new_handler_registry();
//! registry.register(Arc::new(MyHandler::new()));
//!
//! // 2. Implement Job backed by Router + HandlerRegistry.
//! // 3. Runtime holds Arc<dyn Job<MyReq, MyResponse>> and calls .run(req).
//! ```
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

pub use saf::ApplicationConfigBuilder;
pub use saf::CommandBus;
pub use saf::ComponentHealth;
pub use saf::HandlerContext;
pub use saf::HandlerError;
pub use saf::HealthReport;
pub use saf::HealthStatus;
pub use saf::Job;
pub use saf::JobError;
pub use saf::LifecycleError;
pub use saf::LifecycleMonitor;
pub use saf::NoopValidator;
pub use saf::NullJob;
pub use saf::NullJobMarker;
pub use saf::NullLifecycleMonitor;
pub use saf::NullMonitor;
pub use saf::NullRouter;
pub use saf::NullRouterMarker;
pub use saf::ProxyComposer;
pub use saf::ProxyPattern;
pub use saf::ProxySvc;
pub use saf::Router;
pub use saf::RoutingError;
pub use saf::SecurityContext;
pub use saf::Validator;
pub use saf::JOB_CONCERN;
pub use saf::LIFECYCLE_MONITOR_CONCERN;
pub use saf::MONITOR_CONCERN;
pub use saf::NOOP_VALIDATOR_CONCERN;
pub use saf::NULL_LIFECYCLE_MONITOR_CONCERN;
pub use saf::PROXY_COMPOSER_CONCERN;
pub use saf::ROUTER_CONCERN;
pub use saf::VALIDATOR_CONCERN;
