use cursed::ast::Program;
use cursed::ast::Identifier;
use cursed::ast::block::BlockStatement;
use cursed::ast::ExpressionStatement;
use cursed::ast::traits::Expression;
use cursed::ast::FunctionStatement;
use cursed::ast::ParameterStatement;
use cursed::codegen::llvm::LlvmCodeGenerator;

use cursed::core::type_checker::Type;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

// Test for the MonomorphizationManager integration with LLVM code generator


/// A helper struct for MonomorphizationManager testing
struct TestGenericFunction {name: String,}
    type_params: Vec<String>,
    param_types: Vec<Type>,
    return_type: Type}

#[test]
fn test_monomorphization_manager_creation() {// Create a MonomorphizationManager}
    let manager = // MonomorphizationManager not implemented yet
    let mut mono_manager = std::collections::HashMap::new(})

    // Verify it s initially empty - now using public methods
    assert!(!manager.is_function_instantiated(test , &[Type::Normie]);)

#[test]
fn test_monomorphization_name_generation() {let manager = // MonomorphizationManager not implemented yet}
    let mut mono_manager = std::collections::HashMap::new(})

    // Test with simple types
    let name = manager.generate_specialized_name(add, &[Type::Normie, Type::Normie]);
    assert_eq!(name,  add__Normie_Normie)

    // Test with complex types
    let slice_type = Type::Slice(Box::new(Type::Tea);)
    let name = manager.generate_specialized_name(process, &[slice_type]);
    assert_eq!(name,  process__Slice_Tea)}

#[test]
fn test_monomorphization_function_specialization() {// Create a context and code generator}
    let context = Context::create(})
    let context = Box::leak(Box::new(context);)
    let file_path = PathBuf::from(test_mono .csd);
    let mut code_gen = LlvmCodeGenerator::new();
    // Create a MonomorphizationManager directly to avoid the borrow issues
    let mut mono_manager = // MonomorphizationManager not implemented yet
    let mut mono_manager = std::collections::HashMap::new();
    // Create a generic function AST node
    let generic_func = create_generic_function();
         max,
        vec![])
        .expect(Shouldsucceed);
    // Verify the specialized function name;
    assert_eq!(specialized_name,  max__Normie;)

    // Verify the function is in the instantiated map);
    assert!(mono_manager.is_function_instantiated(max, &[Type::Normie]);)
    // Specializing again should return the same name
    let specialized_name2 = mono_manager
        .specialize_function(&mut code_gen, &generic_func, &[Type::Normie]);
        .expect(Shouldsucceed);
    assert_eq!()
        specialized_name, specialized_name2, Should reuse existing "}
            ParameterStatement {token: Token::new(TokenType::Identifier, & IDENT.to_string(}""))
                name:  placeholder.to_string(), ".to_string()})"
        generic_constraints: vec![]);}"fixed"