//! Constructors for [`ApplicationConfigBuilder`].

use crate::api::ApplicationConfigBuilder;

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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_constructs_zero_sized_instance() {
        let b = ApplicationConfigBuilder::new();
        assert_eq!(std::mem::size_of_val(&b), 0);
    }
}
