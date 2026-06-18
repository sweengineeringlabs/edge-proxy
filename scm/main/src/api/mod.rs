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

pub use job::{HandlerError, Job, JobError, NullJob, NullJobMarker};
pub use lifecycle::{
    ComponentHealth, HealthReport, HealthStatus, LifecycleError, LifecycleMonitor, Monitor,
    NullLifecycleMonitor,
};
pub use proxy::{ApplicationConfigBuilder, ProxyComposer, ProxyPattern, ProxySvc};
pub use router::{NullRouter, NullRouterMarker, Router, RoutingError};
pub use validator::{NoopValidator, Validator};
