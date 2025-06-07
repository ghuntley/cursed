use cursed::ast::expressions::identifiers::Identifier;
use cursed::ast::expressions::literals::{IntegerLiteral, FloatLiteral};
use cursed::ast::expressions::struct_expr::{StructLiteral, KeyValuePair};
use cursed::ast::traits::Expression;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::struct_field_inference::StructFieldInference;
use cursed::lexer::Token;
use inkwell::context::Context;
use std::path::PathBuf;

// Simplified test for type inference in struct field initialization


#[test]
fn test_struct_field_incompatible_types() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_struct_field_incompatible", PathBuf::from("test.csd"));

    // Create a function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_struct_field_incompatible", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Register a Person struct type
    let person_name = "Person";
    let string_ptr = generator.context().i8_type().ptr_type(Default::default(); // String pointer
    let person_ty = generator.context().struct_type(&[
        string_ptr.into(), // name: string
        generator.context().i32_type().into(), // age: i32
    ], false);
    generator.register_struct_type(person_name, person_ty).unwrap();
    
    // Create a struct literal with incompatible field type
    let struct_literal = StructLiteral {
        token: new_token(TokenType::LBrace, "{"),
        struct_name: person_name.to_string(),
        fields: vec![
            KeyValuePair {
                key: Identifier {
                    token: new_token(TokenType::Identifier, "name"),
                    value: "name".to_string(),
                },
                value: Box::new(FloatLiteral { // Float assigned to string field - should fail
                    token: new_token(TokenType::Float, "42.5"),
                    value: 42.5,
                }),
            },
            KeyValuePair {
                key: Identifier {
                    token: new_token(TokenType::Identifier, "age"),
                    value: "age".to_string(),
                },
                value: Box::new(IntegerLiteral {
                    token: new_token(TokenType::Int, "42"),
                    value: 42,
                }),
            },
        ],
    };
    
    // Compile the struct literal - should fail with type error
    let result = generator.compile_struct_literal(&struct_literal);
    assert!(result.is_err(), "Should fail due to incompatible field type");
    
    // Check error message
    if let Err(err) = result {
        println!("Got expected error: {}", err);
        assert!(err.to_string().contains("mismatch"), 
                "Error should mention type mismatch: {}", err);
    }
    
    // Return a dummy value to finalize function
    let ret_val = generator.builder().build_return(Some(&context.i32_type().const_int(0, false));
    assert!(ret_val.is_ok(), "Failed to build return: {:?}", ret_val.err();
    
    // Verify the module
    let verification = generator.module().verify());
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err();
}

// Helper function to create tokens correctly
fn new_token(token_type: TokenType, literal: &str) -> Token {
    match token_type {
        TokenType::Identifier => Token::Identifier(literal.to_string(),
        TokenType::Int => {
            if let Ok(value) = literal.parse::<i64>() {
                Token::Int(value)
            } else {
                Token::Illegal(format!("Invalid integer: {}", literal))
            }
        },
        TokenType::Float => {
            if let Ok(value) = literal.parse::<f64>() {
                Token::Float(value)
            } else {
                Token::Illegal(format!("Invalid float: {}", literal))
            }
        },
        TokenType::String => Token::String(literal.to_string(),
        TokenType::LBrace => Token::LBrace,
        TokenType::RBrace => Token::RBrace,
        TokenType::Sus => Token::Sus,
        _ => Token::Illegal(format!("Unsupported token type: {:?}", token_type)),
    }
}

#[test]
fn test_simple_struct_field_type_inference() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_simple_struct", PathBuf::from("test.csd"));

    // Create a function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_simple_struct", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // First, register a struct type with the code generator
    let struct_name = "Point";
    let struct_ty = generator.context().struct_type(&[
        generator.context().f64_type().into(), // x: f64
        generator.context().f64_type().into(), // y: f64
    ], false);
    
    // Register the struct with the code generator's type system
    generator.register_struct_type(struct_name, struct_ty).unwrap();
    
    // Create a struct literal with fields that need type inference
    let struct_literal = StructLiteral {
        token: new_token(TokenType::LBrace, "{"),
        struct_name: struct_name.to_string(),
        fields: vec![
            KeyValuePair {
                key: Identifier {
                    token: new_token(TokenType::Identifier, "x"),
                    value: "x".to_string(),
                },
                value: Box::new(IntegerLiteral { // Note: integer assigned to float field
                    token: new_token(TokenType::Int, "10"),
                    value: 10,
                }),
            },
            KeyValuePair {
                key: Identifier {
                    token: new_token(TokenType::Identifier, "y"),
                    value: "y".to_string(),
                },
                value: Box::new(FloatLiteral {
                    token: new_token(TokenType::Float, "20.5"),
                    value: 20.5,
                }),
            },
        ],
    };
    
    // Compile the struct literal
    let result = generator.compile_struct_literal(&struct_literal);
    assert!(result.is_ok(), "Failed to compile struct literal with type inference: {:?}", result.err())
    
    // Return a dummy value to finalize function
    let ret_val = generator.builder().build_return(Some(&context.i32_type().const_int(0, false));
    assert!(ret_val.is_ok(), "Failed to build return: {:?}", ret_val.err();
    
    // Verify the module
    let verification = generator.module().verify());
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err();
}