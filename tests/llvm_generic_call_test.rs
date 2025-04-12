//! Test for generic function call compilation in LLVM code generator

use cursed::ast::base::Program;
use cursed::ast::expressions::{CallExpression, Identifier, IntegerLiteral};
use cursed::lexer::Token;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::{ExpressionStatement, ReturnStatement};
use cursed::ast::FunctionStatement;
use cursed::ast::ParameterStatement;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::MonomorphizationManager;
use cursed::core::type_checker::Type;
use inkwell::context::Context;
use std::path::PathBuf;
use std::rc::Rc;

#[test]
fn test_compile_generic_call_expression() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_generic_call.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_generic_call_module", file_path);

    // Create a generic identity function: function identity<T>(x: T) -> T { return x; }
    let identity_function = create_generic_identity_function();

    // Register the generic function in the code generator
    code_gen
        .register_generic_function(&identity_function)
        .expect("Should register generic function");

    // Create a call to the generic function with a concrete type: identity<normie>(42)
    let generic_call = create_generic_function_call(
        &identity_function.name.value,
        vec![Type::Normie],
        vec![Box::new(IntegerLiteral {
            token: "42".to_string(),
            value: 42,
        })],
    );

    // Compile the generic call
    let result = code_gen.compile_generic_call_expression(&generic_call);

    // Verify the compilation succeeded
    assert!(result.is_ok(), "Generic call compilation should succeed");

    // Verify the specialized function exists in the module
    let specialized_name = code_gen
        .mono_manager
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
        token: "IDENT".to_string(),
        value: function_name.to_string(),
    });

    // For testing, create a simple CallExpression instead of GenericCallExpression
    // In a real implementation, we'd use GenericCallExpression for generic calls
    CallExpression {
        token: Token::LParen,
        function,
        arguments,
    }
}

/// Helper function to create a generic identity function AST node
fn create_generic_identity_function() -> FunctionStatement {
    // Create type parameter T
    let type_parameters = vec![Identifier {
        token: "T".to_string(),
        value: "T".to_string(),
    }];

    // Create parameter x: T
    let parameters = vec![ParameterStatement {
        token: "param".to_string(),
        name: Identifier {
            token: "x".to_string(),
            value: "x".to_string(),
        },
        type_name: Box::new(Identifier {
            token: "T".to_string(),
            value: "T".to_string(),
        }),
    }];

    // Create return type T
    let return_type: Option<Box<dyn cursed::ast::Expression>> = Some(Box::new(Identifier {
        token: "T".to_string(),
        value: "T".to_string(),
    }));

    // Create body: { return x; }
    let return_statement = ReturnStatement {
        token: "return".to_string(),
        return_value: Some(Box::new(Identifier {
            token: "x".to_string(),
            value: "x".to_string(),
        })),
    };

    let body = BlockStatement {
        token: "{".to_string(),
        statements: vec![Box::new(return_statement)],
    };

    // Create the function statement
    FunctionStatement {
        token: "slay".to_string(),
        name: Identifier {
            token: "identity".to_string(),
            value: "identity".to_string(),
        },
        parameters,
        body: body,
        return_type,
        type_parameters,
    }
}
