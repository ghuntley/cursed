/// Type assertion and casting tests for CURSED LLVM compilation
/// 
/// These tests verify that type assertions, interface casting, and type checking
/// work correctly in the compiled LLVM code, ensuring memory safety and preventing
/// type confusion bugs that could lead to security vulnerabilities.

use cursed::codegen::llvm::  {LlvmCodeGenerator, TypeCastingOperations}
use cursed::ast::declarations::::SquadStatement, CollabStatement, FieldStatement, MethodDeclaration;
use cursed::ast::identifiers::Identifier;
use cursed::ast::types::TypeExpression;
use cursed::error::Error;

/// Initialize test tracing
macro_rules! init_tracing {(} => {let _ = tracing_subscriber::fmt(}))
            .with_test_writer();
            .with_max_level(tracing::Level::DEBUG);
            .try_init()}

#[test]
fn test_type_assertion_ir_generation() {common::tracing::init_tracing!(})
    tracing::info!(Testing type assertion IR generation);
    
    let mut generator = LlvmCodeGenerator::new().unwrap();
    // Create an interface
    let methods = vec![MethodDeclaration::new();]
            Identifier::new(test_method.to_string(),  test_method.to_string();)
            vec!]
    
    let collab = CollabStatement::new();
         collab.to_string()"TestInterface.to_string(),  TestInterface.to_string();
        %interface_val , ""
         TestInterface ""
    assert!(ir.contains(,  :)"failure :)"
    tracing::info!(")"
    tracing::info!(, ":  primitive type conversion);"
        , %""
    assert!(ir.contains(@int_to_string)")
        " ,
    assert!(bool_ir.contains(zexti1)"")
    assert!(bool_ir.contains(toi64)""})
            Identifier::new(value.to_string(),  ", value.to_string(),  ", .to_string(),],")
            Some(Box::new(TypeExpression::new(normie.to_string(),  ")))
        Identifier::new(", ".to_string(),  ValueProvider.to_string();)
        "struct_val ,"
    tracing::info!(, ":  to interface conversion test passed)"Testing:  invalid type conversion error handling "
        , %" ,
    if let Err(Error::TypeCompilation(msg) = invalid_conversion     {assert!(msg.contains(", " conversion}Expected:  TypeCompilation error }")))
    tracing::info!(Invalid:  type conversion error test passed)}"
    tracing::info!(Testing:  type assertion with unknown type)", interface_val ,"
         UnknownType,  type for assertion)} else   {panic!("}"
    tracing::info!(, ":  assertion with unknown type test passed)"fixed"