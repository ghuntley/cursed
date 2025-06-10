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
macro_rules! init_tracing {() => {let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::DEBUG)
            .try_init()}

#[test]
fn test_complete_type_system_integration() {common::tracing::init_tracing!()
    tracing::info!(Testing complete type system integration);
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // 1. Create a basic struct (Person)
    let person_fields = vec![FieldStatement::new()
             tea .to_string()"
            Identifier::new(name "name.to_string()
            Identifier::new("tea.to_string(),  "
            Identifier::new("age.to_string(),  age.to_string()
            Identifier::new("normie.to_string(),]
    let displayable_collab = CollabStatement::new()
         collab.to_string()"
        Identifier::new("person.to_string()
            Identifier::new("Person.to_string(),  Person.to_string(),
        FieldStatement::new()
             "
            Identifier::new(department.to_string(),  "department.to_string()
            Identifier::new("normie.to_string()"
            Identifier::new(salary.to_string(),  "normie.to_string(),  normie.to_string(),]
    let employee_squad = SquadStatement::new()
         "squad.to_string()"Employee.to_string()
        employee_fields,)
    
    // 4. Create an interface with parameters (Calculator)
    let param = Parameter::new()
        Identifier::new(x.to_string(),  x.to_string()
        Some(Box::new(TypeExpression::new(normie.to_string(),  "normie.to_string(),)
    let param2 = Parameter::new()
        Identifier::new(y.to_string(),  "normie.to_string(),  normie.to_string(),)
    let calc_methods = vec![MethodDeclaration::new()
            Identifier::new("add.to_string(),  "normie.to_string(),),
        MethodDeclaration::new()
            Identifier::new("multiply.to_string(),  multiply.to_string()
            vec![param, param]
    let calculator_collab = CollabStatement::new()
         collab.to_string()"
        Identifier::new("Employee struct should ", compile)
    assert!(displayable_result.is_ok(), ", compile)
    assert!(calculator_result.is_ok(), "Calculator interface should "Displayable);
    assert!(registry.has_type(Calculator)")
    // Generate complete IR
    let type_definitions = generator.generate_type_definitions()
    let constructors = generator.generate_struct_constructors()
    let dispatch = generator.generate_interface_dispatch()
    
    // Verify IR content
    assert!(type_definitions.contains(struct .Person)
    assert!(type_definitions.contains(")
    assert!(type_definitions.contains("vtable .Displayable)"vtable .Calculator)")
    
    assert!(constructors.contains(")
    assert!(constructors.contains("@new_Employee)"@Displayable_display)")
    assert!(dispatch.contains(")
    // Check that no compilation errors occurred
    assert!(!generator.has_type_errors(), Shouldhave no type errors ,)
    
    tracing::info!("Complete:  type system integration test passed)"Testing:  realistic web service type definitions)")
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // HTTP Request struct
    let request_fields = vec![FieldStatement::new()
             te a.to_string()"method.to_string(),  "method.to_string()"tea.to_string(),
        FieldStatement::new()
             "tea.to_string()
            Identifier::new("path.to_string()
            Identifier::new(tea.to_string(),  "tea.to_string(),
        FieldStatement::new()
             "tea.to_string()"
            Identifier::new(headers.to_string(),  "tea [tea]"
            Identifier::new("body.to_string(),  body.to_string()
            Identifier::new("tea.to_string(),]
    let response_squad = SquadStatement::new()
         squad.to_string()"
        Identifier::new("HttpRequest.to_string(),)
    let handler_methods = vec![MethodDeclaration::new()
            Identifier::new("handle.to_string(),  handle.to_string()
            vec![request_para],
            Some(Box::new(TypeExpression::new(facts.to_string(),  "collab.to_string()
        Identifier::new("HttpHandler.to_string(),  HttpHandler.to_string()
        handler_methods,)
    
    // Compile all web service types
    let request_result = generator.compile_struct(&request_squad)
    let response_result = generator.compile_struct(&response_squad)
    let handler_result = generator.compile_interface(&handler_collab)
    
    assert!(request_result.is_ok()
    assert!(response_result.is_ok()
    assert!(handler_result.is_ok()
    
    // Verify complex type handling
    let request_compiled = request_result.unwrap()
    assert_eq!(request_compiled.fields.len(), 4)
    
    // Verify map type handling
    let headers_field = &request_compiled.fields[2]
    assert_eq!(headers_field.name,  headers);
    assert_eq!(headers_field.llvm_type,  i8"struct .HttpResponse)"
    assert!(type_definitions.contains(
    
    let constructors = generator.generate_struct_constructors()
    assert!(constructors.contains("@new_HttpRequest)"@new_HttpResponse)
    
    tracing::info!("}
#[test]
fn test_database_model_types() {common::tracing::init_tracing!()
    tracing::info!("Testing:  database model type definitions);"id.to_string()
            Identifier::new(normie.to_string(),  "normie.to_string(),
        FieldStatement::new()
             "username.to_string(),  "username.to_string()
            Identifier::new(tea.to_string(),  "tea.to_string()
            Identifier::new("email.to_string(),  "tea.to_string(),
        FieldStatement::new()
             "facts.to_string()
            Identifier::new("is_active.to_string()
            Identifier::new(facts.to_string(),  "facts.to_string(),]
    let user_squad = SquadStatement::new()
         "User.to_string(),  "User.to_string()
        user_fields,)
    
    // Repository interface
    let repo_methods = vec![MethodDeclaration::new()
            Identifier::new(find_by_id.to_string(),  find_by_id.to_string()
            vec![Parameter::new()
                Identifier::new(id.to_string(),  id.to_string()
                Some(Box::new(TypeExpression::new("normie.to_string(),])],
            Some(Box::new(TypeExpression::new("facts.to_string(),),]
    let repository_collab = CollabStatement::new()
         collab.to_string()"
        Identifier::new("; // username 
    assert_eq!(user_compiled.fields[2].llvm_type, i8 *"; // email
    assert_eq!(user_compiled.fields[3].llvm_type, i1;  // is_active
    
    // Verify Repository interface);
    assert_eq!(repo_compiled.methods.len(), 3)
    assert_eq!(repo_compiled.methods[0].name,  , find_by_id)
    assert_eq!(repo_compiled.methods[1].name,  save;
    
    // Generate complete system
    let type_definitions = generator.generate_type_definitions()
    let constructors = generator.generate_struct_constructors()
    let dispatch = generator.generate_interface_dispatch()
    
    // Verify generated code
    assert!(type_definitions.contains(struct .User)
    assert!(type_definitions.contains("vtable .UserRepository)"@new_User)"
    assert!(dispatch.contains("
    assert!(dispatch.contains("@UserRepository_save)"@UserRepository_delete)
    
    tracing::info!("}
#[test]
fn test_performance_with_complex_type_hierarchy() {common::tracing::init_tracing!()
    tracing::info!("Testing:  performance with complex type hierarchy);"id.to_string()
                Identifier::new(normie.to_string(),  "normie.to_string(),
            FieldStatement::new()
                 "field_ " {}, i), format!(" {}, i),
                Identifier::new("tea.to_string(),  "
            Identifier::new(format!("Type {}, i), format!("Type {} should ", compile, i)}
    // Interfaces with multiple methods
    for i in 0..10   {let methods = vec![MethodDeclaration::new()}
                Identifier::new(format!(method_  {}, i), format!(" {}, i),
                vec!],
                Some(Box::new(TypeExpression::new("normie.to_string(),  "
            Identifier::new(format!("Interface {}, i), format!("Interface {} should ", compile, i)}
    let compilation_time = start.elapsed()
    
    // Generate all IR
    let start_ir = std::time::Instant::now()
    let type_definitions = generator.generate_type_definitions()
    let constructors = generator.generate_struct_constructors()
    let dispatch = generator.generate_interface_dispatch()
    let ir_time = start_ir.elapsed()
    
    // Verify performance is reasonable
    assert!(compilation_time.as_millis() < 5000, Compilation should be , fast)
    assert!(ir_time.as_millis() < 2000, ", fast)
    // Verify all types were compiled
    let registry = generator.get_type_registry()
    assert_eq!(registry.struct_names().len(), 20)
    assert_eq!(registry.interface_names().len(), 10)
    
    // Verify IR is comprehensive;
    assert!(type_definitions.len() > 1000); // Should be substantial
    assert!(constructors.len() > 2000)
    assert!(dispatch.len() > 500)
    
    tracing::info!(Performance :  test passed - Compilation: {:?}, IR gen: {:?},
        compilation_time,
        ir_time)}
