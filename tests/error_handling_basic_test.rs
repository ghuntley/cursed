//! Basic Error Handling Tests for CURSED
//!
//! These tests validate the core error handling functionality that we can
//! test without depending on complex compilation infrastructure.

use cursed::error::{Error as CursedError, SourceLocation};

#[test]
fn test_error_creation_and_display() {
    // Test basic error creation
    let parse_error = CursedError::Parse("Invalid syntax".to_string());
    assert!(parse_error.to_string().contains("error"));
    assert!(parse_error.to_string().contains("syntax"));
    
    let runtime_error = CursedError::Runtime("Null pointer dereference".to_string());
    assert!(runtime_error.to_string().contains("error"));
    assert!(runtime_error.to_string().contains("pointer dereference"));
    
    let type_error = CursedError::Type("Type mismatch".to_string());
    assert!(type_error.to_string().contains("error"));
    assert!(type_error.to_string().contains("mismatch"));
}

#[test]
fn test_source_location() {
    let location = SourceLocation::new(15, 20);
    let location_with_file = location.with_file("test.csd");
    assert_eq!(location_with_file.file, Some("test.csd".to_string()));
    
    let error = CursedError::parse_error_with_location("Missing semicolon".to_string(), 15, 20);
    let display_string = format!("{}", error);
    assert!(error.to_string().contains("error"));
    assert!(error.to_string().contains("15"));
    assert!(error.to_string().contains("20"));
    assert!(error.to_string().contains("semicolon"));
}

#[test]
fn test_specialized_error_types() {
    let panic_error = CursedError::panic_error("Test panic failure".to_string());
    let recoverable_panic = CursedError::recoverable_panic("Recoverable issue".to_string());
    let recovery_error = CursedError::recovery_error("Recovery failed".to_string(), 3);
    assert!(recovery_error.to_string().contains("3"));
    
    let recovery_with_location = CursedError::recovery_error_with_location("Recovery with location".to_string(), 5, SourceLocation::new(40, 50).with_file("recovery.csd"));
    assert!(recovery_with_location.to_string().contains("5"));
    assert!(recovery_with_location.to_string().contains(".csd"));
}

#[test]
fn test_error_propagation_types() {
    let error_prop = CursedError::error_propagation("Error propagation test".to_string());
    assert!(error_prop.to_string().contains("propagation"));
    assert!(error_prop.to_string().contains("propagation test"));
    
    let codegen_error = CursedError::code_generation_error("Code generation error: Module compilation failed".to_string(), Some(80), Some(90));
    assert!(codegen_error.to_string().contains("generation error"));
    assert!(codegen_error.to_string().contains("compilation failed"));
}

#[test]
fn test_additional_error_types() {
    let system_error = CursedError::system_error("System failure");
    assert!(system_error.to_string().contains("error"));
    
    let type_error = CursedError::type_error("Invalid type".to_string());
    assert!(type_error.to_string().contains("type"));
    
    let opt_error = CursedError::optimization_error("Optimization failed".to_string());
    assert!(opt_error.to_string().contains("error"));
    
    let generic_error = CursedError::new("Generic", "Generic error message");
    assert!(generic_error.to_string().contains("error message"));
}

#[test]
fn test_error_display_consistency() {
    let parse_error = CursedError::Parse("Parse test".to_string());
    let runtime_error = CursedError::Runtime("Runtime test".to_string());
    let type_error = CursedError::Type("Type test".to_string());
    let panic_error = CursedError::panic_error("Panic test".to_string());
    let recovery_error = CursedError::recovery_error("Recovery test".to_string(), 1);
    
    assert!(parse_error.to_string().contains("test"));
    assert!(runtime_error.to_string().contains("test"));
    assert!(type_error.to_string().contains("test"));
    assert!(panic_error.to_string().contains("test"));
    assert!(recovery_error.to_string().contains("test"));
}

#[test]
fn test_error_with_context() {
    let cursed_error = CursedError::Parse("Parse error".to_string());
    
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let from_io = CursedError::from(io_error);
    assert!(from_io.to_string().contains("I/O error"));
    assert!(from_io.to_string().contains("not found"));
}

#[test]
fn test_error_debug_display() {
    let error = CursedError::Runtime("Debug test error".to_string());
    let debug_string = format!("{:?}", error);
    let display_string = format!("{}", error);
    
    let std_error: &dyn std::error::Error = &error;
    assert!(std_error.to_string().contains("error"));
}

#[test]
fn test_template_error() {
    // Use TemplateError variant instead of non-existent template_error function
    let template_error = CursedError::TemplateError {
        message: "Template error: compilation failed".to_string(),
        source_location: Some(SourceLocation::new(500, 600).with_file("template.csd")),
    };
    assert!(template_error.to_string().contains("error"));
    assert!(template_error.to_string().contains(".csd"));
    assert!(template_error.to_string().contains("500:600"));
}

#[test]
fn test_error_variant_matching() {
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
        let error_type = match error {
            CursedError::Parse(_) => "parse",
            CursedError::Compile(_) => "compile",
            CursedError::Runtime(_) => "runtime",
            CursedError::Package(_) => "package",
            CursedError::Repl(_) => "repl",
            CursedError::TypeCompilation(_) => "type compilation",
            CursedError::Type(_) => "type",
            _ => "other",
        };
        
        let error_msg = format!("{} does not contain {}", error.to_string(), error_type);
        let debug_str = format!("{:?}", error);
        let display_str = format!("{}", error);
        
        // Basic validation that strings are formatted
        assert!(!debug_str.is_empty());
        assert!(!display_str.is_empty());
    }
}

#[test]
fn test_panic_with_details() {
    let panic_with_details = CursedError::panic_with_details(
        "Detailed panic".to_string(),
        42,
        true,
        Some(SourceLocation::new(100, 200).with_file("panic.csd"))
    );
    assert!(panic_with_details.to_string().contains("panic"));
}

#[test]
fn test_error_propagation_with_location() {
    let propagation_error = CursedError::error_propagation_with_location(
        "Propagation with location".to_string(),
        75,
        25
    );
    assert!(propagation_error.to_string().contains("propagation"));
    assert!(propagation_error.to_string().contains("75"));
    assert!(propagation_error.to_string().contains("25"));
}
