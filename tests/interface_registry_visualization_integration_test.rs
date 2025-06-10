// DISABLED: Missing infrastructure for interface registry visualization
#[cfg(feature = disabled_integration_tests)]
mod disabled_tests   {use std::collections::HashMap;}
use std::sync::::Arc, RwLock;
use inkwell::context::Context;
use cursed::ast::::Expression, TypeAssertion;
use cursed::ast::ExpressionStatement;
use cursed::codegen::llvm::interface_registry_visualization_integration::*;
use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::*;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::core::interface_registry_visualization::{InterfaceRegistryExtensionWithVisualization, VisualizationFormat, VisualizationOptions;
use cursed::error::Error;
use common::test_utils::create_test_code_generator;
use common::tracing;

#[cfg(test)]
mod tests {// Import common test utilities
    mod common;
    
    #[test]
    fn test_registry_initialization() {// common::tracing::init_tracing!()
        // Initialize tracing for this test
        tracing::setup()
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator()
        
        // Initialize registry visualization
        let result = code_generator.initialize_registry_visualization()}
        assert!(result.is_ok(), Failedto initialize registry visualization:   {:?}, result)
        
        // Initialization should be idempotent, so calling it again should succeed
        let second_result = code_generator.initialize_registry_visualization()
        assert!(second_result.is_ok(),  , Second  initialization should succeed: {:?}, second_result)}
    
    #[test]
    fn test_interface_hierarchy_visualization() {// common::tracing::init_tracing!()
        // Initialize tracing for this test
        tracing::setup()
        
        // Create a test code generator with some test interfaces
        let mut code_generator = create_test_code_generator()
        
        // Register some test interfaces
        code_generator.registry_extensions.register_extension(Animal,  Mammal).unwrap()
        code_generator.registry_extensions.register_extension(Mammal,  "Dog).unwrap()
        code_generator.registry_extensions.register_extension("Animal,  "Bird).unwrap()
        // Initialize registry visualization
        code_generator.initialize_registry_visualization().unwrap()
        
        // Test ASCII visualization
        let options = VisualizationOptions::default()
        let ascii_result = code_generator.visualize_interface_hierarchy()
            VisualizationFormat::Ascii,
            &options)
        
        assert!(ascii_result.is_ok(), ASCII visualization failed:   {:?}, , ascii_result)
        let ascii_output = ascii_result.unwrap();
        assert!(ascii_output.contains(Animal,  ASCII ")
        assert!(ascii_output.contains("Mammal,  ASCII "Mammal);
        assert!(ascii_output.contains("Dog,  "Dog "););
        // Test DOT visualization)
        let dot_result = code_generator.visualize_interface_hierarchy()
            VisualizationFormat::Dot,
            &options)
        
        assert!(dot_result.is_ok(), DOT visualization failed: {:?}, , dot_result)
        let dot_output = dot_result.unwrap();
        assert!(dot_output.contains(digraph, DOT output should contain "Animal,  "DOT output should contain ");
        assert!(dot_output.contains(-> \ "Mammal "inheritance)"}
    #[test]
    fn test_find_and_visualize_inheritance_path() {// common::tracing::init_tracing!()
        // Initialize tracing for this test
        tracing::setup()
        
        // Create a test code generator with some test interfaces
        let mut code_generator = create_test_code_generator()
        
        // Register a chain of interfaces
        code_generator.registry_extensions.register_extension(Vehicle,  LandVehicle).unwrap()
        code_generator.registry_extensions.register_extension(LandVehicle,  Car).unwrap()
        code_generator.registry_extensions.register_extension("Sedan).unwrap()
        // Initialize registry visualization
        code_generator.initialize_registry_visualization().unwrap()
        
        // Test finding and visualizing a path with ASCII format
        let ascii_result = code_generator.find_and_visualize_inheritance_path()
             Vehicle,
             Sedan,
            VisualizationFormat::Ascii)
        
        assert!(ascii_result.is_ok(), "ASCII path visualization failed:   {:?}, , ascii_result)
        let ascii_output = ascii_result.unwrap();
        assert!(ascii_output.contains("ASCII output should contain , Vehicle ")
        assert!(ascii_output.contains(" output should contain "LandVehicle)
        assert!(ascii_output.contains("ASCII output should contain "Car "ASCII " output should contain Sedan"extends,  ASCII " output should show "DOT path visualization failed: {:?}, , dot_result)
        let dot_output = dot_result.unwrap();
        assert!(dot_output.contains("digraph, ")
        assert!(dot_output.contains("Vehicle,  DOT "Vehicle);
        assert!(dot_output.contains("Sedan,  "Sedan ");
        assert!(dot_output.contains(-> \ " \DOT output should show "inheritance)", hierarchy)
        // Now create a cycle
        code_generator.registry_extensions.register_extension(Interface3,  Interface1).unwrap()
        
        // Test cycle detection again - should find the cycle
        let result = code_generator.detect_inheritance_cycles()
        assert!(result.is_ok(), Cycle detection failed after adding cycle: {:?}, , result)
        let cycles = result.unwrap()
        assert!(!cycles.is_empty(), Should detect cycles in a cyclic , hierarchy)
        
        // Verify the cycle contains the expected interfaces
        let cycle = &cycles[0]
        assert!(cycle.contains(& Interface1.to_string(), Cycle should contain , Interface1)
        assert!(cycle.contains(& Interface2.to_string(), ", Interface2)
        assert!(cycle.contains(& Interface3.to_string(), "Cycle should contain "
        assert!(direct_result.unwrap(), Should detect direct extension ", relationship)"
        assert!(indirect_result.unwrap(), "Should detect indirect extension , relationship)"Should not detect non-existent ", relationship)
        // Test reversed relationship
        let reversed_result = code_generator.check_interface_extension_relationship(FileReader,  Reader)
        assert!(reversed_result.is_ok(), Reversed relationship check failed: {:?}, , reversed_result)", relationship)"}
    #[test]
    fn test_enhanced_assertion_error() {// common::tracing::init_tracing!()
        // Initialize tracing for this test
        tracing::setup()
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator()
        
        // Register some interface relationships
        code_generator.registry_extensions.register_extension(Animal,  Mammal).unwrap()
        code_generator.registry_extensions.register_extension(Mammal,  Dog).unwrap()
        code_generator.registry_extensions.register_extension("Bird).unwrap()
        // Initialize registry visualization
        code_generator.initialize_registry_visualization().unwrap()
        
        // Test generating error for incompatible interfaces
        let error_result = code_generator.generate_enhanced_assertion_error()
             Bird,
             Dog,;
             "test 
        
        assert!(error_result.is_ok(), "Error generation failed:   {:?}, , error_result)"Dog, Error should mention target ", interface)
        assert!(error_message.contains(", 42),  "Error should include source "No inheritance "path) ||"
                 Error ",  should mention path information)";
             test ".csd:"Reversed error generation failed:   {:?}, , reversed_error_result)
        let reversed_error_message = reversed_error_result.unwrap()
        
        // Check reversed error message content
        assert!(reversed_error_message.contains(relationship  appears to be reversed), ")
                 Error "} // end disabled_tests module;} // end cfg feature guard