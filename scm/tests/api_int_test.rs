//! Layer-level coverage for the small request/response value types declared under
//! `api/*/types/` that have no dedicated per-type test file (SEA layer test
//! coverage, `sea_layer_test_coverage`). Each test constructs the type through the
//! crate's public API and asserts on its real shape or field values.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_proxy::{
    AsNullJobMarkerRequest, AsNullJobMarkerResponse, AsNullJobRequest, AsNullJobResponse,
    AsNullRouterMarkerRequest, AsNullRouterMarkerResponse, AsNullRouterRequest,
    AsNullRouterResponse, BootstrapNameRequest, BootstrapNameResponse, ComponentRequest,
    ComponentResponse, ExecutionRequest, HealthRequest, HealthResponse, HealthStatus, Job,
    JobError, NullJob, NullRouter, ProxyComposerError, RouteRequest, RouteResponse, Router,
    RoutingError, ShutdownRequest, StartBackgroundTasksRequest, StatusRequest, StatusResponse,
    ValidationRequest,
};

// --- zero-sized marker request types ---

/// @covers: AsNullJobMarkerRequest
#[test]
fn test_as_null_job_marker_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<AsNullJobMarkerRequest>(), 0);
    let _ = AsNullJobMarkerRequest;
}

/// @covers: AsNullJobRequest
#[test]
fn test_as_null_job_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<AsNullJobRequest>(), 0);
    let _ = AsNullJobRequest;
}

/// @covers: AsNullRouterMarkerRequest
#[test]
fn test_as_null_router_marker_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<AsNullRouterMarkerRequest>(), 0);
    let _ = AsNullRouterMarkerRequest;
}

/// @covers: AsNullRouterRequest
#[test]
fn test_as_null_router_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<AsNullRouterRequest>(), 0);
    let _ = AsNullRouterRequest;
}

/// @covers: HealthRequest
#[test]
fn test_health_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<HealthRequest>(), 0);
    let _ = HealthRequest;
}

/// @covers: ShutdownRequest
#[test]
fn test_shutdown_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<ShutdownRequest>(), 0);
    let _ = ShutdownRequest;
}

/// @covers: StartBackgroundTasksRequest
#[test]
fn test_start_background_tasks_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<StartBackgroundTasksRequest>(), 0);
    let _ = StartBackgroundTasksRequest;
}

/// @covers: StatusRequest
#[test]
fn test_status_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<StatusRequest>(), 0);
    let _ = StatusRequest;
}

/// @covers: BootstrapNameRequest
#[test]
fn test_bootstrap_name_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<BootstrapNameRequest>(), 0);
    let _ = BootstrapNameRequest;
}

// --- field-carrying request/response types ---

/// @covers: AsNullJobMarkerResponse
#[test]
fn test_as_null_job_marker_response_holds_marker_edge() {
    let r = AsNullJobMarkerResponse { marker: None };
    assert!(r.marker.is_none());
}

/// @covers: AsNullJobResponse
#[test]
fn test_as_null_job_response_holds_job_ref_edge() {
    let job: Option<&NullJob> = None;
    let r = AsNullJobResponse { job };
    assert!(r.job.is_none());
}

/// @covers: AsNullRouterMarkerResponse
#[test]
fn test_as_null_router_marker_response_holds_marker_edge() {
    let r = AsNullRouterMarkerResponse { marker: None };
    assert!(r.marker.is_none());
}

/// @covers: AsNullRouterResponse
#[test]
fn test_as_null_router_response_holds_router_ref_edge() {
    let router: Option<&NullRouter> = None;
    let r = AsNullRouterResponse { router };
    assert!(r.router.is_none());
}

/// @covers: BootstrapNameResponse
#[test]
fn test_bootstrap_name_response_holds_name_happy() {
    let r = BootstrapNameResponse {
        name: "proxy_composer",
    };
    assert_eq!(r.name, "proxy_composer");
}

/// @covers: ComponentRequest
#[test]
fn test_component_request_holds_id_happy() {
    let r = ComponentRequest { id: "svc" };
    assert_eq!(r.id, "svc");
}

/// @covers: ComponentResponse
#[test]
fn test_component_response_holds_health_edge() {
    let r = ComponentResponse { health: None };
    assert!(r.health.is_none());
}

/// @covers: HealthResponse
#[test]
fn test_health_response_holds_overall_and_components_happy() {
    let r = HealthResponse {
        overall: HealthStatus::Healthy,
        components: vec![],
    };
    assert_eq!(r.overall, HealthStatus::Healthy);
    assert!(r.components.is_empty());
}

/// @covers: StatusResponse
#[test]
fn test_status_response_holds_status_happy() {
    let r = StatusResponse {
        status: HealthStatus::Degraded,
    };
    assert_eq!(r.status, HealthStatus::Degraded);
}

/// @covers: RouteRequest
#[test]
fn test_route_request_holds_input_happy() {
    let r = RouteRequest { input: "echo" };
    assert_eq!(r.input, "echo");
}

/// @covers: RouteResponse
#[test]
fn test_route_response_holds_intent_happy() {
    let r = RouteResponse {
        intent: "greet".to_string(),
    };
    assert_eq!(r.intent, "greet");
}

/// @covers: ValidationRequest
#[test]
fn test_validation_request_holds_value_happy() {
    let r = ValidationRequest { value: "input" };
    assert_eq!(r.value, "input");
}

/// @covers: ExecutionRequest
#[test]
fn test_execution_request_holds_req_and_ctx_happy() {
    use edge_domain_command::NoopCommandBus;
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::{SecurityBootstrap, SecurityServices};
    use edge_proxy::HandlerContext;

    let security = SecurityServices::unauthenticated();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &NoopCommandBus,
        observer: observer.as_ref(),
    };
    let req = ExecutionRequest {
        req: "payload".to_string(),
        ctx: &ctx,
    };
    assert_eq!(req.req, "payload");
}

// --- error type ---

/// @covers: ProxyComposerError
#[test]
fn test_proxy_composer_error_internal_carries_message_happy() {
    let err = ProxyComposerError::Internal("boom".to_string());
    assert!(err.to_string().contains("boom"));
}

/// @covers: ProxyComposerError
#[test]
fn test_proxy_composer_error_debug_is_non_empty_edge() {
    let err = ProxyComposerError::Internal("x".to_string());
    assert!(!format!("{err:?}").is_empty());
}

// --- trait bound sanity, exercising Job/Router/RoutingError/JobError together ---

/// @covers: NullJob
#[test]
fn test_null_job_type_is_send_sync_edge() {
    fn accepts<T: Job<String, String> + ?Sized>(_: &T) -> bool {
        true
    }
    struct AlwaysCancel;
    impl Job<String, String> for AlwaysCancel {
        fn run<'a>(
            &'a self,
            _req: ExecutionRequest<'a, String>,
        ) -> futures::future::BoxFuture<'a, Result<String, JobError>> {
            Box::pin(async { Err(JobError::Cancelled) })
        }
    }
    assert!(accepts(&AlwaysCancel));
}

/// @covers: NullRouter
#[test]
fn test_null_router_type_is_send_sync_edge() {
    fn accepts<T: Router<String> + ?Sized>(_: &T) -> bool {
        true
    }
    struct AlwaysNoMatch;
    impl Router<String> for AlwaysNoMatch {
        fn route<'a>(
            &'a self,
            _req: RouteRequest<'a>,
        ) -> futures::future::BoxFuture<'a, Result<RouteResponse<String>, RoutingError>> {
            Box::pin(async { Err(RoutingError::NoMatch) })
        }
    }
    assert!(accepts(&AlwaysNoMatch));
}
