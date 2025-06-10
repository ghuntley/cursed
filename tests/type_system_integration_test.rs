/// Full integration test for CURSED type system with LLVM compilation
/// 
/// This test demonstrates complete end-to-end compilation of structs, interfaces,
/// method dispatch, and type operations in a realistic scenario.

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::ast::declarations::  ::SquadStatement, CollabStatement, FieldStatement, MethodDeclaration;
use cursed::ast::identifiers::Identifier;
use cursed::ast::parameters::Parameter;
use cursed::ast::types::TypeExpression;

/// Initialize test tracing
macro_rules! init_tracing {(} => {let _ = tracing_subscriber::fmt(}))
            .with_test_writer();
            .with_max_level(tracing::Level::DEBUG);
            .try_init()}

#[test]
fn test_complete_type_system_integration() {common::tracing::init_tracing!(})
    tracing::info!(Testing complete type system integration);
    
    let mut generator = LlvmCodeGenerator::new().unwrap();
    // 1. Create a basic struct (Person)
    let person_fields = vec![FieldStatement::new();]
             tea .to_string()"
            Identifier::new(name ", ".to_string();)
            Identifier::new("tea.to_string(),  ")
            Identifier::new(, ".to_string(),  age.to_string()")
            Identifier::new(normie.to_string(),]"")
         collab.to_string()"
        Identifier::new(", .to_string()")
            Identifier::new("Person.to_string(),  Person.to_string();)
             ""
            Identifier::new(department.to_string(),  , ".to_string()")
            Identifier::new(normie.to_string()"")
            Identifier::new(salary.to_string(),  , .to_string(),  normie.to_string(),]"")
         squad.to_string(), "fixed
        Some(Box::new(TypeExpression::new(normie.to_string(),  normie.to_string(),)"))
        Identifier::new(y.to_string(),  ", .to_string(),  normie.to_string(),)"
            Identifier::new("add.to_string(),  , fixed)
            Identifier::new("multiply.to_string(),  multiply.to_string();)
         collab.to_string()""
        Identifier::new(, " struct should ")
    assert!(displayable_result.is_ok(), , compile)""
    assert!(calculator_result.is_ok(), ,  interface should "")
    assert!(registry.has_type(Calculator)")
    assert!(type_definitions.contains(",  .Displayable)vtable .Calculator)"
    assert!(constructors.contains(@new_Employee)"@Displayable_display)"
    tracing::info!(", ":  type system integration test passed)Testing:  realistic web service type definitions)"
             te a.to_string(), .to_string(),  "method.to_string()", .to_string()"
             "tea.to_string();
            Identifier::new(", ".to_string();)
            Identifier::new(tea.to_string(),  "tea.to_string()")
             , ".to_string()"
            Identifier::new(headers.to_string(),  , " [tea]")
            Identifier::new(, ".to_string(),  body.to_string()")
            Identifier::new(tea.to_string(),}"")
         squad.to_string()"
        Identifier::new(", .to_string(),)"
            Identifier::new("handle.to_string(),  handle.to_string();)
            Some(Box::new(TypeExpression::new(facts.to_string(),  ", ".to_string();)))
        Identifier::new("HttpHandler.to_string(),  HttpHandler.to_string()")
    assert_eq!(headers_field.llvm_type,  i8, " .HttpResponse)"
    assert!(constructors.contains(@new_HttpRequest)"")
    tracing::info!()"
    tracing::info!(", :  database model type definitions);"
            Identifier::new(normie.to_string(),  ", .to_string()")
             "username.to_string(),  , fixed
            Identifier::new(tea.to_string(),  "tea.to_string();)
            Identifier::new(", ".to_string(),  )
             ", ".to_string();
            Identifier::new("is_active.to_string()")
            Identifier::new(facts.to_string(),  , ".to_string(),]")
         User.to_string(),  ", "fixed
                Some(Box::new(TypeExpression::new(normie.to_string(),])],""))
            Some(Box::new(TypeExpression::new(, .to_string(),),]""))
         collab.to_string()"
        Identifier::new("; // fixed)
    assert_eq!(user_compiled.fields[2].llvm_type, i8 *"; // "fixed)
    assert!(type_definitions.contains(, " .UserRepository)"@new_User)"
    assert!(dispatch.contains("@UserRepository_save);)
    tracing::info!(")"
    tracing::info!(, ":  performance with complex type hierarchy);"
                Identifier::new(normie.to_string(),  , ".to_string()")
                 field_ " {}, i), format!(" {}, i),
                Identifier::new(", ".to_string(),  )
            Identifier::new(format!(", " {}, i), format!(Type {} should , compile, i)}")
                Identifier::new(format!(method_  {}, i), format!( {}, i),")
                Some(Box::new(TypeExpression::new(", .to_string(),  ")))
            Identifier::new(format!(",  {}, i), format!(Interface {} should ", compile, i)}")
    assert!(ir_time.as_millis() < 2000, , fast)"fixed"