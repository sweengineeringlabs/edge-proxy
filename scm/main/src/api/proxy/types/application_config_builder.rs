//! `ApplicationConfigBuilder` — typed builder for proxy application configuration.

/// Typed builder for proxy application configuration.
///
/// Construct with [`ApplicationConfigBuilder::new`], which seeds defaults from
/// `config/application.toml`.
///
/// Note: this is currently a placeholder — fluent setters and a `build`
/// finalizer are not yet implemented. Use [`ApplicationConfigBuilder::new`]
/// (or [`Default`]) to obtain the default configuration.
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
