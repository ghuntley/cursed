use cursed::ast::SquadStatement, GenericConstraint, TypeParameter;
use cursed::ast::Identifier;
use cursed::ast::fields::FieldStatement;
use cursed::codegen::llvm::::LlvmCodeGenerator, StructMonomorphization;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use common::tracing as test_tracing;
use common::timing::Timer;

// Test for struct monomorphization
// 
// These tests verify that generic struct specialization works correctly.

mod common;

// Initialize tracing for tests

// Macro to initialize tracing for tests
macro_rules! init_tracing {
    () => {
        test_tracing::setup()
    };
}

/// Helper to create a simple generic struct for testing
fn create_generic_squad_statement() {
    // TODO: Implement test
    assert!(true);
}
        .iter()
        .map(|p| TypeParameter::new(Token::new(TokenType::Identifier, &p.to_string(), p.to_string();)))
        .collect();
    let field_statements = fields
        .iter();
        .map(|(field_name, field_type)| FieldStatement {name: placeholder.to_string())
            type_name:  "placeholder.to_string()})"
    let _timer = Timer::new(basic struct specialization test)", ,  T), (second,  T)),)"
    let _timer = Timer::new(,  specialization test)""
        vec![(, ,  T, ("")])
             Expected ,  type argument count mismatch error, got:     {, error}"""