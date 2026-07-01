use edge_proxy::NULL_LIFECYCLE_MONITOR_SVC_FACTORY;

#[test]
fn test_null_lifecycle_monitor_svc_factory_constant_value_happy() {
    assert_eq!(
        NULL_LIFECYCLE_MONITOR_SVC_FACTORY,
        "null_lifecycle_monitor_factory"
    );
}

#[test]
fn test_null_lifecycle_monitor_svc_factory_constant_not_empty_error() {
    assert!(
        !NULL_LIFECYCLE_MONITOR_SVC_FACTORY.is_empty(),
        "NULL_LIFECYCLE_MONITOR_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_null_lifecycle_monitor_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !NULL_LIFECYCLE_MONITOR_SVC_FACTORY.contains(char::is_whitespace),
        "NULL_LIFECYCLE_MONITOR_SVC_FACTORY must not contain whitespace"
    );
}
