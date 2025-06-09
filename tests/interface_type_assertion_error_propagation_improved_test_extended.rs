use std::sync::Arc;
use std::cell::RefCell;
use cursed::ast::expressions::TypeAssertion;
use cursed::ast::expressions::Identifier;
use cursed::ast::traits::{Expression, Node};
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use cursed::ast::statements::InterfaceDeclaration;
use cursed::ast::statements::StructDeclaration;
use cursed::ast::Method;
use cursed::ast::TypeParameter;
use cursed::codegen::llvm::interface_type_assertion_error_propagation_improved::*;
use cursed::codegen::llvm::interface_type_assertion_error_propagation_improved::ImprovedTypeAssertionErrorPropagation;
use cursed::codegen::llvm::interface_type_assertion::ImprovedTypeAssertion;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use inkwell::context::Context;
use tracing::{debug, info, warn};

// Extended integration tests for the improved error propagation in interface type assertions
// with actual interface hierarchies and type assertion scenarios



// Import common test utilities
#[path = "common/mod.rs"]
pub mod common;


/// Helper function to create a simple JIT compiler for testing
fn create_test_compiler<'ctx>(context: &'ctx Context) -> LlvmCodeGenerator<'ctx> {
    let module_name = "test_module";
    let file_path = std::path::PathBuf::from("test.csd");
    
    LlvmCodeGenerator::new(context, module_name, file_path)
}

#[test]
fn test_realistic_interface_hierarchy_with_error_propagation() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    let context = Context::create();
    let mut code_gen = create_test_compiler(&context);
    
    // Create a realistic interface hierarchy with the following structure:
    // Animal <- Mammal <- Dog
    // Animal <- Bird
    // Countable interface (unrelated)
    
    // Setup the mock interface registry
    code_gen.initialize_interface_type_registry().expect("Failed to initialize registry");
    
    // Register interfaces and types
    let interfaces = [
        "Animal", "Mammal", "Bird", "Countable", "Dog", "Cat", "Parrot"
    ];
    
    for interface in &interfaces {
        code_gen.register_interface(interface).expect("Failed to register interface");
    }
    
    // Register inheritance relationships
    code_gen.register_interface_extension("Mammal", "Animal").expect("Failed to register extension");
    code_gen.register_interface_extension("Bird", "Animal").expect("Failed to register extension");
    code_gen.register_interface_extension("Dog", "Mammal").expect("Failed to register extension");
    code_gen.register_interface_extension("Cat", "Mammal").expect("Failed to register extension");
    code_gen.register_interface_extension("Parrot", "Bird").expect("Failed to register extension");
    
    // Create a type assertion AST node for Dog as Animal (should succeed)
    let valid_assertion = TypeAssertion {
        token: "token".to_string(),
        expression: Box::new(Identifier {
            token: "token".to_string(),
            value: "pet".to_string(),
        }),
        type_name: "Animal".to_string(),
    };
    
    // Create a type assertion AST node for Dog as Countable (should fail)
    let invalid_assertion = TypeAssertion {
        token: Token::new(TokenType::Assert, ".".to_string(), 1, 1),
        expression: Box::new(Identifier {
            token: "token".to_string(),
            value: "pet".to_string(),
        }),
        type_name: "Countable".to_string(),
    };
    
    // Create a type assertion AST node for Dog as Bird (should fail but with informative error)
    let cross_hierarchy_assertion = TypeAssertion {
        token: Token::new(TokenType::Assert, ".".to_string(), 1, 1),
        expression: Box::new(Identifier {
            token: "token".to_string(), 1, 1),
            value: "pet".to_string(),
        }),
        type_name: "Bird".to_string(),
    };
    
    // Create a type assertion AST node for Mammal as Dog (reversed, should fail)
    let reversed_assertion = TypeAssertion {
        token: Token::new(TokenType::Assert, ".".to_string(), 1, 1),
        expression: Box::new(Identifier {
            token: "token".to_string(), 1, 1),
            value: "pet".to_string(),
        }),
        type_name: "Dog".to_string(),
    };
    
    // Test error message for invalid type assertion
    let error_result = code_gen.generate_type_assertion_error(
        "Dog",
        "Countable",
        "test.csd:10:5",
        None
    ).expect("Failed to generate error");
    
    // Verify the error contains the correct information
    assert_eq!(error_result.source_type, "Dog");
    assert_eq!(error_result.target_type, "Countable");
    assert!(error_result.message.contains("cannot be asserted");
    assert!(error_result.recovery_hint.is_some();
    
    // Test error message for cross-hierarchy assertion
    let cross_hierarchy_error = code_gen.generate_type_assertion_error(
        "Dog",
        "Bird",
        "test.csd:11:5",
        None
    ).expect("Failed to generate error");
    
    // Should suggest common ancestor (Animal)
    assert!(cross_hierarchy_error.message.contains("Animal");
    
    // Test error message for reversed inheritance
    let reversed_error = code_gen.generate_type_assertion_error(
        "Mammal",
        "Dog",
        "test.csd:12:5",
        None
    ).expect("Failed to generate error");
    
    // Should suggest the correct direction
    assert!(reversed_error.message.contains("inheritance direction") || 
           reversed_error.recovery_hint.as_ref().unwrap().contains("inheritance direction");
}

#[test]
fn test_improved_error_propagation_with_complex_interfaces() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    let context = Context::create();
    let mut code_gen = create_test_compiler(&context);
    
    // Initialize registry
    code_gen.initialize_interface_type_registry().expect("Failed to initialize registry");
    
    // Register a complex type hierarchy with multiple inheritance paths
    // Drawable <- Shape <- Circle
    //          \- Colorable <- Circle
    // (Circle implements both Shape and Colorable, which both extend Drawable)
    
    // Register interfaces
    for interface in &["Drawable", "Shape", "Colorable", "Circle", "Rectangle", "Line"] {
        code_gen.register_interface(interface).expect("Failed to register interface");
    }
    
    // Register inheritance relationships
    code_gen.register_interface_extension("Shape", "Drawable").expect("Failed to register extension");
    code_gen.register_interface_extension("Colorable", "Drawable").expect("Failed to register extension");
    code_gen.register_interface_extension("Circle", "Shape").expect("Failed to register extension");
    code_gen.register_interface_extension("Circle", "Colorable").expect("Failed to register extension");
    code_gen.register_interface_extension("Rectangle", "Shape").expect("Failed to register extension");
    code_gen.register_interface_extension("Rectangle", "Colorable").expect("Failed to register extension");
    code_gen.register_interface_extension("Line", "Drawable").expect("Failed to register extension");
    
    // Test complex error cases that require path visualization
    
    // Create error for Line -> Circle (same root but different paths)
    let error_message = code_gen.generate_type_assertion_error(
        "Line",
        "Circle",
        "test.csd:15:10",
        None
    ).expect("Failed to generate error");
    
    // Should identify common ancestor (Drawable)
    assert!(error_message.message.contains("Drawable");
    
    // Create error for Rectangle -> Circle (siblings relationship)
    let sibling_error = code_gen.generate_type_assertion_error(
        "Rectangle",
        "Circle",
        "test.csd:16:10",
        None
    ).expect("Failed to generate error");
    
    // Should suggest multiple common interfaces
    assert!(sibling_error.message.contains("Shape") && sibling_error.message.contains("Colorable");
    
    // Test diamond inheritance path suggestions
    let path_suggestions = code_gen.suggest_recovery_options("Circle", "Drawable")
        .expect("Failed to get suggestions")
        .expect("Should have suggestions");
    
    // Should mention multiple paths
    assert!(path_suggestions.contains("multiple paths") || path_suggestions.contains("multiple inheritance");
}

#[test]
fn test_detailed_error_context_in_propagation() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    let context = Context::create();
    let mut code_gen = create_test_compiler(&context);
    
    // Initialize registry
    code_gen.initialize_interface_type_registry().expect("Failed to initialize registry");
    
    // Register interfaces for a simple hierarchy
    for interface in &["A", "B", "C", "D"] {
        code_gen.register_interface(interface).expect("Failed to register interface");
    }
    
    // A <- B <- C
    // D (unrelated)
    code_gen.register_interface_extension("B", "A").expect("Failed to register extension");
    code_gen.register_interface_extension("C", "B").expect("Failed to register extension");
    
    // Test adding additional error context
    let error_with_context = code_gen.generate_type_assertion_error(
        "C",
        "D",
        "test.csd:20:15",
        Some("Expected D but got C at runtime".to_string()
    ).expect("Failed to generate error");
    
    // Check that additional context is included
    assert!(error_with_context.message.contains("Expected D but got C at runtime");
    
    // Check that source location is included
    assert!(error_with_context.message.contains("test.csd:20:15");
    
    // Check that runtime information is included
    assert!(error_with_context.message.contains("C is not a D");
    
    // Check that recovery suggestion points to the unrelated interfaces
    let hint = error_with_context.recovery_hint.expect("Should have recovery hint");
    assert!(hint.contains("implement the 'D' interface");
}