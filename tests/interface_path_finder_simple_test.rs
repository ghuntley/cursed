use std::collections::HashSet;
use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use common::test_utils::create_test_code_generator;
use common::tracing;

#[cfg(test)]
mod tests {
    
    
    // Import common test utilities
    mod common;
    
    #[test]
    fn test_find_interface_path_simple() {
        // Initialize tracing for this test
        tracing::setup();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register some test interfaces
        code_generator.interface_registry().register_interface("Animal").unwrap();
        code_generator.interface_registry().register_interface("Mammal").unwrap();
        code_generator.interface_registry().register_interface("Dog").unwrap();
        
        // Register the inheritance relationships
        code_generator.interface_registry().register_extension("Animal", "Mammal").unwrap();
        code_generator.interface_registry().register_extension("Mammal", "Dog").unwrap();
        
        // Test finding a path
        let path_result = code_generator.find_interface_path_simple("Animal", "Dog");
        assert!(path_result.is_ok(), "Path finding failed: {:?}", path_result);
        
        let path = path_result.unwrap();
        assert_eq!(path.len(), 3, "Path should have 3 elements");
        assert_eq!(path[0], "Animal");
        assert_eq!(path[1], "Mammal");
        assert_eq!(path[2], "Dog");
        
        // Test path to same interface
        let same_path_result = code_generator.find_interface_path_simple("Animal", "Animal");
        assert!(same_path_result.is_ok(), "Path to same interface failed: {:?}", same_path_result);
        
        let same_path = same_path_result.unwrap();
        assert_eq!(same_path.len(), 1, "Path to same interface should have 1 element");
        assert_eq!(same_path[0], "Animal");
        
        // Test non-existent path
        let non_existent_path_result = code_generator.find_interface_path_simple("Dog", "Animal");
        assert!(non_existent_path_result.is_err(), "Non-existent path should fail");
    }
    
    #[test]
    fn test_find_alternative_paths_simple() {
        // Initialize tracing for this test
        tracing::setup();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register some test interfaces with multiple paths
        code_generator.interface_registry().register_interface("Animal").unwrap();
        code_generator.interface_registry().register_interface("Mammal").unwrap();
        code_generator.interface_registry().register_interface("Pet").unwrap();
        code_generator.interface_registry().register_interface("Dog").unwrap();
        
        // Register the inheritance relationships to create multiple paths
        code_generator.interface_registry().register_extension("Animal", "Mammal").unwrap();
        code_generator.interface_registry().register_extension("Animal", "Pet").unwrap();
        code_generator.interface_registry().register_extension("Mammal", "Dog").unwrap();
        code_generator.interface_registry().register_extension("Pet", "Dog").unwrap();
        
        // Test finding alternative paths
        let paths_result = code_generator.find_alternative_paths_simple("Animal", "Dog", 3);
        assert!(paths_result.is_ok(), "Alternative path finding failed: {:?}", paths_result);
        
        let paths = paths_result.unwrap();
        assert!(paths.len() >= 2, "Should find at least 2 paths, found {}", paths.len())
        
        // Verify first path
        let first_path = &paths[0];
        assert!(first_path.contains(&"Animal".to_string(), "First path should contain Animal");
        assert!(first_path.contains(&"Dog".to_string(), "First path should contain Dog");
        
        // Verify second path
        let second_path = &paths[1];
        assert!(second_path.contains(&"Animal".to_string(), "Second path should contain Animal");
        assert!(second_path.contains(&"Dog".to_string(), "Second path should contain Dog");
        
        // Verify paths are different
        assert_ne!(first_path, second_path, "Paths should be different");
    }
    
    #[test]
    fn test_check_extension_relationship_simple() {
        // Initialize tracing for this test
        tracing::setup();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register some test interfaces
        code_generator.interface_registry().register_interface("Vehicle").unwrap();
        code_generator.interface_registry().register_interface("LandVehicle").unwrap();
        code_generator.interface_registry().register_interface("Car").unwrap();
        code_generator.interface_registry().register_interface("Boat").unwrap();
        
        // Register the inheritance relationships
        code_generator.interface_registry().register_extension("Vehicle", "LandVehicle").unwrap();
        code_generator.interface_registry().register_extension("LandVehicle", "Car").unwrap();
        code_generator.interface_registry().register_extension("Vehicle", "Boat").unwrap();
        
        // Test direct extension relationship
        let direct_result = code_generator.check_extension_relationship_simple("Vehicle", "LandVehicle");
        assert!(direct_result.is_ok(), "Direct relationship check failed: {:?}", direct_result);
        assert!(direct_result.unwrap(), "Should detect direct extension relationship");
        
        // Test indirect extension relationship
        let indirect_result = code_generator.check_extension_relationship_simple("Vehicle", "Car");
        assert!(indirect_result.is_ok(), "Indirect relationship check failed: {:?}", indirect_result);
        assert!(indirect_result.unwrap(), "Should detect indirect extension relationship");
        
        // Test unrelated interfaces
        let unrelated_result = code_generator.check_extension_relationship_simple("Car", "Boat");
        assert!(unrelated_result.is_ok(), "Unrelated relationship check failed: {:?}", unrelated_result);
        assert!(!unrelated_result.unwrap(), "Should not detect relationship between unrelated interfaces");
        
        // Test reversed relationship
        let reversed_result = code_generator.check_extension_relationship_simple("Car", "Vehicle");
        assert!(reversed_result.is_ok(), "Reversed relationship check failed: {:?}", reversed_result);
        assert!(!reversed_result.unwrap(), "Should not detect reversed relationship");
    }
    
    #[test]
    fn test_detect_reversed_inheritance_simple() {
        // Initialize tracing for this test
        tracing::setup();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register test interfaces
        code_generator.interface_registry().register_interface("Vehicle").unwrap();
        code_generator.interface_registry().register_interface("Car").unwrap();
        
        // Register inheritance relationship: Vehicle -> Car (Car extends Vehicle)
        code_generator.interface_registry().register_extension("Vehicle", "Car").unwrap();
        
        // Test reversed inheritance detection - when trying to assert Car extends Vehicle (which is backwards)
        let reversed_result = code_generator.detect_reversed_inheritance_simple("Car", "Vehicle");
        assert!(reversed_result.is_ok(), "Reversed inheritance detection failed: {:?}", reversed_result);
        
        let (is_reversed, message) = reversed_result.unwrap();
        assert!(is_reversed, "Should detect reversed inheritance for Car, Vehicle");
        assert!(message.contains("Vehicle"), "Diagnostic message should mention Vehicle");
        assert!(message.contains("Car"), "Diagnostic message should mention Car");
        assert!(message.contains("Reversed inheritance"), "Diagnostic should clearly mention reversed inheritance");
        
        // Test correct inheritance - Vehicle extends Car (which is not a valid inheritance)
        let correct_result = code_generator.detect_reversed_inheritance_simple("Vehicle", "Car");
        assert!(correct_result.is_ok(), "Correct inheritance check failed: {:?}", correct_result);
        
        let (is_reversed, _) = correct_result.unwrap();
        assert!(!is_reversed, "Should not detect reversed inheritance for Vehicle, Car");
        
        // Test with non-related interfaces
        code_generator.interface_registry().register_interface("Boat").unwrap();
        let unrelated_result = code_generator.detect_reversed_inheritance_simple("Boat", "Car");
        assert!(unrelated_result.is_ok(), "Unrelated inheritance check failed: {:?}", unrelated_result);
        
        let (is_reversed, _) = unrelated_result.unwrap();
        assert!(!is_reversed, "Should not detect reversed inheritance for unrelated interfaces");
    }
}