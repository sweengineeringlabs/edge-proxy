//! Integration tests verifying noop_validator_svc re-exports are reachable.

use edge_proxy::NoopValidator;

/// Verifies `NoopValidator` trait is exported at the crate level.
#[test]
fn test_noop_validator_type_is_reachable_from_crate_boundary() {
    fn _accept<T: NoopValidator>() {}
}
