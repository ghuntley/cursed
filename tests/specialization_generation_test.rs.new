//! Tests for the specialization generation of generic functions

use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::MonomorphizationManager;
use cursed::core::type_checker::Type;
use cursed::core::generic_instantiation::GenericInstantiator;
use cursed::ast::expressions::Identifier;
use cursed::ast::expressions::InfixExpression;
use cursed::ast::FunctionStatement;
use cursed::ast::ParameterStatement;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::ReturnStatement;
use cursed::ast::statements::FieldStatement;
use cursed::ast::traits::Expression;
use cursed::lexer::token::Token;

/// Test that specialization generation works correctly
#[test]
fn test_function_specialization() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_specialization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_specialization_module", file_path);
    
    // Create a simple generic function: func add<T>(a: T, b: T) -> T { return a + b; }
    let add_function = create_generic_add_function();
    
    // Create a MonomorphizationManager directly to avoid borrow issues
    let mut mono_manager = MonomorphizationManager::new();
    
    // Specialize the function for the Normie (i32) type
    let specialized_name = mono_manager.specialize_function(
        &mut code_gen,
        &add_function,
        &[Type::Normie],
    ).expect("Specialization should succeed");
    
    // Verify the specialized function name format
    assert_eq!(specialized_name, "add__Normie");
    
    // Verify the function is in the instantiation cache
    assert!(mono_manager.is_function_instantiated("add", &[Type::Normie]));
    
    // Try specializing with a different type
    let specialized_name2 = mono_manager.specialize_function(
        &mut code_gen,
        &add_function,
        &[Type::Thicc],  // i64
    ).expect("Second specialization should succeed");
    
    // Verify the specialized function name for the second specialization
    assert_eq!(specialized_name2, "add__Thicc");
    
    // Verify we have two distinct specializations
    assert_ne!(specialized_name, specialized_name2);
    
    // Verify both functions are in the LLVM module
    let module = code_gen.module();
    let function1 = module.get_function(&specialized_name);
    let function2 = module.get_function(&specialized_name2);
    
    assert!(function1.is_some(), "First specialized function should exist in module");
    assert!(function2.is_some(), "Second specialized function should exist in module");
}

/// Test struct specialization
#[test]
fn test_struct_specialization() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_struct_specialization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_struct_specialization_module", file_path);
    
    // Create a generic pair struct
    let pair_struct = create_generic_pair_struct();
    
    // Create a MonomorphizationManager directly to avoid borrow issues
    let mut mono_manager = MonomorphizationManager::new();
    
    // Specialize with Normie (i32)
    let specialized_name = mono_manager.specialize_struct(
        &mut code_gen,
        &pair_struct,
        &[Type::Normie],
    ).expect("Struct specialization should succeed");
    
    // Verify the specialized name
    assert_eq!(specialized_name, "Pair__Normie");
    
    // Verify it's in the cache
    assert!(mono_manager.is_function_instantiated("Pair", &[Type::Normie]));
    
    // Specializing again should return the same name
    let specialized_name2 = mono_manager.specialize_struct(
        &mut code_gen,
        &pair_struct,
        &[Type::Normie],
    ).expect("Second struct specialization should succeed");
    
    assert_eq!(specialized_name, specialized_name2, "Specializing with same types should return the same name");
}

/// Helper function to create a generic add function AST node
fn create_generic_add_function() -> FunctionStatement {
    // Create type parameter T
    let type_parameters = vec![Identifier {
        token: "IDENT".to_string(),
        value: "T".to_string(),
    }];
    
    // Create parameters a: T, b: T
    let parameters = vec![
        ParameterStatement {
            token: "IDENT".to_string(),
            name: Identifier {
                token: "IDENT".to_string(),
                value: "a".to_string(),
            },
            type_name: Box::new(Identifier {
                token: "IDENT".to_string(),
                value: "T".to_string(),
            }),
        },
        ParameterStatement {
            token: "IDENT".to_string(),
            name: Identifier {
                token: "IDENT".to_string(),
                value: "b".to_string(),
            },
            type_name: Box::new(Identifier {
                token: "IDENT".to_string(),
                value: "T".to_string(),
            }),
        },
    ];
    
    // Create return type T
    let return_type = Some(Box::new(Identifier {
        token: "IDENT".to_string(),
        value: "T".to_string(),
    }) as Box<dyn Expression>);
    
    // Create an infix expression for a + b
    let infix_expr = InfixExpression {
        token: Token::Plus,
        left: Box::new(Identifier {
            token: "IDENT".to_string(),
            value: "a".to_string(),
        }),
        operator: "+".to_string(),
        right: Box::new(Identifier {
            token: "IDENT".to_string(),
            value: "b".to_string(),
        }),
    };
    
    // Create body: { return a + b; }
    let return_statement = ReturnStatement {
        token: "return".to_string(),
        return_value: Some(Box::new(infix_expr)),
    };
    
    let body = BlockStatement {
        token: "{".to_string(),
        statements: vec![Box::new(return_statement)],
    };
    
    // Create the function statement
    FunctionStatement {
        token: "function".to_string(),
        name: Identifier {
            token: "IDENT".to_string(),
            value: "add".to_string(),
        },
        parameters,
        body,
        return_type,
        type_parameters,
    }
}

/// Helper function to create a generic pair struct AST node
fn create_generic_pair_struct() -> cursed::ast::SquadStatement {
    // Create type parameter T
    let type_parameters = vec![Identifier {
        token: "IDENT".to_string(),
        value: "T".to_string(),
    }];
    
    // Create fields first: T, second: T
    let fields = vec![
        FieldStatement {
            token: "IDENT".to_string(),
            name: Identifier {
                token: "IDENT".to_string(),
                value: "first".to_string(),
            },
            type_name: Identifier {
                token: "IDENT".to_string(),
                value: "T".to_string(),
            },
        },
        FieldStatement {
            token: "IDENT".to_string(),
            name: Identifier {
                token: "IDENT".to_string(),
                value: "second".to_string(),
            },
            type_name: Identifier {
                token: "IDENT".to_string(),
                value: "T".to_string(),
            },
        },
    ];
    
    // Create the struct statement
    cursed::ast::SquadStatement {
        token: "squad".to_string(),
        name: Identifier {
            token: "IDENT".to_string(),
            value: "Pair".to_string(),
        },
        type_parameters,
        fields,
    }
}