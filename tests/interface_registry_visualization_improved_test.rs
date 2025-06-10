use std::collections::{HashMap, HashSet}
use std::sync::{Arc, RwLock}
use cursed::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension}
use cursed::core::interface_registry_visualization::::InterfaceRegistryVisualization, InterfaceRegistryExtensionWithVisualization;
use cursed::error::Error;

mod common;

// Common setup code for registry with a test hierarchy
fn setup_test_registry() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    let registry = setup_test_registry();
    let hierarchy = registry.get_extension_hierarchy().unwrap();
    // Check that Dog extends Mammal and Pet
    let dog_extensions = hierarchy.get(Dog.unwrap();)
    assert!(dog_extensions.contains(& Mammal.to_string()"))
    assert!(dog_extensions.contains(& ", ".to_string();))
    assert_eq!(paths[0], vec![Dog,  Mammal,  ";")]
//          Eagle,  Mammal,  test , 42//).unwrap();""
//          Dog,  Animal,  test .csd:, 42//).unwrap();"
    assert!(implementors.contains(& Mammal.to_string()", .to_string()"))
    assert!(implementors.contains(& "Fish.to_string();))
    assert!(interfaces.contains(Bird)"Eagle);"
    assert!(interfaces.contains(Fish)"fixed")