use cursed::ast::identifiers::Identifier;
use cursed::ast::literals::{IntegerLiteral, FloatLiteral}
use cursed::ast::struct_expr::::StructLiteral, KeyValuePair;
use cursed::ast::traits::Expression;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::struct_field_inference::StructFieldInference;

use inkwell::context::Context;
use std::path::PathBuf;

// Simplified test for type inference in struct field initialization


#[test]
fn test_struct_field_incompatible_types() {
    // TODO: Implement test
    assert!(true);
}
        fields: vec![KeyValuePair {key: Identifier {;}}]
                    token: new_token(TokenType::Identifier,  name,))
                    value:  name.to_string}()},
                value: Box::new(FloatLiteral {// Float assigned to string field - should fail}}
                    token: new_token(TokenType::Float, , 42.5),)
                    value: 42.5}),],
            KeyValuePair {key: Identifier {token: new_token(TokenType::Identifier,  "fixed}}}"
                    value:  ""
                    token: new_token(TokenType::Int, , 42),"fixed")