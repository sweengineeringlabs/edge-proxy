//! `ApplicationConfigBuilder` — typed builder for proxy application configuration.

/// Typed builder for proxy application configuration.
///
/// Construct with [`ApplicationConfigBuilder::new`], populate via the fluent
/// setters, then call [`ApplicationConfigBuilder::build`] to produce a
/// validated configuration.
pub struct ApplicationConfigBuilder {
    _private: (),
}

impl ApplicationConfigBuilder {
    /// Create a builder pre-seeded with `config/application.toml` defaults.
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for ApplicationConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
