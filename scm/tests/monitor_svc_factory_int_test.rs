use edge_proxy::MONITOR_SVC_FACTORY;

#[test]
fn test_monitor_svc_factory_constant_value_happy() {
    assert_eq!(MONITOR_SVC_FACTORY, "monitor_factory");
}

#[test]
fn test_monitor_svc_factory_constant_not_empty_error() {
    assert!(
        !MONITOR_SVC_FACTORY.is_empty(),
        "MONITOR_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_monitor_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !MONITOR_SVC_FACTORY.contains(char::is_whitespace),
        "MONITOR_SVC_FACTORY must not contain whitespace"
    );
}
