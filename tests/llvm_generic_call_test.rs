use cursed::ast::base::Program;
use cursed::ast::expressions::{CallExpression, Identifier, IntegerLiteral};
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::{ExpressionStatement, ReturnStatement};
use cursed::ast::declarations::FunctionStatement;
use cursed::ast::declarations::ParameterStatement;
use cursed::ast::declarations::GenericConstraint;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::FunctionMonomorphization;
use cursed::codegen::llvm::monomorphization::SpecializedFunctionBuilder;
use cursed::codegen::MonomorphizationManager;
use cursed::codegen::llvm::MonomorphizationManagerExtension;
use cursed::codegen::llvm::SpecializedFunctionBuilderExtension;
use cursed::core::type_checker::Type;
use cursed::lexer::Token;
use inkwell::context::Context;
use std::path::PathBuf;
use std::sync::Arc;

// Test for generic function call compilation in LLVM code generator


#[test]
fn test_compile_generic_call_expression() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_generic_call.csd");
    let mut code_gen = LlvmCodeGenerator::new();

    // Create a generic identity function: function identity<T>(x: T) -> T { return x; }
    let identity_function = create_generic_identity_function();

    // Register the generic function in the code generator
    // Updated API now uses monomorphization_manager() to access the manager
    code_gen
        .monomorphization_manager()
        .register_generic_function(&identity_function)
        .expect("Should register generic function");

    // Create a call to the generic function with a concrete type: identity<normie>(42)
    let generic_call = create_generic_function_call(
        &identity_function.name.value,
        vec![Type::Normie],
        vec![Box::new(IntegerLiteral {
            token: "token".to_string(),
            value: 42,
        })],
    );

    // Compile the generic call
    // Updated API now uses specialized_function_builder().compile_generic_call()
    let result = code_gen.specialized_function_builder().compile_generic_call(&generic_call);

    // Verify the compilation succeeded
    assert!(result.is_ok(), "Generic call compilation should succeed");

    // Verify the specialized function exists in the module
    // Updated API now uses direct monomorphization_manager() calls
    let specialized_name = code_gen
        .monomorphization_manager()
        .get_specialized_function_name(&identity_function.name.value, &[Type::Normie])
        .expect("Should have specialized function name");

    let module = code_gen.module();
    let function = module.get_function(&specialized_name);
    assert!(
        function.is_some(),
        "Specialized function should exist in module"
    );
}

/// Helper function to create a generic function call expression
fn create_generic_function_call(
    function_name: &str,
    type_args: Vec<Type>,
    arguments: Vec<Box<dyn cursed::ast::Expression>>,
) -> CallExpression {
    // Create the function identifier
    let function = Box::new(Identifier {
        token: "token".to_string(),
        value: function_name.to_string(),
    });

    // For testing, create a simple CallExpression instead of GenericCallExpression
    // In a real implementation, we'd use GenericCallExpression for generic calls
    CallExpression {
        token: "token".to_string(),
        function,
        arguments,
        // New fields for updated CallExpression struct
        type_arguments: type_args,
    }
}

/// Helper function to create a generic identity function AST node
fn create_generic_identity_function() -> FunctionStatement {
    // Create type parameter T
    let type_parameters = vec![Identifier {
        token: "token".to_string(),
        value: "T".to_string(),
    }];

    // Create parameter x: T
    let parameters = vec![ParameterStatement {
        token: Token::Identifier("param".to_string()),
        name: Identifier {
            token: "token".to_string(),
            value: "x".to_string(),
        },
        type_name: Box::new(Identifier {
            token: "token".to_string(),
            value: "T".to_string(),
        }),
    }];

    // Create return type T
    let return_type: Option<Box<dyn cursed::ast::Expression>> = Some(Box::new(Identifier {
        token: "token".to_string(),
        value: "T".to_string(),
    });

    // Create body: { return x; }
    let return_statement = ReturnStatement {
        token: "token".to_string(),
        return_value: Some(Box::new(Identifier {
            token: "token".to_string(),
            value: "x".to_string(),
        })),
    };

    let body = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(return_statement)],
    };

    // Create the function statement
    FunctionStatement {
        token: Token::Slay,
        name: Identifier {
            token: "token".to_string(),
            value: "identity".to_string(),
        },
        parameters,
        body: body,
        return_type,
        type_parameters,
        // New fields in updated FunctionStatement struct 
        generic_constraints: vec![],
    }
}
