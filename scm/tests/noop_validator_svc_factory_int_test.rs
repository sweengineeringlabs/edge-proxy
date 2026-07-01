use edge_proxy::NOOP_VALIDATOR_SVC_FACTORY;

#[test]
fn test_noop_validator_svc_factory_constant_value_happy() {
    assert_eq!(NOOP_VALIDATOR_SVC_FACTORY, "noop_validator_factory");
}

#[test]
fn test_noop_validator_svc_factory_constant_not_empty_error() {
    assert!(
        !NOOP_VALIDATOR_SVC_FACTORY.is_empty(),
        "NOOP_VALIDATOR_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_noop_validator_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !NOOP_VALIDATOR_SVC_FACTORY.contains(char::is_whitespace),
        "NOOP_VALIDATOR_SVC_FACTORY must not contain whitespace"
    );
}
