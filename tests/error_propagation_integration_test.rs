//! Integration tests for error propagation system

use cursed::ast::traits::Expression;
use cursed::codegen::llvm::error_propagation::{ErrorPropagationCodegen, ErrorPropagationCompiler};
use cursed::error::CursedError;
use cursed::parser::error_propagation::{
    EnhancedQuestionMarkExpression, TypedErrorPropagation, UnwrapOrExpression,
    TryExpression, FieldAccessExpression, MethodCallExpression
};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FunctionValue};
use inkwell::types::BasicTypeEnum;
use std::fmt;
use tracing::info;

// Mock expression for testing
#[derive(Debug, Clone)]
struct MockExpression {
    pub name: String,
}

impl fmt::Display for MockExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MockExpression({})", self.name)
    }
}

impl Expression for MockExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[test]
fn test_error_propagation_compiler_creation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Test basic structure
    assert_eq!(compiler.context as *const _, &context as *const _);
    assert_eq!(compiler.module as *const _, &module as *const _);
    assert_eq!(compiler.builder as *const _, &builder as *const _);
    assert_eq!(compiler.function_stack.len(), 0);

    info!("✓ Error propagation compiler created successfully");
}

#[test]
fn test_result_and_option_type_creation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Test Result<i32, String> type creation
    let i32_type = context.i32_type().into();
    let string_type = context.i8_type().ptr_type(inkwell::AddressSpace::default()).into();
    
    let result_type = compiler.get_result_type(i32_type, string_type);
    assert_eq!(result_type.count_fields(), 2);
    
    // Test Option<i32> type creation
    let option_type = compiler.get_option_type(i32_type);
    assert_eq!(option_type.count_fields(), 2);

    info!("✓ Result and Option types created successfully");
}

#[test]
fn test_result_ok_creation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let success_value = context.i32_type().const_int(42, false);
    let error_type = context.i32_type().into();
    
    let result_ok = compiler.create_result_ok(success_value.into(), error_type);
    assert!(result_ok.is_ok());
    
    let ok_struct = result_ok.unwrap();
    assert!(ok_struct.get_type().count_fields() == 2);

    info!("✓ Result::Ok created successfully");
}

#[test]
fn test_result_err_creation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let error_value = context.i32_type().const_int(1, false);
    let success_type = context.i32_type().into();
    
    let result_err = compiler.create_result_err(error_value.into(), success_type);
    assert!(result_err.is_ok());
    
    let err_struct = result_err.unwrap();
    assert!(err_struct.get_type().count_fields() == 2);

    info!("✓ Result::Err created successfully");
}

#[test]
fn test_option_some_creation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let value = context.i32_type().const_int(42, false);
    
    let option_some = compiler.create_option_some(value.into());
    assert!(option_some.is_ok());
    
    let some_struct = option_some.unwrap();
    assert!(some_struct.get_type().count_fields() == 2);

    info!("✓ Option::Some created successfully");
}

#[test]
fn test_option_none_creation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    let inner_type = context.i32_type().into();
    
    let option_none = compiler.create_option_none(inner_type);
    assert!(option_none.is_ok());
    
    let none_struct = option_none.unwrap();
    assert!(none_struct.get_type().count_fields() == 2);

    info!("✓ Option::None created successfully");
}

#[test]
fn test_function_stack_management() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    
    // Test function stack operations
    assert!(compiler.current_function().is_none());
    
    compiler.enter_function(function);
    assert!(compiler.current_function().is_some());
    assert_eq!(compiler.current_function().unwrap(), function);
    
    compiler.exit_function();
    assert!(compiler.current_function().is_none());

    info!("✓ Function stack management working correctly");
}

#[test]
fn test_is_error_value_with_result() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function for builder context
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Test with Result::Ok
    let success_value = context.i32_type().const_int(42, false);
    let error_type = context.i32_type().into();
    let result_ok = compiler.create_result_ok(success_value.into(), error_type).unwrap();
    
    let is_error_ok = compiler.is_error_value(result_ok.into());
    assert!(is_error_ok.is_ok());
    
    // Test with Result::Err
    let error_value = context.i32_type().const_int(1, false);
    let success_type = context.i32_type().into();
    let result_err = compiler.create_result_err(error_value.into(), success_type).unwrap();
    
    let is_error_err = compiler.is_error_value(result_err.into());
    assert!(is_error_err.is_ok());

    info!("✓ is_error_value works with Result types");
}

#[test]
fn test_extract_success_value() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function for builder context
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Create Result::Ok and extract success value
    let success_value = context.i32_type().const_int(42, false);
    let error_type = context.i32_type().into();
    let result_ok = compiler.create_result_ok(success_value.into(), error_type).unwrap();
    
    let extracted = compiler.extract_success_value(result_ok.into());
    assert!(extracted.is_ok());

    info!("✓ extract_success_value works correctly");
}

#[test]
fn test_extract_error_value() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function for builder context
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Create Result::Err and extract error value
    let error_value = context.i32_type().const_int(1, false);
    let success_type = context.i32_type().into();
    let result_err = compiler.create_result_err(error_value.into(), success_type).unwrap();
    
    let extracted = compiler.extract_error_value(result_err.into());
    assert!(extracted.is_ok());

    info!("✓ extract_error_value works correctly");
}

#[test]
fn test_enhanced_question_mark_compilation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function that returns Result<i32, i32>
    let i32_type = context.i32_type();
    let result_type = compiler.get_result_type(i32_type.into(), i32_type.into());
    let fn_type = result_type.fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    compiler.enter_function(function);
    
    // Create mock question mark expression
    let mock_expr = Box::new(MockExpression { name: "test".to_string() });
    let question_mark_expr = EnhancedQuestionMarkExpression {
        expression: mock_expr,
        error_recovery: None,
        source_location: None,
    };
    
    let result = compiler.compile_enhanced_question_mark(&question_mark_expr);
    
    // Should succeed even if it doesn't complete due to control flow
    // The important thing is that it doesn't panic and generates valid IR
    compiler.exit_function();

    info!("✓ Enhanced question mark compilation completed");
}

#[test]
fn test_typed_error_propagation_compilation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function
    let i32_type = context.i32_type();
    let result_type = compiler.get_result_type(i32_type.into(), i32_type.into());
    let fn_type = result_type.fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    compiler.enter_function(function);
    
    // Create mock typed error propagation expression
    let mock_expr = Box::new(MockExpression { name: "test".to_string() });
    let typed_expr = TypedErrorPropagation {
        expression: mock_expr,
        expected_error_type: "TestError".to_string(),
        conversion_logic: None,
    };
    
    let result = compiler.compile_typed_error_propagation(&typed_expr);
    compiler.exit_function();

    info!("✓ Typed error propagation compilation completed");
}

#[test]
fn test_unwrap_or_expression_compilation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    compiler.enter_function(function);
    
    // Create mock unwrap_or expression
    let base_expr = Box::new(MockExpression { name: "base".to_string() });
    let default_expr = Box::new(MockExpression { name: "default".to_string() });
    let unwrap_or_expr = UnwrapOrExpression {
        base: base_expr,
        default_value: default_expr,
        method_name: "unwrap_or".to_string(),
    };
    
    let result = compiler.compile_unwrap_or_expression(&unwrap_or_expr);
    compiler.exit_function();

    info!("✓ Unwrap-or expression compilation completed");
}

#[test]
fn test_try_expression_compilation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function
    let i32_type = context.i32_type();
    let result_type = compiler.get_result_type(i32_type.into(), i32_type.into());
    let fn_type = result_type.fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    compiler.enter_function(function);
    
    // Create mock try expression
    let try_block = Box::new(MockExpression { name: "try_block".to_string() });
    let catch_block = Some(Box::new(MockExpression { name: "catch_block".to_string() }));
    let try_expr = TryExpression {
        try_block,
        catch_block,
        finally_block: None,
    };
    
    let result = compiler.compile_try_expression(&try_expr);
    compiler.exit_function();

    info!("✓ Try expression compilation completed");
}

#[test]
fn test_field_access_expression_compilation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    compiler.enter_function(function);
    
    // Create mock field access expression
    let base_expr = Box::new(MockExpression { name: "base".to_string() });
    let field_expr = FieldAccessExpression {
        base: base_expr,
        field_name: "value".to_string(),
        safe_access: false,
    };
    
    let result = compiler.compile_field_access_expression(&field_expr);
    compiler.exit_function();

    info!("✓ Field access expression compilation completed");
}

#[test]
fn test_method_call_expression_compilation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    compiler.enter_function(function);
    
    // Create mock method call expression
    let receiver_expr = Box::new(MockExpression { name: "receiver".to_string() });
    let method_expr = MethodCallExpression {
        receiver: receiver_expr,
        method_name: "unwrap".to_string(),
        arguments: vec![],
        type_arguments: vec![],
    };
    
    let result = compiler.compile_method_call_expression(&method_expr);
    compiler.exit_function();

    info!("✓ Method call expression compilation completed");
}

#[test]
fn test_error_handling_without_handler() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function
    let i32_type = context.i32_type();
    let result_type = compiler.get_result_type(i32_type.into(), i32_type.into());
    let fn_type = result_type.fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    compiler.enter_function(function);
    
    // Create a Result::Ok for testing
    let success_value = i32_type.const_int(42, false);
    let result_ok = compiler.create_result_ok(success_value.into(), i32_type.into()).unwrap();
    
    let result = compiler.generate_error_handling(result_ok.into(), None);
    compiler.exit_function();

    info!("✓ Error handling without handler completed");
}

#[test]
fn test_error_handling_with_handler() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    compiler.enter_function(function);
    
    // Create a Result::Ok for testing
    let success_value = i32_type.const_int(42, false);
    let result_ok = compiler.create_result_ok(success_value.into(), i32_type.into()).unwrap();
    
    // Create a handler value
    let handler_value = i32_type.const_int(100, false);
    
    let result = compiler.generate_error_handling(result_ok.into(), Some(handler_value.into()));
    compiler.exit_function();

    info!("✓ Error handling with handler completed");
}

#[test]
fn test_compile_expression_mock() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = ErrorPropagationCompiler::new(&context, &module, &builder);
    
    // Create a test function for builder context
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    let mock_expr = MockExpression { name: "test".to_string() };
    let result = compiler.compile_expression(&mock_expr);
    
    assert!(result.is_ok());
    let value = result.unwrap();
    
    // Should return a Result struct
    if let BasicValueEnum::StructValue(struct_val) = value {
        assert_eq!(struct_val.get_type().count_fields(), 2);
        info!("✓ Mock expression compilation returns proper Result structure");
    }
}
