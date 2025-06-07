use cursed::ast::declarations::FunctionStatement;
use cursed::ast::expressions::{CallExpression, Identifier, IntegerLiteral, StringLiteral};
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::ReturnStatement;
use cursed::ast::declarations::ParameterStatement;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::FunctionMonomorphization;
use cursed::codegen::MonomorphizationManager;
use cursed::core::type_checker::Type;
use cursed::lexer::Token;
use inkwell::context::Context;
use std::path::PathBuf;

//! Test for proper function specialization implementation


/// Test the specialization of a simple identity function with different types
#[test]
fn test_identity_function_specialization() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("identity_function_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "identity_function_test", file_path);
    
    // Create a generic identity function: slay identity[T](x T) T { yolo x }
    let identity_function = create_generic_identity_function();
    
    // Create generic function calls with different type parameters
    let int_call = create_generic_call(&identity_function.name.value, vec![Type::Normie], vec![Box::new(
        IntegerLiteral {
            token: "token".to_string()),
            value: 42,
        }
    )]);
    
    let string_call = create_generic_call(&identity_function.name.value, vec![Type::Tea], vec![Box::new(
        StringLiteral {
            token: "token".to_string()),
            value: "hello".to_string()),
        }
    )]);
    
    // Compile the generic function calls
    let result1 = code_gen.compile_generic_call_expression(&int_call);
    assert!(result1.is_ok(), "Failed to compile integer identity function: {:?}", result1.err());
    
    let result2 = code_gen.compile_generic_call_expression(&string_call);
    assert!(result2.is_ok(), "Failed to compile string identity function: {:?}", result2.err());
    
    // Verify the module has the specialized functions
    let module = code_gen.module();
    let int_specialized_name = format!("identity__Normie");
    let string_specialized_name = format!("identity__Tea");
    
    assert!(module.get_function(&int_specialized_name).is_some(), 
            "Integer specialized function '{}' not found in module", int_specialized_name);
    assert!(module.get_function(&string_specialized_name).is_some(), 
            "String specialized function '{}' not found in module", string_specialized_name);
    
    // Verify the functions can be executed (can't actually run them in this test,
    // but we can check that they've been properly verified)
    let int_function = module.get_function(&int_specialized_name).unwrap();
    assert!(int_function.verify(true).is_ok(), "Integer function verification failed");
    
    let string_function = module.get_function(&string_specialized_name).unwrap();
    assert!(string_function.verify(true).is_ok(), "String function verification failed");
}

/// Helper function to create a generic identity function
fn create_generic_identity_function() -> FunctionStatement {
    // Create type parameter T
    let type_parameters = vec![Identifier {
        token: "token".to_string()),
        value: "T".to_string()),
    }];
    
    // Create parameter x: T
    let parameters = vec![ParameterStatement {
        token: Token::Identifier("param".to_string()),
        name: Identifier {
            token: "token".to_string()),
            value: "x".to_string()),
        },
        type_name: Box::new(Identifier {
            token: "token".to_string()),
            value: "T".to_string()),
        }),
    }];
    
    // Create return type T
    let return_type = Some(Box::new(Identifier {
        token: "token".to_string()),
        value: "T".to_string()),
    }));
    
    // Create body: { return x; }
    let return_statement = ReturnStatement {
        token: "token".to_string()),
        return_value: Some(Box::new(Identifier {
            token: "token".to_string()),
            value: "x".to_string()),
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
            token: "token".to_string()),
            value: "identity".to_string()),
        },
        params: parameters,
        body,
        return_type,
        type_parameters,
        generic_constraints: vec![],
    }
}

/// Helper function to create a generic function call
fn create_generic_call(
    function_name: &str,
    type_args: Vec<Type>,
    args: Vec<Box<dyn cursed::ast::Expression>>,
) -> CallExpression {
    CallExpression {
        token: "token".to_string()),
        function: Box::new(Identifier {
            token: "token".to_string()),
            value: function_name.to_string()),
        }),
        args,
        type_args,
    }
}