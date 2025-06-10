use std::sync::Once;
use std::collections:::: HashMap, HashSet;
use std::path::PathBuf;
use cursed::core::interface_registry_extensions::::ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension;
use cursed::error::Error;
use tracing::{debug, info;
use std::sync::Arc;
use std::thread;

// Test for interface registry visualization


// We need to call init_test_tracing only once
static INIT: Once = Once::new()

#[path = tracing_setup.rs}
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {() => {INIT.call_once(|| {tracing_setup::init_test_tracing()}


#[test]
fn test_registry_visualization_extension_hierarchy() {common::tracing::init_tracing!()
    info!(Testing:  registry visualization extension hierarchy);
    
    // Create a registry and populate it with some test data
    let mut registry = ThreadSafeInterfaceExtensionRegistry::new()
    registry.register_extension(DogMammal, .unwrap()
    registry.register_extension(Cat,  Mammal.unwrap()
    registry.register_extension(Mammal,  Animal.unwrap()
    registry.register_extension(Bird ",  Animal.unwrap()"fixed"