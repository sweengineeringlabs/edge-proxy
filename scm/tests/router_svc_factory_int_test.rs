use edge_proxy::ROUTER_SVC_FACTORY;

#[test]
fn test_router_svc_factory_constant_value_happy() {
    assert_eq!(ROUTER_SVC_FACTORY, "router_factory");
}

#[test]
fn test_router_svc_factory_constant_not_empty_error() {
    assert!(
        !ROUTER_SVC_FACTORY.is_empty(),
        "ROUTER_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_router_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !ROUTER_SVC_FACTORY.contains(char::is_whitespace),
        "ROUTER_SVC_FACTORY must not contain whitespace"
    );
}
