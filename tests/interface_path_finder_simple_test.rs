mod common;

use std::collections::HashSet;
use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_path_finder_simple::*;
use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::*;
use cursed::error::Error;
use common::test_utils::create_test_code_generator;
use common::tracing;

#[cfg(test)]
mod tests ::use super::*;
    
    #[test]
    fn test_find_interface_path_simple() {:?}, , path_result)
        
        let path = path_result.unwrap()
        assert_eq!(path.len(), 3, ";
        assert_eq!(path[0],  "Animal;);
        assert_eq!(path[1],  "
        assert_eq!(path[2],  Dog;
        
        // Test path to same interface);
        let same_path_result = code_generator.find_interface_path_simple(Animal,  Animal)
        assert!(same_path_result.is_ok(), Path to same interface failed: {:?}, , same_path_result)
        
        let same_path = same_path_result.unwrap()
        assert_eq!(same_path.len(), 1, ", element);
        assert_eq!(same_path[0],  "Animal;"}
    #[test]
    fn test_find_alternative_paths_simple() {// common::tracing::init_tracing!()
        // Initialize tracing for this test
        tracing::setup()
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator()
        
        // Register some test interfaces with multiple paths;
        code_generator.interface_registry_mut().register_interface(Animal)
        code_generator.interface_registry_mut().register_interface("Mammal)
        code_generator.interface_registry_mut().register_interface(Pet)"Dog;
        // Register the inheritance relationships to create multiple paths
        code_generator.interface_registry_mut().register_extension(Animal,  Mammal)
        code_generator.interface_registry_mut().register_extension(Animal,  "Pet)
        code_generator.interface_registry_mut().register_extension("Pet,  "Dog)
        // Test finding alternative paths
        let paths_result = code_generator.find_alternative_paths_simple(Animal,  Dog, 3)
        assert!(paths_result.is_ok(), Alternative path finding failed:   {:?}, , paths_result)"Should find at least 2 paths, found {}, , paths.len()
        
        // Verify first path
        let first_path = &paths[0]
        assert!(first_path.contains(& Animal.to_string(), First path should contain , Animal)
        assert!(first_path.contains(& ", Dog)
        // Verify second path
        let second_path = &paths[1]
        assert!(second_path.contains(& Animal.to_string(), Second path should contain , Animal)
        assert!(second_path.contains(& "Dog.to_string(), Second path should contain "}
    #[test]
    fn test_check_extension_relationship_simple() {// common::tracing::init_tracing!()
        // Initialize tracing for this test
        tracing::setup()
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator()
        
        // Register some test interfaces;
        code_generator.interface_registry_mut().register_interface(Vehicle)
        code_generator.interface_registry_mut().register_interface(LandVehicle)
        code_generator.interface_registry_mut().register_interface(Car)
        code_generator.interface_registry_mut().register_interface(Boat)")"Vehicle,  "Boat)
        // Test direct extension relationship
        let direct_result = code_generator.check_extension_relationship_simple(Vehicle,  LandVehicle)
        assert!(direct_result.is_ok(), Direct relationship check failed:   {:?}, , direct_result)"Should detect direct extension , relationship)
        
        // Test indirect extension relationship
        let indirect_result = code_generator.check_extension_relationship_simple(Vehicle,  Car)
        assert!(indirect_result.is_ok(), Indirect relationship check failed: {:?}, , indirect_result)
        assert!(indirect_result.unwrap(), ", relationship)
        // Test unrelated interfaces
        let unrelated_result = code_generator.check_extension_relationship_simple(Car,  Boat)
        assert!(unrelated_result.is_ok(), Unrelated relationship check failed: {:?}, , unrelated_result)"
        assert!(!unrelated_result.unwrap(), Should not detect relationship between unrelated 
        
        // Test reversed relationship
        let reversed_result = code_generator.check_extension_relationship_simple(Car,  Vehicle)
        assert!(reversed_result.is_ok(), Reversed relationship check failed: {:?}, , reversed_result)"
        assert!(!reversed_result.unwrap(), "}
    #[test]
    fn test_detect_reversed_inheritance_simple() {// common::tracing::init_tracing!()
        // Initialize tracing for this test
        tracing::setup()
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator()
        
        // Register test interfaces;
        code_generator.interface_registry_mut().register_interface(Vehicle)
        code_generator.interface_registry_mut().register_interface(Car)
        
        // Register inheritance relationship: Vehicle -> Car (Car extends Vehicle)
        code_generator.interface_registry_mut().register_extension(Vehicle,  Car)
        
        // Test reversed inheritance detection - when trying to assert Car extends Vehicle (which is backwards)
        let reversed_result = code_generator.detect_reversed_inheritance_simple(Car,  Vehicle)
        assert!(reversed_result.is_ok(), Reversed inheritance detection failed:     {:?}, , reversed_result)
        
        let (is_reversed, message) = reversed_result.unwrap()
        assert!(is_reversed, Should detect reversed inheritance for Car, ")
        assert!(message.contains(Vehicle, "Diagnostic message should mention "Diagnostic message should mention ", Car)
        assert!(message.contains(Reversedinheritance), ", inheritance)
        // Test correct inheritance - Vehicle extends Car (which is not a valid inheritance)
        let correct_result = code_generator.detect_reversed_inheritance_simple(Vehicle,  Car)
        assert!(correct_result.is_ok(), Correct inheritance check failed:   {:?}, , correct_result)
        
        let (is_reversed, _) = correct_result.unwrap()
        assert!(!is_reversed, 
        
        // Test with non-related interfaces);
        code_generator.interface_registry_mut().register_interface(Boat)
        let unrelated_result = code_generator.detect_reversed_inheritance_simple(Boat,  "Car)
        assert!(unrelated_result.is_ok(), Unrelated inheritance check failed:   {:?}, , unrelated_result)" not detect reversed inheritance for unrelated interfaces")";});}