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

// Concrete types — sourced directly from api/, never through saf/ (saf/ re-exports
// trait contracts and service identity constants only).
pub use api::ApplicationConfigBuilder;
pub use api::AsNullJobMarkerRequest;
pub use api::AsNullJobMarkerResponse;
pub use api::AsNullJobRequest;
pub use api::AsNullJobResponse;
pub use api::AsNullRouterMarkerRequest;
pub use api::AsNullRouterMarkerResponse;
pub use api::AsNullRouterRequest;
pub use api::AsNullRouterResponse;
pub use api::BootstrapNameRequest;
pub use api::BootstrapNameResponse;
pub use api::ComponentHealth;
pub use api::ComponentRequest;
pub use api::ComponentResponse;
pub use api::ExecutionRequest;
pub use api::HandlerError;
pub use api::HealthRequest;
pub use api::HealthResponse;
pub use api::HealthStatus;
pub use api::JobError;
pub use api::LifecycleError;
pub use api::NullJob;
pub use api::NullJobMarker;
pub use api::NullRouter;
pub use api::NullRouterMarker;
pub use api::ProxyComposerError;
pub use api::ProxyPattern;
pub use api::ProxySvc;
pub use api::RouteRequest;
pub use api::RouteResponse;
pub use api::RoutingError;
pub use api::ShutdownRequest;
pub use api::StartBackgroundTasksRequest;
pub use api::StatusRequest;
pub use api::StatusResponse;
pub use api::ValidationRequest;

// Trait contracts — via saf/
pub use saf::CommandBus;
pub use saf::HandlerContext;
pub use saf::Job;
pub use saf::LifecycleMonitor;
pub use saf::NoopValidator;
pub use saf::NullLifecycleMonitor;
pub use saf::NullMonitor;
pub use saf::ProxyComposer;
pub use saf::Router;
pub use saf::SecurityContext;
pub use saf::Validator;

// SAF service identity constants
pub use saf::JOB_CONCERN;
pub use saf::JOB_SVC_FACTORY;
pub use saf::LIFECYCLE_MONITOR_CONCERN;
pub use saf::LIFECYCLE_MONITOR_SVC_FACTORY;
pub use saf::MONITOR_CONCERN;
pub use saf::MONITOR_SVC_FACTORY;
pub use saf::NOOP_VALIDATOR_CONCERN;
pub use saf::NOOP_VALIDATOR_SVC_FACTORY;
pub use saf::NULL_LIFECYCLE_MONITOR_CONCERN;
pub use saf::NULL_LIFECYCLE_MONITOR_SVC_FACTORY;
pub use saf::PROXY_COMPOSER_CONCERN;
pub use saf::PROXY_COMPOSER_SVC_FACTORY;
pub use saf::ROUTER_CONCERN;
pub use saf::ROUTER_SVC_FACTORY;
pub use saf::VALIDATOR_CONCERN;
pub use saf::VALIDATOR_SVC_FACTORY;
