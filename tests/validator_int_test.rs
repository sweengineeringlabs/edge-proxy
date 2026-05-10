//! Integration tests for the Validator trait contract.

use edge_proxy::{validate, Validator};

struct RejectEmpty;
impl Validator for RejectEmpty {
    type Target = str;
    type Error  = String;
    fn validate(&self, value: &str) -> Result<(), String> {
        if value.is_empty() { Err("must not be empty".into()) } else { Ok(()) }
    }
}

#[test]
fn test_validate_wrapper_returns_ok_for_valid_input() {
    let v = RejectEmpty;
    assert!(validate(&v, "hello").is_ok());
}

#[test]
fn test_validate_wrapper_returns_err_for_invalid_input() {
    let v = RejectEmpty;
    assert!(validate(&v, "").is_err());
}
