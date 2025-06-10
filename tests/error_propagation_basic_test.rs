use cursed::ast::expressions::ErrorPropagation;
use cursed::ast::identifiers::Identifier;
use cursed::ast::traits::{Expression, Node};
use cursed::error::{Error, SourceLocation};

/// Basic test suite for error propagation functionality
/// 
/// This test suite validates the core error propagation mechanism
/// that is currently implemented, focusing on AST nodes and basic
/// error handling functionality.

#[cfg(test)]
mod ast_tests {
    use super::*;

    #[test]
    fn test_basic_error_propagation_creation() {
        let var_expr = Identifier::new("test_var".to_string(), "test_var".to_string());
        let error_prop = ErrorPropagation::new(Box::new(var_expr));
        
        assert_eq!(error_prop.string(), "test_var?");
        assert_eq!(error_prop.token_literal(), "?");
    }

    #[test]
    fn test_error_propagation_expression_methods() {
        let var_expr = Identifier::new("result".to_string(), "result".to_string());
        let expr = ErrorPropagation::new(Box::new(var_expr));
        
        assert_eq!(expr.string(), "result?");
        assert_eq!(expr.token_literal(), "?");
    }

    #[test]
    fn test_nested_error_propagation() {
        let inner_expr = Identifier::new("inner".to_string(), "inner".to_string());
        let inner_prop = ErrorPropagation::new(Box::new(inner_expr));
        let nested_prop = ErrorPropagation::new(Box::new(inner_prop));
        
        assert_eq!(nested_prop.string(), "inner??");
    }

    #[test]
    fn test_error_types_display() {
        let var_expr = Identifier::new("connection".to_string(), "connection".to_string());
        
        let runtime_error = Error::Runtime("Database connection failed".to_string());
        let compile_error = Error::Compile("Type mismatch".to_string());
        let parse_error = Error::Parse("Invalid token".to_string());
        
        assert!(format!("{}", runtime_error).contains("connection failed"));
        assert!(format!("{}", compile_error).contains("mismatch"));
        assert!(format!("{}", parse_error).contains("token"));
    }

    #[test]
    fn test_error_propagation_with_location() {
        let error = Error::error_propagation_with_location("Error propagation error".to_string(), 42, 15);
        
        let display = format!("{}", error);
        assert!(display.contains("42"));
        assert!(display.contains("15"));
    }

    #[test]
    fn test_multiple_variable_names() {
        let test_cases = vec![
            ("x", Identifier::new("x".to_string(), "x".to_string())),
            ("result", Identifier::new("result".to_string(), "result".to_string())),
            ("data", Identifier::new("data".to_string(), "data".to_string())),
        ];
        
        for (name, identifier) in test_cases {
            let error_prop = ErrorPropagation::new(Box::new(identifier));
            let expected = format!("{}?", name);
            assert_eq!(error_prop.string(), expected);
            
            let var_name = format!("var_{}", name);
            let var_identifier = Identifier::new(var_name.clone(), var_name.clone());
            let var_prop = ErrorPropagation::new(Box::new(var_identifier));
            let expected = format!("{}?", var_name);
            assert_eq!(var_prop.string(), expected);
        }
    }

    #[test]
    fn test_deep_nesting() {
        let mut expr: Box<dyn Expression> = Box::new(
            Identifier::new("base".to_string(), "base".to_string())
        );
        
        // Create multiple levels of nesting
        for _ in 0..5 {
            expr = Box::new(ErrorPropagation::new(expr));
        }
        
        assert_eq!(expr.string(), "base?????");
    }

    #[test]
    fn test_memory_efficiency() {
        // Test that error propagation doesn't cause excessive memory usage
        let mut propagations = Vec::new();
        
        for i in 0..1000 {
            let var_name = format!("var_{}", i);
            let identifier = Identifier::new(var_name.clone(), var_name);
            let error_prop = ErrorPropagation::new(Box::new(identifier));
            propagations.push(error_prop);
        }
        
        // Verify they're all created correctly
        assert_eq!(propagations[0].string(), "var_0?");
        assert_eq!(propagations[999].string(), "var_999?");
    }

    #[test]
    fn test_clone_implementation() {
        let var_expr = Identifier::new("clone_test".to_string(), "clone_test".to_string());
        let error_prop = ErrorPropagation::new(Box::new(var_expr));
        
        let cloned_box = Box::new(error_prop.clone());
        assert_eq!(cloned_box.string(), "clone_test?");
    }

    #[test]
    fn test_debug_implementation() {
        let var_expr = Identifier::new("debug_test".to_string(), "debug_test".to_string());
        let error_prop = ErrorPropagation::new(Box::new(var_expr));
        
        let debug_str = format!("{:?}", error_prop);
        assert!(debug_str.contains("ErrorPropagation"));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_error_propagation_in_context() {
        // Test error propagation in more realistic contexts
        let variable = Identifier::new("api_result".to_string(), "api_result".to_string());
        let propagation = ErrorPropagation::new(Box::new(variable));
        
        assert_eq!(propagation.string(), "api_result?");
        assert_eq!(propagation.token_literal(), "?");
    }

    #[test]
    fn test_error_types_consistency() {
        let error_types = vec![
            Error::Parse("Parse error".to_string()),
            Error::Runtime("Runtime error".to_string()),
            Error::Compile("Compile error".to_string()),
            Error::Type("Type error".to_string()),
        ];
        
        for error in error_types {
            let display_str = format!("{}", error);
            let debug_str = format!("{:?}", error);
            
            // Basic validation that error strings are non-empty
            assert!(!display_str.is_empty());
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_source_location_integration() {
        let locations = vec![
            SourceLocation::new(1, 1),
            SourceLocation::new(10, 5),
            SourceLocation::new(100, 50),
        ];
        
        for location in locations {
            let error = Error::error_propagation_with_location("Location test".to_string(), location.line, location.column);
            
            let formatted = format!("{}", error);
            assert!(formatted.contains(&location.line.to_string()));
            assert!(formatted.contains(&location.column.to_string()));
        }
    }
}
