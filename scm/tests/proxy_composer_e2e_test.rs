//! End-to-end contract tests for the `ProxyComposer` trait, exercised through
//! `ProxySvc` via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_proxy::{
    ApplicationConfigBuilder, BootstrapNameRequest, ProxyComposer, ProxyPattern, ProxySvc,
};

/// @covers: ProxyComposer::bootstrap_name
#[test]
fn test_bootstrap_name_returns_stable_literal_happy() {
    let svc = ProxySvc::compose();
    assert_eq!(
        svc.bootstrap_name(BootstrapNameRequest).unwrap().name,
        "proxy_composer"
    );
}

/// @covers: ProxyComposer::bootstrap_name
#[test]
fn test_bootstrap_name_nonempty_edge() {
    let svc = ProxySvc::compose();
    assert!(!svc
        .bootstrap_name(BootstrapNameRequest)
        .unwrap()
        .name
        .is_empty());
}

/// @covers: ProxyComposer::compose
#[test]
fn test_compose_returns_zero_sized_proxy_svc_happy() {
    let svc = ProxySvc::compose();
    assert_eq!(std::mem::size_of_val(&svc), 0);
}

/// @covers: ProxyComposer::pattern
#[test]
fn test_pattern_returns_zero_sized_marker_happy() {
    let pattern: ProxyPattern = ProxySvc::pattern();
    assert_eq!(std::mem::size_of_val(&pattern), 0);
}

/// @covers: ProxyComposer::builder
#[test]
fn test_builder_returns_zero_sized_config_builder_edge() {
    let builder: ApplicationConfigBuilder = ProxySvc::builder();
    assert_eq!(std::mem::size_of_val(&builder), 0);
}
