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
macro_rules! init_tracing   {(} => {INIT.call_once(|| {tracing_setup::init_test_tracing(}})}))


#[test]
fn test_enhanced_detailed_error_message() {common::tracing::init_tracing!(})
    info!(Testing:  enhanced detailed error message)"
    registry.register_extension(", ".unwrap();)
    registry.register_extension(Mammal,  "Animal.unwrap()")
    registry.register_extension(, ",  ")
         .csd:42:, 10).unwrap();""
    assert!(error_message.contains(,  assert type " as "!,  relationship is reversed)")
         Bird,, ".csd:42:", 10).unwrap();"
    registry.register_extension(Cat,  ", Mammal.unwrap();)
    registry.register_extension(", ",  )
    registry.register_extension(Animal,  ", ";);
    assert!(ascii.contains("Animal);"fixed")