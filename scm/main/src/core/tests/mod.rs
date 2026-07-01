//! Scaffold for external tests of standalone `pub fn` items declared directly in `core/`.
//!
//! `edge-proxy`'s `core/` layer has no standalone public functions reachable
//! outside their owning `impl` blocks — all public constructors are exercised
//! through the crate-root `tests/` directory via the `api/` re-exports that
//! wrap them (e.g. `tests/component_health_int_test.rs`,
//! `tests/application_config_builder_int_test.rs`).
