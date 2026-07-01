//! Integration tests for ApplicationConfigBuilder.

use edge_proxy::ApplicationConfigBuilder;

/// @covers: ApplicationConfigBuilder::new
#[test]
fn test_application_config_builder_new_creates_instance() {
    let _builder = ApplicationConfigBuilder::new();
    assert_eq!(std::mem::size_of::<ApplicationConfigBuilder>(), 0);
}

/// @covers: ApplicationConfigBuilder::default
#[test]
fn test_application_config_builder_default_creates_instance() {
    let _builder = ApplicationConfigBuilder::default();
    assert_eq!(std::mem::size_of::<ApplicationConfigBuilder>(), 0);
}
