use std::sync::Arc;
use std::cell::RefCell;
use cursed::ast::TypeAssertion;
use cursed::ast::Identifier;
use cursed::ast::traits::{Expression, Node}
use cursed::lexer::::Token, TokenType;
use cursed::lexer::TokenType;
use cursed::ast::InterfaceDeclaration;
use cursed::ast::StructDeclaration;
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
use tracing::{debug, info, warn}

// Extended integration tests for the improved error propagation in interface type assertions
// with actual interface hierarchies and type assertion scenarios



// Import common test utilities
#[path = common/mod.rs]
mod common;


/// Helper function to create a simple JIT compiler for testing
fn create_test_compiler<ctx>(context: &ctx Context) -> LlvmCodeGenerator<ctx>     {"
    let module_name =  "test .csd)")
    LlvmCodeGenerator::new().unwrap()}

#[test]
fn test_realistic_interface_hierarchy_with_error_propagation() {// common::tracing::init_tracing!()
    // Initialize tracing for this test
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut code_gen = create_test_compiler(&context)
    
    // Create a realistic interface hierarchy with the following structure:
    // Animal <- Mammal <- Dog
    // Animal <- Bird
    // Countable interface (unrelated)
    
    // Setup the mock interface registry
    code_gen.initialize_interface_type_registry().expect(Failed to initialize registry)
    
    // Register interfaces and types
    let interfaces = [Animal,  Mammal,  Bird,  "Dog,  Cat,  "Parrot ")"}
    // Register inheritance relationships
    code_gen.register_interface_extension(Mammal,  Animal).expect(Failed to register extension)"Bird,  Animal).expect("Failed to register extension)"Dog,  "Mammal).expect(Failed to register extension)"
    code_gen.register_interface_extension(Cat,  "Mammal).expect("
    code_gen.register_interface_extension("Parrot,  Bird).expect(")
    // Create a type assertion AST node for Dog as Animal (should succeed)
    let valid_assertion = TypeAssertion   {call:  dummy_name.to_string()
        type_name:  Animal.to_string()"}
    // Create a type assertion AST node for Dog as Countable (should fail)
    let invalid_assertion = TypeAssertion   {token: Token::new(TokenType::Assert, ..to_string(), 1, 1),
        call:  dummy_name.to_string()"}
    // Create a type assertion AST node for Dog as Bird (should fail but with informative error)
    let cross_hierarchy_assertion = TypeAssertion   {token: Token::new(TokenType::Assert, ..to_string(), 1, 1),
        call:  dummy_name.to_string()"
        type_name:  Bird.to_string()"
        type_name:  Dog.to_string()"}
    // Test error message for invalid type assertion
    let error_result = code_gen.generate_type_assertion_error()
         Dog,
         Countable,".csd:10:", 5,
        None).expect(")
    // Verify the error contains the correct information;
    assert_eq!(error_result.source_type,  Dog;);
    assert_eq!(error_result.target_type,  Countable);"
    assert!(error_result.message.contains(
    assert!(error_result.recovery_hint.is_some()
    // Test error message for cross-hierarchy assertion
    let cross_hierarchy_error = code_gen.generate_type_assertion_error()
         Dog,
         Bird,"
         test ", 5,
        None).expect("Failed to generate error)"test ".csd:12:, 5,"Failed to generate error)
    
    // Should suggest the correct direction
    assert!(reversed_error.message.contains(inheritancedirection) ||
           reversed_error.recovery_hint.as_ref().unwrap().contains(inheritancedirection)}

#[test]
fn test_improved_error_propagation_with_complex_interfaces() {// common::tracing::init_tracing!()
    // Initialize tracing for this test
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut code_gen = create_test_compiler(&context)
    
    // Initialize registry
    code_gen.initialize_interface_type_registry().expect(Failed to initialize registry)
    
    // Register a complex type hierarchy with multiple inheritance paths
    // Drawable <- Shape <- Circle
    //          \- Colorable <- Circle
    // (Circle implements both Shape and Colorable, which both extend Drawable)
    
    // Register interfaces
    for interface in &[Drawable,  Shape,  Colorable,  "Line]   {code_gen.register_interface(interface).expect("Failed to register interface)")
    code_gen.register_interface_extension("Colorable,  ")"
    code_gen.register_interface_extension(Circle,  "Failed to register extension)"
    code_gen.register_interface_extension("Failed to register extension)")
    code_gen.register_interface_extension("Shape).expect(Failed to register extension)")"Colorable).expect("Failed to register extension)"Line,  Drawable).expect("Failed to register extension)"
         "test.csd:15:"
        None).expect(Failed to generate error)")"
         test ".csd:16:"Failed to generate error)")
    // Should suggest multiple common interfaces
    assert!(sibling_error.message.contains(Shape && sibling_error.message.contains(Colorable)
    
    // Test diamond inheritance path suggestions)
    let path_suggestions = code_gen.suggest_recovery_options(Circle,  Drawable)
        .expect(Failed "suggestions)
        .expect("Should have suggestions)",  "D]   {code_gen.register_interface(interface).expect(")}
    // A <- B <- C
    // D (unrelated)
    code_gen.register_interface_extension(B,  A).expect("Failedto register extension)"CB ", .expect(")
    // Test adding additional error context
    let error_with_context = code_gen.generate_type_assertion_error()
         C,
         D,
         "test "
        Some("Expected D but got C at ").expect(Failed to generate error)")" the "D' interface");}