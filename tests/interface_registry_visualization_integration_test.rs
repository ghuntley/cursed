// DISABLED: Missing infrastructure for interface registry visualization
#[cfg(feature = "disabled_integration_tests")]
mod disabled_tests {
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use inkwell::context::Context;
use cursed::ast::{Expression, TypeAssertion};
use cursed::ast::ExpressionStatement;
use cursed::codegen::llvm::interface_registry_visualization_integration::*;
use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::*;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::core::interface_registry_visualization::{InterfaceRegistryExtensionWithVisualization, VisualizationFormat, VisualizationOptions};
use cursed::error::Error;
use common::test_utils::create_test_code_generator;
use common::tracing;

#[cfg(test)]
mod tests {
    
    
    
    // Import common test utilities
    mod common;
    
    #[test]
    fn test_registry_initialization() {
    // init_tracing!();
        // Initialize tracing for this test
        tracing::setup();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Initialize registry visualization
        let result = code_generator.initialize_registry_visualization();
        assert!(result.is_ok(), "Failed to initialize registry visualization: {:?}", result);
        
        // Initialization should be idempotent, so calling it again should succeed
        let second_result = code_generator.initialize_registry_visualization();
        assert!(second_result.is_ok(), "Second initialization should succeed: {:?}", second_result);
    }
    
    #[test]
    fn test_interface_hierarchy_visualization() {
    // init_tracing!();
        // Initialize tracing for this test
        tracing::setup();
        
        // Create a test code generator with some test interfaces
        let mut code_generator = create_test_code_generator();
        
        // Register some test interfaces
        code_generator.registry_extensions.register_extension("Animal", "Mammal").unwrap();
        code_generator.registry_extensions.register_extension("Mammal", "Dog").unwrap();
        code_generator.registry_extensions.register_extension("Mammal", "Cat").unwrap();
        code_generator.registry_extensions.register_extension("Animal", "Bird").unwrap();
        
        // Initialize registry visualization
        code_generator.initialize_registry_visualization().unwrap();
        
        // Test ASCII visualization
        let options = VisualizationOptions::default();
        let ascii_result = code_generator.visualize_interface_hierarchy(
            VisualizationFormat::Ascii,
            &options
        );
        
        assert!(ascii_result.is_ok(), "ASCII visualization failed: {:?}", ascii_result);
        let ascii_output = ascii_result.unwrap();
        assert!(ascii_output.contains("Animal"), "ASCII output should contain 'Animal'");
        assert!(ascii_output.contains("Mammal"), "ASCII output should contain 'Mammal'");
        assert!(ascii_output.contains("Dog"), "ASCII output should contain 'Dog'");
        
        // Test DOT visualization
        let dot_result = code_generator.visualize_interface_hierarchy(
            VisualizationFormat::Dot,
            &options
        );
        
        assert!(dot_result.is_ok(), "DOT visualization failed: {:?}", dot_result);
        let dot_output = dot_result.unwrap();
        assert!(dot_output.contains("digraph"), "DOT output should contain 'digraph'");
        assert!(dot_output.contains("Animal"), "DOT output should contain 'Animal'");
        assert!(dot_output.contains("-> \"Mammal\""), "DOT output should show inheritance");
    }
    
    #[test]
    fn test_find_and_visualize_inheritance_path() {
    // init_tracing!();
        // Initialize tracing for this test
        tracing::setup();
        
        // Create a test code generator with some test interfaces
        let mut code_generator = create_test_code_generator();
        
        // Register a chain of interfaces
        code_generator.registry_extensions.register_extension("Vehicle", "LandVehicle").unwrap();
        code_generator.registry_extensions.register_extension("LandVehicle", "Car").unwrap();
        code_generator.registry_extensions.register_extension("Car", "Sedan").unwrap();
        
        // Initialize registry visualization
        code_generator.initialize_registry_visualization().unwrap();
        
        // Test finding and visualizing a path with ASCII format
        let ascii_result = code_generator.find_and_visualize_inheritance_path(
            "Vehicle",
            "Sedan",
            VisualizationFormat::Ascii
        );
        
        assert!(ascii_result.is_ok(), "ASCII path visualization failed: {:?}", ascii_result);
        let ascii_output = ascii_result.unwrap();
        assert!(ascii_output.contains("Vehicle"), "ASCII output should contain 'Vehicle'");
        assert!(ascii_output.contains("LandVehicle"), "ASCII output should contain 'LandVehicle'");
        assert!(ascii_output.contains("Car"), "ASCII output should contain 'Car'");
        assert!(ascii_output.contains("Sedan"), "ASCII output should contain 'Sedan'");
        assert!(ascii_output.contains("extends"), "ASCII output should show relationship");
        
        // Test finding and visualizing a path with DOT format
        let dot_result = code_generator.find_and_visualize_inheritance_path(
            "Vehicle",
            "Sedan",
            VisualizationFormat::Dot
        );
        
        assert!(dot_result.is_ok(), "DOT path visualization failed: {:?}", dot_result);
        let dot_output = dot_result.unwrap();
        assert!(dot_output.contains("digraph"), "DOT output should contain 'digraph'");
        assert!(dot_output.contains("Vehicle"), "DOT output should contain 'Vehicle'");
        assert!(dot_output.contains("Sedan"), "DOT output should contain 'Sedan'");
        assert!(dot_output.contains("-> \"LandVehicle\""), "DOT output should show inheritance");
    }
    
    #[test]
    fn test_cycle_detection() {
    // init_tracing!();
        // Initialize tracing for this test
        tracing::setup();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register interfaces without cycles
        code_generator.registry_extensions.register_extension("Interface1", "Interface2").unwrap();
        code_generator.registry_extensions.register_extension("Interface2", "Interface3").unwrap();
        
        // Initialize registry visualization
        code_generator.initialize_registry_visualization().unwrap();
        
        // Test cycle detection - should find no cycles
        let result = code_generator.detect_inheritance_cycles();
        assert!(result.is_ok(), "Cycle detection failed: {:?}", result);
        let cycles = result.unwrap();
        assert!(cycles.is_empty(), "Should not detect cycles in a valid hierarchy");
        
        // Now create a cycle
        code_generator.registry_extensions.register_extension("Interface3", "Interface1").unwrap();
        
        // Test cycle detection again - should find the cycle
        let result = code_generator.detect_inheritance_cycles();
        assert!(result.is_ok(), "Cycle detection failed after adding cycle: {:?}", result);
        let cycles = result.unwrap();
        assert!(!cycles.is_empty(), "Should detect cycles in a cyclic hierarchy");
        
        // Verify the cycle contains the expected interfaces
        let cycle = &cycles[0];
        assert!(cycle.contains(&"Interface1".to_string()), "Cycle should contain Interface1");
        assert!(cycle.contains(&"Interface2".to_string()), "Cycle should contain Interface2");
        assert!(cycle.contains(&"Interface3".to_string()), "Cycle should contain Interface3");
    }
    
    #[test]
    fn test_check_interface_extension_relationship() {
    // init_tracing!();
        // Initialize tracing for this test
        tracing::setup();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register some interface relationships
        code_generator.registry_extensions.register_extension("Reader", "FileReader").unwrap();
        code_generator.registry_extensions.register_extension("FileReader", "BufferedFileReader").unwrap();
        
        // Initialize registry visualization
        code_generator.initialize_registry_visualization().unwrap();
        
        // Test direct relationship
        let direct_result = code_generator.check_interface_extension_relationship("Reader", "FileReader");
        assert!(direct_result.is_ok(), "Direct relationship check failed: {:?}", direct_result);
        assert!(direct_result.unwrap(), "Should detect direct extension relationship");
        
        // Test indirect relationship
        let indirect_result = code_generator.check_interface_extension_relationship("Reader", "BufferedFileReader");
        assert!(indirect_result.is_ok(), "Indirect relationship check failed: {:?}", indirect_result);
        assert!(indirect_result.unwrap(), "Should detect indirect extension relationship");
        
        // Test non-existent relationship
        let nonexistent_result = code_generator.check_interface_extension_relationship("Reader", "NetworkReader");
        assert!(nonexistent_result.is_ok(), "Non-existent relationship check failed: {:?}", nonexistent_result);
        assert!(!nonexistent_result.unwrap(), "Should not detect non-existent relationship");
        
        // Test reversed relationship
        let reversed_result = code_generator.check_interface_extension_relationship("FileReader", "Reader");
        assert!(reversed_result.is_ok(), "Reversed relationship check failed: {:?}", reversed_result);
        assert!(!reversed_result.unwrap(), "Should not detect reversed relationship");
    }
    
    #[test]
    fn test_enhanced_assertion_error() {
    // init_tracing!();
        // Initialize tracing for this test
        tracing::setup();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register some interface relationships
        code_generator.registry_extensions.register_extension("Animal", "Mammal").unwrap();
        code_generator.registry_extensions.register_extension("Mammal", "Dog").unwrap();
        code_generator.registry_extensions.register_extension("Animal", "Bird").unwrap();
        
        // Initialize registry visualization
        code_generator.initialize_registry_visualization().unwrap();
        
        // Test generating error for incompatible interfaces
        let error_result = code_generator.generate_enhanced_assertion_error(
            "Bird",
            "Dog",
            "test.csd:42"
        );
        
        assert!(error_result.is_ok(), "Error generation failed: {:?}", error_result);
        let error_message = error_result.unwrap();
        
        // Check error message content
        assert!(error_message.contains("Bird"), "Error should mention source interface");
        assert!(error_message.contains("Dog"), "Error should mention target interface");
        assert!(error_message.contains("test.csd:42"), "Error should include source location");
        assert!(error_message.contains("No inheritance path") || 
                error_message.contains("Alternative paths"), 
                "Error should mention path information");
        
        // Test generating error for reversed relationship
        let reversed_error_result = code_generator.generate_enhanced_assertion_error(
            "Dog",
            "Animal",
            "test.csd:42"
        );
        
        assert!(reversed_error_result.is_ok(), "Reversed error generation failed: {:?}", reversed_error_result);
        let reversed_error_message = reversed_error_result.unwrap();
        
        // Check reversed error message content
        assert!(reversed_error_message.contains("relationship appears to be reversed"), 
                "Error should detect reversed relationship: {}", reversed_error_message);
    }
} // end disabled_tests module
} // end cfg feature guard