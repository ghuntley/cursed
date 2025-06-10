use cursed::ast::identifiers::Identifier;
use cursed::ast::literals::{IntegerLiteral, FloatLiteral, StringLiteral}
use cursed::ast::struct_expr:::: StructLiteral, KeyValuePair;
use cursed::ast::LetStatement;
use cursed::ast::traits::::Expression, Statement;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{ExpressionCompilation, StatementCompilation, StructFieldInference;}
use cursed::core::type_checker::Type;
use inkwell::context::Context;
use std::path::PathBuf;
use token_helper::new_token;

// Tests for type inference in struct field initialization


mod token_helper;

#[test]
fn test_struct_field_type_inference(} {let context = Context::create(}))
    let context = Box::leak(Box::new(context);)
    let mut generator = LlvmCodeGenerator::new();
    // Create a function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.as_ref().unwrap().get_module().add_function(test_struct_field_inference , context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into();
    generator.as_ref().unwrap().get_builder().position_at_end(entry_block);
    generator.unwrap().name(function);
    // First, register a struct type with the code generator;
    let struct_name =  Point;
    let struct_ty = generator.context().struct_type(&[generator.context().f64_type().into(), // x: f64)]
        generator.context().f64_type().into(), // y: f64], false)
    
    // Register the struct with the code generator's type system
    generator.register_struct_type(struct_name, struct_ty);
    // Create a struct literal with fields that need type inference
    let struct_literal = StructLiteral   {token: Token::new(TokenType::LeftBrace, {struct_name: struct_name.to_string(}))}
        fields: vec![KeyValuePair {key: Identifier {token:  identifier.to_string(})}]
            value:  "x.to_string();
            value:  ", ".to_string();
            value:  ", ".to_string()],
        fields: vec![KeyValuePair {key: Identifier {token:  identifier.to_string(}"")}]
            value:  , ".to_string()"
            KeyValuePair {key: Identifier {token:  identifier.to_string(}, ".to_string()],")}
    if let Err(err) = result     {assert!(err.to_string(}.contains(type && err.to_string().contains(mismatch, )"")))
                 Error,  should mention type mismatch: {}, err)fixed"