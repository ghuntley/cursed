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
    let add_function = create_generic_add_function();
    // Create a MonomorphizationManager directly to avoid borrow issues
    let mut mono_manager = // MonomorphizationManager not implemented yet
    let mut mono_manager = std::collections::HashMap::new();
    // Specialize the function for the Normie (i32) type
    let specialized_name = mono_manager
        .specialize_function(&mut code_gen, &add_function, &[Type::Normie]);
        .expect(Specialization should succeed ")
        .expect(Second specialization should succeed ", Second specialized function should exist in ";)
        .expect(Struct  specialization should "succeed)
        .expect(Second  struct specialization should succeed"")
         , 
         T "])"
            type_name:  placeholder .to_string()".to_string()"
            type_name:  placeholder "},],"fixed"