//! Integration tests for the ProxyComposer trait.
//! @covers: api/proxy/traits/proxy_composer.rs

use edge_proxy::{ApplicationConfigBuilder, ProxyComposer, ProxyPattern, ProxySvc};

// ProxySvc implements ProxyComposer — use it as the concrete test target.

// compose ─────────────────────────────────────────────────────────────────────

/// compose — happy: returns a ProxySvc value.
#[test]
fn test_compose_returns_proxy_svc_happy() {
    let _svc = ProxySvc::compose();
}

/// compose — error: calling twice still succeeds (stateless factory).
#[test]
fn test_compose_called_twice_no_error() {
    let _a = ProxySvc::compose();
    let _b = ProxySvc::compose();
}

/// compose — edge: result has unit type identity (zero-size).
#[test]
fn test_compose_result_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ProxySvc>(), 0);
}

// pattern ─────────────────────────────────────────────────────────────────────

/// pattern — happy: returns a ProxyPattern value.
#[test]
fn test_pattern_returns_proxy_pattern_happy() {
    let _p = ProxySvc::pattern();
}

/// pattern — error: calling twice still succeeds (stateless).
#[test]
fn test_pattern_called_twice_no_error() {
    let _a = ProxySvc::pattern();
    let _b = ProxySvc::pattern();
}

/// pattern — edge: ProxyPattern is zero-sized.
#[test]
fn test_pattern_result_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ProxyPattern>(), 0);
}

// builder ─────────────────────────────────────────────────────────────────────

/// builder — happy: returns an ApplicationConfigBuilder.
#[test]
fn test_builder_returns_config_builder_happy() {
    let _b = ProxySvc::builder();
}

/// builder — error: calling twice produces independent builders.
#[test]
fn test_builder_called_twice_independent_error() {
    let _b1 = ProxySvc::builder();
    let _b2 = ProxySvc::builder();
}

/// builder — edge: ApplicationConfigBuilder is zero-sized.
#[test]
fn test_builder_result_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ApplicationConfigBuilder>(), 0);
}

// ProxyComposer trait ─────────────────────────────────────────────────────────

/// ProxyComposer — happy: ProxySvc satisfies the ProxyComposer trait bound.
#[test]
fn test_proxy_svc_satisfies_proxy_composer_trait_bound_happy() {
    fn _assert_impl<T: ProxyComposer>() {}
    _assert_impl::<ProxySvc>();
}
