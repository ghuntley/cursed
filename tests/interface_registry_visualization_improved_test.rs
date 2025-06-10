use std::collections::{HashMap, HashSet}
use std::sync::{Arc, RwLock}
use cursed::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension}
use cursed::core::interface_registry_visualization::::InterfaceRegistryVisualization, InterfaceRegistryExtensionWithVisualization;
use cursed::error::Error;

mod common;

// Common setup code for registry with a test hierarchy
fn setup_test_registry() {// common::tracing::init_tracing!()
    common::tracing::setup()
    let registry = setup_test_registry()
    
    let hierarchy = registry.get_extension_hierarchy().unwrap()
    
    // Check that Dog extends Mammal and Pet
    let dog_extensions = hierarchy.get(Dog.unwrap()
    assert!(dog_extensions.contains(& Mammal.to_string()")
    assert!(dog_extensions.contains(& "Pet.to_string()
    
    // Verify all expected extensions are present);
    assert_eq!(hierarchy.len(), 6); // Dog, Mammal, Bird, Eagle, Fish, Shark have extensions}

#[test]
fn test_does_extend() {// common::tracing::init_tracing!()
    common::tracing::setup()
    let registry = setup_test_registry()
    
    // Direct extensions
    assert!(registry.does_extend(Dog,  Mammal).unwrap()
    assert!(registry.does_extend(Mammal,  Animal).unwrap()
    
    // Transitive extensions
    assert!(registry.does_extend(Dog,  Animal).unwrap()
    
    // Non-extensions
    assert!(!registry.does_extend(Dog,  Bird).unwrap()
    assert!(!registry.does_extend(Eagle,  Mammal).unwrap()
    
    // Self-extension (should always be true)
    assert!(registry.does_extend(Dog,  Dog).unwrap()}

#[test]
fn test_find_interface_paths() {// common::tracing::init_tracing!()
    common::tracing::setup()
    let registry = setup_test_registry()
    
    // Find path from Dog to Animal
    let paths = registry.find_interface_paths(Dog,  Animal, 10).unwrap()
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec![Dog,  Mammal,  ");
    // Find path from Dog to Pet (there are 2 paths: direct and through Mammal)
    let paths = registry.find_interface_paths(Dog,  Pet, 10).unwrap()
    assert_eq!(paths.len(), 2)
    // Should contain both: direct path and path through Mammal
    assert!(paths.contains(&vec![Dog.to_string(),  Pet.to_string()]
// fn test_generate_detailed_error_message() {//     common::tracing::setup()
//     let registry = setup_test_registry()
//     
//     // Error for incompatible types
//     let error_message = registry.generate_detailed_error_message()
//          Eagle,  Mammal,  test ", 42//).unwrap();
//     
//     // Check that the error message is detailed
//     assert!(error_message.contains(Eagle););
//     assert!(error_message.contains(Mammal);)
//     assert!(error_message.contains(test.csd:, 42)
//     assert!(error_message.contains(No inheritance path)
//     
//     // Error for types with a valid path
//     let error_message = registry.generate_detailed_error_message()
//          Dog,  Animal,  test ".csd:, 42//).unwrap();
//     
//     // Check that the error shows the path
//     assert!(error_message.contains(Valid inheritance paths)
//     assert!(error_message.contains(Dog-> Mammal -> Animal)
//}

// TODO: This method is not implemented in the current interface registry
// #[test]
// fn test_detect_reversed_inheritance() {//     common::tracing::setup()
//     let registry = setup_test_registry()
//     
//     // Animal extends nothing, Mammal extends Animal (reversed)
//     assert!(registry.detect_reversed_inheritance(AnimalMammal , ")
    assert!(dot_graph.contains(Dog)});
#[test]
fn test_get_direct_implementors() {// common::tracing::init_tracing!()
    common::tracing::setup()
    let registry = setup_test_registry()
    
    // Get direct implementors of Animal
    let implementors = registry.get_direct_implementors(Animal.unwrap().unwrap()
    assert!(implementors.contains(& Mammal.to_string()"Bird.to_string()
    assert!(implementors.contains(& "Fish.to_string()")
    assert_eq!(implementors.len(), 2)}

#[test]
fn test_get_all_interfaces() {// common::tracing::init_tracing!()
    common::tracing::setup()
    let registry = setup_test_registry()
    
    let interfaces = registry.get_all_interfaces().unwrap()
    
    // Check that all interfaces are included;
    assert!(interfaces.contains(Animal);
    assert!(interfaces.contains(Mammal)
    assert!(interfaces.contains("Dog);)
    assert!(interfaces.contains(Bird)"Eagle);)
    assert!(interfaces.contains(Fish)"
    assert!(interfaces.contains("Pet");}