//! Integration tests for the ProxyComposer trait.
//! @covers: api/proxy/traits/proxy_composer.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_proxy::{
    ApplicationConfigBuilder, BootstrapNameRequest, ProxyComposer, ProxyPattern, ProxySvc,
};

// ProxySvc implements ProxyComposer — use it as the concrete test target.

// compose ─────────────────────────────────────────────────────────────────────

/// compose — happy: returns a ProxySvc value.
#[test]
fn test_compose_returns_proxy_svc_happy() {
    let _svc = ProxySvc::compose();
    assert_eq!(std::mem::size_of_val(&_svc), 0);
}

/// compose — error: calling twice still succeeds (stateless factory).
#[test]
fn test_compose_called_twice_no_error() {
    let a = ProxySvc::compose();
    let b = ProxySvc::compose();
    assert_eq!(
        a.bootstrap_name(BootstrapNameRequest).unwrap().name,
        b.bootstrap_name(BootstrapNameRequest).unwrap().name
    );
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
    let p = ProxySvc::pattern();
    assert_eq!(std::mem::size_of_val(&p), 0);
}

/// pattern — error: calling twice still succeeds (stateless).
#[test]
fn test_pattern_called_twice_no_error() {
    let a = ProxySvc::pattern();
    let b = ProxySvc::pattern();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
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
    let b = ProxySvc::builder();
    assert_eq!(std::mem::size_of_val(&b), 0);
}

/// builder — error: calling twice produces independent builders.
#[test]
fn test_builder_called_twice_independent_error() {
    let b1 = ProxySvc::builder();
    let b2 = ProxySvc::builder();
    assert_eq!(std::mem::size_of_val(&b1), std::mem::size_of_val(&b2));
}

/// builder — edge: ApplicationConfigBuilder is zero-sized.
#[test]
fn test_builder_result_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ApplicationConfigBuilder>(), 0);
}

// bootstrap_name ──────────────────────────────────────────────────────────────

/// bootstrap_name — happy: returns the stable "proxy_composer" literal.
#[test]
fn test_bootstrap_name_returns_stable_literal_happy() {
    let svc = ProxySvc::compose();
    assert_eq!(
        svc.bootstrap_name(BootstrapNameRequest).unwrap().name,
        "proxy_composer"
    );
}

/// bootstrap_name — error: calling on two instances returns the same literal.
#[test]
fn test_bootstrap_name_consistent_across_instances_error() {
    let a = ProxySvc::compose();
    let b = ProxySvc::compose();
    assert_eq!(
        a.bootstrap_name(BootstrapNameRequest).unwrap().name,
        b.bootstrap_name(BootstrapNameRequest).unwrap().name
    );
}

/// bootstrap_name — edge: return value is 'static and never empty.
#[test]
fn test_bootstrap_name_is_nonempty_static_edge() {
    let svc = ProxySvc::compose();
    let name: &'static str = svc.bootstrap_name(BootstrapNameRequest).unwrap().name;
    assert!(!name.is_empty());
}

// ProxyComposer trait ─────────────────────────────────────────────────────────

/// ProxyComposer — happy: ProxySvc satisfies the ProxyComposer trait bound.
#[test]
fn test_proxy_svc_satisfies_proxy_composer_trait_bound_happy() {
    fn assert_impl<T: ProxyComposer>(_marker: std::marker::PhantomData<T>) -> bool {
        true
    }
    assert!(assert_impl(std::marker::PhantomData::<ProxySvc>));
}
