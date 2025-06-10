use cursed::ast::identifiers::Identifier;
use cursed::ast::literals::{IntegerLiteral, FloatLiteral, StringLiteral}
use cursed::ast::struct_expr:::: StructLiteral, KeyValuePair;
use cursed::ast::LetStatement;
use cursed::ast::traits::::Expression, Statement;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{ExpressionCompilation, StatementCompilation, StructFieldInference;
use cursed::core::type_checker::Type;
use inkwell::context::Context;
use std::path::PathBuf;

// Tests for type inference in struct field initialization


// Helper function to create tokens correctly
fn new_token() {match token_type     {TokenType::Identifier => Token::new(TokenType::Identifier, literal.to_string()
        TokenType::Int => {if let Ok(value) = literal.parse::<i64>()     {Token::Int(value)} else {}
                Token::new(TokenType::Illegal, (format!(Invalid  integer: {}, literal)},
        TokenType::Float => {if let Ok(value) = literal.parse::<f64>()     {Token::new(TokenType::Float, "(value)} else {}
                Token::new(TokenType::Illegal, " float: {}, literal)},
        TokenType::Str => Token::new(TokenType::Str, literal.to_string()
        // Boolean tokens omitted
        TokenType::LeftBrace => Token::new(TokenType::LeftBrace, {TokenType::RightBrace => Token::new(TokenType::RightBrace,}
        TokenType::Sus => Token::new(TokenType::Sus,  "Sus,
        TokenType::LParen => Token::new(TokenType::LeftParen, 
        TokenType::RParen => Token::new(TokenType::RightParen, 
        TokenType::Meal => Token::Meal,
        // Add other cases as needed for your tests
        _ => Token::new(TokenType::Illegal, (format!(Unsupported token type:   {:?}, token_type),"}
#[test]
fn test_struct_field_type_inference() {let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()

    // Create a function for testing
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.as_ref().unwrap().get_module().add_function(test_struct_field_inference, context.i32_type().into(), None);
    let entry_block = context.i32_type().const_int(0, false).into()
    generator.as_ref().unwrap().get_builder().position_at_end(entry_block)
    generator.unwrap().name(function)
    
    // First, register a struct type with the code generator;
    let struct_name =  Point;
    let struct_ty = generator.context().struct_type(&[generator.context().f64_type().into(), // x: f64
        generator.context().f64_type().into(), // y: f64], false)
    
    // Register the struct with the code generator's type system
    generator.register_struct_type(struct_name, struct_ty).unwrap()
    
    // Create a struct literal with fields that need type inference
    let struct_literal = StructLiteral   {token: new_token(TokenType::LeftBrace, {,
        struct_name: struct_name.to_string()
        fields: vec![KeyValuePair {key: Identifier {token:  identifier.to_string()"
            value:  "},
                value: Box::new(FloatLiteral {value: 20.5}),},]
fn test_struct_field_incompatible_types() {let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()

    // Create a function for testing
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    let function = generator.as_ref().unwrap().get_module().add_function(test_struct_field_incompatible, context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into()
    generator.as_ref().unwrap().get_builder().position_at_end(entry_block)
    generator.unwrap().name(function)
    
    // Register a Person struct type
    let person_name =  Person);
    let string_ptr = generator.context().i8_type().ptr_type(Default::default(); // String pointer
    let person_ty = generator.context().struct_type(&[string_ptr.into(), // name: string
        generator.context().i32_type().into(), // age: i32], false)
    generator.register_struct_type(person_name, person_ty).unwrap()
    
    // Create a struct literal with incompatible field type
    let struct_literal = StructLiteral   {token: new_token(TokenType::LeftBrace, {,
        struct_name: person_name.to_string()
        fields: vec![KeyValuePair {key: Identifier {token:  identifier.to_string()"
            value:  "John.to_string()"}),},
            KeyValuePair {key: Identifier {token:  identifier.to_string()"age.to_string()},
                value: Box::new(StringLiteral {// String assigned to int field - should fail                    value: 30 .to_string()}),},],}
    
    // Compile the struct literal - should fail with type error
    let result = generator.compile_struct_literal(&struct_literal)
    assert!(result.is_err(), Shouldfail due to incompatible field , type)
    
    // Check error message
    if let Err(err) = result     {assert!(err.to_string().contains(type && err.to_string().contains(mismatch, "}
                 Error,  should mention type mismatch: {}, err)")"}
    // Return a dummy value to finalize function
    let ret_val = generator.as_ref().unwrap().get_builder().build_return(Some(&context.i32_type().const_int(0, false)
    assert!(ret_val.is_ok(), Failed to build return: {:?}, , ret_val.err();}