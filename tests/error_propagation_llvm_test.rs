//! LLVM code generation tests for error propagation
//!
//! This test suite validates the LLVM code generation aspects of the error propagation
//! system, including Result/Option type generation, error handling code, and FFI integration.

use cursed::codegen::llvm::error_propagation::{ErrorPropagationCodegen, ErrorPropagationCompiler};
use cursed::parser::error_propagation::{
    EnhancedQuestionMarkExpression, TypedErrorPropagation, UnwrapOrExpression,
    TryExpression, FieldAccessExpression, MethodCallExpression
};
use cursed::error::{CursedError, SourceLocation};
use inkwell::context::Context;
use inkwell::AddressSpace;
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
fn test_result_ok_creation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let i32_type = context.i32_type();
    let string_type = context.i8_type().ptr_type(AddressSpace::default());
    let value = i32_type.const_int(42, false);
    
    let result_ok = compiler.create_result_ok(value.into(), string_type.into());
    assert!(result_ok.is_ok());
    
    let result_struct = result_ok.unwrap();
    assert_eq!(result_struct.get_type().count_fields(), 2);
}

#[traced_test]
#[test]
fn test_result_err_creation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let i32_type = context.i32_type();
    let string_type = context.i8_type().ptr_type(AddressSpace::default());
    let error_value = string_type.const_null();
    
    let result_err = compiler.create_result_err(error_value.into(), i32_type.into());
    assert!(result_err.is_ok());
    
    let result_struct = result_err.unwrap();
    assert_eq!(result_struct.get_type().count_fields(), 2);
}

#[traced_test]
#[test]
fn test_option_some_creation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let i32_type = context.i32_type();
    let value = i32_type.const_int(42, false);
    
    let option_some = compiler.create_option_some(value.into());
    assert!(option_some.is_ok());
    
    let option_struct = option_some.unwrap();
    assert_eq!(option_struct.get_type().count_fields(), 2);
}

#[traced_test]
#[test]
fn test_option_none_creation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let i32_type = context.i32_type();
    
    let option_none = compiler.create_option_none(i32_type.into());
    assert!(option_none.is_ok());
    
    let option_struct = option_none.unwrap();
    assert_eq!(option_struct.get_type().count_fields(), 2);
}

#[traced_test]
#[test]
fn test_enhanced_question_mark_compilation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a mock enhanced question mark expression
    let inner_expr = cursed::ast::identifiers::Identifier::new("test".to_string(), "test".to_string());
    let location = SourceLocation { line: 1, column: 1 };
    
    let enhanced_expr = EnhancedQuestionMarkExpression::new(
        Box::new(inner_expr),
        location,
        Some("test_function".to_string()),
        Some("Result<i32, String>".to_string()),
    );
    
    let result = compiler.compile_enhanced_question_mark(&enhanced_expr);
    assert!(result.is_ok());
    
    let compiled_value = result.unwrap();
    assert!(compiled_value.is_int_value());
}

#[traced_test]
#[test]
fn test_typed_error_propagation_compilation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a mock typed error propagation expression
    let inner_expr = cursed::ast::identifiers::Identifier::new("test".to_string(), "test".to_string());
    let location = SourceLocation { line: 1, column: 1 };
    
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
    
    let result = compiler.compile_typed_error_propagation(&typed_expr);
    assert!(result.is_ok());
    
    let compiled_value = result.unwrap();
    assert!(compiled_value.is_int_value());
}

#[traced_test]
#[test]
fn test_unwrap_or_expression_compilation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a mock unwrap-or expression
    let default_expr = cursed::ast::identifiers::Identifier::new("default".to_string(), "default".to_string());
    
    let unwrap_or_expr = UnwrapOrExpression::new(
        "unwrap_or".to_string(),
        Box::new(default_expr),
    );
    
    let result = compiler.compile_unwrap_or_expression(&unwrap_or_expr);
    assert!(result.is_ok());
    
    let compiled_value = result.unwrap();
    assert!(compiled_value.is_int_value());
}

#[traced_test]
#[test]
fn test_try_expression_compilation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a mock try expression
    let try_body = cursed::ast::identifiers::Identifier::new("try_body".to_string(), "try_body".to_string());
    let catch_body = cursed::ast::identifiers::Identifier::new("catch_body".to_string(), "catch_body".to_string());
    
    let try_expr = TryExpression::new(
        Box::new(try_body),
        Some(Box::new(catch_body)),
    );
    
    let result = compiler.compile_try_expression(&try_expr);
    assert!(result.is_ok());
    
    let compiled_value = result.unwrap();
    assert!(compiled_value.is_int_value());
}

#[traced_test]
#[test]
fn test_field_access_expression_compilation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a mock field access expression
    let object_expr = cursed::ast::identifiers::Identifier::new("object".to_string(), "object".to_string());
    let location = SourceLocation { line: 1, column: 1 };
    
    let field_access_expr = FieldAccessExpression::new(
        Box::new(object_expr),
        "field".to_string(),
        location,
    );
    
    let result = compiler.compile_field_access_expression(&field_access_expr);
    assert!(result.is_ok());
    
    let compiled_value = result.unwrap();
    assert!(compiled_value.is_int_value());
}

#[traced_test]
#[test]
fn test_method_call_expression_compilation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a mock method call expression
    let base_expr = cursed::ast::identifiers::Identifier::new("base".to_string(), "base".to_string());
    let location = SourceLocation { line: 1, column: 1 };
    
    let method_call_expr = MethodCallExpression::new(
        Box::new(base_expr),
        "method".to_string(),
        vec![],
        location,
    );
    
    let result = compiler.compile_method_call_expression(&method_call_expr);
    assert!(result.is_ok());
    
    let compiled_value = result.unwrap();
    assert!(compiled_value.is_int_value());
}

#[traced_test]
#[test]
fn test_error_handling_generation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let i32_type = context.i32_type();
    let result_value = i32_type.const_int(42, false);
    let error_handler = i32_type.const_int(0, false);
    
    let result = compiler.generate_error_handling(
        result_value.into(),
        Some(error_handler.into()),
    );
    assert!(result.is_ok());
    
    let handled_value = result.unwrap();
    assert!(handled_value.is_int_value());
}

#[traced_test]
#[test]
fn test_error_value_checking() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let i32_type = context.i32_type();
    let value = i32_type.const_int(42, false);
    
    let result = compiler.is_error_value(value.into());
    assert!(result.is_ok());
    
    let is_error = result.unwrap();
    assert!(is_error.is_int_value());
    assert_eq!(is_error.get_zero_extended_constant(), Some(0)); // Should be false
}

#[traced_test]
#[test]
fn test_success_value_extraction() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let i32_type = context.i32_type();
    let result_value = i32_type.const_int(42, false);
    
    let result = compiler.extract_success_value(result_value.into());
    assert!(result.is_ok());
    
    let success_value = result.unwrap();
    assert!(success_value.is_int_value());
}

#[traced_test]
#[test]
fn test_error_value_extraction() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let i32_type = context.i32_type();
    let result_value = i32_type.const_int(42, false);
    
    let result = compiler.extract_error_value(result_value.into());
    assert!(result.is_ok());
    
    let error_value = result.unwrap();
    assert!(error_value.is_int_value());
}

#[traced_test]
#[test]
fn test_early_return_generation() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let i32_type = context.i32_type();
    let error_value = i32_type.const_int(1, false);
    
    let result = compiler.generate_early_return(error_value.into());
    assert!(result.is_ok());
}

#[traced_test]
#[test]
fn test_function_context_management() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Initially no function context
    assert_eq!(compiler.function_stack.len(), 0);
    assert!(compiler.current_function().is_none());
    
    // Create and enter function
    let function_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_function", function_type, None);
    
    compiler.enter_function(function);
    assert_eq!(compiler.function_stack.len(), 1);
    assert_eq!(compiler.current_function(), Some(function));
    
    // Exit function
    compiler.exit_function();
    assert_eq!(compiler.function_stack.len(), 0);
    assert!(compiler.current_function().is_none());
}

#[traced_test]
#[test]
fn test_nested_function_contexts() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create functions
    let function_type = context.void_type().fn_type(&[], false);
    let outer_function = module.add_function("outer_function", function_type, None);
    let inner_function = module.add_function("inner_function", function_type, None);
    
    // Enter outer function
    compiler.enter_function(outer_function);
    assert_eq!(compiler.current_function(), Some(outer_function));
    
    // Enter inner function
    compiler.enter_function(inner_function);
    assert_eq!(compiler.current_function(), Some(inner_function));
    assert_eq!(compiler.function_stack.len(), 2);
    
    // Exit inner function
    compiler.exit_function();
    assert_eq!(compiler.current_function(), Some(outer_function));
    assert_eq!(compiler.function_stack.len(), 1);
    
    // Exit outer function
    compiler.exit_function();
    assert!(compiler.current_function().is_none());
    assert_eq!(compiler.function_stack.len(), 0);
}

#[traced_test]
#[test]
fn test_result_and_option_type_compatibility() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Test that Result and Option types have compatible structure
    let i32_type = context.i32_type().into();
    let string_type = context.i8_type().ptr_type(AddressSpace::default()).into();
    
    let result_type = compiler.get_result_type(i32_type, string_type);
    let option_type = compiler.get_option_type(i32_type);
    
    // Both should have 2 fields
    assert_eq!(result_type.count_fields(), 2);
    assert_eq!(option_type.count_fields(), 2);
    
    // Both should have boolean as first field
    assert!(result_type.get_field_type_at_index(0).unwrap().is_int_type());
    assert!(option_type.get_field_type_at_index(0).unwrap().is_int_type());
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[traced_test]
    #[test]
    fn benchmark_result_creation() {
        init_tracing!();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
        
        let i32_type = context.i32_type();
        let string_type = context.i8_type().ptr_type(AddressSpace::default());
        let value = i32_type.const_int(42, false);
        let error = string_type.const_null();
        
        let start = Instant::now();
        
        // Benchmark Result creation
        for _ in 0..1000 {
            let _ = compiler.create_result_ok(value.into(), string_type.into());
            let _ = compiler.create_result_err(error.into(), i32_type.into());
        }
        
        let duration = start.elapsed();
        println!("Result creation operations took: {:?}", duration);
        
        // Should be reasonably fast
        assert!(duration.as_millis() < 1000);
    }

    #[traced_test]
    #[test]
    fn benchmark_option_creation() {
        init_tracing!();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
        
        let i32_type = context.i32_type();
        let value = i32_type.const_int(42, false);
        
        let start = Instant::now();
        
        // Benchmark Option creation
        for _ in 0..1000 {
            let _ = compiler.create_option_some(value.into());
            let _ = compiler.create_option_none(i32_type.into());
        }
        
        let duration = start.elapsed();
        println!("Option creation operations took: {:?}", duration);
        
        // Should be reasonably fast
        assert!(duration.as_millis() < 1000);
    }

    #[traced_test]
    #[test]
    fn benchmark_expression_compilation() {
        init_tracing!();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
        
        // Create mock expressions
        let inner_expr = cursed::ast::identifiers::Identifier::new("test".to_string(), "test".to_string());
        let location = SourceLocation { line: 1, column: 1 };
        
        let enhanced_expr = EnhancedQuestionMarkExpression::new(
            Box::new(inner_expr),
            location,
            Some("test_function".to_string()),
            Some("Result<i32, String>".to_string()),
        );
        
        let start = Instant::now();
        
        // Benchmark expression compilation
        for _ in 0..100 {
            let _ = compiler.compile_enhanced_question_mark(&enhanced_expr);
        }
        
        let duration = start.elapsed();
        println!("Expression compilation took: {:?}", duration);
        
        // Should be reasonably fast
        assert!(duration.as_millis() < 500);
    }
}
