/// Comprehensive Integration Tests for Panic and Recovery System
/// 
/// Tests the LLVM code generation for panic and recovery statements,
/// integration with the runtime panic system, and FFI functionality.

use cursed::ast::{
    statements::{PanicStatement, RecoveryStatement},
    expressions::Literal,
    identifiers::Identifier,
    parser_support::ExpressionStatement,
    traits::{Statement, Expression},
};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::runtime::panic::{
    PanicRuntime, PanicConfig, CursedPanicInfo, PanicSeverity, PanicCategory,
    initialize_panic_runtime, shutdown_panic_runtime, get_panic_runtime
};
use cursed::error::{Error, SourceLocation};
use cursed::debug::DebugConfig;
use std::sync::Arc;
use std::time::Duration;

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

/// Initialize test environment with tracing
fn init_test_environment() {
    tracing_setup::init_test_tracing();
}

/// Test panic statement compilation
#[test]
fn test_panic_statement_compilation() {
    init_test_environment();
    tracing::info!("Testing panic statement compilation");
    
    // Create a simple panic statement
    let message = Box::new(Literal::string("Test panic message"));
    let panic_stmt = PanicStatement::new("yeet_error".to_string(), message);
    
    // Create LLVM code generator
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create code generator");
    
    // Compile the panic statement
    let result = generator.compile_panic_statement(&panic_stmt);
    
    // Should succeed in compilation (not execution)
    assert!(result.is_ok(), "Panic statement compilation should succeed");
    
    tracing::info!("Panic statement compilation test completed successfully");
}

/// Test recovery statement compilation
#[test]
fn test_recovery_statement_compilation() {
    init_test_environment();
    tracing::info!("Testing recovery statement compilation");
    
    // Create a protected block (simple expression statement)
    let protected_expr = Box::new(Literal::string("protected code"));
    let protected_block = Box::new(ExpressionStatement::from_expr(protected_expr));
    
    // Create recovery statement
    let recovery_stmt = RecoveryStatement::new("catch".to_string(), protected_block);
    
    // Create LLVM code generator
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create code generator");
    
    // Compile the recovery statement
    let result = generator.compile_recovery_statement(&recovery_stmt);
    
    // Should succeed in compilation
    assert!(result.is_ok(), "Recovery statement compilation should succeed");
    
    tracing::info!("Recovery statement compilation test completed successfully");
}

/// Test recovery statement with custom recovery block
#[test]
fn test_recovery_statement_with_handler() {
    init_test_environment();
    tracing::info!("Testing recovery statement with custom recovery handler");
    
    // Create protected block
    let protected_expr = Box::new(Literal::string("protected code"));
    let protected_block = Box::new(ExpressionStatement::from_expr(protected_expr));
    
    // Create recovery block
    let recovery_expr = Box::new(Literal::string("recovery handler"));
    let recovery_block = Box::new(ExpressionStatement::from_expr(recovery_expr));
    
    // Create recovery statement with handler
    let recovery_stmt = RecoveryStatement::new("catch".to_string(), protected_block)
        .with_recovery(recovery_block);
    
    // Create LLVM code generator
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create code generator");
    
    // Compile the recovery statement
    let result = generator.compile_recovery_statement(&recovery_stmt);
    
    // Should succeed in compilation
    assert!(result.is_ok(), "Recovery statement with handler compilation should succeed");
    
    tracing::info!("Recovery statement with handler test completed successfully");
}

/// Test recovery statement with error variable binding
#[test]
fn test_recovery_statement_with_error_variable() {
    init_test_environment();
    tracing::info!("Testing recovery statement with error variable binding");
    
    // Create protected block
    let protected_expr = Box::new(Literal::string("protected code"));
    let protected_block = Box::new(ExpressionStatement::from_expr(protected_expr));
    
    // Create error variable
    let error_var = Identifier::from_name("error");
    
    // Create recovery statement with error variable
    let recovery_stmt = RecoveryStatement::new("catch".to_string(), protected_block)
        .with_error_var(error_var);
    
    // Create LLVM code generator
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create code generator");
    
    // Compile the recovery statement
    let result = generator.compile_recovery_statement(&recovery_stmt);
    
    // Should succeed in compilation
    assert!(result.is_ok(), "Recovery statement with error variable should succeed");
    
    tracing::info!("Recovery statement with error variable test completed successfully");
}

/// Test panic runtime function declarations
#[test]
fn test_panic_runtime_function_declarations() {
    init_test_environment();
    tracing::info!("Testing panic runtime function declarations");
    
    // Create LLVM code generator
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create code generator");
    
    // Declare panic runtime functions
    let result = generator.declare_panic_runtime_functions();
    
    // Should succeed
    assert!(result.is_ok(), "Panic runtime function declarations should succeed");
    
    tracing::info!("Panic runtime function declarations test completed successfully");
}

/// Test recovery runtime function declarations
#[test]
fn test_recovery_runtime_function_declarations() {
    init_test_environment();
    tracing::info!("Testing recovery runtime function declarations");
    
    // Create LLVM code generator
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create code generator");
    
    // Declare recovery runtime functions
    let result = generator.declare_recovery_runtime_functions();
    
    // Should succeed
    assert!(result.is_ok(), "Recovery runtime function declarations should succeed");
    
    tracing::info!("Recovery runtime function declarations test completed successfully");
}

/// Test string constant addition
#[test]
fn test_string_constant_addition() {
    init_test_environment();
    tracing::info!("Testing string constant addition");
    
    // Create LLVM code generator
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create code generator");
    
    // Add string constants
    let test_strings = vec![
        "Hello, World!",
        "Test panic message",
        "Error occurred here",
        "Unicode test: 测试",
        "",  // Empty string
    ];
    
    for test_string in test_strings {
        let result = generator.add_string_constant(test_string);
        assert!(result.is_ok(), "String constant addition should succeed for: {}", test_string);
        
        let string_id = result.unwrap();
        assert!(string_id.starts_with("str_"), "String ID should have correct prefix");
        
        tracing::debug!(test_string = %test_string, string_id = %string_id, "Added string constant");
    }
    
    tracing::info!("String constant addition test completed successfully");
}

/// Test source location handling
#[test]
fn test_source_location_handling() {
    init_test_environment();
    tracing::info!("Testing source location handling");
    
    // Create LLVM code generator with debug enabled
    let debug_config = DebugConfig::default().with_debug_enabled(true);
    let mut generator = LlvmCodeGenerator::new_with_debug(debug_config)
        .expect("Failed to create debug code generator");
    
    // Test getting current source location
    let location = generator.get_current_source_location();
    
    // Should return Some location when debug is enabled
    assert!(location.is_some(), "Should return source location when debug enabled");
    
    if let Some(loc) = location {
        tracing::debug!(
            line = loc.line,
            column = loc.column,
            file = %loc.file,
            "Retrieved source location"
        );
    }
    
    tracing::info!("Source location handling test completed successfully");
}

/// Test enhanced panic statement compilation
#[test]
fn test_enhanced_panic_statement_compilation() {
    init_test_environment();
    tracing::info!("Testing enhanced panic statement compilation");
    
    // Create a panic statement with string literal
    let message = Box::new(Literal::string("Enhanced panic test"));
    let panic_stmt = PanicStatement::new("yeet_error".to_string(), message);
    
    // Create source location
    let source_location = Some(SourceLocation::new(42, 10).with_file("test.csd"));
    
    // Create LLVM code generator
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create code generator");
    
    // Compile enhanced panic statement
    let result = generator.compile_panic_statement_enhanced(&panic_stmt, source_location);
    
    // Should succeed in compilation
    assert!(result.is_ok(), "Enhanced panic statement compilation should succeed");
    
    tracing::info!("Enhanced panic statement compilation test completed successfully");
}

/// Test FFI function integration
#[test]
fn test_ffi_function_integration() {
    init_test_environment();
    tracing::info!("Testing FFI function integration");
    
    // Test basic FFI functions (these are safe to call)
    
    // Test value to string conversion
    let test_ptr = b"test_value".as_ptr();
    let result_ptr = unsafe { cursed::runtime::panic::cursed_value_to_string(test_ptr) };
    assert!(!result_ptr.is_null(), "Value to string should return non-null pointer");
    
    // Test recovery mode functions
    cursed::runtime::panic::cursed_enter_recovery_mode();
    cursed::runtime::panic::cursed_exit_recovery_mode();
    
    // Test safe point marking
    cursed::runtime::panic::cursed_mark_safe_point();
    
    // Test error context recording
    cursed::runtime::panic::cursed_record_error_context(10, 5, std::ptr::null());
    
    // Test error propagation
    cursed::runtime::panic::cursed_error_propagation(std::ptr::null(), 20, 15);
    
    tracing::info!("FFI function integration test completed successfully");
}

/// Test panic runtime initialization and shutdown
#[test]
fn test_panic_runtime_lifecycle() {
    init_test_environment();
    tracing::info!("Testing panic runtime lifecycle");
    
    // Test initialization
    let init_result = initialize_panic_runtime();
    assert!(init_result.is_ok(), "Panic runtime initialization should succeed");
    
    // Test getting runtime
    let runtime = get_panic_runtime();
    assert!(runtime.is_some(), "Should be able to get panic runtime after initialization");
    
    // Test runtime functionality
    if let Some(runtime) = runtime {
        let stats_result = runtime.get_statistics();
        assert!(stats_result.is_ok(), "Should be able to get runtime statistics");
        
        let stats = stats_result.unwrap();
        tracing::debug!(
            total_panics = stats.total_panics,
            successful_recoveries = stats.successful_recoveries,
            failed_recoveries = stats.failed_recoveries,
            "Retrieved panic runtime statistics"
        );
    }
    
    // Test shutdown
    let shutdown_result = shutdown_panic_runtime();
    assert!(shutdown_result.is_ok(), "Panic runtime shutdown should succeed");
    
    tracing::info!("Panic runtime lifecycle test completed successfully");
}

/// Test recovery with custom configuration
#[test]
fn test_recovery_with_custom_config() {
    init_test_environment();
    tracing::info!("Testing recovery with custom configuration");
    
    // Create custom panic configuration
    let custom_config = PanicConfig {
        capture_backtraces: true,
        capture_stack_traces: true,
        max_stack_depth: 50,
        log_to_stderr: false, // Disable for testing
        abort_on_fatal: false,
        default_recovery: cursed::runtime::panic::RecoveryAction::Continue(
            Error::Runtime("Test recovery error".to_string())
        ),
        recovery_timeout: Duration::from_secs(5),
        debug_manager: None,
        stack_trace_config: cursed::runtime::debug_info::StackTraceConfig::default(),
    };
    
    // Create runtime with custom config
    let runtime = PanicRuntime::with_config(custom_config);
    let init_result = runtime.initialize();
    assert!(init_result.is_ok(), "Custom runtime initialization should succeed");
    
    // Test recovery functionality
    let recovery_result = runtime.recover(|| {
        // This operation should succeed
        42
    });
    
    assert!(recovery_result.is_ok(), "Recovery should succeed for non-panicking operation");
    assert_eq!(recovery_result.unwrap(), 42, "Recovery should return correct value");
    
    // Test recovery with panic
    let panic_recovery_result = runtime.recover(|| {
        panic!("Test panic for recovery");
    });
    
    assert!(panic_recovery_result.is_err(), "Recovery should fail for panicking operation");
    
    let shutdown_result = runtime.shutdown();
    assert!(shutdown_result.is_ok(), "Custom runtime shutdown should succeed");
    
    tracing::info!("Recovery with custom configuration test completed successfully");
}

/// Test panic info creation and formatting
#[test]
fn test_panic_info_creation() {
    init_test_environment();
    tracing::info!("Testing panic info creation and formatting");
    
    // Create panic info with various configurations
    let basic_panic = CursedPanicInfo::new(
        "Basic panic test".to_string(),
        PanicSeverity::Recoverable,
        PanicCategory::User
    );
    
    assert_eq!(basic_panic.message, "Basic panic test");
    assert_eq!(basic_panic.severity, PanicSeverity::Recoverable);
    assert_eq!(basic_panic.category, PanicCategory::User);
    assert!(basic_panic.panic_id > 0, "Panic ID should be assigned");
    
    // Test with location
    let source_location = SourceLocation::new(25, 8).with_file("test_panic.csd");
    let panic_with_location = CursedPanicInfo::new(
        "Panic with location".to_string(),
        PanicSeverity::Critical,
        PanicCategory::TypeAssertion
    ).with_location(source_location);
    
    assert!(panic_with_location.source_location.is_some(), "Should have source location");
    
    // Test with goroutine
    let panic_with_goroutine = CursedPanicInfo::new(
        "Goroutine panic".to_string(),
        PanicSeverity::Fatal,
        PanicCategory::Goroutine
    ).with_goroutine(123);
    
    assert_eq!(panic_with_goroutine.goroutine_id, Some(123), "Should have goroutine ID");
    
    // Test with metadata
    let panic_with_metadata = CursedPanicInfo::new(
        "Panic with metadata".to_string(),
        PanicSeverity::Critical,
        PanicCategory::Memory
    ).with_metadata("context".to_string(), "test_context".to_string())
     .with_metadata("operation".to_string(), "allocation".to_string());
    
    assert_eq!(panic_with_metadata.metadata.len(), 2, "Should have metadata entries");
    assert_eq!(panic_with_metadata.metadata.get("context"), Some(&"test_context".to_string()));
    assert_eq!(panic_with_metadata.metadata.get("operation"), Some(&"allocation".to_string()));
    
    // Test formatting
    let formatted = format!("{}", basic_panic);
    assert!(formatted.contains("Basic panic test"), "Formatted output should contain message");
    assert!(formatted.contains("Recoverable"), "Formatted output should contain severity");
    assert!(formatted.contains("User"), "Formatted output should contain category");
    
    tracing::debug!(
        formatted_panic = %formatted,
        "Formatted panic info"
    );
    
    tracing::info!("Panic info creation and formatting test completed successfully");
}

/// Test different panic categories and severities
#[test]
fn test_panic_categories_and_severities() {
    init_test_environment();
    tracing::info!("Testing different panic categories and severities");
    
    // Test all panic categories
    let categories = vec![
        PanicCategory::Memory,
        PanicCategory::TypeAssertion,
        PanicCategory::BoundsCheck,
        PanicCategory::Arithmetic,
        PanicCategory::Channel,
        PanicCategory::Goroutine,
        PanicCategory::User,
        PanicCategory::System,
        PanicCategory::Generic,
    ];
    
    for category in categories {
        let panic_info = CursedPanicInfo::new(
            format!("Test panic for category: {:?}", category),
            PanicSeverity::Critical,
            category.clone()
        );
        
        assert_eq!(panic_info.category, category, "Category should match");
        tracing::debug!(category = ?category, "Created panic for category");
    }
    
    // Test all panic severities
    let severities = vec![
        PanicSeverity::Recoverable,
        PanicSeverity::Critical,
        PanicSeverity::Fatal,
    ];
    
    for severity in severities {
        let panic_info = CursedPanicInfo::new(
            format!("Test panic for severity: {:?}", severity),
            severity,
            PanicCategory::User
        );
        
        assert_eq!(panic_info.severity, severity, "Severity should match");
        tracing::debug!(severity = ?severity, "Created panic for severity");
    }
    
    // Test severity ordering
    assert!(PanicSeverity::Recoverable < PanicSeverity::Critical);
    assert!(PanicSeverity::Critical < PanicSeverity::Fatal);
    
    tracing::info!("Panic categories and severities test completed successfully");
}

/// Test comprehensive integration scenario
#[test]
fn test_comprehensive_integration_scenario() {
    init_test_environment();
    tracing::info!("Testing comprehensive integration scenario");
    
    // Initialize panic runtime
    let init_result = initialize_panic_runtime();
    assert!(init_result.is_ok(), "Runtime initialization should succeed");
    
    // Create LLVM code generator
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create code generator");
    
    // Declare all runtime functions
    let panic_decl_result = generator.declare_panic_runtime_functions();
    assert!(panic_decl_result.is_ok(), "Panic function declarations should succeed");
    
    let recovery_decl_result = generator.declare_recovery_runtime_functions();
    assert!(recovery_decl_result.is_ok(), "Recovery function declarations should succeed");
    
    // Create complex recovery statement with all features
    let protected_expr = Box::new(Literal::string("Complex protected operation"));
    let protected_block = Box::new(ExpressionStatement::from_expr(protected_expr));
    
    let recovery_expr = Box::new(Literal::string("Complex recovery handler"));
    let recovery_block = Box::new(ExpressionStatement::from_expr(recovery_expr));
    
    let error_var = Identifier::from_name("caught_error");
    
    let comprehensive_recovery = RecoveryStatement::new("catch".to_string(), protected_block)
        .with_recovery(recovery_block)
        .with_error_var(error_var);
    
    // Compile the comprehensive recovery statement
    let compile_result = generator.compile_recovery_statement(&comprehensive_recovery);
    assert!(compile_result.is_ok(), "Comprehensive recovery compilation should succeed");
    
    // Create and compile panic statement
    let panic_message = Box::new(Literal::string("Comprehensive panic test"));
    let panic_stmt = PanicStatement::new("yeet_error".to_string(), panic_message);
    
    let panic_compile_result = generator.compile_panic_statement(&panic_stmt);
    assert!(panic_compile_result.is_ok(), "Panic compilation should succeed");
    
    // Test some FFI functions
    cursed::runtime::panic::cursed_enter_recovery_mode();
    cursed::runtime::panic::cursed_mark_recovery_entry();
    cursed::runtime::panic::cursed_record_recovery_completion();
    cursed::runtime::panic::cursed_exit_recovery_mode();
    
    // Get runtime statistics
    if let Some(runtime) = get_panic_runtime() {
        if let Ok(stats) = runtime.get_statistics() {
            tracing::info!(
                total_panics = stats.total_panics,
                successful_recoveries = stats.successful_recoveries,
                "Final runtime statistics"
            );
        }
    }
    
    // Shutdown runtime
    let shutdown_result = shutdown_panic_runtime();
    assert!(shutdown_result.is_ok(), "Runtime shutdown should succeed");
    
    tracing::info!("Comprehensive integration scenario test completed successfully");
}
