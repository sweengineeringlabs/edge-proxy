//! API layer — proxy dispatch concerns, organised by theme.
//!
//! | Theme | Concern |
//! |-------|---------|
//! | [`job`] | Job — top-level entry called by the runtime |
//! | [`router`] | Routing — classify input into an intent |
//! | [`lifecycle`] | Lifecycle — health, background tasks, shutdown |
//! | [`proxy`] | Proxy — facade handle, config builder, pattern marker |
//! | [`validator`] | Validation — pre-dispatch input checks |

mod job;
mod lifecycle;
mod proxy;
mod router;
mod validator;

pub use job::{
    AsNullJobMarkerRequest, AsNullJobMarkerResponse, AsNullJobRequest, AsNullJobResponse,
    ExecutionRequest, HandlerError, Job, JobError, NullJob, NullJobMarker,
};
pub use lifecycle::{
    ComponentHealth, ComponentRequest, ComponentResponse, HealthRequest, HealthResponse,
    HealthStatus, LifecycleError, LifecycleMonitor, Monitor, NullLifecycleMonitor, ShutdownRequest,
    StartBackgroundTasksRequest, StatusRequest, StatusResponse,
};
pub use proxy::{
    ApplicationConfigBuilder, BootstrapNameRequest, BootstrapNameResponse, ProxyComposer,
    ProxyComposerError, ProxyPattern, ProxySvc,
};
// Crate-internal only: exposes the SEA Rule 121 api/core mirror shim
// (`proxy::proxy_svc`) so core/ can reference its alias in tests without
// widening `proxy`'s own module visibility.
pub(crate) use proxy::proxy_svc;
pub use router::{
    AsNullRouterMarkerRequest, AsNullRouterMarkerResponse, AsNullRouterRequest,
    AsNullRouterResponse, NullRouter, NullRouterMarker, RouteRequest, RouteResponse, Router,
    RoutingError,
};
pub use validator::{NoopValidator, ValidationRequest, Validator};
