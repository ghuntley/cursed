/// Integration tests for the LLVM expression compilation system
/// 
/// This test suite validates the complete expression compilation pipeline
/// including literals, binary operations, unary operations, variable access,
/// and basic type checking.

use cursed::ast::expressions::{Literal, LiteralValue};
use cursed::ast::operators::{BinaryExpression, UnaryExpression};
use cursed::ast::identifiers::Identifier;
use cursed::ast::statements::LetStatement;
use cursed::codegen::llvm::expression_compiler::{LlvmExpressionCompiler, LlvmValue, LlvmType};
use cursed::codegen::llvm::variable_management::VariableManager;
use cursed::core::type_checker::Type;
use inkwell::context::Context;
use std::cell::RefCell;
use std::rc::Rc;
use tracing_test::traced_test;

#[traced_test]
#[test]
fn test_literal_compilation() {
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let variable_manager = Rc::new(RefCell::new(VariableManager::new(&context, &module, &builder)));
    
    let mut compiler = LlvmExpressionCompiler::new(&context, &module, &builder, variable_manager);
    
    // Test integer literal
    let int_literal = Literal::integer(42);
    let result = compiler.compile_literal(&int_literal).unwrap();
    assert_eq!(result.value_type, LlvmType::Int64);
    assert!(result.is_constant);
    assert!(result.is_int_value());
    
    // Test float literal
    let float_literal = Literal::float(3.14);
    let result = compiler.compile_literal(&float_literal).unwrap();
    assert_eq!(result.value_type, LlvmType::Float64);
    assert!(result.is_constant);
    assert!(result.is_float_value());
    
    // Test boolean literal
    let bool_literal = Literal::boolean(true);
    let result = compiler.compile_literal(&bool_literal).unwrap();
    assert_eq!(result.value_type, LlvmType::Boolean);
    assert!(result.is_constant);
    
    // Test string literal
    let string_literal = Literal::string("hello".to_string());
    let result = compiler.compile_literal(&string_literal).unwrap();
    assert_eq!(result.value_type, LlvmType::String);
    assert!(result.is_constant);
    
    // Test nil literal
    let nil_literal = Literal::nil();
    let result = compiler.compile_literal(&nil_literal).unwrap();
    assert!(matches!(result.value_type, LlvmType::Pointer(_)));
    assert!(result.is_constant);
}

#[traced_test]
#[test]
fn test_binary_expression_compilation() {
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let variable_manager = Rc::new(RefCell::new(VariableManager::new(&context, &module, &builder)));
    
    let mut compiler = LlvmExpressionCompiler::new(&context, &module, &builder, variable_manager);
    
    // Create a basic block for IR generation
    let function_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_func", function_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Test integer addition
    let left = Box::new(Literal::integer(10));
    let right = Box::new(Literal::integer(20));
    let add_expr = BinaryExpression::new("+".to_string(), left, right);
    
    let result = compiler.compile_binary_expression(&add_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Int64);
    assert!(!result.is_constant);
    
    // Test float multiplication
    let left = Box::new(Literal::float(2.5));
    let right = Box::new(Literal::float(4.0));
    let mul_expr = BinaryExpression::new("*".to_string(), left, right);
    
    let result = compiler.compile_binary_expression(&mul_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Float64);
    assert!(!result.is_constant);
    
    // Test boolean comparison
    let left = Box::new(Literal::integer(10));
    let right = Box::new(Literal::integer(20));
    let cmp_expr = BinaryExpression::new("<".to_string(), left, right);
    
    let result = compiler.compile_binary_expression(&cmp_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Boolean);
    assert!(!result.is_constant);
    
    // Test logical and
    let left = Box::new(Literal::boolean(true));
    let right = Box::new(Literal::boolean(false));
    let and_expr = BinaryExpression::new("&&".to_string(), left, right);
    
    let result = compiler.compile_binary_expression(&and_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Boolean);
    assert!(!result.is_constant);
}

#[traced_test]
#[test]
fn test_unary_expression_compilation() {
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let variable_manager = Rc::new(RefCell::new(VariableManager::new(&context, &module, &builder)));
    
    let mut compiler = LlvmExpressionCompiler::new(&context, &module, &builder, variable_manager);
    
    // Create a basic block for IR generation
    let function_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_func", function_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Test integer negation
    let operand = Box::new(Literal::integer(42));
    let neg_expr = UnaryExpression::new("-".to_string(), operand);
    
    let result = compiler.compile_unary_expression(&neg_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Int64);
    assert!(!result.is_constant);
    
    // Test float negation
    let operand = Box::new(Literal::float(3.14));
    let neg_expr = UnaryExpression::new("-".to_string(), operand);
    
    let result = compiler.compile_unary_expression(&neg_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Float64);
    assert!(!result.is_constant);
    
    // Test logical not
    let operand = Box::new(Literal::boolean(true));
    let not_expr = UnaryExpression::new("!".to_string(), operand);
    
    let result = compiler.compile_unary_expression(&not_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Boolean);
    assert!(!result.is_constant);
    
    // Test bitwise not
    let operand = Box::new(Literal::integer(42));
    let bitnot_expr = UnaryExpression::new("~".to_string(), operand);
    
    let result = compiler.compile_unary_expression(&bitnot_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Int64);
    assert!(!result.is_constant);
}

#[traced_test]
#[test]
fn test_variable_access_compilation() {
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let variable_manager = Rc::new(RefCell::new(VariableManager::new(&context, &module, &builder)));
    
    let mut compiler = LlvmExpressionCompiler::new(&context, &module, &builder, variable_manager.clone());
    
    // Create a basic block for IR generation
    let function_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_func", function_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Set current function in variable manager
    variable_manager.borrow_mut().set_current_function(Some(function));
    
    // Declare a variable first
    let var_name = Identifier::new("x".to_string(), "x".to_string());
    let var_value = Some(Box::new(Literal::integer(42)) as Box<dyn cursed::ast::traits::Expression>);
    let let_stmt = LetStatement::new(
        "sus".to_string(),
        var_name.clone(),
        var_value,
        None,
    );
    
    // Declare the variable
    let _alloca = variable_manager.borrow_mut().declare_variable(&let_stmt).unwrap();
    
    // Now test variable access
    let identifier = Identifier::new("x".to_string(), "x".to_string());
    let result = compiler.compile_identifier(&identifier).unwrap();
    
    // The result should be a loaded value
    assert!(!result.is_constant);
    // Type depends on the variable's type
    assert!(matches!(result.value_type, LlvmType::Int64));
}

#[traced_test]
#[test]
fn test_type_resolution() {
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let variable_manager = Rc::new(RefCell::new(VariableManager::new(&context, &module, &builder)));
    
    let compiler = LlvmExpressionCompiler::new(&context, &module, &builder, variable_manager);
    
    // Test integer arithmetic type resolution
    let result = compiler.resolve_binary_type(&LlvmType::Int64, &LlvmType::Int64, "+").unwrap();
    assert_eq!(result, LlvmType::Int64);
    
    // Test float arithmetic type resolution
    let result = compiler.resolve_binary_type(&LlvmType::Float64, &LlvmType::Float64, "*").unwrap();
    assert_eq!(result, LlvmType::Float64);
    
    // Test comparison type resolution
    let result = compiler.resolve_binary_type(&LlvmType::Int64, &LlvmType::Int64, "==").unwrap();
    assert_eq!(result, LlvmType::Boolean);
    
    // Test logical operation type resolution
    let result = compiler.resolve_binary_type(&LlvmType::Boolean, &LlvmType::Boolean, "&&").unwrap();
    assert_eq!(result, LlvmType::Boolean);
    
    // Test bitwise operation type resolution
    let result = compiler.resolve_binary_type(&LlvmType::Int64, &LlvmType::Int64, "&").unwrap();
    assert_eq!(result, LlvmType::Int64);
}

#[traced_test]
#[test]
fn test_error_propagation_declarations() {
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let variable_manager = Rc::new(RefCell::new(VariableManager::new(&context, &module, &builder)));
    
    let mut compiler = LlvmExpressionCompiler::new(&context, &module, &builder, variable_manager);
    
    // Add error propagation declarations
    compiler.add_error_propagation_declarations();
    
    // Verify that the functions were declared in the module
    assert!(module.get_function("cursed_error_propagation_init").is_some());
    assert!(module.get_function("cursed_error_propagation_cleanup").is_some());
    assert!(module.get_function("cursed_error_propagation").is_some());
    assert!(module.get_function("cursed_error_propagation_panic").is_some());
}

#[traced_test]
#[test]
fn test_complex_expression() {
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let variable_manager = Rc::new(RefCell::new(VariableManager::new(&context, &module, &builder)));
    
    let mut compiler = LlvmExpressionCompiler::new(&context, &module, &builder, variable_manager);
    
    // Create a basic block for IR generation
    let function_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_func", function_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Test complex expression: (10 + 20) * 2
    let left_inner = Box::new(Literal::integer(10));
    let right_inner = Box::new(Literal::integer(20));
    let add_expr = BinaryExpression::new("+".to_string(), left_inner, right_inner);
    
    let left_outer = Box::new(add_expr);
    let right_outer = Box::new(Literal::integer(2));
    let mul_expr = BinaryExpression::new("*".to_string(), left_outer, right_outer);
    
    let result = compiler.compile_binary_expression(&mul_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Int64);
    assert!(!result.is_constant);
}

#[traced_test]
#[test]
fn test_expression_context() {
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let variable_manager = Rc::new(RefCell::new(VariableManager::new(&context, &module, &builder)));
    
    let mut compiler = LlvmExpressionCompiler::new(&context, &module, &builder, variable_manager);
    
    // Test context functionality
    let initial_temp = compiler.get_context().temp_counter;
    
    let _new_temp = compiler.get_context_mut().next_temp();
    assert!(compiler.get_context().temp_counter > initial_temp);
    
    // Test source location setting
    let location = cursed::debug::SourceLocation::default();
    compiler.set_location(location);
    assert!(compiler.get_context().current_location.is_some());
}

#[traced_test]
#[test]
fn test_llvm_type_strings() {
    use cursed::codegen::llvm::expression_compiler::LlvmType;
    
    assert_eq!(LlvmType::Int32.to_llvm_string(), "i32");
    assert_eq!(LlvmType::Int64.to_llvm_string(), "i64");
    assert_eq!(LlvmType::Float64.to_llvm_string(), "double");
    assert_eq!(LlvmType::Boolean.to_llvm_string(), "i1");
    assert_eq!(LlvmType::String.to_llvm_string(), "i8*");
    assert_eq!(LlvmType::Void.to_llvm_string(), "void");
    
    let ptr_type = LlvmType::Pointer(Box::new(LlvmType::Int32));
    assert_eq!(ptr_type.to_llvm_string(), "i32*");
    
    let func_type = LlvmType::Function {
        return_type: Box::new(LlvmType::Int32),
        param_types: vec![LlvmType::Int64, LlvmType::Float64],
    };
    assert_eq!(func_type.to_llvm_string(), "i32 (i64, double)");
}
