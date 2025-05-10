use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_registry_visualization_integration::InterfaceVisualizationIntegration;
use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::interface_registry_visualization_improved::ImprovedInterfaceRegistryVisualization;
use cursed::ast::expressions::{TypeAssertion, Expression};
use cursed::ast::expressions::literal::StringLiteral;
use cursed::error::Error;

// Import the test helpers from common module
mod common;
use common::llvm_helpers;

#[test]
fn test_generate_path_visualization() {
    // Use the helper to create a code generator with proper setup
    let (ctx, module, builder, code_gen) = llvm_helpers::create_test_code_generator();
    
    // Setup the interface registry with a test hierarchy
    setup_test_registry(&code_gen);
    
    // Test generating a path visualization
    let visualization = code_gen.generate_path_visualization("Dog", "Animal").unwrap();
    
    // Should find a path from Dog to Animal
    assert!(visualization.is_some());
    
    let vis_string = visualization.unwrap();
    assert!(vis_string.contains("Dog"));
    assert!(vis_string.contains("Mammal"));
    assert!(vis_string.contains("Animal"));
    
    // No path should exist between unrelated types
    let visualization = code_gen.generate_path_visualization("Dog", "Bird").unwrap();
    assert!(visualization.is_none());
}

#[test]
fn test_create_enhanced_type_error() {
    // Use the helper to create a code generator with proper setup
    let (ctx, module, builder, code_gen) = llvm_helpers::create_test_code_generator();
    
    // Setup the interface registry with a test hierarchy
    setup_test_registry(&code_gen);
    
    // Test creating an enhanced error message
    let error_message = code_gen.create_enhanced_type_error(
        "Dog", "Bird", "test.csd:42"
    ).unwrap();
    
    // Error message should include context information
    assert!(error_message.contains("Dog"));
    assert!(error_message.contains("Bird"));
    assert!(error_message.contains("test.csd:42"));
    assert!(error_message.contains("inheritance path"));
    
    // Test with types that have a valid path
    let error_message = code_gen.create_enhanced_type_error(
        "Dog", "Animal", "test.csd:42"
    ).unwrap();
    
    // Should include path information
    assert!(error_message.contains("Dog"));
    assert!(error_message.contains("Animal"));
    assert!(error_message.contains("Path"));
}

// Helper function to set up a test registry with a hierarchy
fn setup_test_registry(code_gen: &LlvmCodeGenerator) {
    let registry = code_gen.interface_registry.as_ref().unwrap();
    
    // Define a test hierarchy
    // Animal <- Mammal <- Dog
    //        <- Bird   <- Eagle
    //        <- Fish   <- Shark
    // Mammal also implements Pet
    // Dog also implements Pet
    
    // Register interfaces and their extensions
    registry.register_interface_extension("Dog", "Mammal").unwrap();
    registry.register_interface_extension("Mammal", "Animal").unwrap();
    registry.register_interface_extension("Dog", "Pet").unwrap();
    registry.register_interface_extension("Mammal", "Pet").unwrap();
    registry.register_interface_extension("Bird", "Animal").unwrap();
    registry.register_interface_extension("Eagle", "Bird").unwrap();
    registry.register_interface_extension("Fish", "Animal").unwrap();
    registry.register_interface_extension("Shark", "Fish").unwrap();
}

// Add a stub implementation of the llvm_helpers module in case it doesn't exist
#[cfg(not(exists("tests/common/llvm_helpers.rs")))]  
mod llvm_helpers_stub {
    use inkwell::context::Context;
    use inkwell::module::Module;
    use inkwell::builder::Builder;
    use cursed::codegen::llvm::LlvmCodeGenerator;
    
    pub fn create_test_code_generator<'ctx>() -> (Context, Module<'ctx>, Builder<'ctx>, LlvmCodeGenerator<'ctx>) {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        // Create a minimal code generator with interface registry
        let mut code_gen = LlvmCodeGenerator::new_with_context(context, module, builder);
        
        // Initialize with an interface registry
        code_gen.initialize_interface_registry();
        
        (context, module, builder, code_gen)
    }
}