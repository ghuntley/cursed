/// Full integration test for CURSED type system with LLVM compilation
/// 
/// This test demonstrates complete end-to-end compilation of structs, interfaces,
/// method dispatch, and type operations in a realistic scenario.

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::ast::declarations::{SquadStatement, CollabStatement, FieldStatement, MethodDeclaration};
use cursed::ast::identifiers::Identifier;
use cursed::ast::parameters::Parameter;
use cursed::ast::types::TypeExpression;

/// Initialize test tracing
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::DEBUG)
            .try_init();
    };
}

#[test]
fn test_complete_type_system_integration() {
    init_tracing!();
    tracing::info!("Testing complete type system integration");
    
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // 1. Create a basic struct (Person)
    let person_fields = vec![
        FieldStatement::new(
            "tea".to_string(),
            Identifier::new("name".to_string(), "name".to_string()),
            Identifier::new("tea".to_string(), "tea".to_string()),
        ),
        FieldStatement::new(
            "normie".to_string(),
            Identifier::new("age".to_string(), "age".to_string()),
            Identifier::new("normie".to_string(), "normie".to_string()),
        ),
    ];
    
    let person_squad = SquadStatement::new(
        "squad".to_string(),
        Identifier::new("Person".to_string(), "Person".to_string()),
        person_fields,
    );
    
    // 2. Create an interface (Displayable)
    let display_methods = vec![
        MethodDeclaration::new(
            Identifier::new("display".to_string(), "display".to_string()),
            vec![],
            Some(Box::new(TypeExpression::new("tea".to_string(), "tea".to_string()))),
        ),
        MethodDeclaration::new(
            Identifier::new("get_info".to_string(), "get_info".to_string()),
            vec![],
            Some(Box::new(TypeExpression::new("tea".to_string(), "tea".to_string()))),
        ),
    ];
    
    let displayable_collab = CollabStatement::new(
        "collab".to_string(),
        Identifier::new("Displayable".to_string(), "Displayable".to_string()),
        display_methods,
    );
    
    // 3. Create a more complex struct (Employee) 
    let employee_fields = vec![
        FieldStatement::new(
            "Person".to_string(),
            Identifier::new("person".to_string(), "person".to_string()),
            Identifier::new("Person".to_string(), "Person".to_string()),
        ),
        FieldStatement::new(
            "tea".to_string(),
            Identifier::new("department".to_string(), "department".to_string()),
            Identifier::new("tea".to_string(), "tea".to_string()),
        ),
        FieldStatement::new(
            "normie".to_string(),
            Identifier::new("salary".to_string(), "salary".to_string()),
            Identifier::new("normie".to_string(), "normie".to_string()),
        ),
    ];
    
    let employee_squad = SquadStatement::new(
        "squad".to_string(),
        Identifier::new("Employee".to_string(), "Employee".to_string()),
        employee_fields,
    );
    
    // 4. Create an interface with parameters (Calculator)
    let param = Parameter::new(
        Identifier::new("x".to_string(), "x".to_string()),
        Some(Box::new(TypeExpression::new("normie".to_string(), "normie".to_string()))),
    );
    let param2 = Parameter::new(
        Identifier::new("y".to_string(), "y".to_string()),
        Some(Box::new(TypeExpression::new("normie".to_string(), "normie".to_string()))),
    );
    
    let calc_methods = vec![
        MethodDeclaration::new(
            Identifier::new("add".to_string(), "add".to_string()),
            vec![param.clone(), param2.clone()],
            Some(Box::new(TypeExpression::new("normie".to_string(), "normie".to_string()))),
        ),
        MethodDeclaration::new(
            Identifier::new("multiply".to_string(), "multiply".to_string()),
            vec![param, param2],
            Some(Box::new(TypeExpression::new("normie".to_string(), "normie".to_string()))),
        ),
    ];
    
    let calculator_collab = CollabStatement::new(
        "collab".to_string(),
        Identifier::new("Calculator".to_string(), "Calculator".to_string()),
        calc_methods,
    );
    
    // Compile all types
    let person_result = generator.compile_struct(&person_squad);
    let employee_result = generator.compile_struct(&employee_squad);
    let displayable_result = generator.compile_interface(&displayable_collab);
    let calculator_result = generator.compile_interface(&calculator_collab);
    
    // Verify all compilations succeeded
    assert!(person_result.is_ok(), "Person struct should compile");
    assert!(employee_result.is_ok(), "Employee struct should compile");
    assert!(displayable_result.is_ok(), "Displayable interface should compile");
    assert!(calculator_result.is_ok(), "Calculator interface should compile");
    
    // Verify registry state
    let registry = generator.get_type_registry();
    assert!(registry.has_type("Person"));
    assert!(registry.has_type("Employee"));
    assert!(registry.has_type("Displayable"));
    assert!(registry.has_type("Calculator"));
    
    // Generate complete IR
    let type_definitions = generator.generate_type_definitions();
    let constructors = generator.generate_struct_constructors();
    let dispatch = generator.generate_interface_dispatch();
    
    // Verify IR content
    assert!(type_definitions.contains("struct.Person"));
    assert!(type_definitions.contains("struct.Employee"));
    assert!(type_definitions.contains("vtable.Displayable"));
    assert!(type_definitions.contains("vtable.Calculator"));
    
    assert!(constructors.contains("@new_Person"));
    assert!(constructors.contains("@new_Employee"));
    
    assert!(dispatch.contains("@Displayable_display"));
    assert!(dispatch.contains("@Calculator_add"));
    
    // Check that no compilation errors occurred
    assert!(!generator.has_type_errors(), "Should have no type errors");
    
    tracing::info!("Complete type system integration test passed");
}

#[test]
fn test_realistic_web_service_types() {
    init_tracing!();
    tracing::info!("Testing realistic web service type definitions");
    
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // HTTP Request struct
    let request_fields = vec![
        FieldStatement::new(
            "tea".to_string(),
            Identifier::new("method".to_string(), "method".to_string()),
            Identifier::new("tea".to_string(), "tea".to_string()),
        ),
        FieldStatement::new(
            "tea".to_string(),
            Identifier::new("path".to_string(), "path".to_string()),
            Identifier::new("tea".to_string(), "tea".to_string()),
        ),
        FieldStatement::new(
            "tea[tea]tea".to_string(),
            Identifier::new("headers".to_string(), "headers".to_string()),
            Identifier::new("tea[tea]tea".to_string(), "tea[tea]tea".to_string()),
        ),
        FieldStatement::new(
            "tea".to_string(),
            Identifier::new("body".to_string(), "body".to_string()),
            Identifier::new("tea".to_string(), "tea".to_string()),
        ),
    ];
    
    let request_squad = SquadStatement::new(
        "squad".to_string(),
        Identifier::new("HttpRequest".to_string(), "HttpRequest".to_string()),
        request_fields,
    );
    
    // HTTP Response struct
    let response_fields = vec![
        FieldStatement::new(
            "normie".to_string(),
            Identifier::new("status_code".to_string(), "status_code".to_string()),
            Identifier::new("normie".to_string(), "normie".to_string()),
        ),
        FieldStatement::new(
            "tea[tea]tea".to_string(),
            Identifier::new("headers".to_string(), "headers".to_string()),
            Identifier::new("tea[tea]tea".to_string(), "tea[tea]tea".to_string()),
        ),
        FieldStatement::new(
            "tea".to_string(),
            Identifier::new("body".to_string(), "body".to_string()),
            Identifier::new("tea".to_string(), "tea".to_string()),
        ),
    ];
    
    let response_squad = SquadStatement::new(
        "squad".to_string(),
        Identifier::new("HttpResponse".to_string(), "HttpResponse".to_string()),
        response_fields,
    );
    
    // HTTP Handler interface
    let request_param = Parameter::new(
        Identifier::new("request".to_string(), "request".to_string()),
        Some(Box::new(TypeExpression::new("HttpRequest".to_string(), "HttpRequest".to_string()))),
    );
    
    let handler_methods = vec![
        MethodDeclaration::new(
            Identifier::new("handle".to_string(), "handle".to_string()),
            vec![request_param],
            Some(Box::new(TypeExpression::new("HttpResponse".to_string(), "HttpResponse".to_string()))),
        ),
        MethodDeclaration::new(
            Identifier::new("supports_method".to_string(), "supports_method".to_string()),
            vec![Parameter::new(
                Identifier::new("method".to_string(), "method".to_string()),
                Some(Box::new(TypeExpression::new("tea".to_string(), "tea".to_string()))),
            )],
            Some(Box::new(TypeExpression::new("facts".to_string(), "facts".to_string()))),
        ),
    ];
    
    let handler_collab = CollabStatement::new(
        "collab".to_string(),
        Identifier::new("HttpHandler".to_string(), "HttpHandler".to_string()),
        handler_methods,
    );
    
    // Compile all web service types
    let request_result = generator.compile_struct(&request_squad);
    let response_result = generator.compile_struct(&response_squad);
    let handler_result = generator.compile_interface(&handler_collab);
    
    assert!(request_result.is_ok());
    assert!(response_result.is_ok());
    assert!(handler_result.is_ok());
    
    // Verify complex type handling
    let request_compiled = request_result.unwrap();
    assert_eq!(request_compiled.fields.len(), 4);
    
    // Verify map type handling
    let headers_field = &request_compiled.fields[2];
    assert_eq!(headers_field.name, "headers");
    assert_eq!(headers_field.llvm_type, "i8*"); // Map pointer
    
    // Generate and verify IR
    let type_definitions = generator.generate_type_definitions();
    assert!(type_definitions.contains("struct.HttpRequest"));
    assert!(type_definitions.contains("struct.HttpResponse"));
    assert!(type_definitions.contains("vtable.HttpHandler"));
    
    let constructors = generator.generate_struct_constructors();
    assert!(constructors.contains("@new_HttpRequest"));
    assert!(constructors.contains("@new_HttpResponse"));
    
    tracing::info!("Realistic web service types test passed");
}

#[test]
fn test_database_model_types() {
    init_tracing!();
    tracing::info!("Testing database model type definitions");
    
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // User model
    let user_fields = vec![
        FieldStatement::new(
            "normie".to_string(),
            Identifier::new("id".to_string(), "id".to_string()),
            Identifier::new("normie".to_string(), "normie".to_string()),
        ),
        FieldStatement::new(
            "tea".to_string(),
            Identifier::new("username".to_string(), "username".to_string()),
            Identifier::new("tea".to_string(), "tea".to_string()),
        ),
        FieldStatement::new(
            "tea".to_string(),
            Identifier::new("email".to_string(), "email".to_string()),
            Identifier::new("tea".to_string(), "tea".to_string()),
        ),
        FieldStatement::new(
            "facts".to_string(),
            Identifier::new("is_active".to_string(), "is_active".to_string()),
            Identifier::new("facts".to_string(), "facts".to_string()),
        ),
    ];
    
    let user_squad = SquadStatement::new(
        "squad".to_string(),
        Identifier::new("User".to_string(), "User".to_string()),
        user_fields,
    );
    
    // Repository interface
    let repo_methods = vec![
        MethodDeclaration::new(
            Identifier::new("find_by_id".to_string(), "find_by_id".to_string()),
            vec![Parameter::new(
                Identifier::new("id".to_string(), "id".to_string()),
                Some(Box::new(TypeExpression::new("normie".to_string(), "normie".to_string()))),
            )],
            Some(Box::new(TypeExpression::new("User".to_string(), "User".to_string()))),
        ),
        MethodDeclaration::new(
            Identifier::new("save".to_string(), "save".to_string()),
            vec![Parameter::new(
                Identifier::new("user".to_string(), "user".to_string()),
                Some(Box::new(TypeExpression::new("User".to_string(), "User".to_string()))),
            )],
            Some(Box::new(TypeExpression::new("facts".to_string(), "facts".to_string()))),
        ),
        MethodDeclaration::new(
            Identifier::new("delete".to_string(), "delete".to_string()),
            vec![Parameter::new(
                Identifier::new("id".to_string(), "id".to_string()),
                Some(Box::new(TypeExpression::new("normie".to_string(), "normie".to_string()))),
            )],
            Some(Box::new(TypeExpression::new("facts".to_string(), "facts".to_string()))),
        ),
    ];
    
    let repository_collab = CollabStatement::new(
        "collab".to_string(),
        Identifier::new("UserRepository".to_string(), "UserRepository".to_string()),
        repo_methods,
    );
    
    // Compile database types
    let user_result = generator.compile_struct(&user_squad);
    let repo_result = generator.compile_interface(&repository_collab);
    
    assert!(user_result.is_ok());
    assert!(repo_result.is_ok());
    
    let user_compiled = user_result.unwrap();
    let repo_compiled = repo_result.unwrap();
    
    // Verify User struct
    assert_eq!(user_compiled.fields.len(), 4);
    assert_eq!(user_compiled.fields[0].llvm_type, "i64"); // id
    assert_eq!(user_compiled.fields[1].llvm_type, "i8*"); // username
    assert_eq!(user_compiled.fields[2].llvm_type, "i8*"); // email
    assert_eq!(user_compiled.fields[3].llvm_type, "i1");  // is_active
    
    // Verify Repository interface
    assert_eq!(repo_compiled.methods.len(), 3);
    assert_eq!(repo_compiled.methods[0].name, "find_by_id");
    assert_eq!(repo_compiled.methods[1].name, "save");
    assert_eq!(repo_compiled.methods[2].name, "delete");
    
    // Generate complete system
    let type_definitions = generator.generate_type_definitions();
    let constructors = generator.generate_struct_constructors();
    let dispatch = generator.generate_interface_dispatch();
    
    // Verify generated code
    assert!(type_definitions.contains("struct.User"));
    assert!(type_definitions.contains("vtable.UserRepository"));
    assert!(constructors.contains("@new_User"));
    assert!(dispatch.contains("@UserRepository_find_by_id"));
    assert!(dispatch.contains("@UserRepository_save"));
    assert!(dispatch.contains("@UserRepository_delete"));
    
    tracing::info!("Database model types test passed");
}

#[test]
fn test_performance_with_complex_type_hierarchy() {
    init_tracing!();
    tracing::info!("Testing performance with complex type hierarchy");
    
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Create a complex hierarchy of types
    let start = std::time::Instant::now();
    
    // Base types
    for i in 0..20 {
        let fields = vec![
            FieldStatement::new(
                "normie".to_string(),
                Identifier::new("id".to_string(), "id".to_string()),
                Identifier::new("normie".to_string(), "normie".to_string()),
            ),
            FieldStatement::new(
                "tea".to_string(),
                Identifier::new(format!("field_{}", i), format!("field_{}", i)),
                Identifier::new("tea".to_string(), "tea".to_string()),
            ),
        ];
        
        let squad = SquadStatement::new(
            "squad".to_string(),
            Identifier::new(format!("Type{}", i), format!("Type{}", i)),
            fields,
        );
        
        let result = generator.compile_struct(&squad);
        assert!(result.is_ok(), "Type{} should compile", i);
    }
    
    // Interfaces with multiple methods
    for i in 0..10 {
        let methods = vec![
            MethodDeclaration::new(
                Identifier::new(format!("method_{}", i), format!("method_{}", i)),
                vec![],
                Some(Box::new(TypeExpression::new("normie".to_string(), "normie".to_string()))),
            ),
        ];
        
        let collab = CollabStatement::new(
            "collab".to_string(),
            Identifier::new(format!("Interface{}", i), format!("Interface{}", i)),
            methods,
        );
        
        let result = generator.compile_interface(&collab);
        assert!(result.is_ok(), "Interface{} should compile", i);
    }
    
    let compilation_time = start.elapsed();
    
    // Generate all IR
    let start_ir = std::time::Instant::now();
    let type_definitions = generator.generate_type_definitions();
    let constructors = generator.generate_struct_constructors();
    let dispatch = generator.generate_interface_dispatch();
    let ir_time = start_ir.elapsed();
    
    // Verify performance is reasonable
    assert!(compilation_time.as_millis() < 5000, "Compilation should be fast");
    assert!(ir_time.as_millis() < 2000, "IR generation should be fast");
    
    // Verify all types were compiled
    let registry = generator.get_type_registry();
    assert_eq!(registry.struct_names().len(), 20);
    assert_eq!(registry.interface_names().len(), 10);
    
    // Verify IR is comprehensive
    assert!(type_definitions.len() > 1000); // Should be substantial
    assert!(constructors.len() > 2000);
    assert!(dispatch.len() > 500);
    
    tracing::info!(
        "Performance test passed - Compilation: {:?}, IR gen: {:?}",
        compilation_time,
        ir_time
    );
}
