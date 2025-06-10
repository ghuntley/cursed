use std::sync::Once;
use cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use cursed::core::interface_registry_visualization_enhanced::EnhancedVisualizationIntegration;
use cursed::error::Error;
use tracing::{debug, info}

// Test for enhanced interface registry visualization integration


// We need to call init_test_tracing only once
static INIT: Once = Once::new();
#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {tracing_setup::init_test_tracing(
    };
})}



#[test]
fn test_enhanced_detailed_error_message() {common::tracing::init_tracing!())
    // TODO: Implement test
    assert!(true);
}