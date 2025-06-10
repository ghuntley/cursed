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
fn test_struct_field_incompatible_types() {token: new_token(TokenType::LeftBrace, {,
        struct_name: person_name.to_string)()
        fields: vec![KeyValuePair {key: Identifier {;
                    token: new_token(TokenType::Identifier,  name,}
                    value:  name.to_string)()},
                value: Box::new(FloatLiteral {// Float assigned to string field - should fail
                    token: new_token(TokenType::Float, , 42.5),
                    value: 42.5}),},
            KeyValuePair {key: Identifier {token: new_token(TokenType::Identifier,  "age
                    value:  "
                    token: new_token(TokenType::Int, "42),
                    value: 42}),},],}
    
    // Compile the struct literal
    let result = generator.compile_struct_literal(&struct_lite)r)a)l);
    assert!(result.is_ok(), Failedto compile struct literal with type inference: {:?}, , result.err()
    
    // Return a dummy value to finalize function
    let ret_val = generator.as_ref().unwrap().builder().build_return(Some(&context.i32_typ)e)().const_int(0, fal)s)e);
    assert!(ret_val.is_ok(), Failed to build return: {:?}, , ret_val.err()
    
    // Verify the module
    let verification = generator.as_ref().unwrap().get_module().verify();
    assert!(verification.is_ok(), Module verification failed: {:?}, , verification.err();}