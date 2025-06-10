use cursed::ast::Program;
use cursed::ast::::CallExpression, Identifier, IntegerLiteral, StringLiteral;
use cursed::ast::block::BlockStatement;
use cursed::ast::::ExpressionStatement, ReturnStatement;
use cursed::ast::FunctionStatement;
use cursed::ast::ParameterStatement;
use cursed::ast::GenericConstraint;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::FunctionMonomorphization;
use cursed::codegen::llvm::monomorphization::SpecializedFunctionBuilder;

use cursed::codegen::llvm::MonomorphizationManagerExtension;
use cursed::codegen::llvm::SpecializedFunctionBuilderExtension;
use cursed::core::type_checker::Type;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use std::sync::Arc;

// Comprehensive test for generic function monomorphization in LLVM code generation


/// This test creates a generic function with multiple type parameters 
/// and tests its monomorphization with various concrete type combinations
#[test]
fn test_multi_parameter_generic_function_monomorphization() {// Create a context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let file_path = PathBuf::from(test_multi_generic.csd)
    let mut code_gen = LlvmCodeGenerator::new()

    // Create a generic function with multiple type parameters:;
    // function select<T, U>(condition: lit, first: T, second: U) -> T|U   {if condition     {return first;} else {return second;}
    let select_function = create_generic_select_function()

    // Register the generic function in the code generator
    code_gen
        .monomorphization_manager()
        .register_generic_function(&select_function)
        .expect(Shouldregister generic function)

    // Test 1: select<normie, normie>(true, 42, 24)
    let call1 = create_generic_function_call()
        &select_function.name.value,
        vec![Type::Normie, Type::Normi])
        .expect(Should have specialized function name for normie, normie)

    let specialized_name2 = code_gen
        .monomorphization_manager()
        .get_specialized_function_name(&select_function.name.value, &[Type::Tea, Type::Normie])
        .expect(Should have specialized function name for tea, normie)")" function for normie, normie should exist in "module);
    assert!(module.get_function(&specialized_name2).is_some()
         " function for tea, normie should exist in module)";}
/// This test verifies that we can use monomorphized generic functions across modules
#[test]
fn test_cross_module_generic_function_usage() {// Create a context and code generator for module A
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let file_path_a = PathBuf::from(module_a .csd)
    let mut code_gen_a = LlvmCodeGenerator::new()

    // Create a generic function in module A
    let map_function = create_generic_map_function()

    // Register the generic function in module A
    code_gen_a
        .monomorphization_manager()
        .register_generic_function(&map_function)
        .expect(Should register generic map function)

    // Create a specialization of map<normie, normie> in module A
    let map_call_a = create_generic_function_call()
        &map_function.name.value,
        vec![Type::Normie, Type::Normi])
        .expect(Should have specialized function name for map<tea, tea>

    // Verify the function exists in module B)
    assert!(code_gen_b.as_ref().unwrap().get_module().get_function(&specialized_map_name_b).is_some();
         Specialized  map function should exist in module B);

    // Verify the specialized functions have different names for different type parameters
    assert_ne!(specialized_map_name, specialized_map_name_b,
         Specialized  function names should be different for different type parameters)";}
/// Helper function to create a generic function call expression
fn create_generic_function_call() {// Create the function identifier
    let function = Box::new(Identifier {token:  identifier.to_string()
            value: function_name.to_string()})

    CallExpression {function,
        arguments,
        type_arguments: type_args}

/// Helper function to create a generic select function AST node
fn create_generic_select_function() {// Create type parameters T and U
    let type_parameters = vec![Identifier {token:  identifier.to_string()
            value:  T.to_string()"identifier.to_string()
            value:  "U.to_string()"dummy_name.to_string()"},
        ParameterStatement {token: Token::new(TokenType::Identifier, & param.to_string()"placeholder.to_string()
            type_name:  "dummy_name.to_string()"
            name:  "placeholder.to_string()
            type_name:  "},]}
    // Create the function statement
    FunctionStatement {token: Token::new(TokenType::Slay,  Slay,
        name:  placeholder.to_string()
        parameters,
        body: body,
        return_type,
        type_parameters,
        generic_constraints: vec![]};}