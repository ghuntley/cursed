/// Type assertion and casting tests for CURSED LLVM compilation
/// 
/// These tests verify that type assertions, interface casting, and type checking
/// work correctly in the compiled LLVM code, ensuring memory safety and preventing
/// type confusion bugs that could lead to security vulnerabilities.

use cursed::codegen::llvm::{LlvmCodeGenerator, TypeCastingOperations};
use cursed::ast::declarations::{SquadStatement, CollabStatement, FieldStatement, MethodDeclaration};
use cursed::ast::identifiers::Identifier;
use cursed::ast::types::TypeExpression;
use cursed::error::Error;

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
fn test_type_assertion_ir_generation() {
    init_tracing!();
    tracing::info!("Testing type assertion IR generation");
    
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Create an interface
    let methods = vec![
        MethodDeclaration::new(
            Identifier::new("test_method".to_string(), "test_method".to_string()),
            vec![],
            None,
        ),
    ];
    
    let collab = CollabStatement::new(
        "collab".to_string(),
        Identifier::new("TestInterface".to_string(), "TestInterface".to_string()),
        methods,
    );
    
    let interface_result = generator.compile_interface(&collab);
    assert!(interface_result.is_ok());
    
    // Generate type assertion IR
    let registry = generator.get_type_registry();
    let assertion_ir = TypeCastingOperations::generate_type_assertion(
        "%interface_val",
        "TestInterface",
        registry,
    );
    
    assert!(assertion_ir.is_ok());
    let ir = assertion_ir.unwrap();
    
    // Verify IR contains type checking logic
    assert!(ir.contains("getelementptr"));
    assert!(ir.contains("icmp eq"));
    assert!(ir.contains("br i1"));
    assert!(ir.contains("success:"));
    assert!(ir.contains("failure:"));
    
    tracing::info!("Type assertion IR generation test passed");
}

#[test]
fn test_primitive_type_conversion() {
    init_tracing!();
    tracing::info!("Testing primitive type conversion");
    
    let generator = LlvmCodeGenerator::new().unwrap();
    let registry = generator.get_type_registry();
    
    // Test normie to tea conversion
    let conversion_ir = TypeCastingOperations::generate_type_conversion(
        "normie",
        "tea",
        "%int_val",
        registry,
    );
    
    assert!(conversion_ir.is_ok());
    let ir = conversion_ir.unwrap();
    
    assert!(ir.contains("@int_to_string"));
    assert!(ir.contains("%int_val"));
    
    // Test facts to normie conversion
    let bool_conversion = TypeCastingOperations::generate_type_conversion(
        "facts",
        "normie",
        "%bool_val",
        registry,
    );
    
    assert!(bool_conversion.is_ok());
    let bool_ir = bool_conversion.unwrap();
    
    assert!(bool_ir.contains("zext i1"));
    assert!(bool_ir.contains("to i64"));
    
    tracing::info!("Primitive type conversion test passed");
}

#[test]
fn test_struct_to_interface_conversion() {
    init_tracing!();
    tracing::info!("Testing struct to interface conversion");
    
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Create a struct
    let struct_fields = vec![
        FieldStatement::new(
            "normie".to_string(),
            Identifier::new("value".to_string(), "value".to_string()),
            Identifier::new("normie".to_string(), "normie".to_string()),
        ),
    ];
    
    let squad = SquadStatement::new(
        "squad".to_string(),
        Identifier::new("ConcreteType".to_string(), "ConcreteType".to_string()),
        struct_fields,
    );
    
    // Create an interface
    let interface_methods = vec![
        MethodDeclaration::new(
            Identifier::new("get_value".to_string(), "get_value".to_string()),
            vec![],
            Some(Box::new(TypeExpression::new("normie".to_string(), "normie".to_string()))),
        ),
    ];
    
    let collab = CollabStatement::new(
        "collab".to_string(),
        Identifier::new("ValueProvider".to_string(), "ValueProvider".to_string()),
        interface_methods,
    );
    
    // Compile both
    let struct_result = generator.compile_struct(&squad);
    let interface_result = generator.compile_interface(&collab);
    
    assert!(struct_result.is_ok());
    assert!(interface_result.is_ok());
    
    // Test conversion
    let registry = generator.get_type_registry();
    let conversion_ir = TypeCastingOperations::generate_type_conversion(
        "ConcreteType",
        "ValueProvider",
        "%struct_val",
        registry,
    );
    
    assert!(conversion_ir.is_ok());
    let ir = conversion_ir.unwrap();
    
    assert!(ir.contains("@convert_to_interface"));
    assert!(ir.contains("ValueProvider"));
    assert!(ir.contains("%struct_val"));
    
    tracing::info!("Struct to interface conversion test passed");
}

#[test]
fn test_invalid_type_conversion_error() {
    init_tracing!();
    tracing::info!("Testing invalid type conversion error handling");
    
    let generator = LlvmCodeGenerator::new().unwrap();
    let registry = generator.get_type_registry();
    
    // Try invalid conversion
    let invalid_conversion = TypeCastingOperations::generate_type_conversion(
        "InvalidType1",
        "InvalidType2",
        "%value",
        registry,
    );
    
    assert!(invalid_conversion.is_err());
    if let Err(Error::TypeCompilation(msg)) = invalid_conversion {
        assert!(msg.contains("Unsupported type conversion"));
    } else {
        panic!("Expected TypeCompilation error");
    }
    
    tracing::info!("Invalid type conversion error test passed");
}

#[test]
fn test_type_assertion_with_unknown_type() {
    init_tracing!();
    tracing::info!("Testing type assertion with unknown type");
    
    let generator = LlvmCodeGenerator::new().unwrap();
    let registry = generator.get_type_registry();
    
    let assertion_result = TypeCastingOperations::generate_type_assertion(
        "%interface_val",
        "UnknownType",
        registry,
    );
    
    assert!(assertion_result.is_err());
    if let Err(Error::TypeCompilation(msg)) = assertion_result {
        assert!(msg.contains("Unknown type for assertion"));
    } else {
        panic!("Expected TypeCompilation error");
    }
    
    tracing::info!("Type assertion with unknown type test passed");
}
