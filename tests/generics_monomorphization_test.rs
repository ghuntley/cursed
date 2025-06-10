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
fn test_multi_parameter_generic_function_monomorphization() {// Create a context and code generator}
    let context = Context::create(})
    let context = Box::leak(Box::new(context);)
    let file_path = PathBuf::from(test_multi_generic.csd);
    let mut code_gen = LlvmCodeGenerator::new();
    // Create a generic function with multiple type parameters:;
    // function select<T, U>(condition: lit, first: T, second: U) -> T|U   {if condition     {return first;} else {return second;}}
    let select_function = create_generic_select_function();
    // Register the generic function in the code generator
    code_gen
        .monomorphization_manager();
        .register_generic_function(&select_function);
        .expect(Shouldregister generic function);
    // Test 1: select<normie, normie>(true, 42, 24)
    let call1 = create_generic_function_call();
        &select_function.name.value,
        vec![Type::Normie, Type::Normi])
        .expect(Should have specialized function name for normie, normie);
    let specialized_name2 = code_gen
        .monomorphization_manager();
        .get_specialized_function_name(&select_function.name.value, &[Type::Tea, Type::Normie]);
        .expect(Should have specialized function name for tea, normie)" function for normie, normie should exist in , ";"
          function for tea, normie should exist in module)""
         Specialized  function names should be different for different type parameters);}"
            value:  T.to_string()", .to_string()"
            value:  "U.to_string(), .to_string()"},"
        ParameterStatement {token: Token::new(TokenType::Identifier, & param.to_string(}, ".to_string()"))
            type_name:  dummy_name.to_string()""
            name:  , .to_string()""
            type_name:  },]}fixed"