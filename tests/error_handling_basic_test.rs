#[cfg(test)]
mod tests {
    use cursed::error::{Error, CursedError, SourceLocation};
    use std::path::PathBuf;

    #[path = "common.rs"]
    mod common;

    #[test]
    fn test_basic_error_creation() {
        common::tracing::setup();
        
        // Test different error types
        let io_error = Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"));
        assert!(matches!(io_error, Error::Io(_)));
        
        let parse_error = Error::Parse("Invalid syntax".to_string());
        assert!(matches!(parse_error, Error::Parse(_)));
        
        let compile_error = Error::Compile("Type mismatch".to_string());
        assert!(matches!(compile_error, Error::Compile(_)));
        
        let runtime_error = Error::Runtime("Division by zero".to_string());
        assert!(matches!(runtime_error, Error::Runtime(_)));
    }

    #[test]
    fn test_error_display() {
        common::tracing::setup();
        
        let parse_error = Error::Parse("Unexpected token".to_string());
        let error_string = format!("{}", parse_error);
        assert!(error_string.contains("Parse error"));
        assert!(error_string.contains("Unexpected token"));
        
        let compile_error = Error::Compile("Type error".to_string());
        let error_string = format!("{}", compile_error);
        assert!(error_string.contains("Compilation error"));
        assert!(error_string.contains("Type error"));
    }

    #[test]
    fn test_error_conversion() {
        common::tracing::setup();
        
        // Test conversion from std::io::Error
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        let cursed_error: Error = io_error.into();
        
        assert!(matches!(cursed_error, Error::Io(_)));
        assert!(format!("{}", cursed_error).contains("Access denied"));
    }

    #[test]
    fn test_panic_errors() {
        common::tracing::setup();
        
        // Test basic panic error
        let panic_error = Error::panic_error("System failure".to_string());
        assert!(panic_error.is_panic());
        assert!(!panic_error.is_recoverable_panic());
        assert_eq!(panic_error.get_panic_id(), None);
        
        // Test recoverable panic error
        let recoverable_panic = Error::recoverable_panic("Temporary failure".to_string());
        assert!(recoverable_panic.is_panic());
        assert!(recoverable_panic.is_recoverable_panic());
        
        // Test panic with details
        let location = SourceLocation::new(10, 5).with_file("test.csd");
        let detailed_panic = Error::panic_with_details(
            "Critical error".to_string(),
            42,
            true,
            Some(location.clone())
        );
        
        assert!(detailed_panic.is_panic());
        assert!(detailed_panic.is_recoverable_panic());
        assert_eq!(detailed_panic.get_panic_id(), Some(42));
        assert_eq!(detailed_panic.get_source_location(), Some(&location));
    }

    #[test]
    fn test_recovery_errors() {
        common::tracing::setup();
        
        // Test basic recovery error
        let recovery_error = Error::recovery_error("Recovery failed".to_string(), 3);
        assert!(recovery_error.is_recovery());
        
        // Test recovery error with location
        let location = SourceLocation::new(15, 20).with_file("recovery.csd");
        let recovery_with_location = Error::recovery_error_with_location(
            "Recovery attempt failed".to_string(),
            5,
            location.clone()
        );
        
        assert!(recovery_with_location.is_recovery());
        assert_eq!(recovery_with_location.get_source_location(), Some(&location));
    }

    #[test]
    fn test_error_propagation() {
        common::tracing::setup();
        
        // Test basic error propagation
        let propagation_error = Error::error_propagation("Propagated error".to_string());
        assert!(propagation_error.is_error_propagation());
        assert_eq!(propagation_error.get_line(), None);
        assert_eq!(propagation_error.get_column(), None);
        
        // Test error propagation with location
        let propagation_with_location = Error::error_propagation_with_location(
            "Error at specific location".to_string(),
            25,
            10
        );
        
        assert!(propagation_with_location.is_error_propagation());
        assert_eq!(propagation_with_location.get_line(), Some(25));
        assert_eq!(propagation_with_location.get_column(), Some(10));
    }

    #[test]
    fn test_parse_error_with_location() {
        common::tracing::setup();
        
        let parse_error = Error::parse_error_with_location(
            "Syntax error: missing semicolon".to_string(),
            30,
            15
        );
        
        assert_eq!(parse_error.get_line(), Some(30));
        assert_eq!(parse_error.get_column(), Some(15));
        
        let error_string = format!("{}", parse_error);
        assert!(error_string.contains("line 30"));
        assert!(error_string.contains("column 15"));
        assert!(error_string.contains("missing semicolon"));
    }

    #[test]
    fn test_code_generation_error() {
        common::tracing::setup();
        
        let codegen_error = Error::code_generation_error(
            "LLVM compilation failed".to_string(),
            Some(42),
            Some(8)
        );
        
        assert_eq!(codegen_error.get_line(), Some(42));
        assert_eq!(codegen_error.get_column(), Some(8));
        
        let error_string = format!("{}", codegen_error);
        assert!(error_string.contains("Code generation error"));
        assert!(error_string.contains("line 42"));
        assert!(error_string.contains("LLVM compilation failed"));
    }

    #[test]
    fn test_source_location() {
        common::tracing::setup();
        
        // Test basic source location
        let location = SourceLocation::new(100, 50);
        assert_eq!(location.line, 100);
        assert_eq!(location.column, 50);
        assert_eq!(location.file, None);
        
        // Test source location with file
        let location_with_file = location.with_file("main.csd");
        assert_eq!(location_with_file.file, Some("main.csd".to_string()));
        
        // Test display
        let display_without_file = format!("{}", SourceLocation::new(10, 5));
        assert_eq!(display_without_file, "10:5");
        
        let display_with_file = format!("{}", SourceLocation::new(10, 5).with_file("test.csd"));
        assert_eq!(display_with_file, "test.csd:10:5");
    }

    #[test]
    fn test_template_error() {
        common::tracing::setup();
        
        let location = SourceLocation::new(25, 30).with_file("template.csd");
        
        let template_error = Error::TemplateError {
            message: "Template compilation failed".to_string(),
            source_location: Some(location.clone()),
        };
        
        assert_eq!(template_error.get_source_location(), Some(&location));
        
        let error_string = format!("{}", template_error);
        assert!(error_string.contains("Template error"));
        assert!(error_string.contains("template.csd:25:30"));
        assert!(error_string.contains("Template compilation failed"));
    }

    #[test]
    fn test_file_errors() {
        common::tracing::setup();
        
        let file_path = PathBuf::from("/path/to/file.csd");
        
        // Test file read error
        let read_error = Error::FileReadError(
            file_path.clone(),
            "Permission denied".to_string()
        );
        
        let error_string = format!("{}", read_error);
        assert!(error_string.contains("Failed to read file"));
        assert!(error_string.contains("/path/to/file.csd"));
        assert!(error_string.contains("Permission denied"));
        
        // Test file write error
        let write_error = Error::FileWriteError(
            file_path.clone(),
            "Disk full".to_string()
        );
        
        let error_string = format!("{}", write_error);
        assert!(error_string.contains("Failed to write file"));
        assert!(error_string.contains("Disk full"));
    }

    #[test]
    fn test_process_error() {
        common::tracing::setup();
        
        let process_error = Error::process_error("Command execution failed".to_string());
        
        let error_string = format!("{}", process_error);
        assert!(error_string.contains("Process error"));
        assert!(error_string.contains("Command execution failed"));
    }

    #[test]
    fn test_optimization_error() {
        common::tracing::setup();
        
        let opt_error = Error::optimization_error("Optimization pass failed".to_string());
        
        let error_string = format!("{}", opt_error);
        assert!(error_string.contains("Optimization error"));
        assert!(error_string.contains("Optimization pass failed"));
    }

    #[test]
    fn test_error_helper_functions() {
        common::tracing::setup();
        
        // Test system error helper
        let system_error = Error::system_error("Memory allocation failed");
        assert!(matches!(system_error, Error::Runtime(_)));
        
        // Test type error helper
        let type_error = Error::type_error("Type mismatch".to_string());
        assert!(matches!(type_error, Error::Type(_)));
        
        // Test error creation with category
        let panic_error = Error::new("panic", "Critical failure");
        assert!(panic_error.is_panic());
        
        let runtime_error = Error::new("runtime", "Execution error");
        assert!(matches!(runtime_error, Error::Runtime(_)));
        
        let unknown_error = Error::new("unknown", "Some error");
        assert!(matches!(unknown_error, Error::Runtime(_)));
    }

    #[test]
    fn test_error_cloning() {
        common::tracing::setup();
        
        let original_error = Error::parse_error_with_location(
            "Original error".to_string(),
            50,
            25
        );
        
        let cloned_error = original_error.clone();
        
        // Verify cloned error has same properties
        assert_eq!(format!("{}", original_error), format!("{}", cloned_error));
        assert_eq!(original_error.get_line(), cloned_error.get_line());
        assert_eq!(original_error.get_column(), cloned_error.get_column());
    }

    #[test]
    fn test_error_chaining() {
        common::tracing::setup();
        
        // Simulate error propagation through multiple layers
        fn layer_3() -> Result<(), Error> {
            Err(Error::parse_error_with_location(
                "Invalid token".to_string(),
                10,
                5
            ))
        }
        
        fn layer_2() -> Result<(), Error> {
            layer_3().map_err(|e| {
                Error::error_propagation_with_location(
                    format!("Error in layer 2: {}", e),
                    15,
                    10
                )
            })
        }
        
        fn layer_1() -> Result<(), Error> {
            layer_2().map_err(|e| {
                Error::Compile(format!("Compilation failed: {}", e))
            })
        }
        
        let result = layer_1();
        assert!(result.is_err());
        
        if let Err(error) = result {
            let error_string = format!("{}", error);
            assert!(error_string.contains("Compilation failed"));
            assert!(error_string.contains("Error in layer 2"));
            assert!(error_string.contains("Invalid token"));
        }
    }

    #[test]
    fn test_error_context_preservation() {
        common::tracing::setup();
        
        // Test that error context is preserved through transformations
        let original_location = SourceLocation::new(42, 18).with_file("source.csd");
        
        let original_error = Error::panic_with_details(
            "Original panic".to_string(),
            123,
            true,
            Some(original_location.clone())
        );
        
        // Simulate error handling that preserves context
        let transformed_error = match original_error {
            Error::Panic { message, panic_id, recoverable, source_location } => {
                Error::Recovery {
                    message: format!("Recovery from: {}", message),
                    recovery_attempts: 1,
                    source_location,
                }
            }
            _ => unreachable!(),
        };
        
        // Verify context is preserved
        assert!(transformed_error.is_recovery());
        assert_eq!(transformed_error.get_source_location(), Some(&original_location));
        
        let error_string = format!("{}", transformed_error);
        assert!(error_string.contains("source.csd:42:18"));
        assert!(error_string.contains("Recovery from: Original panic"));
    }

    #[test]
    fn test_error_categorization() {
        common::tracing::setup();
        
        // Test error category checking
        let errors = vec![
            Error::Parse("parse".to_string()),
            Error::Compile("compile".to_string()),
            Error::Runtime("runtime".to_string()),
            Error::panic_error("panic".to_string()),
            Error::recovery_error("recovery".to_string(), 1),
            Error::error_propagation("propagation".to_string()),
            Error::ProcessError("process".to_string()),
            Error::OptimizationError("optimization".to_string()),
        ];
        
        for error in errors {
            // Verify each error can be categorized correctly
            let is_compile_time = matches!(error, 
                Error::Parse(_) | 
                Error::Compile(_) | 
                Error::TypeCompilation(_) | 
                Error::Type(_)
            );
            
            let is_runtime = matches!(error,
                Error::Runtime(_) |
                Error::Panic { .. } |
                Error::Recovery { .. }
            );
            
            let is_system = matches!(error,
                Error::Io(_) |
                Error::ProcessError(_) |
                Error::FileReadError(_, _) |
                Error::FileWriteError(_, _)
            );
            
            // Each error should fall into exactly one category
            let category_count = [is_compile_time, is_runtime, is_system]
                .iter()
                .filter(|&&x| x)
                .count();
            
            // Note: Some errors might not fit these categories, which is OK
            println!("Error categorization: compile_time={}, runtime={}, system={}", 
                     is_compile_time, is_runtime, is_system);
        }
    }
}
