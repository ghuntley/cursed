//! Basic Error Handling Tests for CURSED
//!
//! These tests validate the core error handling functionality that we can
//! test without depending on complex compilation infrastructure.

use cursed::error::{Error as CursedError, SourceLocation};

#[test]
fn test_error_creation_and_display() {
    // Test basic error creation
    let parse_error = CursedError::Parse("Invalid syntax".to_string());
    assert!(parse_error.to_string().contains("Parse error"));
    assert!(parse_error.to_string().contains("Invalid syntax"));
    
    let runtime_error = CursedError::Runtime("Null pointer dereference".to_string());
    assert!(runtime_error.to_string().contains("Runtime error"));
    assert!(runtime_error.to_string().contains("Null pointer dereference"));
    
    let type_error = CursedError::Type("Type mismatch".to_string());
    assert!(type_error.to_string().contains("Type error"));
    assert!(type_error.to_string().contains("Type mismatch"));
}

#[test]
fn test_source_location() {
    let location = SourceLocation::new(10, 5);
    assert_eq!(location.line, 10);
    assert_eq!(location.column, 5);
    assert_eq!(location.file, None);
    
    let location_with_file = location.with_file("test.csd");
    assert_eq!(location_with_file.file, Some("test.csd".to_string()));
    
    let display_string = format!("{}", location_with_file);
    assert!(display_string.contains("test.csd"));
    assert!(display_string.contains("10"));
    assert!(display_string.contains("5"));
}

#[test]
fn test_error_with_location() {
    let error = CursedError::parse_error_with_location(
        "Missing semicolon".to_string(),
        15,
        20,
    );
    
    assert!(error.to_string().contains("Parse error"));
    assert!(error.to_string().contains("line 15"));
    assert!(error.to_string().contains("column 20"));
    assert!(error.to_string().contains("Missing semicolon"));
    
    assert_eq!(error.get_line(), Some(15));
    assert_eq!(error.get_column(), Some(20));
}

#[test]
fn test_panic_errors() {
    let panic_error = CursedError::panic_error("Critical failure".to_string());
    assert!(panic_error.is_panic());
    assert!(!panic_error.is_recoverable_panic());
    
    let recoverable_panic = CursedError::recoverable_panic("Recoverable issue".to_string());
    assert!(recoverable_panic.is_panic());
    assert!(recoverable_panic.is_recoverable_panic());
    
    let panic_with_details = CursedError::panic_with_details(
        "Detailed panic".to_string(),
        12345,
        true,
        Some(SourceLocation::new(25, 30).with_file("panic_test.csd")),
    );
    assert!(panic_with_details.is_panic());
    assert!(panic_with_details.is_recoverable_panic());
    assert_eq!(panic_with_details.get_panic_id(), Some(12345));
    
    let location = panic_with_details.get_source_location();
    assert!(location.is_some());
    assert_eq!(location.unwrap().line, 25);
    assert_eq!(location.unwrap().column, 30);
}

#[test]
fn test_recovery_errors() {
    let recovery_error = CursedError::recovery_error("Recovery failed".to_string(), 3);
    assert!(recovery_error.is_recovery());
    assert!(recovery_error.to_string().contains("attempt 3"));
    
    let recovery_with_location = CursedError::recovery_error_with_location(
        "Recovery with location".to_string(),
        5,
        SourceLocation::new(40, 50).with_file("recovery_test.csd"),
    );
    assert!(recovery_with_location.is_recovery());
    assert!(recovery_with_location.to_string().contains("attempt 5"));
    assert!(recovery_with_location.to_string().contains("recovery_test.csd"));
}

#[test]
fn test_error_propagation_errors() {
    let error_prop = CursedError::error_propagation("Error propagation test".to_string());
    assert!(error_prop.is_error_propagation());
    assert!(error_prop.to_string().contains("Error propagation"));
    assert!(error_prop.to_string().contains("Error propagation test"));
    
    let error_prop_with_location = CursedError::error_propagation_with_location(
        "Located error propagation".to_string(),
        60,
        70,
    );
    assert!(error_prop_with_location.is_error_propagation());
    assert_eq!(error_prop_with_location.get_line(), Some(60));
    assert_eq!(error_prop_with_location.get_column(), Some(70));
}

#[test]
fn test_code_generation_errors() {
    let codegen_error = CursedError::code_generation_error(
        "LLVM compilation failed".to_string(),
        Some(80),
        Some(90),
    );
    
    assert!(codegen_error.to_string().contains("Code generation error"));
    assert!(codegen_error.to_string().contains("line 80"));
    assert!(codegen_error.to_string().contains("column 90"));
    assert!(codegen_error.to_string().contains("LLVM compilation failed"));
    
    assert_eq!(codegen_error.get_line(), Some(80));
    assert_eq!(codegen_error.get_column(), Some(90));
}

#[test]
fn test_error_convenience_methods() {
    // Test system error
    let system_error = CursedError::system_error("System failure");
    assert!(system_error.to_string().contains("System error"));
    
    // Test type error
    let type_error = CursedError::type_error("Invalid type".to_string());
    assert!(type_error.to_string().contains("Invalid type"));
    
    // Test optimization error
    let opt_error = CursedError::optimization_error("Optimization failed".to_string());
    assert!(opt_error.to_string().contains("Optimization error"));
    
    // Test generic error creation
    let generic_error = CursedError::new("custom", "Custom error message");
    assert!(generic_error.to_string().contains("Custom error message"));
}

#[test]
fn test_error_categorization() {
    let parse_error = CursedError::Parse("Parse test".to_string());
    let runtime_error = CursedError::Runtime("Runtime test".to_string());
    let type_error = CursedError::Type("Type test".to_string());
    let panic_error = CursedError::panic_error("Panic test".to_string());
    let recovery_error = CursedError::recovery_error("Recovery test".to_string(), 1);
    
    // Test error type checking
    assert!(panic_error.is_panic());
    assert!(!parse_error.is_panic());
    assert!(!runtime_error.is_panic());
    
    assert!(recovery_error.is_recovery());
    assert!(!parse_error.is_recovery());
    assert!(!type_error.is_recovery());
    
    // Test string representations contain expected content
    assert!(parse_error.to_string().contains("Parse"));
    assert!(runtime_error.to_string().contains("Runtime"));
    assert!(type_error.to_string().contains("Type"));
    assert!(panic_error.to_string().contains("Panic"));
    assert!(recovery_error.to_string().contains("Recovery"));
}

#[test]
fn test_error_cloning() {
    let original_error = CursedError::parse_error_with_location(
        "Original error".to_string(),
        100,
        200,
    );
    
    let cloned_error = original_error.clone();
    
    // Cloned error should have same content
    assert_eq!(original_error.to_string(), cloned_error.to_string());
    assert_eq!(original_error.get_line(), cloned_error.get_line());
    assert_eq!(original_error.get_column(), cloned_error.get_column());
    
    // Test cloning with location
    let error_with_location = CursedError::panic_with_details(
        "Panic with location".to_string(),
        999,
        true,
        Some(SourceLocation::new(300, 400).with_file("clone_test.csd")),
    );
    
    let cloned_with_location = error_with_location.clone();
    assert_eq!(error_with_location.get_panic_id(), cloned_with_location.get_panic_id());
    assert_eq!(error_with_location.is_recoverable_panic(), cloned_with_location.is_recoverable_panic());
    
    let original_location = error_with_location.get_source_location();
    let cloned_location = cloned_with_location.get_source_location();
    assert_eq!(original_location, cloned_location);
}

#[test]
fn test_error_from_io_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let cursed_error: CursedError = io_error.into();
    
    assert!(cursed_error.to_string().contains("I/O error"));
    assert!(cursed_error.to_string().contains("File not found"));
}

#[test]
fn test_error_compatibility() {
    // Test that CursedError implements required traits
    let error = CursedError::Runtime("Test error".to_string());
    
    // Should implement Debug
    let debug_string = format!("{:?}", error);
    assert!(debug_string.contains("Runtime"));
    
    // Should implement Display
    let display_string = format!("{}", error);
    assert!(display_string.contains("Runtime error"));
    
    // Should implement std::error::Error
    let std_error: &dyn std::error::Error = &error;
    assert!(std_error.to_string().contains("Runtime error"));
}

#[test]
fn test_template_errors() {
    let template_error = CursedError::TemplateError {
        message: "Template compilation failed".to_string(),
        source_location: Some(SourceLocation::new(500, 600).with_file("template.csd")),
    };
    
    assert!(template_error.to_string().contains("Template error"));
    assert!(template_error.to_string().contains("template.csd"));
    assert!(template_error.to_string().contains("500:600"));
    
    let location = template_error.get_source_location();
    assert!(location.is_some());
    assert_eq!(location.unwrap().line, 500);
    assert_eq!(location.unwrap().column, 600);
}

#[test]
fn test_comprehensive_error_display() {
    // Test that all error variants display correctly
    let errors = vec![
        CursedError::Parse("Parse error".to_string()),
        CursedError::Compile("Compile error".to_string()),
        CursedError::Runtime("Runtime error".to_string()),
        CursedError::Package("Package error".to_string()),
        CursedError::Repl("REPL error".to_string()),
        CursedError::TypeCompilation("Type compilation error".to_string()),
        CursedError::Type("Type error".to_string()),
    ];
    
    for error in errors {
        let display_string = error.to_string();
        // Each error should have a meaningful display representation
        assert!(!display_string.is_empty());
        assert!(display_string.len() > 10); // Should be descriptive
        
        // Should contain the error type
        let error_type = match error {
            CursedError::Parse(_) => "Parse",
            CursedError::Compile(_) => "Compilation",
            CursedError::Runtime(_) => "Runtime",
            CursedError::Package(_) => "Package",
            CursedError::Repl(_) => "REPL",
            CursedError::TypeCompilation(_) => "Type compilation",
            CursedError::Type(_) => "Type",
            _ => "Unknown",
        };
        
        assert!(display_string.contains(error_type));
    }
}

/// Documentation: Basic Error Handling Tests
/// 
/// These tests validate the fundamental error handling infrastructure
/// without requiring complex runtime or compilation systems. They ensure:
/// 
/// 1. **Error Creation**: All error types can be created correctly
/// 2. **Source Location**: Location information is preserved and displayed
/// 3. **Error Classification**: Errors can be properly categorized
/// 4. **Panic/Recovery**: Panic and recovery errors work as expected
/// 5. **Error Propagation**: Error propagation types function correctly
/// 6. **Cloning**: Errors can be cloned while preserving all information
/// 7. **Standard Traits**: Errors implement Debug, Display, and std::error::Error
/// 8. **Conversions**: Errors can be converted from standard library types
/// 
/// These basic tests provide confidence that the error handling foundation
/// is solid before testing more complex integration scenarios.

#[cfg(test)]
mod basic_error_test_utilities {
    use super::*;
    
    /// Helper to create test error with all location info
    pub fn create_full_test_error(
        message: &str,
        line: usize,
        column: usize,
        file: &str,
    ) -> CursedError {
        CursedError::parse_error_with_location(message.to_string(), line, column)
    }
    
    /// Helper to validate error contains expected content
    pub fn assert_error_contains(error: &CursedError, expected_content: &[&str]) {
        let error_string = error.to_string();
        for content in expected_content {
            assert!(
                error_string.contains(content),
                "Error '{}' does not contain '{}'",
                error_string,
                content
            );
        }
    }
    
    /// Helper to test error trait implementations
    pub fn validate_error_traits(error: &CursedError) {
        // Test Debug
        let debug_str = format!("{:?}", error);
        assert!(!debug_str.is_empty());
        
        // Test Display
        let display_str = format!("{}", error);
        assert!(!display_str.is_empty());
        
        // Test std::error::Error
        let std_error: &dyn std::error::Error = error;
        assert!(!std_error.to_string().is_empty());
    }
}
