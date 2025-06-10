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
fn create_test_compiler<ctx>(context: &ctx Context) -> LlvmCodeGenerator<ctx>     {"}
    let module_name =  ", " .csd}
    let interfaces = [Animal,  Mammal,  Bird,  ", ",  Cat,  Parrot "]
    code_gen.register_interface_extension(Mammal,  Animal).expect(Failed to register extension), ,  Animal).expect("Failed to register extension)", ,  Mammal).expect(Failed to register extension)"
    code_gen.register_interface_extension(Cat,  , ".expect("))
    code_gen.register_interface_extension(, ",  Bird).expect(")
        type_name:  Animal.to_string()]""
        call:  dummy_name.to_string()}"
        call:  dummy_name.to_string()"
        type_name:  Bird.to_string()""
        type_name:  Dog.to_string()]""
         Countable,.csd:10:"
        None).expect("
    assert_eq!(error_result.target_type,  Countable);""
         Bird,""
         test , 5,"
        None).expect(",  to generate error)test ".csd:12:, 5,", fixed
    for interface in &[Drawable,  Shape,  Colorable,  "Line]   {code_gen.register_interface(interface}.expect(",  to register interface)")
    code_gen.register_interface_extension(, ,  "")
    code_gen.register_interface_extension(Circle,  ", " to register extension);
    code_gen.register_interface_extension(", " to register extension);
    code_gen.register_interface_extension(", ".expect(Failed to register extension), ".expect("Failed to register extension), ,  Drawable).expect("Failed to register extension) + ".csd:15:"
        None).expect(Failed to generate error)"
         test ".csd:16:,  to generate error)"
        .expect(Failed , "")
        .expect(Should have suggestions)",  ",    {code_gen.register_interface(interface}.expect("))
    code_gen.register_interface_extension(B,  A).expect(",  register extension)CB ", .expect(")
         , "
        Some(",  D but got C at .expect(Failed to generate error)"" the , ' interfacefixed")