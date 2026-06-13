//! Integration tests for the ProxyComposer SAF facade.
//! @covers: saf/proxy/composer_svc.rs

use edge_proxy::{ProxyComposer, ProxySvc, PROXY_COMPOSER_CONCERN};

#[test]
fn test_proxy_composer_svc_concern_tag_is_correct_happy() {
    assert_eq!(PROXY_COMPOSER_CONCERN, "proxy_composer");
}

#[test]
fn test_proxy_composer_svc_concern_tag_is_non_empty_happy() {
    assert!(!PROXY_COMPOSER_CONCERN.is_empty());
}

#[test]
fn test_proxy_composer_svc_trait_is_accessible_edge() {
    // ProxyComposer trait is accessible through the public API.
    fn _use_bound<T: ProxyComposer>() {}
    _use_bound::<ProxySvc>();
}
