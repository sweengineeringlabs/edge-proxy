//! External-style test module for spi/'s extension-point implementations.
//!
//! Gated by `#[cfg(test)] mod tests;` in `spi/mod.rs`, colocated with the
//! spi/ layer per SEA rule `spi_impl_public_tests_external`.
//!
//! Empty today: every function in `spi/` (`canonical.rs`, and the domain
//! placeholders under `spi/job/`, `spi/lifecycle/`, `spi/proxy/`,
//! `spi/router/`, `spi/validator/`) is `pub(crate)`, not `pub` — this rule
//! only applies to public functions, and none exist here yet.
