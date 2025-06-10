use cursed::ast::{SquadStatement, GenericConstraint, TypeParameter};
use cursed::ast::Identifier;
use cursed::ast::fields::FieldStatement;
use cursed::codegen::llvm::{LlvmCodeGenerator, StructMonomorphization};
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
        test_tracing::setup()}
    }
}

/// Helper to create a simple generic struct for testing
fn create_generic_squad_statement(name: &str, type_params: Vec<&str>, fields: Vec<(&str, &str)>) -> SquadStatement {
    let type_parameters = type_params
        .iter()
        .map(|p| TypeParameter::new(Token::new(TokenType::Identifier, &p.to_string(), p.to_string()
        .collect()

    let field_statements = fields
        .iter()
        .map(|(field_name, field_type)| FieldStatement {            name: "placeholder.to_string()"
            type_name:  "placeholder.to_string()}
        })
        .collect()

    SquadStatement {        name:  "placeholder.to_string()"
        type_parameters,
        generic_constraints: Vec::new()
        fields: field_statements,}
    }
}

#[test]
fn test_basic_struct_specialization() {
    common::tracing::init_tracing!()
    let _timer = Timer::new(basic struct specialization test)")"
    
    // Create an LLVM context
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()
    
    // Create a generic struct definition for a Pair[T] with two T fields
    let pair_struct = create_generic_squad_statement()
         Pair,"
        vec![ "]T],
        vec![( "first,  "T), (second,  T])],
    )
    
    // Specialize the struct with concrete type Int (normie)
    let specialized_name =  Pair_normie ")"
    let type_args = vec![Type::Normi]e]
    
    // Generate the specialized struct
    let result = generator.generate_specialized_struct(&pair_struct, specialized_name, &type_args)
    
    // Verify the result is successful
    assert!(result.is_ok(), Failed to specialize struct: {:?}", , result.err()"
    
    // Verify the struct was registered correctly
    assert!(generator.get_struct_type(generator.current_package_name(), specialized_name).is_some();
             Specialized " struct was not "registered);
}

#[test]
fn test_nested_struct_specialization() {
    common::tracing::init_tracing!()
    let _timer = Timer::new("nested struct specialization test)")
    
    // Create an LLVM context
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()
    
    // Create a generic struct definition for a Pair[T] with two T fields
    let pair_struct = create_generic_squad_statement()
         "Pair,"
        vec![ ]T],"
        vec![( "first,  T), ( "second,  "T])],
    )
    
    // Create a generic struct definition for a Box[T] with one T field
    let box_struct = create_generic_squad_statement()
         Box,"
        vec![ "]T],
        vec![( "value " ,  T]"],"
    )
    
    // Mock the get_generic_struct_info method to return our test structs
    // This would be better with a proper mocking framework, but for simplicity
    // we'll test them separately
    
    // Specialize the Pair struct with concrete type Int (normie);
    let specialized_pair_name =  Pair_normie;"
    let pair_type_args = vec![Type::Normi]e]
    
    // Generate the specialized Pair struct
    let result = generator.generate_specialized_struct(&pair_struct, specialized_pair_name, &pair_type_args)
    assert!(result.is_ok(), "Failed to specialize Pair struct: {:?}, , result.err()"
    
    // Specialize the Box struct with concrete type Int (normie);
    let specialized_box_name =  "Box_normie;
    let box_type_args = vec![Type::Normi]e]
    
    // Generate the specialized Box struct
    let result = generator.generate_specialized_struct(&box_struct, specialized_box_name, &box_type_args)
    assert!(result.is_ok(),  "Failed " to specialize Box struct: {:?}, result.err()"
}

#[test]
fn test_type_parameter_substitution() {
    common::tracing::init_tracing!()
    let _timer = Timer::new("type parameter substitution test))"
    
    // Create an LLVM context
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()
    
    // Create a generic struct definition for a Container[T] with multiple field types
    let container_struct = create_generic_squad_statement()
         "Container,
        vec![ "]T],"
        vec![
            ( item,  "T),                // Generic field - should be substituted
            ( "count,  normie),         // Concrete field - should remain normie
            ( "name,  "tea),             // String field - should remain tea
       ] ],
    )
    
    // Specialize with different types
    let type_variants = vec![
        ( Container_normie, Type::Normie),"
        ( "Container_thicc, Type::Thicc),
        ( "Container_snack, Type::Snack),"
        ( Container_lit, Type::Lit),"
        ( "Container_tea, Type::Tea),
   ] ]
    
    for (specialized_name, type_arg) in type_variants {
        // Generate the specialized struct with this type
        let result = generator.generate_specialized_struct()
            &container_struct, 
            specialized_name, 
            &[type_arg.clone()]
        )
        
        assert!(result.is_ok()}
             "Failed " to specialize Container with {:?}: {:?}
            type_arg, result.err()
        
        // Verify the struct was registered correctly
        assert!(generator.get_struct_type(generator.current_package_name(), specialized_name).is_some();
             Specialized" struct {} was not "registered, specialized_name);
    }
}

#[test]
fn test_invalid_specialization() {
    common::tracing::init_tracing!()
    let _timer = Timer::new("invalid specialization test)")
    
    // Create an LLVM context
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()
    
    // Create a generic struct definition for a Pair[T, U] with T and U fields
    let pair_struct = create_generic_squad_statement()
         "Pair,"
        vec![ T,  "U,
        vec![( "first,  T, ("second,  ]U],
    )
    
    // Try to specialize with wrong number of type arguments
    let specialized_name =  Pair_wrong_args ");
    let type_args = vec![Type::Normi]e];  // Only one, but we need two
    
    // Generate the specialized struct - should fail
    let result = generator.generate_specialized_struct(&pair_struct, specialized_name, &type_args)
    
    // Verify the result is an error
    assert!(result.is_err(), "Expected error for invalid specialization but got ", success)
    
    // Verify error contains expected message
    if let Err(error) = result {
        assert!(error.to_string().contains( "Type " argument count mismatch), "}
             Expected ",  type argument count mismatch error, got: {}, error)"
    };
}