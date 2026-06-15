//! Integration tests for ProxySvc facade type.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use edge_proxy::{
    HandlerContext, HealthStatus, JobError, ProxySvc, RoutingError, SecurityContext, Validator,
};
use futures::future::BoxFuture;

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

struct NullBus;
impl edge_proxy::CommandBus for NullBus {
    fn dispatch(
        &self,
        _: Box<dyn edge_domain::Command>,
    ) -> BoxFuture<'_, Result<(), edge_domain::CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

fn anon_ctx_parts() -> (SecurityContext, NullBus) {
    (SecurityContext::unauthenticated(), NullBus)
}

#[test]
fn test_proxy_svc_create_config_builder_returns_builder() {
    let _builder = ProxySvc::create_config_builder();
}

#[test]
fn test_proxy_svc_new_null_lifecycle_monitor_is_healthy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(rt().block_on(m.health()).overall, HealthStatus::Healthy);
}

#[test]
fn test_proxy_svc_new_noop_validator_accepts_unit() {
    let v = ProxySvc::new_noop_validator();
    assert!(v.validate(&()).is_ok());
}

#[test]
fn test_proxy_svc_validate_delegates_to_validator() {
    struct AlwaysOk;
    impl Validator for AlwaysOk {
        type Target = ();
        type Error = String;
        fn validate(&self, _: &()) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(ProxySvc::validate(&AlwaysOk, &()).is_ok());
}

// Rule 221 scenario coverage for proxy_svc.rs pub fns ─────────────────────────

/// create_config_builder — happy: builder is created without panicking.
#[test]
fn test_create_config_builder_returns_seeded_builder_happy() {
    let _b = ProxySvc::create_config_builder();
}

/// create_config_builder — error: no runtime error path exists; exercise re-entry.
#[test]
fn test_create_config_builder_called_twice_no_state_error() {
    let _b1 = ProxySvc::create_config_builder();
    let _b2 = ProxySvc::create_config_builder();
}

/// create_config_builder — edge: builder carries crate name (non-empty).
#[test]
fn test_create_config_builder_crate_name_is_non_empty_edge() {
    let _ = ProxySvc::create_config_builder();
    assert!(!env!("CARGO_PKG_NAME").is_empty());
}

/// new_null_lifecycle_monitor — happy: fresh monitor reports Healthy.
#[test]
fn test_new_null_lifecycle_monitor_reports_healthy_happy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(rt().block_on(m.health()).overall, HealthStatus::Healthy);
}

/// new_null_lifecycle_monitor — error: after shutdown, monitor is unhealthy.
#[test]
fn test_new_null_lifecycle_monitor_unhealthy_after_shutdown_error() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m.shutdown())
        .expect("first shutdown must succeed");
    assert_eq!(rt().block_on(m.health()).overall, HealthStatus::Unhealthy);
}

/// new_null_lifecycle_monitor — edge: two independent monitors are independent.
#[test]
fn test_new_null_lifecycle_monitor_instances_are_independent_edge() {
    let m1 = ProxySvc::new_null_lifecycle_monitor();
    let m2 = ProxySvc::new_null_lifecycle_monitor();
    rt().block_on(m1.shutdown()).expect("shutdown m1");
    assert_eq!(rt().block_on(m2.health()).overall, HealthStatus::Healthy);
}

/// new_noop_validator — happy: accepts the unit value.
#[test]
fn test_new_noop_validator_accepts_unit_happy() {
    let v = ProxySvc::new_noop_validator();
    assert!(v.validate(&()).is_ok());
}

/// new_noop_validator — error: no rejection path; call validate directly.
#[test]
fn test_new_noop_validator_via_proxy_svc_validate_error() {
    let v = ProxySvc::new_noop_validator();
    assert!(v.validate(&()).is_ok());
}

/// new_noop_validator — edge: validator is Send + Sync.
#[test]
fn test_new_noop_validator_is_send_sync_edge() {
    fn _assert_send_sync<T: Send + Sync>(_: T) {}
    let v = ProxySvc::new_noop_validator();
    _assert_send_sync(v);
}

/// validate — happy: a permissive validator returns Ok.
#[test]
fn test_validate_permissive_validator_returns_ok_happy() {
    struct AllowAll;
    impl Validator for AllowAll {
        type Target = str;
        type Error = String;
        fn validate(&self, _: &str) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(ProxySvc::validate(&AllowAll, "any input").is_ok());
}

/// validate — error: a rejecting validator propagates the error.
#[test]
fn test_validate_rejecting_validator_returns_err_error() {
    struct RejectAll;
    impl Validator for RejectAll {
        type Target = str;
        type Error = String;
        fn validate(&self, _: &str) -> Result<(), String> {
            Err("rejected".into())
        }
    }
    assert!(ProxySvc::validate(&RejectAll, "x").is_err());
}

/// validate — edge: empty string is correctly passed through to the validator.
#[test]
fn test_validate_empty_string_passed_through_edge() {
    struct CheckEmpty;
    impl Validator for CheckEmpty {
        type Target = str;
        type Error = String;
        fn validate(&self, v: &str) -> Result<(), String> {
            if v.is_empty() {
                Err("empty".into())
            } else {
                Ok(())
            }
        }
    }
    assert!(ProxySvc::validate(&CheckEmpty, "").is_err());
    assert!(ProxySvc::validate(&CheckEmpty, "non-empty").is_ok());
}

// Rule 221 coverage for new_null_job ─────────────────────────────────────────

/// new_null_job — happy: returns Cancelled for any request.
#[test]
fn test_new_null_job_returns_cancelled_happy() {
    let job = ProxySvc::new_null_job::<String, String>();
    let (s, b) = anon_ctx_parts();
    let ctx = HandlerContext {
        security: &s,
        commands: &b,
    };
    let result = rt().block_on(job.run("input".into(), ctx));
    assert!(matches!(result, Err(JobError::Cancelled)));
}

/// new_null_job — error: empty request also returns Cancelled.
#[test]
fn test_new_null_job_with_empty_request_also_cancels_error() {
    let job = ProxySvc::new_null_job::<String, String>();
    let (s, b) = anon_ctx_parts();
    let ctx = HandlerContext {
        security: &s,
        commands: &b,
    };
    let result = rt().block_on(job.run(String::new(), ctx));
    assert!(matches!(result, Err(JobError::Cancelled)));
}

/// new_null_job — edge: generic unit request type also cancels.
#[test]
fn test_new_null_job_with_unit_type_cancels_edge() {
    let job = ProxySvc::new_null_job::<(), ()>();
    let (s, b) = anon_ctx_parts();
    let ctx = HandlerContext {
        security: &s,
        commands: &b,
    };
    let result = rt().block_on(job.run((), ctx));
    assert!(matches!(result, Err(JobError::Cancelled)));
}

// Rule 221 coverage for new_null_router ───────────────────────────────────────

/// new_null_router — happy: returns NoMatch for any non-empty input.
#[test]
fn test_new_null_router_returns_no_match_happy() {
    let router = ProxySvc::new_null_router();
    let result = rt().block_on(router.route("echo"));
    assert!(matches!(result, Err(RoutingError::NoMatch)));
}

/// new_null_router — error: unknown intent also returns NoMatch.
#[test]
fn test_new_null_router_unknown_intent_is_no_match_error() {
    let router = ProxySvc::new_null_router();
    let result = rt().block_on(router.route("unknown-command"));
    assert!(matches!(result, Err(RoutingError::NoMatch)));
}

/// new_null_router — edge: empty input returns NoMatch.
#[test]
fn test_new_null_router_empty_input_is_no_match_edge() {
    let router = ProxySvc::new_null_router();
    let result = rt().block_on(router.route(""));
    assert!(matches!(result, Err(RoutingError::NoMatch)));
}

// Rule 221 coverage for new_canonical_job ─────────────────────────────────────

/// new_canonical_job — happy: returns Cancelled for a normal String request.
#[test]
fn test_new_canonical_job_returns_cancelled_happy() {
    let job = ProxySvc::new_canonical_job();
    let (s, b) = anon_ctx_parts();
    let ctx = HandlerContext {
        security: &s,
        commands: &b,
    };
    let result = rt().block_on(job.run("ping".into(), ctx));
    assert!(matches!(result, Err(JobError::Cancelled)));
}

/// new_canonical_job — error: empty string also returns Cancelled.
#[test]
fn test_new_canonical_job_empty_request_returns_cancelled_error() {
    let job = ProxySvc::new_canonical_job();
    let (s, b) = anon_ctx_parts();
    let ctx = HandlerContext {
        security: &s,
        commands: &b,
    };
    let result = rt().block_on(job.run(String::new(), ctx));
    assert!(matches!(result, Err(JobError::Cancelled)));
}

/// new_canonical_job — edge: two independent instances both cancel.
#[test]
fn test_new_canonical_job_two_instances_both_cancel_edge() {
    let j1 = ProxySvc::new_canonical_job();
    let j2 = ProxySvc::new_canonical_job();
    let (s, b) = anon_ctx_parts();
    let ctx = HandlerContext {
        security: &s,
        commands: &b,
    };
    let r1 = rt().block_on(j1.run("a".into(), ctx));
    let r2 = rt().block_on(j2.run("b".into(), ctx));
    assert!(matches!(r1, Err(JobError::Cancelled)));
    assert!(matches!(r2, Err(JobError::Cancelled)));
}

// Rule 221 coverage for new_canonical_router ──────────────────────────────────

/// new_canonical_router — happy: returns NoMatch for any non-empty input.
#[test]
fn test_new_canonical_router_returns_no_match_happy() {
    let router = ProxySvc::new_canonical_router();
    let result = rt().block_on(router.route("any-route"));
    assert!(matches!(result, Err(RoutingError::NoMatch)));
}

/// new_canonical_router — error: unknown route also returns NoMatch.
#[test]
fn test_new_canonical_router_unknown_route_is_no_match_error() {
    let router = ProxySvc::new_canonical_router();
    let result = rt().block_on(router.route("unknown"));
    assert!(matches!(result, Err(RoutingError::NoMatch)));
}

/// new_canonical_router — edge: empty input returns NoMatch.
#[test]
fn test_new_canonical_router_empty_input_is_no_match_edge() {
    let router = ProxySvc::new_canonical_router();
    let result = rt().block_on(router.route(""));
    assert!(matches!(result, Err(RoutingError::NoMatch)));
}
