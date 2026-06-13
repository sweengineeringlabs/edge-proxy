//! Integration tests for the Validator trait contract.

use edge_proxy::{ProxySvc, Validator};

struct RejectEmpty;
impl Validator for RejectEmpty {
    type Target = str;
    type Error = String;
    fn validate(&self, value: &str) -> Result<(), String> {
        if value.is_empty() {
            Err("must not be empty".into())
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_validate_wrapper_returns_ok_for_valid_input() {
    let v = RejectEmpty;
    assert!(ProxySvc::validate(&v, "hello").is_ok());
}

#[test]
fn test_validate_wrapper_returns_err_for_invalid_input() {
    let v = RejectEmpty;
    assert!(ProxySvc::validate(&v, "").is_err());
}

// Rule 222 scenario coverage for Validator::validate ─────────────────────────

/// validate — happy: valid non-empty input passes.
#[test]
fn test_validate_non_empty_input_passes_happy() {
    assert!(RejectEmpty.validate("hello").is_ok());
}

/// validate — error: empty string is rejected.
#[test]
fn test_validate_empty_string_is_rejected_error() {
    assert!(RejectEmpty.validate("").is_err());
}

/// validate — edge: unicode input is accepted when non-empty.
#[test]
fn test_validate_unicode_input_is_accepted_edge() {
    assert!(RejectEmpty.validate("héllo wörld 🌍").is_ok());
}
