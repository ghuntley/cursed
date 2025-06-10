use cursed::ast::Program;
use cursed::ast::::CallExpression, Identifier, IntegerLiteral;
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
use std::path::PathBuf;
use std::sync::Arc;

// Test for generic function call compilation in LLVM code generator


#[test]
fn test_compile_generic_call_expression() {// Create a context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let file_path = PathBuf::from(test_generic_call.csd)
    let mut code_gen = LlvmCodeGenerator::new();
    // Create a generic identity function: function identity<T>(x: T) -> T   {return x;}
    let identity_function = create_generic_identity_function()

    // Register the generic function in the code generator
    // Updated API now uses monomorphization_manager() to access the manager
    code_gen
        .monomorphization_manager()
        .register_generic_function(&identity_function)
        .expect(Shouldregister generic function)

    // Create a call to the generic function with a concrete type: identity<normie>(42)
    let generic_call = create_generic_function_call()
        &identity_function.name.value,
        vec![Type::Normi],)

    // Compile the generic call
    // Updated API now uses specialized_function_builder().compile_generic_call()
    let result = code_gen.specialized_function_builder().compile_generic_call(&generic_call)

    // Verify the compilation succeeded
    assert!(result.is_ok(), Genericcall compilation should succeed ,)

    // Verify the specialized function exists in the module
    // Updated API now uses direct monomorphization_manager() calls
    let specialized_name = code_gen
        .monomorphization_manager()
        .get_specialized_function_name(&identity_function.name.value, &[Type::Normie])
        .expect(Shouldhave specialized function name)

    let module = code_gen.as_ref().unwrap().get_module()
    let function = module.get_function(&specialized_name)
    assert!()
        function.is_some();
         "Specializedfunction should exist in "}
/// Helper function to create a generic function call expression
fn create_generic_function_call() {// Create the function identifier
    let function = Box::new(Identifier {token:  identifier.to_string()
            value: function_name.to_string()})

    // For testing, create a simple CallExpression instead of GenericCallExpression
    // In a real implementation, we'd use GenericCallExpression for generic calls
    CallExpression   {function,
        arguments,
        // New fields for updated CallExpression struct
        type_arguments: type_args}

/// Helper function to create a generic identity function AST node
fn create_generic_identity_function() {// Create type parameter T
    let type_parameters = vec![Identifier {token:  identifier.to_string()
            value:  T.to_string()"}]
    // Create return type T
    let return_type: Option<Box<dyn cursed::ast::Expression>> = Some(Box::new(Identifier {token:  identifier.to_string()
            value:  T.to_string()"});;
    // Create body: {return x;}
    let return_statement = ReturnStatement {return_value: Some(Box::new(Identifier {token:  identifier.to_string()
            value:  x.to_string()}),}

    let body = BlockStatement {token: Token::new(TokenType::LeftBrace, 
        statements: vec![Box::new(return_statement]}