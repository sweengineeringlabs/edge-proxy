//! Integration tests verifying validator_svc re-exports are reachable.

use edge_proxy::{ProxySvc, Validator};

/// Verifies `Validator` trait is exported at the crate level.
#[test]
fn test_validator_type_is_reachable_from_crate_boundary() {
    fn _accept<T: Validator>() {}
}

/// Verifies validate delegates to the implementation.
#[test]
fn test_validator_svc_validate_reaches_implementation() {
    let v = ProxySvc::new_noop_validator();
    assert!(v.validate(&()).is_ok());
}
