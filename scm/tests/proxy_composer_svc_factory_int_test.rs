use edge_proxy::PROXY_COMPOSER_SVC_FACTORY;

#[test]
fn test_proxy_composer_svc_factory_constant_value_happy() {
    assert_eq!(PROXY_COMPOSER_SVC_FACTORY, "proxy_composer_factory");
}

#[test]
fn test_proxy_composer_svc_factory_constant_not_empty_error() {
    assert!(
        !PROXY_COMPOSER_SVC_FACTORY.is_empty(),
        "PROXY_COMPOSER_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_proxy_composer_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !PROXY_COMPOSER_SVC_FACTORY.contains(char::is_whitespace),
        "PROXY_COMPOSER_SVC_FACTORY must not contain whitespace"
    );
}
