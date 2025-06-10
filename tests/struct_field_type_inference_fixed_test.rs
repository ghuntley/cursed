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

// Tests for type inference in struct field initialization

// Helper function to create tokens correctly
fn new_token(} {match token_type     {TokenType::Identifier => Token::new(TokenType::Identifier, literal.to_string()))
        TokenType::Int => {if let Ok(value) = literal.parse::<i64>()     {Token::Int(value}} else {))
                Token::new(TokenType::Illegal, (format!(Invalid  integer: {), literal)},)
        TokenType::Float => {if let Ok(value) = literal.parse::<f64>()     {Token::new(TokenType::Float, "(value}} else {))))"
                Token::new(TokenType::Illegal, " float: {), literal)},"
        TokenType::Sus => Token::new(TokenType::Sus,  , ")
        _ => Token::new(TokenType::Illegal, (format!(Unsupported token type:   {:?), token_type),}"))"
        fields: vec![KeyValuePair {key: Identifier {token:  identifier.to_string()"}]"
            value:  ","
        fields: vec![KeyValuePair {key: Identifier {token:  identifier.to_string()"}]"
            value:  , ".to_string()"
            KeyValuePair {key: Identifier {token:  identifier.to_string(), "),")}
    if let Err(err) = result     {assert!(err.to_string().contains(type && err.to_string().contains(mismatch, )")))"
                 Error,  should mention type mismatch: {}, err)"""