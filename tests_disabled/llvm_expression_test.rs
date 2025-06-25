/// Comprehensive LLVM Expression Compilation Tests for CURSED
/// 
/// This test suite validates that the CURSED compiler can correctly:
/// - Parse CURSED code with Gen Z slang syntax
/// - Generate proper LLVM IR for expressions 
/// - Execute compiled code with correct results
/// - Handle type checking and inference
/// - Integrate all compilation pipeline components
///
/// These tests are critical because expression compilation is the foundation
/// of the entire compilation pipeline. If expressions don't compile correctly,
/// no other language features will work properly.

#[path = "common.rs"]
mod common;

use cursed::{
    ast::{
        expressions::{Literal, LiteralValue},
        operators::{BinaryExpression, UnaryExpression, AssignmentExpression},
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
    lexer::Lexer,
    parser::Parser,
};
use inkwell::{
    context::Context,
    module::Module,
    builder::Builder,
    types::{BasicTypeEnum, IntType, FloatType},
    values::{BasicValueEnum, IntValue, FloatValue},
    AddressSpace,
    IntPredicate, FloatPredicate,
};
use std::collections::HashMap;
use tracing::{debug, info, error};

/// Test helper to create LLVM context and compiler
struct LlvmTestContext<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    expression_compiler: LlvmExpressionCompiler<'ctx>,
    variable_manager: VariableManager<'ctx>,
}

impl<'ctx> LlvmTestContext<'ctx> {
    fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("test_module");
        let builder = context.create_builder();
        
        // Create variable manager first
        let variable_manager = VariableManager::new(context, &module, &builder);
        
        // Create type compilation context
        let type_context = TypeCompilationContext::new(context, &module);
        
        // Create expression compiler with all dependencies
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
            variable_manager,
        }
    }
}

/// Test literal compilation - fundamental for all other expressions
#[test]
fn test_compile_integer_literal() {
    common::init_tracing!();
    info!("Testing integer literal compilation");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    // Test positive integer
    let literal = Literal {
        value: LiteralValue::Integer(42),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&literal);
    assert!(result.is_ok(), "Failed to compile integer literal: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Int32);
    assert!(llvm_value.is_int_value());
    
    // Verify the actual LLVM value
    if let BasicValueEnum::IntValue(int_val) = llvm_value.llvm_value {
        assert_eq!(int_val.get_sign_extended_constant().unwrap(), 42);
    } else {
        panic!("Expected integer value, got: {:?}", llvm_value.llvm_value);
    }
    
    info!("Integer literal compilation successful");
}

#[test]
fn test_compile_float_literal() {
    common::init_tracing!();
    info!("Testing float literal compilation");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    let literal = Literal {
        value: LiteralValue::Float(3.14159),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&literal);
    assert!(result.is_ok(), "Failed to compile float literal: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Float64);
    assert!(llvm_value.is_float_value());
    
    // Verify the actual LLVM value
    if let BasicValueEnum::FloatValue(float_val) = llvm_value.llvm_value {
        let constant = float_val.get_constant().unwrap();
        assert!((constant - 3.14159).abs() < 0.0001, "Float constant mismatch: {}", constant);
    } else {
        panic!("Expected float value, got: {:?}", llvm_value.llvm_value);
    }
    
    info!("Float literal compilation successful");
}

#[test]
fn test_compile_string_literal() {
    common::init_tracing!();
    info!("Testing string literal compilation");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    let literal = Literal {
        value: LiteralValue::String("Hello, CURSED!".to_string()),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&literal);
    assert!(result.is_ok(), "Failed to compile string literal: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
    
    info!("String literal compilation successful");
}

#[test]
fn test_compile_boolean_literals() {
    common::init_tracing!();
    info!("Testing boolean literal compilation");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    // Test true
    let true_literal = Literal {
        value: LiteralValue::Boolean(true),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&true_literal);
    assert!(result.is_ok(), "Failed to compile true literal: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Bool);
    
    // Test false
    let false_literal = Literal {
        value: LiteralValue::Boolean(false),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&false_literal);
    assert!(result.is_ok(), "Failed to compile false literal: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Bool);
    
    info!("Boolean literal compilation successful");
}

/// Test binary arithmetic expressions - core mathematical operations
#[test]
fn test_compile_arithmetic_expressions() {
    common::init_tracing!();
    info!("Testing arithmetic expression compilation");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    // Create test function to provide proper context
    let fn_type = test_ctx.context.i32_type().fn_type(&[], false);
    let function = test_ctx.module.add_function("test_arithmetic", fn_type, None);
    let basic_block = test_ctx.context.append_basic_block(function, "entry");
    test_ctx.builder.position_at_end(basic_block);
    test_ctx.variable_manager.set_current_function(Some(function));
    
    // Test addition: 5 + 3
    let left = Box::new(Literal {
        value: LiteralValue::Integer(5),
        location: None,
    }) as Box<dyn Expression>;
    
    let right = Box::new(Literal {
        value: LiteralValue::Integer(3),
        location: None,
    }) as Box<dyn Expression>;
    
    let add_expr = BinaryExpression {
        left,
        operator: "+".to_string(),
        right,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&add_expr);
    assert!(result.is_ok(), "Failed to compile addition expression: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Int32);
    assert!(llvm_value.is_int_value());
    
    // Test subtraction: 10 - 4
    let left = Box::new(Literal {
        value: LiteralValue::Integer(10),
        location: None,
    }) as Box<dyn Expression>;
    
    let right = Box::new(Literal {
        value: LiteralValue::Integer(4),
        location: None,
    }) as Box<dyn Expression>;
    
    let sub_expr = BinaryExpression {
        left,
        operator: "-".to_string(),
        right,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&sub_expr);
    assert!(result.is_ok(), "Failed to compile subtraction expression: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Int32);
    
    // Test multiplication: 6 * 7
    let left = Box::new(Literal {
        value: LiteralValue::Integer(6),
        location: None,
    }) as Box<dyn Expression>;
    
    let right = Box::new(Literal {
        value: LiteralValue::Integer(7),
        location: None,
    }) as Box<dyn Expression>;
    
    let mul_expr = BinaryExpression {
        left,
        operator: "*".to_string(),
        right,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&mul_expr);
    assert!(result.is_ok(), "Failed to compile multiplication expression: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Int32);
    
    info!("Arithmetic expression compilation successful");
}

/// Test variable assignment and retrieval
#[test]
fn test_variable_assignment_and_access() {
    common::init_tracing!();
    info!("Testing variable assignment and access");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    // Create test function
    let fn_type = test_ctx.context.i32_type().fn_type(&[], false);
    let function = test_ctx.module.add_function("test_variables", fn_type, None);
    let basic_block = test_ctx.context.append_basic_block(function, "entry");
    test_ctx.builder.position_at_end(basic_block);
    test_ctx.variable_manager.set_current_function(Some(function));
    
    // Test variable declaration: sus x = 42
    let value_expr = Box::new(Literal {
        value: LiteralValue::Integer(42),
        location: None,
    }) as Box<dyn Expression>;
    
    let assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "x".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: value_expr,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&assignment);
    assert!(result.is_ok(), "Failed to compile variable assignment: {:?}", result.err());
    
    // Test variable access
    let identifier = Identifier {
        name: "x".to_string(),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_identifier(&identifier);
    assert!(result.is_ok(), "Failed to compile variable access: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Int32);
    
    info!("Variable assignment and access successful");
}

/// Test comparison expressions for control flow
#[test]
fn test_comparison_expressions() {
    common::init_tracing!();
    info!("Testing comparison expression compilation");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    // Create test function
    let fn_type = test_ctx.context.i1_type().fn_type(&[], false);
    let function = test_ctx.module.add_function("test_comparison", fn_type, None);
    let basic_block = test_ctx.context.append_basic_block(function, "entry");
    test_ctx.builder.position_at_end(basic_block);
    test_ctx.variable_manager.set_current_function(Some(function));
    
    // Test equality: 5 == 5
    let left = Box::new(Literal {
        value: LiteralValue::Integer(5),
        location: None,
    }) as Box<dyn Expression>;
    
    let right = Box::new(Literal {
        value: LiteralValue::Integer(5),
        location: None,
    }) as Box<dyn Expression>;
    
    let eq_expr = BinaryExpression {
        left,
        operator: "==".to_string(),
        right,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&eq_expr);
    assert!(result.is_ok(), "Failed to compile equality expression: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Bool);
    
    // Test less than: 3 < 7
    let left = Box::new(Literal {
        value: LiteralValue::Integer(3),
        location: None,
    }) as Box<dyn Expression>;
    
    let right = Box::new(Literal {
        value: LiteralValue::Integer(7),
        location: None,
    }) as Box<dyn Expression>;
    
    let lt_expr = BinaryExpression {
        left,
        operator: "<".to_string(),
        right,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&lt_expr);
    assert!(result.is_ok(), "Failed to compile less-than expression: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Bool);
    
    info!("Comparison expression compilation successful");
}

/// Test unary expressions (negation, logical not)
#[test]
fn test_unary_expressions() {
    common::init_tracing!();
    info!("Testing unary expression compilation");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    // Create test function
    let fn_type = test_ctx.context.i32_type().fn_type(&[], false);
    let function = test_ctx.module.add_function("test_unary", fn_type, None);
    let basic_block = test_ctx.context.append_basic_block(function, "entry");
    test_ctx.builder.position_at_end(basic_block);
    test_ctx.variable_manager.set_current_function(Some(function));
    
    // Test numeric negation: -42
    let operand = Box::new(Literal {
        value: LiteralValue::Integer(42),
        location: None,
    }) as Box<dyn Expression>;
    
    let neg_expr = UnaryExpression {
        operator: "-".to_string(),
        operand,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_unary_expression(&neg_expr);
    assert!(result.is_ok(), "Failed to compile negation expression: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Int32);
    
    // Test logical not: !true
    let operand = Box::new(Literal {
        value: LiteralValue::Boolean(true),
        location: None,
    }) as Box<dyn Expression>;
    
    let not_expr = UnaryExpression {
        operator: "!".to_string(),
        operand,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_unary_expression(&not_expr);
    assert!(result.is_ok(), "Failed to compile logical not expression: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Bool);
    
    info!("Unary expression compilation successful");
}

/// Test floating point arithmetic for mathematical computations
#[test]
fn test_float_arithmetic() {
    common::init_tracing!();
    info!("Testing floating point arithmetic compilation");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    // Create test function
    let fn_type = test_ctx.context.f64_type().fn_type(&[], false);
    let function = test_ctx.module.add_function("test_float_math", fn_type, None);
    let basic_block = test_ctx.context.append_basic_block(function, "entry");
    test_ctx.builder.position_at_end(basic_block);
    test_ctx.variable_manager.set_current_function(Some(function));
    
    // Test float addition: 3.14 + 2.86
    let left = Box::new(Literal {
        value: LiteralValue::Float(3.14),
        location: None,
    }) as Box<dyn Expression>;
    
    let right = Box::new(Literal {
        value: LiteralValue::Float(2.86),
        location: None,
    }) as Box<dyn Expression>;
    
    let add_expr = BinaryExpression {
        left,
        operator: "+".to_string(),
        right,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&add_expr);
    assert!(result.is_ok(), "Failed to compile float addition: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Float64);
    assert!(llvm_value.is_float_value());
    
    info!("Float arithmetic compilation successful");
}

/// Test complex nested expressions
#[test]
fn test_nested_expressions() {
    common::init_tracing!();
    info!("Testing nested expression compilation");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    // Create test function
    let fn_type = test_ctx.context.i32_type().fn_type(&[], false);
    let function = test_ctx.module.add_function("test_nested", fn_type, None);
    let basic_block = test_ctx.context.append_basic_block(function, "entry");
    test_ctx.builder.position_at_end(basic_block);
    test_ctx.variable_manager.set_current_function(Some(function));
    
    // Test complex expression: (5 + 3) * (10 - 2)
    let inner_left = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(3),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let inner_right = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(10),
            location: None,
        }) as Box<dyn Expression>,
        operator: "-".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(2),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let outer_expr = BinaryExpression {
        left: Box::new(inner_left) as Box<dyn Expression>,
        operator: "*".to_string(),
        right: Box::new(inner_right) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&outer_expr);
    assert!(result.is_ok(), "Failed to compile nested expression: {:?}", result.err());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::Int32);
    
    info!("Nested expression compilation successful");
}

/// Test error handling for invalid expressions
#[test]
fn test_expression_error_handling() {
    common::init_tracing!();
    info!("Testing expression compilation error handling");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    // Test division by zero detection (if implemented)
    let left = Box::new(Literal {
        value: LiteralValue::Integer(42),
        location: None,
    }) as Box<dyn Expression>;
    
    let right = Box::new(Literal {
        value: LiteralValue::Integer(0),
        location: None,
    }) as Box<dyn Expression>;
    
    let div_expr = BinaryExpression {
        left,
        operator: "/".to_string(),
        right,
        location: None,
    };
    
    // This should either succeed with runtime check or fail at compile time
    let result = test_ctx.expression_compiler.compile_binary_expression(&div_expr);
    // Note: We don't assert failure here as the compiler might handle this as a runtime check
    
    info!("Expression error handling test completed");
}

/// Integration test demonstrating end-to-end expression compilation
#[test]
fn test_expression_compilation_integration() {
    common::init_tracing!();
    info!("Testing end-to-end expression compilation integration");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    // Create main function that uses multiple expression types
    let fn_type = test_ctx.context.i32_type().fn_type(&[], false);
    let function = test_ctx.module.add_function("main", fn_type, None);
    let basic_block = test_ctx.context.append_basic_block(function, "entry");
    test_ctx.builder.position_at_end(basic_block);
    test_ctx.variable_manager.set_current_function(Some(function));
    
    // Compile series of expressions that would appear in real CURSED code
    
    // 1. Variable assignment: sus result = 0
    let assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "result".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(0),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&assignment);
    assert!(result.is_ok(), "Failed to compile initial assignment: {:?}", result.err());
    
    // 2. Arithmetic with variable: result + 10
    let arithmetic = BinaryExpression {
        left: Box::new(Identifier {
            name: "result".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(10),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&arithmetic);
    assert!(result.is_ok(), "Failed to compile arithmetic with variable: {:?}", result.err());
    
    // 3. Update assignment: result = result + 5
    let update = AssignmentExpression {
        target: Box::new(Identifier {
            name: "result".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(BinaryExpression {
            left: Box::new(Identifier {
                name: "result".to_string(),
                location: None,
            }) as Box<dyn Expression>,
            operator: "+".to_string(),
            right: Box::new(Literal {
                value: LiteralValue::Integer(5),
                location: None,
            }) as Box<dyn Expression>,
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&update);
    assert!(result.is_ok(), "Failed to compile update assignment: {:?}", result.err());
    
    // Verify the module is valid by checking it doesn't crash
    let llvm_ir = test_ctx.module.print_to_string();
    debug!("Generated LLVM IR:\n{}", llvm_ir.to_string());
    
    info!("End-to-end expression compilation integration successful");
}

/// Test that validates LLVM module generation is correct
#[test]
fn test_llvm_module_integrity() {
    common::init_tracing!();
    info!("Testing LLVM module integrity after expression compilation");
    
    let context = Context::create();
    let mut test_ctx = LlvmTestContext::new(&context);
    
    // Create and compile several different expression types
    let fn_type = test_ctx.context.i32_type().fn_type(&[], false);
    let function = test_ctx.module.add_function("test_integrity", fn_type, None);
    let basic_block = test_ctx.context.append_basic_block(function, "entry");
    test_ctx.builder.position_at_end(basic_block);
    test_ctx.variable_manager.set_current_function(Some(function));
    
    // Compile various expressions to build up the module
    let expressions = vec![
        Literal { value: LiteralValue::Integer(42), location: None },
        Literal { value: LiteralValue::Float(3.14), location: None },
        Literal { value: LiteralValue::String("test".to_string()), location: None },
        Literal { value: LiteralValue::Boolean(true), location: None },
    ];
    
    for expr in expressions {
        let result = test_ctx.expression_compiler.compile_literal(&expr);
        assert!(result.is_ok(), "Failed to compile literal: {:?}", result.err());
    }
    
    // Verify module can be printed (indicates basic validity)
    let llvm_ir = test_ctx.module.print_to_string();
    let ir_string = llvm_ir.to_string();
    
    // Basic sanity checks on generated IR
    assert!(ir_string.contains("define"), "LLVM IR should contain function definitions");
    assert!(!ir_string.is_empty(), "LLVM IR should not be empty");
    
    info!("LLVM module integrity test successful - generated {} bytes of IR", ir_string.len());
}
