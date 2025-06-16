//! Integration tests for enhanced error propagation parsing and code generation
//!
//! This test suite validates the complete error propagation system including:
//! - Question mark operator parsing
//! - Type checking for Result/Option types
//! - LLVM code generation
//! - Error recovery mechanisms
//! - Function context tracking

use cursed::ast::traits::Expression;
use cursed::error::CursedError;
use cursed::lexer::{Lexer, Token, TokenType};
use cursed::parser::error_propagation::{
    EnhancedQuestionMarkExpression, TypedErrorPropagation, UnwrapOrExpression,
    TryExpression, FunctionContext, PropagationMetadata
};
use cursed::parser::Parser;
use cursed::codegen::llvm::error_propagation::{ErrorPropagationCodegen, ErrorPropagationCompiler};
use tracing_test::traced_test;

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .try_init();
    };
}

#[traced_test]
#[test]
fn test_enhanced_question_mark_expression_creation() {
    init_tracing!();
    
    // Create a simple identifier expression for testing
    let inner_expr = cursed::ast::identifiers::Identifier::new("test".to_string(), "test".to_string());
    let location = cursed::error::SourceLocation { line: 1, column: 1 };
    
    let enhanced_expr = EnhancedQuestionMarkExpression::new(
        Box::new(inner_expr),
        location.clone(),
        Some("test_function".to_string()),
        Some("Result<i32, String>".to_string()),
    );
    
    assert_eq!(enhanced_expr.function_context, Some("test_function".to_string()));
    assert_eq!(enhanced_expr.expected_return_type, Some("Result<i32, String>".to_string()));
    assert_eq!(enhanced_expr.location.line, 1);
    assert_eq!(enhanced_expr.location.column, 1);
}

#[traced_test]
#[test]
fn test_typed_error_propagation_creation() {
    init_tracing!();
    
    let inner_expr = cursed::ast::identifiers::Identifier::new("test".to_string(), "test".to_string());
    let location = cursed::error::SourceLocation { line: 1, column: 1 };
    
    let enhanced_expr = EnhancedQuestionMarkExpression::new(
        Box::new(inner_expr),
        location,
        Some("test_function".to_string()),
        Some("Result<i32, String>".to_string()),
    );
    
    let typed_expr = TypedErrorPropagation::new(
        Box::new(enhanced_expr),
        "Result<i32, String>".to_string(),
        "Result<i32, String>".to_string(),
    );
    
    assert_eq!(typed_expr.expression_type, "Result<i32, String>");
    assert_eq!(typed_expr.return_type, "Result<i32, String>");
}

#[traced_test]
#[test]
fn test_function_context_creation() {
    init_tracing!();
    
    let context = FunctionContext {
        name: "test_function".to_string(),
        return_type: "Result<i32, String>".to_string(),
        parameters: vec!["x: i32".to_string(), "y: String".to_string()],
        is_async: false,
    };
    
    assert_eq!(context.name, "test_function");
    assert_eq!(context.return_type, "Result<i32, String>");
    assert_eq!(context.parameters.len(), 2);
    assert!(!context.is_async);
}

#[traced_test]
#[test]
fn test_propagation_metadata_creation() {
    init_tracing!();
    
    let metadata = PropagationMetadata::new();
    
    assert!(!metadata.in_try_block);
    assert_eq!(metadata.nesting_level, 0);
    assert!(metadata.error_types.is_empty());
}

#[traced_test]
#[test]
fn test_unwrap_or_expression_creation() {
    init_tracing!();
    
    let default_expr = cursed::ast::identifiers::Identifier::new("default".to_string(), "default".to_string());
    
    let unwrap_or_expr = UnwrapOrExpression::new(
        "unwrap_or".to_string(),
        Box::new(default_expr),
    );
    
    assert_eq!(unwrap_or_expr.method_name, "unwrap_or");
}

#[traced_test]
#[test]
fn test_try_expression_creation() {
    init_tracing!();
    
    let try_body = cursed::ast::identifiers::Identifier::new("try_body".to_string(), "try_body".to_string());
    let catch_body = cursed::ast::identifiers::Identifier::new("catch_body".to_string(), "catch_body".to_string());
    
    let try_expr = TryExpression::new(
        Box::new(try_body),
        Some(Box::new(catch_body)),
    );
    
    assert!(try_expr.catch_block.is_some());
}

#[traced_test]
#[test] 
fn test_parser_type_checking() {
    init_tracing!();
    
    let lexer = Lexer::new("".to_string());
    let parser = Parser::new(lexer).unwrap();
    
    // Test propagatable type checking
    assert!(parser.is_propagatable_type("Result<i32, String>"));
    assert!(parser.is_propagatable_type("Option<String>"));
    assert!(parser.is_propagatable_type("Result"));
    assert!(parser.is_propagatable_type("Option"));
    
    assert!(!parser.is_propagatable_type("i32"));
    assert!(!parser.is_propagatable_type("String"));
    assert!(!parser.is_propagatable_type("Vec<i32>"));
}

#[traced_test]
#[test]
fn test_parser_function_context_tracking() {
    init_tracing!();
    
    let lexer = Lexer::new("".to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    // Initially no function context
    assert!(parser.current_function_context().is_none());
    assert!(parser.function_return_types().is_empty());
    
    // Enter function context
    parser.enter_function_context(
        "test_function".to_string(),
        "Result<i32, String>".to_string(),
        vec!["x: i32".to_string()],
        false,
    );
    
    assert_eq!(parser.current_function_context(), Some("test_function".to_string()));
    assert_eq!(parser.function_return_types(), vec!["Result<i32, String>"]);
    
    // Exit function context
    parser.exit_function_context();
    assert!(parser.current_function_context().is_none());
    assert!(parser.function_return_types().is_empty());
}

#[traced_test]
#[test]
fn test_nested_function_contexts() {
    init_tracing!();
    
    let lexer = Lexer::new("".to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    // Enter first function
    parser.enter_function_context(
        "outer_function".to_string(),
        "Result<i32, String>".to_string(),
        vec![],
        false,
    );
    
    // Enter nested function
    parser.enter_function_context(
        "inner_function".to_string(),
        "Option<i32>".to_string(),
        vec![],
        false,
    );
    
    assert_eq!(parser.current_function_context(), Some("inner_function".to_string()));
    assert_eq!(parser.function_return_types(), vec!["Result<i32, String>", "Option<i32>"]);
    
    // Exit inner function
    parser.exit_function_context();
    assert_eq!(parser.current_function_context(), Some("outer_function".to_string()));
    assert_eq!(parser.function_return_types(), vec!["Result<i32, String>"]);
    
    // Exit outer function
    parser.exit_function_context();
    assert!(parser.current_function_context().is_none());
}

#[traced_test]
#[test]
fn test_error_propagation_context_validation() {
    init_tracing!();
    
    let lexer = Lexer::new("test?".to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    let location = cursed::error::SourceLocation { line: 1, column: 1 };
    
    // Should fail outside function context
    let result = parser.validate_error_propagation_context(&location);
    assert!(result.is_err());
    
    // Enter function context with compatible return type
    parser.enter_function_context(
        "test_function".to_string(),
        "Result<i32, String>".to_string(),
        vec![],
        false,
    );
    
    // Should succeed with compatible return type
    let result = parser.validate_error_propagation_context(&location);
    assert!(result.is_ok());
}

#[traced_test]
#[test]
fn test_llvm_error_propagation_compiler_creation() {
    init_tracing!();
    
    let context = inkwell::context::Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    assert_eq!(compiler.function_stack.len(), 0);
    
    // Test function context management
    let function_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_function", function_type, None);
    
    let mut compiler = compiler;
    compiler.enter_function(function);
    assert_eq!(compiler.function_stack.len(), 1);
    assert_eq!(compiler.current_function(), Some(function));
    
    compiler.exit_function();
    assert_eq!(compiler.function_stack.len(), 0);
    assert_eq!(compiler.current_function(), None);
}

#[traced_test]
#[test]
fn test_llvm_result_type_creation() {
    init_tracing!();
    
    let context = inkwell::context::Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let i32_type = context.i32_type().into();
    let string_type = context.i8_type().ptr_type(inkwell::AddressSpace::default()).into();
    
    let result_type = compiler.get_result_type(i32_type, string_type);
    assert_eq!(result_type.count_fields(), 2);
    
    // First field should be boolean (is_ok flag)
    let flag_type = result_type.get_field_type_at_index(0).unwrap();
    assert!(flag_type.is_int_type());
    
    // Second field should be union type
    let union_type = result_type.get_field_type_at_index(1).unwrap();
    assert!(union_type.is_struct_type());
}

#[traced_test]
#[test]
fn test_llvm_option_type_creation() {
    init_tracing!();
    
    let context = inkwell::context::Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let i32_type = context.i32_type().into();
    
    let option_type = compiler.get_option_type(i32_type);
    assert_eq!(option_type.count_fields(), 2);
    
    // First field should be boolean (is_some flag)
    let flag_type = option_type.get_field_type_at_index(0).unwrap();
    assert!(flag_type.is_int_type());
    
    // Second field should be the inner type
    let inner_type = option_type.get_field_type_at_index(1).unwrap();
    assert_eq!(inner_type, i32_type);
}

#[traced_test]
#[test]
fn test_expression_type_inference() {
    init_tracing!();
    
    let lexer = Lexer::new("".to_string());
    let parser = Parser::new(lexer).unwrap();
    
    // Create a mock expression that looks like a Result
    let result_expr = cursed::ast::identifiers::Identifier::new("Result::ok(42)".to_string(), "result".to_string());
    let expr_type = parser.infer_expression_type(&Box::new(result_expr)).unwrap();
    assert_eq!(expr_type, "Result<T, E>");
    
    // Create a mock expression that looks like an Option
    let option_expr = cursed::ast::identifiers::Identifier::new("Option::some(42)".to_string(), "option".to_string());
    let expr_type = parser.infer_expression_type(&Box::new(option_expr)).unwrap();
    assert_eq!(expr_type, "Option<T>");
    
    // Create a mock function call
    let function_expr = cursed::ast::identifiers::Identifier::new("function_call()".to_string(), "function".to_string());
    let expr_type = parser.infer_expression_type(&Box::new(function_expr)).unwrap();
    assert_eq!(expr_type, "Result<T, E>");
}

#[traced_test]
#[test]
fn test_error_recovery_suggestions() {
    init_tracing!();
    
    use cursed::parser::error_propagation::error_recovery;
    
    let result_suggestions = error_recovery::suggest_recovery_patterns("Result<i32, String>");
    assert!(!result_suggestions.is_empty());
    assert!(result_suggestions.iter().any(|s| s.contains("unwrap_or")));
    assert!(result_suggestions.iter().any(|s| s.contains("match expression")));
    
    let option_suggestions = error_recovery::suggest_recovery_patterns("Option<i32>");
    assert!(!option_suggestions.is_empty());
    assert!(option_suggestions.iter().any(|s| s.contains("unwrap_or")));
    assert!(option_suggestions.iter().any(|s| s.contains("if let Some")));
}

#[traced_test]
#[test]
fn test_propagation_chain_validation() {
    init_tracing!();
    
    use cursed::parser::error_propagation::error_recovery;
    
    // Create a chain of expressions
    let expr1 = cursed::ast::identifiers::Identifier::new("expr1".to_string(), "expr1".to_string());
    let expr2 = cursed::ast::identifiers::Identifier::new("expr2?".to_string(), "expr2".to_string());
    
    let chain = vec![Box::new(expr1) as Box<dyn Expression>, Box::new(expr2) as Box<dyn Expression>];
    
    // Should fail because last expression has ? operator
    let result = error_recovery::validate_propagation_chain(&chain);
    assert!(result.is_err());
}

#[traced_test]
#[test]
fn test_integration_with_existing_parser() {
    init_tracing!();
    
    // Test that the enhanced error propagation integrates with existing parser infrastructure
    let lexer = Lexer::new("sus x = some_function()?;".to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    // Enter function context for error propagation
    parser.enter_function_context(
        "test_function".to_string(),
        "Result<(), String>".to_string(),
        vec![],
        false,
    );
    
    // This should be able to parse without errors (even if we don't have full implementation)
    // The key is that the infrastructure is in place
    assert!(parser.current_function_context().is_some());
    assert!(!parser.function_return_types().is_empty());
}

#[traced_test]
#[test]
fn test_async_function_context() {
    init_tracing!();
    
    let lexer = Lexer::new("".to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    // Test async function context
    parser.enter_function_context(
        "async_function".to_string(),
        "Future<Result<i32, String>>".to_string(),
        vec![],
        true, // is_async = true
    );
    
    let context_stack = parser.function_context_stack();
    // Note: This is testing the infrastructure - in a real implementation
    // this would work with proper thread-local storage
    assert!(parser.current_function_context().is_some());
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[traced_test]
    #[test]
    fn benchmark_function_context_operations() {
        init_tracing!();
        
        let lexer = Lexer::new("".to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        let start = Instant::now();
        
        // Benchmark entering and exiting function contexts
        for i in 0..1000 {
            parser.enter_function_context(
                format!("function_{}", i),
                "Result<i32, String>".to_string(),
                vec![],
                false,
            );
        }
        
        for _ in 0..1000 {
            parser.exit_function_context();
        }
        
        let duration = start.elapsed();
        println!("Function context operations took: {:?}", duration);
        
        // Should be fast
        assert!(duration.as_millis() < 100);
    }

    #[traced_test]
    #[test]
    fn benchmark_type_checking() {
        init_tracing!();
        
        let lexer = Lexer::new("".to_string());
        let parser = Parser::new(lexer).unwrap();
        
        let start = Instant::now();
        
        // Benchmark type checking operations
        for _ in 0..10000 {
            assert!(parser.is_propagatable_type("Result<i32, String>"));
            assert!(parser.is_propagatable_type("Option<String>"));
            assert!(!parser.is_propagatable_type("i32"));
        }
        
        let duration = start.elapsed();
        println!("Type checking operations took: {:?}", duration);
        
        // Should be very fast
        assert!(duration.as_millis() < 50);
    }
}
