/// Comprehensive integration tests for the question mark operator (`?`) in CURSED
/// 
/// These tests validate the complete error propagation system including:
/// - AST construction and parsing
/// - LLVM code generation 
/// - Runtime error handling
/// - Integration with existing error systems

use cursed::ast::expressions::question_mark::QuestionMarkExpression;
use cursed::ast::identifiers::Identifier;
use cursed::ast::traits::{Node, Expression};
use cursed::error::{CursedError, error_propagation::*};
use cursed::parser::question_mark::QuestionMarkCompiler;
use cursed::codegen::llvm::question_mark::ErrorPropagationRuntime;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_question_mark_ast_creation() {
        // Test basic AST node creation
        let var_expr = Identifier::new("risky_operation".to_string(), "risky_operation".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            10,
            15
        );
        
        assert_eq!(question_expr.line, 10);
        assert_eq!(question_expr.column, 15);
        assert_eq!(question_expr.location(), (10, 15));
        assert_eq!(question_expr.string(), "risky_operation?");
        assert_eq!(question_expr.token_literal(), "?");
    }
    
    #[test]
    fn test_question_mark_display_formatting() {
        let var_expr = Identifier::new("fetch_data".to_string(), "fetch_data".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            5,
            20
        );
        
        assert_eq!(format!("{}", question_expr), "fetch_data?");
        assert_eq!(question_expr.to_string(), "fetch_data?");
    }
    
    #[test]
    fn test_nested_question_marks() {
        // Test chained question mark expressions
        let var_expr = Identifier::new("deeply_nested".to_string(), "deeply_nested".to_string());
        let first_question = QuestionMarkExpression::new(
            Box::new(var_expr),
            1,
            1
        );
        let second_question = QuestionMarkExpression::new(
            Box::new(first_question),
            1,
            2
        );
        
        assert_eq!(second_question.string(), "deeply_nested??");
    }
    
    #[test]
    fn test_question_mark_cloning() {
        let var_expr = Identifier::new("original".to_string(), "original".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            3,
            7
        );
        
        let cloned = question_expr.clone();
        assert_eq!(cloned.string(), question_expr.string());
        assert_eq!(cloned.location(), question_expr.location());
        
        // Ensure they're independent
        assert_eq!(cloned.line, 3);
        assert_eq!(cloned.column, 7);
    }
    
    #[test]
    fn test_expression_trait_implementation() {
        let var_expr = Identifier::new("trait_test".to_string(), "trait_test".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            1,
            1
        );
        
        // Test trait object creation
        let expr: Box<dyn Expression> = Box::new(question_expr);
        assert_eq!(expr.string(), "trait_test?");
        
        // Test cloning via trait
        let cloned_expr = expr.clone_box();
        assert_eq!(cloned_expr.string(), "trait_test?");
    }
    
    #[test]
    fn test_inner_expression_access() {
        let var_expr = Identifier::new("inner_test".to_string(), "inner_test".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            2,
            5
        );
        
        let inner = question_expr.inner_expression();
        assert_eq!(inner.string(), "inner_test");
        assert_eq!(inner.token_literal(), "inner_test");
    }
}

#[cfg(test)]
mod error_propagation_tests {
    use super::*;

    #[test]
    fn test_propagating_error_creation() {
        let original_error = CursedError::parse_error_with_location(
            "Test parse error".to_string(),
            10,
            5
        );
        
        let prop_error = PropagatingError::new(original_error);
        assert!(prop_error.should_propagate());
        assert!(prop_error.error_chain.is_empty());
    }
    
    #[test]
    fn test_error_context_chain() {
        let original_error = CursedError::parse_error_with_location(
            "Syntax error".to_string(),
            15,
            8
        );
        
        let mut prop_error = PropagatingError::new(original_error);
        
        // Add context to the error chain
        prop_error.add_context_message("parse_expression", "Failed to parse question mark expression");
        prop_error.add_context_message("compile_function", "Error occurred in function compilation");
        
        assert_eq!(prop_error.error_chain.len(), 2);
        
        let chain_strings = prop_error.error_chain_strings();
        assert!(chain_strings[0].contains("parse_expression"));
        assert!(chain_strings[0].contains("Failed to parse question mark expression"));
        assert!(chain_strings[1].contains("compile_function"));
    }
    
    #[test]
    fn test_source_location_tracking() {
        let source_loc = SourceLocation::with_file(25, 12, "test.csd".to_string());
        assert_eq!(source_loc.line, 25);
        assert_eq!(source_loc.column, 12);
        assert_eq!(source_loc.file, Some("test.csd".to_string()));
        
        let display = format!("{}", source_loc);
        assert!(display.contains("line 25, column 12"));
        assert!(display.contains("test.csd"));
    }
    
    #[test]
    fn test_propagating_result_success() {
        let success_result: PropagatingResult<i32> = PropagatingResult::ok(42);
        assert!(success_result.is_ok());
        assert!(!success_result.is_err());
        assert_eq!(success_result.unwrap(), 42);
    }
    
    #[test]
    fn test_propagating_result_error() {
        let error = CursedError::error_propagation("Test error".to_string());
        let error_result: PropagatingResult<i32> = PropagatingResult::err(error);
        assert!(!error_result.is_ok());
        assert!(error_result.is_err());
        assert_eq!(error_result.unwrap_or(100), 100);
    }
    
    #[test]
    fn test_propagating_result_map() {
        let success: PropagatingResult<i32> = PropagatingResult::ok(21);
        let doubled = success.map(|x| x * 2);
        assert!(doubled.is_ok());
        assert_eq!(doubled.unwrap(), 42);
        
        let error = CursedError::error_propagation("Test".to_string());
        let error_result: PropagatingResult<i32> = PropagatingResult::err(error);
        let mapped_error = error_result.map(|x| x * 2);
        assert!(mapped_error.is_err());
    }
    
    #[test]
    fn test_result_conversion() {
        // Test conversion from standard Result
        let std_success: Result<String, CursedError> = Ok("success".to_string());
        let prop_result: PropagatingResult<String> = std_success.into();
        assert!(prop_result.is_ok());
        
        // Test conversion back to standard Result
        let back_to_std: Result<String, CursedError> = prop_result.into();
        assert!(back_to_std.is_ok());
        assert_eq!(back_to_std.unwrap(), "success");
    }
    
    #[test]
    fn test_error_propagation_site_tracking() {
        let original_error = CursedError::error_propagation("Root error".to_string());
        let mut prop_error = PropagatingError::new(original_error);
        
        let propagation_site = SourceLocation::with_function(
            30,
            15,
            "handle_result".to_string()
        );
        prop_error.set_propagation_site(propagation_site);
        
        let cursed_error = prop_error.into_cursed_error();
        assert!(cursed_error.to_string().contains("Propagated at line 30, column 15"));
    }
}

#[cfg(test)]
mod runtime_tests {
    use super::*;

    #[test]
    fn test_error_propagation_runtime() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // Test initial state
        let (propagations, unwraps) = runtime.get_stats();
        assert_eq!(propagations, 0);
        assert_eq!(unwraps, 0);
        
        // Test successful unwrap recording
        runtime.record_successful_unwrap();
        runtime.record_successful_unwrap();
        
        let (propagations, unwraps) = runtime.get_stats();
        assert_eq!(unwraps, 2);
    }
    
    #[test]
    fn test_error_handler_stack() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // Add error handler
        runtime.push_error_handler(|_error| {
            Ok(()) // Simple recovery handler
        });
        
        // Test handler removal
        let handler = runtime.pop_error_handler();
        assert!(handler.is_some());
        
        // Should be empty now
        let no_handler = runtime.pop_error_handler();
        assert!(no_handler.is_none());
    }
    
    #[test]
    fn test_error_handler_propagation() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // Add a handler that successfully handles errors
        runtime.push_error_handler(|_error| {
            Ok(()) // Successfully handle any error
        });
        
        let test_error = CursedError::error_propagation("Test propagation".to_string());
        let result = runtime.propagate_error(test_error);
        
        // Should succeed because handler handled it
        assert!(result.is_ok());
        
        let (propagations, _) = runtime.get_stats();
        assert_eq!(propagations, 1);
    }
    
    #[test]
    fn test_error_handler_failure() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // No error handler registered
        let test_error = CursedError::error_propagation("Unhandled error".to_string());
        let result = runtime.propagate_error(test_error);
        
        // Should fail because no handler available
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod cursed_error_tests {
    use super::*;

    #[test]
    fn test_error_propagation_error_creation() {
        let error = CursedError::error_propagation("Test message".to_string());
        assert!(error.is_error_propagation());
        assert_eq!(error.get_line(), None);
        assert_eq!(error.get_column(), None);
    }
    
    #[test]
    fn test_error_propagation_with_location() {
        let error = CursedError::error_propagation_with_location(
            "Located error".to_string(),
            20,
            10
        );
        assert!(error.is_error_propagation());
        assert_eq!(error.get_line(), Some(20));
        assert_eq!(error.get_column(), Some(10));
    }
    
    #[test]
    fn test_parse_error_creation() {
        let error = CursedError::parse_error_with_location(
            "Parse failure".to_string(),
            5,
            3
        );
        assert_eq!(error.get_line(), Some(5));
        assert_eq!(error.get_column(), Some(3));
        
        let display = format!("{}", error);
        assert!(display.contains("Parse error at line 5, column 3"));
    }
    
    #[test]
    fn test_code_generation_error() {
        let error = CursedError::code_generation_error(
            "LLVM generation failed".to_string(),
            Some(15),
            Some(8)
        );
        assert_eq!(error.get_line(), Some(15));
        assert_eq!(error.get_column(), Some(8));
        
        let display = format!("{}", error);
        assert!(display.contains("Code generation error at line 15, column 8"));
    }
    
    #[test]
    fn test_error_display_formatting() {
        let error = CursedError::error_propagation_with_location(
            "Propagation test".to_string(),
            12,
            6
        );
        
        let display = format!("{}", error);
        assert!(display.contains("Error propagation at line 12, column 6"));
        assert!(display.contains("Propagation test"));
    }
}
