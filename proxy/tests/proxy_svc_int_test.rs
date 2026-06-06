//! Integration tests for ProxySvc facade type.

use edge_proxy::{HealthStatus, ProxySvc, Validator};

/// @covers: ProxySvc::create_config_builder
#[test]
fn test_proxy_svc_create_config_builder_returns_builder() {
    let _builder = ProxySvc::create_config_builder();
}

/// @covers: ProxySvc::new_null_lifecycle_monitor
#[tokio::test]
async fn test_proxy_svc_new_null_lifecycle_monitor_is_healthy() {
    let m = ProxySvc::new_null_lifecycle_monitor();
    assert_eq!(m.health().await.overall, HealthStatus::Healthy);
}

/// @covers: ProxySvc::new_noop_validator
#[test]
fn test_proxy_svc_new_noop_validator_accepts_unit() {
    let v = ProxySvc::new_noop_validator();
    assert!(v.validate(&()).is_ok());
}

/// @covers: ProxySvc::validate
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
