//! End-to-end contract tests for the `Validator` trait, exercised through a
//! test-double implementation via the crate's public API.

use edge_proxy::{ProxySvc, ValidationRequest, Validator};

struct RejectEmptyDouble;
impl Validator for RejectEmptyDouble {
    type Target = str;
    type Error = String;
    fn validate(&self, req: ValidationRequest<'_, str>) -> Result<(), String> {
        if req.value.is_empty() {
            Err("must not be empty".into())
        } else {
            Ok(())
        }
    }
}

/// @covers: Validator::validate
#[test]
fn test_validate_nonempty_input_passes_happy() {
    assert_eq!(
        RejectEmptyDouble.validate(ValidationRequest { value: "hi" }),
        Ok(())
    );
}

/// @covers: Validator::validate
#[test]
fn test_validate_empty_input_rejected_error() {
    assert!(RejectEmptyDouble
        .validate(ValidationRequest { value: "" })
        .is_err());
}

/// @covers: Validator::validate
#[test]
fn test_validate_via_proxy_svc_facade_edge() {
    assert_eq!(ProxySvc::validate(&RejectEmptyDouble, "hi"), Ok(()));
}
