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
        let var_expr = Identifier::new("result".to_string(), "result".to_string());
        let error_prop = ErrorPropagation::new(Box::new(var_expr));
        
        assert_eq!(error_prop.string(), "result?");
        assert_eq!(error_prop.token_literal(), "?");
    }
    
    #[test]
    fn test_error_propagation_expression_trait() {
        let var_expr = Identifier::new("api_result".to_string(), "api_result".to_string());
        let error_prop = ErrorPropagation::new(Box::new(var_expr));
        
        // Test that it implements Expression trait
        let expr: Box<dyn Expression> = Box::new(error_prop);
        assert_eq!(expr.string(), "api_result?");
        assert_eq!(expr.token_literal(), "?");
    }
    
    #[test]
    fn test_nested_error_propagation() {
        let inner_expr = Identifier::new("inner".to_string(), "inner".to_string());
        let first_prop = ErrorPropagation::new(Box::new(inner_expr));
        let nested_prop = ErrorPropagation::new(Box::new(first_prop));
        
        assert_eq!(nested_prop.string(), "inner??");
    }
    
    #[test]
    fn test_error_propagation_cloning() {
        let var_expr = Identifier::new("test".to_string(), "test".to_string());
        let error_prop = ErrorPropagation::new(Box::new(var_expr));
        
        let cloned = error_prop.clone();
        assert_eq!(cloned.string(), error_prop.string());
        assert_eq!(cloned.token_literal(), error_prop.token_literal());
    }
}

#[cfg(test)]
mod error_types_tests {
    use super::*;
    
    #[test]
    fn test_basic_error_creation() {
        let runtime_error = Error::Runtime("Database connection failed".to_string());
        let compile_error = Error::Compile("Type mismatch".to_string());
        let parse_error = Error::Parse("Unexpected token".to_string());
        
        // Test that errors can be created and displayed
        assert!(format!("{}", runtime_error).contains("Database connection failed"));
        assert!(format!("{}", compile_error).contains("Type mismatch"));
        assert!(format!("{}", parse_error).contains("Unexpected token"));
    }
    
    #[test]
    fn test_error_propagation_error_creation() {
        let error = Error::ErrorPropagation {
            message: "Test propagation error".to_string(),
            line: Some(1),
            column: Some(5),
        };
        
        let display = format!("{}", error);
        assert!(display.contains("Test propagation error"));
    }
    
    #[test]
    fn test_source_location() {
        let location = SourceLocation::new(10, 15);
        assert_eq!(location.line, 10);
        assert_eq!(location.column, 15);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_error_propagation_with_different_expressions() {
        // Test error propagation with various expression types
        let expressions = vec![
            ("variable", Identifier::new("variable".to_string(), "variable".to_string())),
            ("result", Identifier::new("result".to_string(), "result".to_string())),
            ("api_call", Identifier::new("api_call".to_string(), "api_call".to_string())),
        ];
        
        for (name, expr) in expressions {
            let error_prop = ErrorPropagation::new(Box::new(expr));
            let expected = format!("{}?", name);
            assert_eq!(error_prop.string(), expected);
        }
    }
    
    #[test]
    fn test_multiple_error_propagations() {
        // Test creating multiple error propagation expressions
        let mut propagations = Vec::new();
        
        for i in 0..10 {
            let var_name = format!("result_{}", i);
            let var_expr = Identifier::new(var_name.clone(), var_name);
            let error_prop = ErrorPropagation::new(Box::new(var_expr));
            propagations.push(error_prop);
        }
        
        assert_eq!(propagations.len(), 10);
        
        for (i, prop) in propagations.iter().enumerate() {
            let expected = format!("result_{}?", i);
            assert_eq!(prop.string(), expected);
        }
    }
    
    #[test]
    fn test_deeply_nested_propagation() {
        // Test deeply nested error propagation
        let mut expr: Box<dyn Expression> = Box::new(
            Identifier::new("base".to_string(), "base".to_string())
        );
        
        // Create 5 levels of nesting
        for _ in 0..5 {
            expr = Box::new(ErrorPropagation::new(expr));
        }
        
        assert_eq!(expr.string(), "base?????");
    }
    
    #[test]
    fn test_error_propagation_memory_usage() {
        // Test that error propagation doesn't cause excessive memory usage
        let mut propagations = Vec::new();
        
        // Create many error propagation expressions
        for i in 0..1000 {
            let var_name = format!("var_{}", i);
            let var_expr = Identifier::new(var_name.clone(), var_name);
            let error_prop = ErrorPropagation::new(Box::new(var_expr));
            propagations.push(error_prop);
        }
        
        // Verify they're all created correctly
        assert_eq!(propagations.len(), 1000);
        assert_eq!(propagations[0].string(), "var_0?");
        assert_eq!(propagations[999].string(), "var_999?");
        
        // Test string representation performance
        let start = std::time::Instant::now();
        for prop in &propagations {
            let _ = prop.string();
        }
        let duration = start.elapsed();
        
        // Should be fast (less than 10ms for 1000 operations)
        assert!(duration.as_millis() < 10);
    }
}

#[cfg(test)]
mod compatibility_tests {
    use super::*;
    
    #[test]
    fn test_error_propagation_as_any() {
        let var_expr = Identifier::new("test".to_string(), "test".to_string());
        let error_prop = ErrorPropagation::new(Box::new(var_expr));
        
        // Test as_any functionality
        let any_ref = error_prop.as_any();
        assert!(any_ref.is::<ErrorPropagation>());
    }
    
    #[test]
    fn test_error_propagation_clone_box() {
        let var_expr = Identifier::new("test".to_string(), "test".to_string());
        let error_prop = ErrorPropagation::new(Box::new(var_expr));
        
        // Test clone_box functionality
        let cloned_box = error_prop.clone_box();
        assert_eq!(cloned_box.string(), "test?");
    }
    
    #[test]
    fn test_error_propagation_debug() {
        let var_expr = Identifier::new("debug_test".to_string(), "debug_test".to_string());
        let error_prop = ErrorPropagation::new(Box::new(var_expr));
        
        // Test Debug implementation
        let debug_string = format!("{:?}", error_prop);
        assert!(debug_string.contains("ErrorPropagation"));
    }
    
    #[test]
    fn test_node_trait_implementation() {
        let var_expr = Identifier::new("node_test".to_string(), "node_test".to_string());
        let error_prop = ErrorPropagation::new(Box::new(var_expr));
        
        // Test Node trait methods
        assert_eq!(error_prop.string(), "node_test?");
        assert_eq!(error_prop.token_literal(), "?");
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;
    
    #[test]
    fn test_empty_identifier_propagation() {
        let empty_expr = Identifier::new("".to_string(), "".to_string());
        let error_prop = ErrorPropagation::new(Box::new(empty_expr));
        
        assert_eq!(error_prop.string(), "?");
    }
    
    #[test]
    fn test_long_identifier_propagation() {
        let long_name = "a".repeat(1000);
        let long_expr = Identifier::new(long_name.clone(), long_name);
        let error_prop = ErrorPropagation::new(Box::new(long_expr));
        
        let expected = format!("{}?", "a".repeat(1000));
        assert_eq!(error_prop.string(), expected);
    }
    
    #[test]
    fn test_special_character_identifiers() {
        let special_chars = vec!["_", "underscore_var", "var123", "camelCase", "snake_case"];
        
        for name in special_chars {
            let expr = Identifier::new(name.to_string(), name.to_string());
            let error_prop = ErrorPropagation::new(Box::new(expr));
            let expected = format!("{}?", name);
            assert_eq!(error_prop.string(), expected);
        }
    }
    
    #[test]
    fn test_unicode_identifier_propagation() {
        let unicode_names = vec!["变量", "переменная", "متغير", "変数"];
        
        for name in unicode_names {
            let expr = Identifier::new(name.to_string(), name.to_string());
            let error_prop = ErrorPropagation::new(Box::new(expr));
            let expected = format!("{}?", name);
            assert_eq!(error_prop.string(), expected);
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[test]
    fn test_error_propagation_creation_performance() {
        let start = std::time::Instant::now();
        
        // Create many error propagations quickly
        for i in 0..10000 {
            let var_name = format!("perf_test_{}", i);
            let var_expr = Identifier::new(var_name.clone(), var_name);
            let _error_prop = ErrorPropagation::new(Box::new(var_expr));
        }
        
        let duration = start.elapsed();
        
        // Should be very fast (less than 100ms for 10,000 operations)
        assert!(duration.as_millis() < 100);
    }
    
    #[test]
    fn test_string_generation_performance() {
        // Pre-create error propagations
        let mut propagations = Vec::new();
        for i in 0..1000 {
            let var_name = format!("str_test_{}", i);
            let var_expr = Identifier::new(var_name.clone(), var_name);
            let error_prop = ErrorPropagation::new(Box::new(var_expr));
            propagations.push(error_prop);
        }
        
        let start = std::time::Instant::now();
        
        // Generate strings for all propagations
        for prop in &propagations {
            let _ = prop.string();
        }
        
        let duration = start.elapsed();
        
        // Should be fast (less than 10ms for 1000 string generations)
        assert!(duration.as_millis() < 10);
    }
    
    #[test]
    fn test_nested_propagation_performance() {
        let start = std::time::Instant::now();
        
        // Create deeply nested propagation
        let mut expr: Box<dyn Expression> = Box::new(
            Identifier::new("deep".to_string(), "deep".to_string())
        );
        
        for _ in 0..100 {
            expr = Box::new(ErrorPropagation::new(expr));
        }
        
        let duration_creation = start.elapsed();
        
        // Creation should be fast
        assert!(duration_creation.as_millis() < 50);
        
        let start_string = std::time::Instant::now();
        let result = expr.string();
        let duration_string = start_string.elapsed();
        
        // String generation should be reasonable
        assert!(duration_string.as_millis() < 10);
        
        // Verify the result
        assert_eq!(result, format!("deep{}", "?".repeat(100)));
    }
}
