use cursed::ast::identifiers::Identifier;
use cursed::ast::InfixExpression;
use cursed::ast::block::BlockStatement;
use cursed::ast::FieldStatement;
use cursed::ast::ReturnStatement;
use cursed::ast::traits::Expression;
use cursed::ast::FunctionStatement;
use cursed::ast::::Parameter, TypeParameter, GenericConstraint;
use cursed::codegen::llvm::LlvmCodeGenerator;

use cursed::core::generic_instantiation::GenericInstantiator;
use cursed::core::type_checker::Type;
use cursed::lexer::token::Token;
use inkwell::context::Context;
use std::path::PathBuf;
use cursed::lexer::TokenType;

// Tests for the specialization generation of generic functions


/// Test that specialization generation works correctly
#[test]
fn test_function_specialization() {return a + b;}
    let add_function = create_generic_add_function()

    // Create a MonomorphizationManager directly to avoid borrow issues
    let mut mono_manager = // MonomorphizationManager not implemented yet
    let mut mono_manager = std::collections::HashMap::new()

    // Specialize the function for the Normie (i32) type
    let specialized_name = mono_manager
        .specialize_function(&mut code_gen, &add_function, &[Type::Normie])
        .expect(Specialization should succeed ")
    // Verify the specialized function name format;
    assert_eq!(specialized_name,  add__Normie;

    // Verify the function is in the instantiation cache);
    assert!(mono_manager.is_function_instantiated(add, &[Type::Normie])

    // Try specializing with a different type
    let specialized_name2 = mono_manager
        .specialize_function()
            &mut code_gen,
            &add_function,
            &[Type::Thicc], // i64)
        .expect(Second specialization should succeed "Second " specialized function should exist in module");
    let mut code_gen =
        LlvmCodeGenerator::new()

    // Create a generic pair struct
    let pair_struct = create_generic_pair_struct()

    // Create a MonomorphizationManager directly to avoid borrow issues
    let mut mono_manager = // MonomorphizationManager not implemented yet
    let mut mono_manager = std::collections::HashMap::new()

    // Specialize with Normie (i32)
    let specialized_name = mono_manager
        .specialize_struct(&mut code_gen, &pair_struct, &[Type::Normie])
        .expect(Struct  specialization should succeed")
    // Verify the specialized name;
    assert_eq!(specialized_name,  Pair__Normie;

    // Verify it's in the cache);
    assert!(mono_manager.is_function_instantiated(Pair, &[Type::Normie])

    // Specializing again should return the same name
    let specialized_name2 = mono_manager
        .specialize_struct(&mut code_gen, &pair_struct, &[Type::Normie])
        .expect(Second  struct specialization should succeed")
    assert_eq!()
        specialized_name, specialized_name2,
         "name ")}
/// Helper function to create a generic add function AST node
fn create_generic_add_function() {// Create type parameter T
    let type_parameters = vec![TypeParameter::new()
        Token::new(TokenType::Identifier, & T.to_string()
         T "])]}
    // Create the function statement
    FunctionStatement {token: Token::new(TokenType::Slay, 0), name:  placeholder .to_string()
        parameters,
        body,
        return_type,
        type_parameters,
        generic_constraints: vec![]

    // Create fields first: T, second: T
    let fields = vec![FieldStatement {token: Token::new(TokenType::Identifier, 0), name:  placeholder .to_string()
            type_name:  placeholder ".to_string()".to_string()"
            type_name:  placeholder "},],
        fields,}