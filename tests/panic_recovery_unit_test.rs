/// Unit Tests for Panic and Recovery LLVM Code Generation
/// 
/// Tests individual components of the panic and recovery system
/// in isolation to ensure proper functionality.

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::ast::{
    statements::{PanicStatement, RecoveryStatement},
    expressions::Literal,
    identifiers::Identifier,
    parser_support::ExpressionStatement,
};
use cursed::runtime::panic::{PanicSeverity, PanicCategory, CursedPanicInfo};
use cursed::error::SourceLocation;

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

/// Test helper to create a basic LLVM code generator
fn create_test_generator() -> LlvmCodeGenerator {
    tracing_setup::init_test_tracing();
    LlvmCodeGenerator::new().expect("Failed to create test generator")
}

/// Test panic statement creation and basic properties
#[test]
fn test_panic_statement_creation() {
    let message = Box::new(Literal::string("Test panic"));
    let panic_stmt = PanicStatement::new("yeet_error".to_string(), message);
    
    assert_eq!(panic_stmt.token, "yeet_error");
    assert_eq!(panic_stmt.message.string(), "\"Test panic\"");
}

/// Test recovery statement creation and basic properties
#[test]
fn test_recovery_statement_creation() {
    let protected_expr = Box::new(Literal::string("protected"));
    let protected_block = Box::new(ExpressionStatement::from_expr(protected_expr));
    let recovery_stmt = RecoveryStatement::new("catch".to_string(), protected_block);
    
    assert_eq!(recovery_stmt.token, "catch");
    assert!(recovery_stmt.recovery_block.is_none());
    assert!(recovery_stmt.error_variable.is_none());
}

/// Test recovery statement with recovery block
#[test]
fn test_recovery_statement_with_recovery() {
    let protected_expr = Box::new(Literal::string("protected"));
    let protected_block = Box::new(ExpressionStatement::from_expr(protected_expr));
    
    let recovery_expr = Box::new(Literal::string("recovery"));
    let recovery_block = Box::new(ExpressionStatement::from_expr(recovery_expr));
    
    let recovery_stmt = RecoveryStatement::new("catch".to_string(), protected_block)
        .with_recovery(recovery_block);
    
    assert!(recovery_stmt.recovery_block.is_some());
    assert_eq!(recovery_stmt.recovery_block.unwrap().string(), "\"recovery\"");
}

/// Test recovery statement with error variable
#[test]
fn test_recovery_statement_with_error_var() {
    let protected_expr = Box::new(Literal::string("protected"));
    let protected_block = Box::new(ExpressionStatement::from_expr(protected_expr));
    let error_var = Identifier::from_name("err");
    
    let recovery_stmt = RecoveryStatement::new("catch".to_string(), protected_block)
        .with_error_var(error_var);
    
    assert!(recovery_stmt.error_variable.is_some());
    assert_eq!(recovery_stmt.error_variable.unwrap().value, "err");
}

/// Test generator counter methods
#[test]
fn test_generator_counters() {
    let generator = create_test_generator();
    
    // Test temp ID generation
    let id1 = generator.next_temp_id();
    let id2 = generator.next_temp_id();
    assert!(id2 > id1, "Temp IDs should increment");
    
    // Test temp counter (alias)
    let counter1 = generator.next_temp_counter();
    let counter2 = generator.next_temp_counter();
    assert!(counter2 > counter1, "Temp counters should increment");
    
    // Test block counter
    let block1 = generator.next_block_counter();
    let block2 = generator.next_block_counter();
    assert!(block2 > block1, "Block counters should increment");
    
    // Test temp name generation
    let name1 = generator.next_temp_name();
    let name2 = generator.next_temp_name();
    assert!(name1.starts_with("%temp_"), "Temp names should have correct prefix");
    assert!(name2.starts_with("%temp_"), "Temp names should have correct prefix");
    assert_ne!(name1, name2, "Temp names should be unique");
    
    // Test block name generation
    let block_name1 = generator.next_block_name("test");
    let block_name2 = generator.next_block_name("test");
    assert!(block_name1.starts_with("test_block_"), "Block names should have correct prefix");
    assert!(block_name2.starts_with("test_block_"), "Block names should have correct prefix");
    assert_ne!(block_name1, block_name2, "Block names should be unique");
}

/// Test string constant addition functionality
#[test]
fn test_string_constant_functionality() {
    let mut generator = create_test_generator();
    
    let test_cases = vec![
        ("simple", "simple"),
        ("with spaces", "with spaces"),
        ("with\nnewlines", "with\nnewlines"),
        ("with\ttabs", "with\ttabs"),
        ("unicode: 测试", "unicode: 测试"),
        ("", ""), // empty string
    ];
    
    for (input, expected) in test_cases {
        let result = generator.add_string_constant(input);
        assert!(result.is_ok(), "String constant addition should succeed for: {}", input);
        
        let string_id = result.unwrap();
        assert!(string_id.starts_with("str_"), "String ID should have correct prefix: {}", string_id);
    }
}

/// Test panic runtime function declaration functionality
#[test]
fn test_panic_runtime_function_declarations() {
    let mut generator = create_test_generator();
    
    let result = generator.declare_panic_runtime_functions();
    assert!(result.is_ok(), "Panic runtime function declarations should succeed");
    
    // Test multiple calls (should be idempotent)
    let result2 = generator.declare_panic_runtime_functions();
    assert!(result2.is_ok(), "Multiple panic runtime function declarations should succeed");
}

/// Test recovery runtime function declaration functionality
#[test]
fn test_recovery_runtime_function_declarations() {
    let mut generator = create_test_generator();
    
    let result = generator.declare_recovery_runtime_functions();
    assert!(result.is_ok(), "Recovery runtime function declarations should succeed");
    
    // Test multiple calls (should be idempotent)
    let result2 = generator.declare_recovery_runtime_functions();
    assert!(result2.is_ok(), "Multiple recovery runtime function declarations should succeed");
}

/// Test source location handling
#[test]
fn test_source_location_handling() {
    let generator = create_test_generator();
    
    // Test getting source location
    let location = generator.get_current_source_location();
    
    // When debug is not explicitly enabled, may return None or default location
    // This is implementation-dependent
    if let Some(loc) = location {
        assert!(loc.line > 0, "Line should be positive");
        assert!(loc.column > 0, "Column should be positive");
        assert!(!loc.file.is_empty(), "File should not be empty");
    }
}

/// Test counter reset functionality
#[test]
fn test_counter_reset() {
    let generator = create_test_generator();
    
    // Generate some IDs
    let _ = generator.next_temp_id();
    let _ = generator.next_block_counter();
    
    // Reset counters
    generator.reset_counters();
    
    // Next IDs should start from 0 again
    let id_after_reset = generator.next_temp_id();
    let block_after_reset = generator.next_block_counter();
    
    assert_eq!(id_after_reset, 0, "Temp ID should reset to 0");
    assert_eq!(block_after_reset, 0, "Block counter should reset to 0");
}

/// Test type checking helper methods
#[test]
fn test_type_checking_helpers() {
    let generator = create_test_generator();
    
    // Test result type checking
    assert!(generator.is_result_type("Result<i32, String>"), "Should recognize Result type");
    assert!(generator.is_result_type("Result<bool, Error>"), "Should recognize Result type");
    assert!(!generator.is_result_type("Option<i32>"), "Should not recognize Option as Result");
    assert!(!generator.is_result_type("i32"), "Should not recognize i32 as Result");
    
    // Test option type checking
    assert!(generator.is_option_type("Option<i32>"), "Should recognize Option type");
    assert!(generator.is_option_type("Option<String>"), "Should recognize Option type");
    assert!(!generator.is_option_type("Result<i32, String>"), "Should not recognize Result as Option");
    assert!(!generator.is_option_type("Vec<i32>"), "Should not recognize Vec as Option");
    
    // Test type generation
    let result_type = generator.get_result_type("i32", "Error");
    assert_eq!(result_type, "Result<i32, Error>", "Should generate correct Result type");
    
    let option_type = generator.get_option_type("String");
    assert_eq!(option_type, "Option<String>", "Should generate correct Option type");
    
    let error_type = generator.get_error_type();
    assert_eq!(error_type, "CursedError", "Should return standard error type");
}

/// Test type string conversion helpers
#[test]
fn test_type_string_conversion() {
    let generator = create_test_generator();
    
    let test_cases = vec![
        ("bool", "i1"),
        ("i32", "i32"),
        ("int", "i32"),
        ("i64", "i64"),
        ("long", "i64"),
        ("f64", "f64"),
        ("float", "f64"),
        ("double", "f64"),
        ("string", "i8*"),
        ("String", "i8*"),
        ("custom_type", "%custom_type"),
    ];
    
    for (input, expected) in test_cases {
        let result = generator.get_type_string_simple(input);
        assert_eq!(result, expected, "Type conversion should match for: {}", input);
    }
}

/// Test current function context management
#[test]
fn test_current_function_context() {
    let generator = create_test_generator();
    
    // Initially should be None
    assert!(generator.get_current_function().is_none(), "Initial function context should be None");
    
    // Set function context
    generator.set_current_function(Some("test_function".to_string()));
    assert_eq!(generator.get_current_function(), Some("test_function".to_string()), "Should return set function");
    
    // Clear function context
    generator.set_current_function(None);
    assert!(generator.get_current_function().is_none(), "Function context should be cleared");
}

/// Test result and option value creation helpers
#[test]
fn test_result_option_value_creation() {
    let generator = create_test_generator();
    
    // Test Result value creation
    let ok_result = generator.create_result_value("i32", "String", true, "test_value");
    assert!(ok_result.contains("insertvalue"), "Should generate insertvalue instruction");
    assert!(ok_result.contains("Result<i32, String>"), "Should include Result type");
    assert!(ok_result.contains("0"), "Should set OK tag");
    
    let err_result = generator.create_result_value("i32", "String", false, "test_error");
    assert!(err_result.contains("insertvalue"), "Should generate insertvalue instruction");
    assert!(err_result.contains("1"), "Should set Error tag");
    
    // Test Option value creation
    let some_option = generator.create_option_value("i32", true, "test_value");
    assert!(some_option.contains("insertvalue"), "Should generate insertvalue instruction");
    assert!(some_option.contains("Option<i32>"), "Should include Option type");
    assert!(some_option.contains("1"), "Should set Some tag");
    
    let none_option = generator.create_option_value("i32", false, "test_value");
    assert!(none_option.contains("0"), "Should set None tag");
}

/// Test conditional branch generation
#[test]
fn test_conditional_branch_generation() {
    let generator = create_test_generator();
    
    let branch = generator.generate_conditional_branch("%condition", "then_block", "else_block");
    assert!(branch.contains("br i1"), "Should generate conditional branch");
    assert!(branch.contains("%condition"), "Should include condition");
    assert!(branch.contains("then_block"), "Should include then block");
    assert!(branch.contains("else_block"), "Should include else block");
}

/// Test phi node generation
#[test]
fn test_phi_node_generation() {
    let generator = create_test_generator();
    
    let values = vec![
        ("value1".to_string(), "block1".to_string()),
        ("value2".to_string(), "block2".to_string()),
    ];
    
    let phi = generator.generate_phi_node("i32", &values);
    assert!(phi.contains("phi i32"), "Should generate phi instruction");
    assert!(phi.contains("value1"), "Should include first value");
    assert!(phi.contains("block1"), "Should include first block");
    assert!(phi.contains("value2"), "Should include second value");
    assert!(phi.contains("block2"), "Should include second block");
}

/// Test check generation for Result and Option types
#[test]
fn test_check_generation() {
    let generator = create_test_generator();
    
    // Test Result success check
    let result_check = generator.generate_result_success_check("%result", "Result<i32, String>");
    assert!(result_check.contains("extractvalue"), "Should extract tag");
    assert!(result_check.contains("icmp eq"), "Should compare with success tag");
    
    // Test Option presence check
    let option_check = generator.generate_option_presence_check("%option", "Option<i32>");
    assert!(option_check.contains("extractvalue"), "Should extract tag");
    assert!(option_check.contains("icmp eq"), "Should compare with Some tag");
}

/// Test value extraction from Result and Option types
#[test]
fn test_value_extraction() {
    let generator = create_test_generator();
    
    // Test Result value extraction
    let result_extract = generator.extract_result_value("%result", "Result<i32, String>", "i32");
    assert!(result_extract.contains("extractvalue"), "Should extract value");
    assert!(result_extract.contains("%result"), "Should use result value");
    assert!(result_extract.contains("1"), "Should extract from index 1");
    
    // Test Option value extraction
    let option_extract = generator.extract_option_value("%option", "Option<i32>", "i32");
    assert!(option_extract.contains("extractvalue"), "Should extract value");
    assert!(option_extract.contains("%option"), "Should use option value");
    assert!(option_extract.contains("1"), "Should extract from index 1");
}

/// Test panic severity and category enum values
#[test]
fn test_panic_enum_values() {
    // Test that enum values are as expected for FFI compatibility
    assert_eq!(PanicSeverity::Recoverable as u8, 0);
    assert_eq!(PanicSeverity::Critical as u8, 1);
    assert_eq!(PanicSeverity::Fatal as u8, 2);
    
    assert_eq!(PanicCategory::Memory as u8, 0);
    assert_eq!(PanicCategory::TypeAssertion as u8, 1);
    assert_eq!(PanicCategory::BoundsCheck as u8, 2);
    assert_eq!(PanicCategory::Arithmetic as u8, 3);
    assert_eq!(PanicCategory::Channel as u8, 4);
    assert_eq!(PanicCategory::Goroutine as u8, 5);
    assert_eq!(PanicCategory::User as u8, 6);
    assert_eq!(PanicCategory::System as u8, 7);
    assert_eq!(PanicCategory::Generic as u8, 8);
}

/// Test CursedPanicInfo creation and properties
#[test]
fn test_cursed_panic_info() {
    let panic_info = CursedPanicInfo::new(
        "Test panic".to_string(),
        PanicSeverity::Critical,
        PanicCategory::User
    );
    
    assert_eq!(panic_info.message, "Test panic");
    assert_eq!(panic_info.severity, PanicSeverity::Critical);
    assert_eq!(panic_info.category, PanicCategory::User);
    assert!(panic_info.panic_id > 0);
    assert!(panic_info.source_location.is_none());
    assert!(panic_info.goroutine_id.is_none());
    assert!(panic_info.enhanced_stack_trace.is_none());
    assert!(panic_info.metadata.is_empty());
}

/// Test panic info with location
#[test]
fn test_panic_info_with_location() {
    let location = SourceLocation::new(42, 15).with_file("test.csd");
    let panic_info = CursedPanicInfo::new(
        "Test panic with location".to_string(),
        PanicSeverity::Fatal,
        PanicCategory::Memory
    ).with_location(location.clone());
    
    assert!(panic_info.source_location.is_some());
    let stored_location = panic_info.source_location.unwrap();
    assert_eq!(stored_location.line, 42);
    assert_eq!(stored_location.column, 15);
    assert_eq!(stored_location.file, "test.csd");
}

/// Test panic info formatting
#[test]
fn test_panic_info_formatting() {
    let location = SourceLocation::new(10, 5).with_file("example.csd");
    let panic_info = CursedPanicInfo::new(
        "Formatting test".to_string(),
        PanicSeverity::Critical,
        PanicCategory::TypeAssertion
    ).with_location(location)
     .with_goroutine(456)
     .with_metadata("context".to_string(), "test".to_string());
    
    let formatted = format!("{}", panic_info);
    
    assert!(formatted.contains("Formatting test"), "Should contain message");
    assert!(formatted.contains("Critical"), "Should contain severity");
    assert!(formatted.contains("TypeAssertion"), "Should contain category");
    assert!(formatted.contains("example.csd"), "Should contain file name");
    assert!(formatted.contains("10"), "Should contain line number");
    assert!(formatted.contains("5"), "Should contain column number");
    assert!(formatted.contains("456"), "Should contain goroutine ID");
}
