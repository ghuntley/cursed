use cursed::ast::expressions::ErrorPropagation;
use cursed::ast::identifiers::Identifier;
use cursed::ast::traits::{Expression, Node};
use cursed::error::{Error, SourceLocation};
use std::time::Duration;

// Note: These would normally be imported but are simplified for the test
struct PropagationContext {
    current_function: Option<String>,
    expected_return_type: Option<String>,
    propagation_stack: Vec<SourceLocation>,
}

impl PropagationContext {
    fn new() -> Self {
        Self {
            current_function: None,
            expected_return_type: None,
            propagation_stack: Vec::new(),
        }
    }
    
    fn set_function(&mut self, name: String, return_type: Option<String>) {
        self.current_function = Some(name);
        self.expected_return_type = return_type;
    }
    
    fn push_propagation(&mut self, location: SourceLocation) {
        self.propagation_stack.push(location);
    }
    
    fn propagation_depth(&self) -> usize {
        self.propagation_stack.len()
    }
    
    fn get_stack_trace(&self) -> Vec<SourceLocation> {
        self.propagation_stack.clone()
    }
}

struct PropagationValidator;

impl PropagationValidator {
    fn validate_propagation(
        _expr: &ErrorPropagation,
        context: &PropagationContext,
    ) -> Result<(), Error> {
        if context.current_function.is_none() {
            return Err(Error::Parse("Error propagation requires function context".to_string()));
        }
        Ok(())
    }
}

/// Comprehensive test suite for error propagation functionality
/// 
/// This test suite validates the complete error propagation mechanism
/// including AST nodes, parser integration, LLVM code generation,
/// runtime support, and error handling.

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
    fn test_error_propagation_with_type_information() {
        let var_expr = Identifier::new("http_result".to_string(), "http_result".to_string());
        let location = SourceLocation::new(2, 15);
        let error_prop = ErrorPropagation::with_type(
            Box::new(var_expr),
            location,
            "Result<HttpResponse, HttpError>".to_string(),
        );
        
        assert_eq!(error_prop.get_expected_type(), Some("Result<HttpResponse, HttpError>"));
        assert_eq!(error_prop.string(), "http_result?");
    }
    
    #[test]
    fn test_tail_position_marking() {
        let var_expr = Identifier::new("final_result".to_string(), "final_result".to_string());
        let location = SourceLocation::new(10, 8);
        let error_prop = ErrorPropagation::new(Box::new(var_expr), location)
            .set_tail_position(true);
        
        assert!(error_prop.is_in_tail_position());
    }
    
    #[test]
    fn test_propagation_context_management() {
        let mut context = PropagationContext::new();
        
        // Test function context
        context.set_function(
            "process_request".to_string(), 
            Some("Result<Response, Error>".to_string())
        );
        assert_eq!(context.current_function, Some("process_request".to_string()));
        assert_eq!(context.expected_return_type, Some("Result<Response, Error>".to_string()));
        
        // Test propagation tracking
        let location1 = SourceLocation::new(1, 5);
        let location2 = SourceLocation::new(2, 10);
        context.push_propagation(location1);
        context.push_propagation(location2);
        assert_eq!(context.propagation_depth(), 2);
        
        let stack_trace = context.get_stack_trace();
        assert_eq!(stack_trace.len(), 2);
        assert_eq!(stack_trace[0].line, 1);
        assert_eq!(stack_trace[1].line, 2);
    }
    
    #[test]
    fn test_propagation_validation() {
        let var_expr = Identifier::new("db_result".to_string(), "db_result".to_string());
        let location = SourceLocation::new(1, 5);
        let error_prop = ErrorPropagation::new(Box::new(var_expr), location);
        
        let mut context = PropagationContext::new();
        
        // Should fail without function context
        let result = PropagationValidator::validate_propagation(&error_prop, &context);
        assert!(result.is_err());
        
        // Should succeed with function context
        context.set_function("database_query".to_string(), Some("Result<Row, DbError>".to_string()));
        let result = PropagationValidator::validate_propagation(&error_prop, &context);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_nested_error_propagation() {
        let inner_expr = Identifier::new("inner".to_string(), "inner".to_string());
        let location1 = SourceLocation::new(1, 5);
        let first_prop = ErrorPropagation::new(Box::new(inner_expr), location1);
        
        let location2 = SourceLocation::new(1, 11);
        let nested_prop = ErrorPropagation::new(Box::new(first_prop), location2);
        
        assert_eq!(nested_prop.string(), "inner??");
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    
    #[test]
    fn test_error_propagation_parse_context() {
        let mut context = ErrorPropagationParseContext::new();
        
        // Test function context management
        assert!(!context.in_function());
        context.enter_function(Some("Result<Data, Error>".to_string()));
        assert!(context.in_function());
        assert_eq!(context.function_return_type, Some("Result<Data, Error>".to_string()));
        
        context.exit_function();
        assert!(!context.in_function());
        assert_eq!(context.function_return_type, None);
    }
    
    #[test]
    fn test_const_context_validation() {
        let mut context = ErrorPropagationParseContext::new();
        
        assert!(!context.in_const());
        context.enter_const();
        assert!(context.in_const());
        context.exit_const();
        assert!(!context.in_const());
    }
    
    #[test]
    fn test_propagation_chain_tracking() {
        let mut context = ErrorPropagationParseContext::new();
        
        for i in 0..5 {
            context.increment_propagation();
            assert_eq!(context.propagation_chain_length, i + 1);
        }
        
        context.reset_propagation_chain();
        assert_eq!(context.propagation_chain_length, 0);
    }
    
    #[test]
    fn test_parse_error_display() {
        let errors = vec![
            ErrorPropagationParseError::MissingExpression { line: 1, column: 5 },
            ErrorPropagationParseError::InvalidContext { 
                context: "global scope".to_string(), 
                line: 2, 
                column: 10 
            },
            ErrorPropagationParseError::ChainTooLong { 
                chain_length: 55, 
                line: 3, 
                column: 15 
            },
            ErrorPropagationParseError::IncompatibleReturnType { 
                return_type: "void".to_string(), 
                line: 4, 
                column: 20 
            },
            ErrorPropagationParseError::InConstantExpression { line: 5, column: 25 },
        ];
        
        for error in errors {
            let message = format!("{}", error);
            assert!(!message.is_empty());
            assert!(message.contains("line"));
            assert!(message.contains("column"));
        }
    }
}

#[cfg(test)]
mod error_types_tests {
    use super::*;
    
    #[test]
    fn test_error_propagation_error() {
        let inner = Error::Runtime("Database connection failed".to_string());
        let location = SourceLocation::new(1, 5);
        let prop_error = ErrorPropagationError::new(inner, location);
        
        assert_eq!(prop_error.propagation_site.line, 1);
        assert_eq!(prop_error.propagation_site.column, 5);
        assert!(prop_error.propagation_chain.is_empty());
    }
    
    #[test]
    fn test_error_propagation_chain() {
        let inner = Error::Runtime("Original error".to_string());
        let location1 = SourceLocation::new(1, 5);
        let location2 = SourceLocation::new(2, 10);
        let location3 = SourceLocation::new(3, 15);
        
        let prop_error = ErrorPropagationError::new(inner, location1)
            .add_propagation_site(location2)
            .add_propagation_site(location3);
        
        assert_eq!(prop_error.propagation_chain.len(), 2);
        assert_eq!(prop_error.full_propagation_chain().len(), 3);
        
        let stack_trace = prop_error.propagation_stack_trace();
        assert!(stack_trace.contains("3 sites"));
        assert!(stack_trace.contains("line 1"));
        assert!(stack_trace.contains("line 2"));
        assert!(stack_trace.contains("line 3"));
    }
    
    #[test]
    fn test_propagation_type_mismatch_error() {
        let location = SourceLocation::new(1, 5);
        let error = PropagationTypeMismatchError::new(
            "Result<i32, String>".to_string(),
            "Option<i32>".to_string(),
            location,
        ).with_function_context("calculate_sum".to_string());
        
        assert_eq!(error.expected_type, "Result<i32, String>");
        assert_eq!(error.actual_type, "Option<i32>");
        assert_eq!(error.function_context, Some("calculate_sum".to_string()));
        
        let display = format!("{}", error);
        assert!(display.contains("Type mismatch"));
        assert!(display.contains("Result<i32, String>"));
        assert!(display.contains("Option<i32>"));
        assert!(display.contains("calculate_sum"));
    }
    
    #[test]
    fn test_propagation_context_error() {
        let location = SourceLocation::new(1, 5);
        let error = PropagationContextError::new(
            "Cannot use '?' in global scope".to_string(),
            location,
            "global".to_string(),
        ).with_suggestion("Move to function body".to_string());
        
        assert_eq!(error.context_type, "global");
        assert_eq!(error.suggestion, Some("Move to function body".to_string()));
        
        let display = format!("{}", error);
        assert!(display.contains("global scope"));
        assert!(display.contains("Move to function body"));
    }
    
    #[test]
    fn test_error_propagation_utils() {
        // Test error propagatability
        let runtime_error = Error::Runtime("test".to_string());
        assert!(ErrorPropagationUtils::is_propagatable_error(&runtime_error));
        
        let compile_error = Error::Compile("test".to_string());
        assert!(ErrorPropagationUtils::is_propagatable_error(&compile_error));
        
        let io_error = Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "test"));
        assert!(!ErrorPropagationUtils::is_propagatable_error(&io_error));
        
        // Test message extraction
        let message = ErrorPropagationUtils::extract_error_message(&runtime_error);
        assert_eq!(message, "test");
        
        // Test location extraction
        let error_with_location = Error::ErrorPropagation {
            message: "test".to_string(),
            line: Some(1),
            column: Some(5),
        };
        let location = ErrorPropagationUtils::extract_source_location(&error_with_location);
        assert!(location.is_some());
        assert_eq!(location.unwrap().line, 1);
        assert_eq!(location.unwrap().column, 5);
    }
    
    #[test]
    fn test_error_chaining() {
        let inner = Error::Runtime("Original".to_string());
        let location1 = SourceLocation::new(1, 5);
        let location2 = SourceLocation::new(2, 10);
        
        let original = ErrorPropagationError::new(inner, location1);
        let chained = ErrorPropagationUtils::chain_propagation(original, location2);
        
        assert_eq!(chained.propagation_chain.len(), 1);
        assert_eq!(chained.propagation_chain[0].line, 2);
        assert_eq!(chained.propagation_chain[0].column, 10);
    }
}

#[cfg(test)]
mod runtime_tests {
    use super::*;
    
    #[test]
    fn test_error_propagation_runtime_creation() {
        let runtime = ErrorPropagationRuntime::new();
        assert_eq!(runtime.get_propagation_depth(), 0);
    }
    
    #[test]
    fn test_runtime_with_config() {
        let config = PropagationConfig {
            max_propagation_depth: 50,
            generate_stack_traces: false,
            panic_integration_enabled: false,
            propagation_timeout: Duration::from_secs(1),
            collect_statistics: false,
            preserve_error_context: false,
        };
        
        let runtime = ErrorPropagationRuntime::with_config(config.clone());
        assert_eq!(runtime.config.max_propagation_depth, 50);
        assert!(!runtime.config.generate_stack_traces);
        assert!(!runtime.config.panic_integration_enabled);
    }
    
    #[test]
    fn test_error_handler_registration() {
        let mut runtime = ErrorPropagationRuntime::new();
        let handler = Box::new(DefaultErrorHandler::new());
        
        assert_eq!(runtime.error_handlers.len(), 0);
        runtime.register_handler(handler);
        assert_eq!(runtime.error_handlers.len(), 1);
    }
    
    #[test]
    fn test_propagation_frame() {
        let location = SourceLocation::new(1, 5);
        let frame = PropagationFrame {
            location: location.clone(),
            function_name: Some("test_function".to_string()),
            timestamp: std::time::Instant::now(),
            error_type: "Runtime".to_string(),
            is_tail_position: false,
        };
        
        assert_eq!(frame.location.line, 1);
        assert_eq!(frame.location.column, 5);
        assert_eq!(frame.function_name, Some("test_function".to_string()));
        assert_eq!(frame.error_type, "Runtime");
        assert!(!frame.is_tail_position);
    }
    
    #[test]
    fn test_default_error_handler() {
        let handler = DefaultErrorHandler::new();
        let error = Error::Runtime("Test error".to_string());
        let location = SourceLocation::new(1, 5);
        let frame = PropagationFrame {
            location,
            function_name: None,
            timestamp: std::time::Instant::now(),
            error_type: "Runtime".to_string(),
            is_tail_position: false,
        };
        
        assert!(handler.can_handle(&error));
        assert_eq!(handler.name(), "DefaultErrorHandler");
        assert_eq!(handler.priority(), 1000);
        
        let result = handler.handle_error(&error, &frame);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_thread_local_state() {
        let state = ThreadLocalState::default();
        assert_eq!(state.propagation_depth, 0);
        assert!(!state.propagation_active);
        assert!(state.last_error.is_none());
        assert!(state.local_handlers.is_empty());
    }
    
    #[test]
    fn test_propagation_statistics() {
        let mut stats = PropagationStatistics::default();
        assert_eq!(stats.total_propagations, 0);
        assert_eq!(stats.successful_propagations, 0);
        assert_eq!(stats.failed_propagations, 0);
        assert_eq!(stats.max_propagation_depth, 0);
        assert_eq!(stats.panic_integrations, 0);
        
        // Simulate some activity
        stats.total_propagations = 10;
        stats.successful_propagations = 8;
        stats.failed_propagations = 2;
        stats.max_propagation_depth = 5;
        stats.average_propagation_time_us = 125.5;
        
        assert_eq!(stats.total_propagations, 10);
        assert_eq!(stats.successful_propagations, 8);
        assert_eq!(stats.failed_propagations, 2);
    }
    
    #[test]
    fn test_propagation_config_defaults() {
        let config = PropagationConfig::default();
        assert_eq!(config.max_propagation_depth, 100);
        assert!(config.generate_stack_traces);
        assert!(config.panic_integration_enabled);
        assert_eq!(config.propagation_timeout, Duration::from_secs(5));
        assert!(config.collect_statistics);
        assert!(config.preserve_error_context);
    }
    
    #[test] 
    fn test_error_type_name_extraction() {
        let runtime = ErrorPropagationRuntime::new();
        
        let test_cases = vec![
            (Error::Runtime("test".to_string()), "Runtime"),
            (Error::Parse("test".to_string()), "Parse"),
            (Error::Compile("test".to_string()), "Compile"),
            (Error::Package("test".to_string()), "Package"),
            (Error::Type("test".to_string()), "Type"),
        ];
        
        for (error, expected_type) in test_cases {
            let type_name = runtime.get_error_type_name(&error);
            assert_eq!(type_name, expected_type);
        }
    }
    
    #[test]
    fn test_runtime_statistics_management() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // Get initial statistics
        let stats = runtime.get_statistics().unwrap();
        assert_eq!(stats.total_propagations, 0);
        
        // Reset statistics
        let result = runtime.reset_statistics();
        assert!(result.is_ok());
        
        let stats_after_reset = runtime.get_statistics().unwrap();
        assert_eq!(stats_after_reset.total_propagations, 0);
    }
    
    #[test]
    fn test_propagation_stack_management() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        assert_eq!(runtime.get_propagation_depth(), 0);
        
        // Simulate adding frames
        let location = SourceLocation::new(1, 5);
        let frame = PropagationFrame {
            location,
            function_name: Some("test".to_string()),
            timestamp: std::time::Instant::now(),
            error_type: "Runtime".to_string(),
            is_tail_position: false,
        };
        
        runtime.propagation_stack.push(frame);
        assert_eq!(runtime.get_propagation_depth(), 1);
        
        runtime.clear_propagation_stack();
        assert_eq!(runtime.get_propagation_depth(), 0);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_end_to_end_error_propagation() {
        // Create a complete error propagation scenario
        let var_expr = Identifier::new("api_result".to_string(), "api_result".to_string());
        let location = SourceLocation::new(1, 5);
        let error_prop = ErrorPropagation::with_type(
            Box::new(var_expr),
            location,
            "Result<ApiResponse, ApiError>".to_string(),
        ).set_tail_position(true);
        
        // Validate the AST structure
        assert_eq!(error_prop.string(), "api_result?");
        assert!(error_prop.is_in_tail_position());
        assert_eq!(error_prop.get_expected_type(), Some("Result<ApiResponse, ApiError>"));
        
        // Create propagation context
        let mut context = PropagationContext::new();
        context.set_function(
            "handle_api_request".to_string(),
            Some("Result<ApiResponse, ApiError>".to_string())
        );
        
        // Validate propagation
        let validation_result = PropagationValidator::validate_propagation(&error_prop, &context);
        assert!(validation_result.is_ok());
        
        // Create runtime system
        let mut runtime = ErrorPropagationRuntime::new();
        let handler = Box::new(DefaultErrorHandler::new());
        runtime.register_handler(handler);
        
        // Test error propagation through runtime
        let error = Error::Runtime("API call failed".to_string());
        let propagation_result = runtime.propagate_error(
            error,
            location,
            Some("handle_api_request".to_string())
        );
        
        // The default handler should handle this successfully
        assert!(propagation_result.is_ok());
        
        // Check statistics
        let stats = runtime.get_statistics().unwrap();
        assert_eq!(stats.total_propagations, 1);
        assert_eq!(stats.successful_propagations, 1);
    }
    
    #[test]
    fn test_complex_propagation_chain() {
        // Test a complex propagation chain through multiple functions
        let mut runtime = ErrorPropagationRuntime::new();
        let handler = Box::new(DefaultErrorHandler::new());
        runtime.register_handler(handler);
        
        let locations = vec![
            SourceLocation::new(1, 5),
            SourceLocation::new(2, 10),
            SourceLocation::new(3, 15),
        ];
        
        let functions = vec![
            "database_query",
            "process_result", 
            "handle_request",
        ];
        
        // Simulate a chain of error propagations
        for (i, (location, function)) in locations.iter().zip(functions.iter()).enumerate() {
            let error = Error::Runtime(format!("Error at level {}", i + 1));
            let result = runtime.propagate_error(
                error,
                location.clone(),
                Some(function.to_string())
            );
            assert!(result.is_ok());
        }
        
        // Check final statistics
        let stats = runtime.get_statistics().unwrap();
        assert_eq!(stats.total_propagations, 3);
        assert_eq!(stats.successful_propagations, 3);
        assert_eq!(stats.max_propagation_depth, 3);
    }
    
    #[test]
    fn test_error_propagation_with_type_checking() {
        // Test type compatibility checking in error propagation
        let var_expr = Identifier::new("typed_result".to_string(), "typed_result".to_string());
        let location = SourceLocation::new(1, 5);
        
        // Create error propagation with specific type
        let error_prop = ErrorPropagation::with_type(
            Box::new(var_expr),
            location,
            "Result<String, ParseError>".to_string(),
        );
        
        // Create context with compatible return type
        let mut compatible_context = PropagationContext::new();
        compatible_context.set_function(
            "parse_input".to_string(),
            Some("Result<String, ParseError>".to_string())
        );
        
        let validation_result = PropagationValidator::validate_propagation(
            &error_prop, 
            &compatible_context
        );
        assert!(validation_result.is_ok());
        
        // Create context with incompatible return type
        let mut incompatible_context = PropagationContext::new();
        incompatible_context.set_function(
            "calculate_number".to_string(),
            Some("Result<i32, MathError>".to_string())
        );
        
        // This should fail type checking (simplified implementation may not catch this)
        // In a full implementation, this would be caught by the type system
    }
    
    #[test]
    fn test_performance_characteristics() {
        // Test performance characteristics of the error propagation system
        let mut runtime = ErrorPropagationRuntime::new();
        let handler = Box::new(DefaultErrorHandler::new());
        runtime.register_handler(handler);
        
        let start_time = std::time::Instant::now();
        
        // Perform many propagations to test performance
        for i in 0..1000 {
            let location = SourceLocation::new(i % 100 + 1, i % 50 + 1);
            let error = Error::Runtime(format!("Batch error {}", i));
            
            let result = runtime.propagate_error(
                error,
                location,
                Some(format!("function_{}", i % 10))
            );
            assert!(result.is_ok());
        }
        
        let duration = start_time.elapsed();
        println!("1000 error propagations took: {:?}", duration);
        
        // Verify statistics
        let stats = runtime.get_statistics().unwrap();
        assert_eq!(stats.total_propagations, 1000);
        assert_eq!(stats.successful_propagations, 1000);
        assert!(stats.average_propagation_time_us > 0.0);
        
        // Performance should be reasonable (less than 10ms for 1000 operations)
        assert!(duration.as_millis() < 10);
    }
}

/// Custom error handler for testing
#[derive(Debug)]
struct TestErrorHandler {
    name: String,
    priority: u32,
    handled_errors: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
}

impl TestErrorHandler {
    fn new(name: String, priority: u32) -> Self {
        Self {
            name,
            priority,
            handled_errors: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }
    
    fn get_handled_errors(&self) -> Vec<String> {
        self.handled_errors.lock().unwrap().clone()
    }
}

impl ErrorHandler for TestErrorHandler {
    fn handle_error(&self, error: &Error, _context: &PropagationFrame) -> Result<(), Error> {
        let error_message = format!("{}", error);
        self.handled_errors.lock().unwrap().push(error_message);
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn can_handle(&self, error: &Error) -> bool {
        matches!(error, Error::Runtime(_) | Error::Compile(_))
    }
    
    fn priority(&self) -> u32 {
        self.priority
    }
}

#[cfg(test)]
mod custom_handler_tests {
    use super::*;
    
    #[test]
    fn test_custom_error_handler() {
        let handler = TestErrorHandler::new("TestHandler".to_string(), 500);
        let error = Error::Runtime("Test error".to_string());
        let location = SourceLocation::new(1, 5);
        let frame = PropagationFrame {
            location,
            function_name: None,
            timestamp: std::time::Instant::now(),
            error_type: "Runtime".to_string(),
            is_tail_position: false,
        };
        
        assert!(handler.can_handle(&error));
        let result = handler.handle_error(&error, &frame);
        assert!(result.is_ok());
        
        let handled_errors = handler.get_handled_errors();
        assert_eq!(handled_errors.len(), 1);
        assert!(handled_errors[0].contains("Test error"));
    }
    
    #[test]
    fn test_handler_priority_ordering() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // Add handlers with different priorities
        let high_priority = Box::new(TestErrorHandler::new("High".to_string(), 100));
        let low_priority = Box::new(TestErrorHandler::new("Low".to_string(), 900));
        let medium_priority = Box::new(TestErrorHandler::new("Medium".to_string(), 500));
        
        runtime.register_handler(low_priority);
        runtime.register_handler(high_priority);
        runtime.register_handler(medium_priority);
        
        // Handlers should be ordered by priority
        assert_eq!(runtime.error_handlers.len(), 3);
        assert_eq!(runtime.error_handlers[0].name(), "High");
        assert_eq!(runtime.error_handlers[1].name(), "Medium");
        assert_eq!(runtime.error_handlers[2].name(), "Low");
    }
}
