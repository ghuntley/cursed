/// LLVM Basic Expressions Test Suite for CURSED
/// 
/// This test suite focuses on fundamental expression compilation that forms
/// the building blocks of all other language features. These tests validate
/// that basic arithmetic, logical operations, and simple variable operations
/// compile correctly to LLVM IR.
///
/// The importance of these tests cannot be overstated - they verify that the
/// most fundamental operations work correctly. If basic expressions fail,
/// the entire language becomes non-functional.

#[path = "common.rs"]
mod common;

use cursed::{
    ast::{
        expressions::{Literal, LiteralValue},
        operators::{BinaryExpression, UnaryExpression},
        identifiers::Identifier,
        traits::Expression,
    },
    codegen::llvm::{
        expression_compiler::{LlvmExpressionCompiler, LlvmValue, LlvmType},
        variable_management::VariableManager,
        type_system::TypeCompilationContext,
    },
    core::type_checker::Type,
    error::Error,
};
use inkwell::{
    context::Context,
    module::Module,
    builder::Builder,
    values::BasicValueEnum,
    AddressSpace,
};
use tracing::{debug, info, error};

/// Test helper to create minimal LLVM compilation context
struct BasicExpressionContext<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    expression_compiler: LlvmExpressionCompiler<'ctx>,
}

impl<'ctx> BasicExpressionContext<'ctx> {
    fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("basic_expressions");
        let builder = context.create_builder();
        
        let variable_manager = VariableManager::new(context, &module, &builder);
        let type_context = TypeCompilationContext::new(context, &module);
        
        let expression_compiler = LlvmExpressionCompiler::new(
            context,
            &module,
            &builder,
            &variable_manager,
            &type_context,
        );
        
        Self {
            context,
            module,
            builder,
            expression_compiler,
        }
    }
    
    fn create_test_function(&mut self, name: &str) {
        let fn_type = self.context.i32_type().fn_type(&[], false);
        let function = self.module.add_function(name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
    }
}

/// Test basic integer operations that are fundamental to all math
#[test]
fn test_basic_integer_operations() {
    common::init_tracing!();
    info!("Testing basic integer operations");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_integers");
    
    // Test basic integer literal
    let int_literal = Literal {
        value: LiteralValue::Integer(123),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&int_literal);
    assert!(result.is_ok(), "Integer literal compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Int32);
    assert!(value.is_int_value());
    
    // Verify the constant value is correct
    if let BasicValueEnum::IntValue(int_val) = value.llvm_value {
        assert_eq!(int_val.get_sign_extended_constant().unwrap(), 123);
    }
    
    info!("Basic integer operations test passed");
}

/// Test floating point basic operations
#[test]
fn test_basic_float_operations() {
    common::init_tracing!();
    info!("Testing basic float operations");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_floats");
    
    // Test basic float literal
    let float_literal = Literal {
        value: LiteralValue::Float(123.456),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&float_literal);
    assert!(result.is_ok(), "Float literal compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Float64);
    assert!(value.is_float_value());
    
    // Verify the constant value is correct
    if let BasicValueEnum::FloatValue(float_val) = value.llvm_value {
        let constant = float_val.get_constant().unwrap();
        assert!((constant - 123.456).abs() < 0.0001);
    }
    
    info!("Basic float operations test passed");
}

/// Test string literal compilation - important for message output
#[test]
fn test_basic_string_operations() {
    common::init_tracing!();
    info!("Testing basic string operations");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_strings");
    
    // Test basic string literal
    let string_literal = Literal {
        value: LiteralValue::String("Hello World".to_string()),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&string_literal);
    assert!(result.is_ok(), "String literal compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::String);
    
    info!("Basic string operations test passed");
}

/// Test boolean operations - critical for control flow
#[test]
fn test_basic_boolean_operations() {
    common::init_tracing!();
    info!("Testing basic boolean operations");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_booleans");
    
    // Test true literal
    let true_literal = Literal {
        value: LiteralValue::Boolean(true),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&true_literal);
    assert!(result.is_ok(), "True literal compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Bool);
    
    // Test false literal
    let false_literal = Literal {
        value: LiteralValue::Boolean(false),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&false_literal);
    assert!(result.is_ok(), "False literal compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Bool);
    
    info!("Basic boolean operations test passed");
}

/// Test simple arithmetic - addition, subtraction, multiplication
#[test]
fn test_simple_arithmetic() {
    common::init_tracing!();
    info!("Testing simple arithmetic operations");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_arithmetic");
    
    // Test addition: 2 + 3
    let add_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(2),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(3),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&add_expr);
    assert!(result.is_ok(), "Addition compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Int32);
    
    // Test subtraction: 10 - 3
    let sub_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(10),
            location: None,
        }) as Box<dyn Expression>,
        operator: "-".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(3),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&sub_expr);
    assert!(result.is_ok(), "Subtraction compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Int32);
    
    // Test multiplication: 4 * 5
    let mul_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(4),
            location: None,
        }) as Box<dyn Expression>,
        operator: "*".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&mul_expr);
    assert!(result.is_ok(), "Multiplication compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Int32);
    
    info!("Simple arithmetic test passed");
}

/// Test comparison operations - essential for conditionals
#[test]
fn test_simple_comparisons() {
    common::init_tracing!();
    info!("Testing simple comparison operations");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_comparisons");
    
    // Test equality: 5 == 5
    let eq_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        operator: "==".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&eq_expr);
    assert!(result.is_ok(), "Equality compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Bool);
    
    // Test inequality: 3 != 7
    let ne_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(3),
            location: None,
        }) as Box<dyn Expression>,
        operator: "!=".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(7),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&ne_expr);
    assert!(result.is_ok(), "Inequality compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Bool);
    
    // Test less than: 2 < 8
    let lt_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(2),
            location: None,
        }) as Box<dyn Expression>,
        operator: "<".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(8),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&lt_expr);
    assert!(result.is_ok(), "Less than compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Bool);
    
    info!("Simple comparisons test passed");
}

/// Test unary operations - negation and logical not
#[test]
fn test_simple_unary_operations() {
    common::init_tracing!();
    info!("Testing simple unary operations");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_unary");
    
    // Test numeric negation: -42
    let neg_expr = UnaryExpression {
        operator: "-".to_string(),
        operand: Box::new(Literal {
            value: LiteralValue::Integer(42),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_unary_expression(&neg_expr);
    assert!(result.is_ok(), "Negation compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Int32);
    
    // Test logical not: !true
    let not_expr = UnaryExpression {
        operator: "!".to_string(),
        operand: Box::new(Literal {
            value: LiteralValue::Boolean(true),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_unary_expression(&not_expr);
    assert!(result.is_ok(), "Logical not compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Bool);
    
    info!("Simple unary operations test passed");
}

/// Test float arithmetic operations
#[test]
fn test_float_arithmetic() {
    common::init_tracing!();
    info!("Testing float arithmetic operations");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_float_math");
    
    // Test float addition: 1.5 + 2.5
    let add_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Float(1.5),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Float(2.5),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&add_expr);
    assert!(result.is_ok(), "Float addition compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Float64);
    
    // Test float multiplication: 3.0 * 4.0
    let mul_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Float(3.0),
            location: None,
        }) as Box<dyn Expression>,
        operator: "*".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Float(4.0),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&mul_expr);
    assert!(result.is_ok(), "Float multiplication compilation failed: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Float64);
    
    info!("Float arithmetic test passed");
}

/// Test mixed type operations (if supported)
#[test]
fn test_mixed_type_expressions() {
    common::init_tracing!();
    info!("Testing mixed type expressions");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_mixed_types");
    
    // This test validates how the compiler handles type coercion
    // For now, we'll test that it at least doesn't crash
    
    // Test integer with float (may require type coercion)
    let mixed_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Float(2.5),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    // This might fail if type coercion isn't implemented, which is fine
    let result = test_ctx.expression_compiler.compile_binary_expression(&mixed_expr);
    
    // We don't assert success here since mixed types might not be supported yet
    match result {
        Ok(value) => {
            info!("Mixed type expression compiled successfully: {:?}", value.value_type);
        }
        Err(err) => {
            info!("Mixed type expression failed as expected: {:?}", err);
        }
    }
    
    info!("Mixed type expressions test completed");
}

/// Test that verifies basic expression compilation doesn't break LLVM module
#[test]
fn test_expression_module_validity() {
    common::init_tracing!();
    info!("Testing expression compilation preserves module validity");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_validity");
    
    // Compile a variety of basic expressions
    let expressions = vec![
        // Integer literals
        Literal { value: LiteralValue::Integer(42), location: None },
        Literal { value: LiteralValue::Integer(-10), location: None },
        Literal { value: LiteralValue::Integer(0), location: None },
        
        // Float literals
        Literal { value: LiteralValue::Float(3.14), location: None },
        Literal { value: LiteralValue::Float(-2.5), location: None },
        Literal { value: LiteralValue::Float(0.0), location: None },
        
        // Boolean literals
        Literal { value: LiteralValue::Boolean(true), location: None },
        Literal { value: LiteralValue::Boolean(false), location: None },
        
        // String literals
        Literal { value: LiteralValue::String("test".to_string()), location: None },
        Literal { value: LiteralValue::String("".to_string()), location: None },
    ];
    
    let mut compiled_count = 0;
    for expr in expressions {
        let result = test_ctx.expression_compiler.compile_literal(&expr);
        if result.is_ok() {
            compiled_count += 1;
        }
    }
    
    assert!(compiled_count > 0, "Should have compiled at least some expressions");
    
    // Verify the module can be printed without crashing
    let llvm_ir = test_ctx.module.print_to_string();
    let ir_string = llvm_ir.to_string();
    
    assert!(!ir_string.is_empty(), "LLVM IR should not be empty");
    assert!(ir_string.contains("define"), "LLVM IR should contain function definitions");
    
    info!("Compiled {} expressions successfully, generated {} bytes of IR", 
          compiled_count, ir_string.len());
}

/// Test error conditions and edge cases
#[test]
fn test_basic_error_conditions() {
    common::init_tracing!();
    info!("Testing basic error conditions in expression compilation");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_errors");
    
    // Test unsupported operator (should fail gracefully)
    let bad_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        operator: "@@".to_string(), // Invalid operator
        right: Box::new(Literal {
            value: LiteralValue::Integer(3),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&bad_expr);
    // This should fail gracefully
    assert!(result.is_err(), "Expected error for invalid operator");
    
    info!("Basic error conditions test passed");
}

/// Performance test for basic expression compilation
#[test]
fn test_basic_expression_performance() {
    common::init_tracing!();
    info!("Testing basic expression compilation performance");
    
    let context = Context::create();
    let mut test_ctx = BasicExpressionContext::new(&context);
    test_ctx.create_test_function("test_performance");
    
    let start_time = std::time::Instant::now();
    
    // Compile many basic expressions to test performance
    for i in 0..100 {
        let expr = Literal {
            value: LiteralValue::Integer(i),
            location: None,
        };
        
        let result = test_ctx.expression_compiler.compile_literal(&expr);
        assert!(result.is_ok(), "Expression compilation failed at iteration {}", i);
    }
    
    let elapsed = start_time.elapsed();
    info!("Compiled 100 expressions in {:?}", elapsed);
    
    // Should be fast - less than 1 second for 100 basic expressions
    assert!(elapsed.as_secs() < 1, "Expression compilation too slow: {:?}", elapsed);
}
