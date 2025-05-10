#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use inkwell::context::Context;
    
    use cursed::codegen::llvm::LlvmCodeGenerator;
    use cursed::codegen::llvm::interface_path_finder::*;
    use cursed::error::Error;
    
    // Import common test utilities
    mod common;
    use common::test_utils::create_test_code_generator;
    use common::tracing;
    
    #[test]
    fn test_find_interface_path() {
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
        let path_result = code_generator.find_interface_path("Animal", "Dog");
        assert!(path_result.is_ok(), "Path finding failed: {:?}", path_result);
        
        let path = path_result.unwrap();
        assert_eq!(path.len(), 3, "Path should have 3 elements");
        assert_eq!(path[0], "Animal");
        assert_eq!(path[1], "Mammal");
        assert_eq!(path[2], "Dog");
        
        // Test path to same interface
        let same_path_result = code_generator.find_interface_path("Animal", "Animal");
        assert!(same_path_result.is_ok(), "Path to same interface failed: {:?}", same_path_result);
        
        let same_path = same_path_result.unwrap();
        assert_eq!(same_path.len(), 1, "Path to same interface should have 1 element");
        assert_eq!(same_path[0], "Animal");
        
        // Test non-existent path
        let non_existent_path_result = code_generator.find_interface_path("Dog", "Animal");
        assert!(non_existent_path_result.is_err(), "Non-existent path should fail");
    }
    
    #[test]
    fn test_find_alternative_paths_enhanced() {
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
        let paths_result = code_generator.find_alternative_paths_enhanced("Animal", "Dog", 3);
        assert!(paths_result.is_ok(), "Alternative path finding failed: {:?}", paths_result);
        
        let paths = paths_result.unwrap();
        assert!(paths.len() >= 2, "Should find at least 2 paths");
        
        // Verify first path
        let first_path = &paths[0];
        assert!(first_path.contains(&"Animal".to_string()), "First path should contain Animal");
        assert!(first_path.contains(&"Dog".to_string()), "First path should contain Dog");
        
        // Verify second path
        let second_path = &paths[1];
        assert!(second_path.contains(&"Animal".to_string()), "Second path should contain Animal");
        assert!(second_path.contains(&"Dog".to_string()), "Second path should contain Dog");
        
        // Verify paths are different
        assert_ne!(first_path, second_path, "Paths should be different");
    }
    
    #[test]
    fn test_check_extension_relationship_enhanced() {
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
        let direct_result = code_generator.check_extension_relationship_enhanced("Vehicle", "LandVehicle");
        assert!(direct_result.is_ok(), "Direct relationship check failed: {:?}", direct_result);
        assert!(direct_result.unwrap(), "Should detect direct extension relationship");
        
        // Test indirect extension relationship
        let indirect_result = code_generator.check_extension_relationship_enhanced("Vehicle", "Car");
        assert!(indirect_result.is_ok(), "Indirect relationship check failed: {:?}", indirect_result);
        assert!(indirect_result.unwrap(), "Should detect indirect extension relationship");
        
        // Test unrelated interfaces
        let unrelated_result = code_generator.check_extension_relationship_enhanced("Car", "Boat");
        assert!(unrelated_result.is_ok(), "Unrelated relationship check failed: {:?}", unrelated_result);
        assert!(!unrelated_result.unwrap(), "Should not detect relationship between unrelated interfaces");
        
        // Test reversed relationship
        let reversed_result = code_generator.check_extension_relationship_enhanced("Car", "Vehicle");
        assert!(reversed_result.is_ok(), "Reversed relationship check failed: {:?}", reversed_result);
        assert!(!reversed_result.unwrap(), "Should not detect reversed relationship");
    }
    
    #[test]
    fn test_find_all_interface_implementors_enhanced() {
        // Initialize tracing for this test
        tracing::setup();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register test interfaces with a hierarchy
        code_generator.interface_registry().register_interface("Reader").unwrap();
        code_generator.interface_registry().register_interface("FileReader").unwrap();
        code_generator.interface_registry().register_interface("BufferedReader").unwrap();
        code_generator.interface_registry().register_interface("TextFileReader").unwrap();
        code_generator.interface_registry().register_interface("BinaryFileReader").unwrap();
        
        // Register extension relationships
        code_generator.interface_registry().register_extension("Reader", "FileReader").unwrap();
        code_generator.interface_registry().register_extension("Reader", "BufferedReader").unwrap();
        code_generator.interface_registry().register_extension("FileReader", "TextFileReader").unwrap();
        code_generator.interface_registry().register_extension("FileReader", "BinaryFileReader").unwrap();
        
        // Test finding all implementors of Reader
        let implementors_result = code_generator.find_all_interface_implementors_enhanced("Reader");
        assert!(implementors_result.is_ok(), "Finding implementors failed: {:?}", implementors_result);
        
        let implementors = implementors_result.unwrap();
        assert_eq!(implementors.len(), 4, "Should find 4 implementors of Reader");
        assert!(implementors.contains(&"FileReader".to_string()), "Should include FileReader");
        assert!(implementors.contains(&"BufferedReader".to_string()), "Should include BufferedReader");
        assert!(implementors.contains(&"TextFileReader".to_string()), "Should include TextFileReader");
        assert!(implementors.contains(&"BinaryFileReader".to_string()), "Should include BinaryFileReader");
        
        // Test finding implementors of FileReader
        let file_reader_implementors_result = code_generator.find_all_interface_implementors_enhanced("FileReader");
        assert!(file_reader_implementors_result.is_ok(), "Finding FileReader implementors failed: {:?}", file_reader_implementors_result);
        
        let file_implementors = file_reader_implementors_result.unwrap();
        assert_eq!(file_implementors.len(), 2, "Should find 2 implementors of FileReader");
        assert!(file_implementors.contains(&"TextFileReader".to_string()), "Should include TextFileReader");
        assert!(file_implementors.contains(&"BinaryFileReader".to_string()), "Should include BinaryFileReader");
        
        // Test finding implementors of leaf interface
        let leaf_implementors_result = code_generator.find_all_interface_implementors_enhanced("TextFileReader");
        assert!(leaf_implementors_result.is_ok(), "Finding leaf implementors failed: {:?}", leaf_implementors_result);
        
        let leaf_implementors = leaf_implementors_result.unwrap();
        assert_eq!(leaf_implementors.len(), 0, "Leaf interface should have no implementors");
    }
}