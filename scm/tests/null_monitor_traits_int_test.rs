//! Integration tests for null lifecycle monitor marker traits and noop validator.

use edge_proxy::{ProxySvc, ValidationRequest};

/// @covers: ProxySvc::new_noop_validator
#[test]
fn test_noop_validator_always_returns_ok() {
    let v = ProxySvc::new_noop_validator();
    let result = v.validate(ValidationRequest { value: &() });
    assert_eq!(result, Ok(()));
}
