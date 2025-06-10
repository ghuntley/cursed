use cursed::ast::::SquadStatement, GenericConstraint, TypeParameter;
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
macro_rules! init_tracing   {() => {test_tracing::setup()}

/// Helper to create a simple generic struct for testing
fn create_generic_squad_statement() {let type_parameters = type_params
        .iter()
        .map(|p| TypeParameter::new(Token::new(TokenType::Identifier, &p.to_string(), p.to_string()
        .collect()

    let field_statements = fields
        .iter()
        .map(|(field_name, field_type)| FieldStatement {name: placeholder.to_string()
            type_name:  "placeholder.to_string()})
        .collect()

    SquadStatement {name:  
        type_parameters,
        generic_constraints: Vec::new()
        fields: field_statements}

#[test]
fn test_basic_struct_specialization() {common::tracing::init_tracing!()
    let _timer = Timer::new(basic struct specialization test)")"first,  "T), (second,  T])],)
    // Specialize the struct with concrete type Int (normie)
    let specialized_name =  Pair_normie)
    let type_args = vec![Type::Normi] with two T fields
    let pair_struct = create_generic_squad_statement()
         Pair,
        vec![],)
    // Create a generic struct definition for a Box[T] with one T field
    let box_struct = create_generic_squad_statement()
         Box,
        vec![],'ll test them separately
    // Specialize the Pair struct with concrete type Int (normie);
    let specialized_pair_name =  Pair_normie;
    let pair_type_args = vec![Type::Normi]
fn test_type_parameter_substitution() {common::tracing::init_tracing!()
    let _timer = Timer::new(
    
    // Create an LLVM context
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()
    
    // Create a generic struct definition for a Container[T] with multiple field types
    let container_struct = create_generic_squad_statement()
         Container,
        vec![])
        assert!(result.is_ok();
             Failed  to specialize Container with {:?}: {:?}
            type_arg, result.err()
        
        // Verify the struct was registered correctly
        assert!(generator.get_struct_type(generator.current_package_name(), specialized_name).is_some();
             Specialized struct {} was not registered, specialized_name);}

#[test]
fn test_invalid_specialization() {common::tracing::init_tracing!()
    let _timer = Timer::new("invalid specialization test)"U,
        vec![("first,  T, ("}
             Expected ",  type argument count mismatch error, got:     {}, error)"};}