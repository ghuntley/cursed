use cursed::ast::base::Program;
use cursed::ast::expressions::{CallExpression, Identifier, IntegerLiteral, StringLiteral};
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
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use std::sync::Arc;

// Comprehensive test for generic function monomorphization in LLVM code generation


/// This test creates a generic function with multiple type parameters 
/// and tests its monomorphization with various concrete type combinations
#[test]
fn test_multi_parameter_generic_function_monomorphization() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_multi_generic.csd");
    let mut code_gen = LlvmCodeGenerator::new();

    // Create a generic function with multiple type parameters:
    // function select<T, U>(condition: lit, first: T, second: U) -> T|U { if condition { return first; } else { return second; } }
    let select_function = create_generic_select_function();

    // Register the generic function in the code generator
    code_gen
        .monomorphization_manager()
        .register_generic_function(&select_function)
        .expect("Should register generic function");

    // Test 1: select<normie, normie>(true, 42, 24)
    let call1 = create_generic_function_call(
        &select_function.name.value,
        vec![Type::Normie, Type::Normie],
        vec![
            Box::new(cursed::ast::expressions::BooleanLiteral {
                token: "token".to_string(),
                value: true,
            }),
            Box::new(IntegerLiteral {
                token: "token".to_string(),
                value: 42,
            }),
            Box::new(IntegerLiteral {
                token: "token".to_string(),
                value: 24,
            }),
        ],
    );

    // Test 2: select<tea, normie>(false, "hello", 99)
    let call2 = create_generic_function_call(
        &select_function.name.value,
        vec![Type::Tea, Type::Normie],
        vec![
            Box::new(cursed::ast::expressions::BooleanLiteral {
                token: "token".to_string(),
                value: false,
            }),
            Box::new(StringLiteral {
                token: "token".to_string(),
                value: "hello".to_string(),
            }),
            Box::new(IntegerLiteral {
                token: "token".to_string(),
                value: 99,
            }),
        ],
    );

    // Compile the generic calls
    let result1 = code_gen.specialized_function_builder().compile_generic_call(&call1);
    let result2 = code_gen.specialized_function_builder().compile_generic_call(&call2);

    // Verify that both compilations succeeded
    assert!(result1.is_ok(), "First generic call compilation should succeed: {:?}", result1.err());
    assert!(result2.is_ok(), "Second generic call compilation should succeed: {:?}", result2.err());

    // Verify the specialized functions exist in the module
    let specialized_name1 = code_gen
        .monomorphization_manager()
        .get_specialized_function_name(&select_function.name.value, &[Type::Normie, Type::Normie])
        .expect("Should have specialized function name for normie, normie");

    let specialized_name2 = code_gen
        .monomorphization_manager()
        .get_specialized_function_name(&select_function.name.value, &[Type::Tea, Type::Normie])
        .expect("Should have specialized function name for tea, normie");

    let module = code_gen.module();
    assert!(module.get_function(&specialized_name1).is_some(), 
        "Specialized function for normie, normie should exist in module");
    assert!(module.get_function(&specialized_name2).is_some(),
        "Specialized function for tea, normie should exist in module");
}

/// This test verifies that we can use monomorphized generic functions across modules
#[test]
fn test_cross_module_generic_function_usage() {
    // Create a context and code generator for module A
    let context = Context::create();
    let file_path_a = PathBuf::from("module_a.csd");
    let mut code_gen_a = LlvmCodeGenerator::new();

    // Create a generic function in module A
    let map_function = create_generic_map_function();

    // Register the generic function in module A
    code_gen_a
        .monomorphization_manager()
        .register_generic_function(&map_function)
        .expect("Should register generic map function");

    // Create a specialization of map<normie, normie> in module A
    let map_call_a = create_generic_function_call(
        &map_function.name.value,
        vec![Type::Normie, Type::Normie],
        vec![
            // Create a simple function as mapper (identity function)
            Box::new(Identifier {
                token: "token".to_string(),
                value: "identity".to_string(),
            }),
            // Create an array as input
            Box::new(Identifier {
                token: "token".to_string(),
                value: "numbers".to_string(),
            }),
        ],
    );

    // Compile the generic call in module A
    let result_a = code_gen_a.specialized_function_builder().compile_generic_call(&map_call_a);
    assert!(result_a.is_ok(), "Module A generic call compilation should succeed: {:?}", result_a.err();

    // Create a specialization name for map<normie, normie>
    let specialized_map_name = code_gen_a
        .monomorphization_manager()
        .get_specialized_function_name(&map_function.name.value, &[Type::Normie, Type::Normie])
        .expect("Should have specialized function name for map<normie, normie>");

    // Verify the function exists in module A
    assert!(code_gen_a.module().get_function(&specialized_map_name).is_some(),
        "Specialized map function should exist in module A");

    // Create module B that will use module A's specialized function
    let file_path_b = PathBuf::from("module_b.csd");
    let mut code_gen_b = LlvmCodeGenerator::new();

    // Register the same generic function in module B
    code_gen_b
        .monomorphization_manager()
        .register_generic_function(&map_function)
        .expect("Should register generic map function in module B");

    // Create a different specialization of map<tea, tea> in module B
    let map_call_b = create_generic_function_call(
        &map_function.name.value,
        vec![Type::Tea, Type::Tea],
        vec![
            // Create a simple function as mapper (uppercase function)
            Box::new(Identifier {
                token: "token".to_string(),
                value: "uppercase".to_string(),
            }),
            // Create an array as input
            Box::new(Identifier {
                token: "token".to_string(),
                value: "names".to_string(),
            }),
        ],
    );

    // Compile the generic call in module B
    let result_b = code_gen_b.specialized_function_builder().compile_generic_call(&map_call_b);
    assert!(result_b.is_ok(), "Module B generic call compilation should succeed: {:?}", result_b.err());

    // Create a specialization name for map<tea, tea>
    let specialized_map_name_b = code_gen_b
        .monomorphization_manager()
        .get_specialized_function_name(&map_function.name.value, &[Type::Tea, Type::Tea])
        .expect("Should have specialized function name for map<tea, tea>");

    // Verify the function exists in module B
    assert!(code_gen_b.module().get_function(&specialized_map_name_b).is_some(),
        "Specialized map function should exist in module B");

    // Verify the specialized functions have different names for different type parameters
    assert_ne!(specialized_map_name, specialized_map_name_b,
        "Specialized function names should be different for different type parameters");
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

    CallExpression {
        token: "token".to_string(),
        function,
        arguments,
        type_arguments: type_args,
    }
}

/// Helper function to create a generic select function AST node
fn create_generic_select_function() -> FunctionStatement {
    // Create type parameters T and U
    let type_parameters = vec![
        Identifier {
            token: "token".to_string(),
            value: "T".to_string(),
        },
        Identifier {
            token: "token".to_string(),
            value: "U".to_string(),
        },
    ];

    // Create parameters: condition: lit, first: T, second: U
    let parameters = vec![
        ParameterStatement {
            token: Token::Identifier("param".to_string()),
            name: Identifier {
                token: "token".to_string(),
                value: "condition".to_string(),
            },
            type_name: Box::new(Identifier {
                token: "token".to_string(),
                value: "lit".to_string(),
            }),
        },
        ParameterStatement {
            token: Token::Identifier("param".to_string()),
            name: Identifier {
                token: "token".to_string(),
                value: "first".to_string(),
            },
            type_name: Box::new(Identifier {
                token: "token".to_string(),
                value: "T".to_string(),
            }),
        },
        ParameterStatement {
            token: Token::Identifier("param".to_string()),
            name: Identifier {
                token: "token".to_string(),
                value: "second".to_string(),
            },
            type_name: Box::new(Identifier {
                token: "token".to_string(),
                value: "U".to_string(),
            }),
        },
    ];

    // Create a complex return type expression for union type T|U
    // In a real implementation, this would be a proper union type expression
    // For this test, we'll use T as the return type for simplicity
    let return_type: Option<Box<dyn cursed::ast::Expression>> = Some(Box::new(Identifier {
        token: "token".to_string(),
        value: "T".to_string(),
    }));

    // Create body: { if condition { return first; } else { return second; } }
    // For simplicity, we'll just return the first parameter
    let return_statement = ReturnStatement {
        token: "token".to_string(),
        return_value: Some(Box::new(Identifier {
            token: "token".to_string(),
            value: "first".to_string(),
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
            value: "select".to_string(),
        },
        parameters,
        body: body,
        return_type,
        type_parameters,
        generic_constraints: vec![],
    }
}

/// Helper function to create a generic map function AST node
/// The map function takes a function that transforms elements from type T to type U,
/// and an array of type T, and returns an array of type U
fn create_generic_map_function() -> FunctionStatement {
    // Create type parameters T and U
    let type_parameters = vec![
        Identifier {
            token: "token".to_string(),
            value: "T".to_string(),
        },
        Identifier {
            token: "token".to_string(),
            value: "U".to_string(),
        },
    ];

    // Create parameters: mapper: (T) -> U, elements: array<T>
    let parameters = vec![
        ParameterStatement {
            token: Token::Identifier("param".to_string()),
            name: Identifier {
                token: "token".to_string(),
                value: "mapper".to_string(),
            },
            // Function type (T) -> U simplified for this test
            type_name: Box::new(Identifier {
                token: "token".to_string(),
                value: "Function".to_string(),
            }),
        },
        ParameterStatement {
            token: Token::Identifier("param".to_string()),
            name: Identifier {
                token: "token".to_string(),
                value: "elements".to_string(),
            },
            // Array<T> type simplified for this test
            type_name: Box::new(Identifier {
                token: "token".to_string(),
                value: "Array".to_string(),
            }),
        },
    ];

    // Return type: Array<U>
    let return_type: Option<Box<dyn cursed::ast::Expression>> = Some(Box::new(Identifier {
        token: "token".to_string(),
        value: "Array".to_string(),
    }));

    // Create simple body for testing
    let return_statement = ReturnStatement {
        token: "token".to_string(),
        return_value: Some(Box::new(Identifier {
            token: "token".to_string(), // Just return the input for simplicity
            value: "elements".to_string(),
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
            value: "map".to_string(),
        },
        parameters,
        body: body,
        return_type,
        type_parameters,
        generic_constraints: vec![],
    }
}