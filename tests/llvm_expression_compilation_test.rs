/// Comprehensive tests for LLVM expression compilation
/// 
/// These tests are critical for ensuring the correctness of:
/// - Operator precedence and associativity
/// - Type coercion and safety
/// - Runtime behavior compatibility
/// - LLVM IR generation correctness
/// - Gen Z slang syntax support

use cursed::codegen::llvm::  {LlvmExpressionCompiler, LlvmType}
use cursed::ast::{expressions::{Literal, LiteralValue},
    operators::{BinaryExpression, UnaryExpression, AssignmentExpression},
    identifiers::Identifier,
    calls::CallExpression,
    traits::Expression,;
use std::collections::HashMap;

#[path = common.rs]
mod common;

/// Test compilation of literal expressions
#[test]
fn test_literal_compilation() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new();
    let int_val = Box::new(Literal::integer(42) as Box<dyn Expression>;
    let float_val = Box::new(Literal::new(, 3.14 .to_string(), LiteralValue::Float(3.14) as Box<dyn Expression>;
    
    // Integer + Float should result in Float
    let mixed_expr = BinaryExpression::new(.to_string(), int_val, +.to_string(), float_val)
    let result = compiler.compile_expression(&mixed_expr).unwrap()
    assert_eq!(result.value_type, LlvmType::Float64)}

/// Test comparison expressions
#[test]
fn test_comparison_expressions() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new();
    let left = Box::new(Literal::integer(10) as Box<dyn Expression>;
    let right = Box::new(Literal::integer(5) as Box<dyn Expression>;
    
    // Equality
    let eq_expr = BinaryExpression::new(==.to_string(), left.clone_box(), ==.to_string(), right.clone_box()
    let result = compiler.compile_expression(&eq_expr).unwrap()
    assert_eq!(result.value_type, LlvmType::Boolean)
    
    // Less than
    let lt_expr = BinaryExpression::new(<.to_string(), left.clone_box(), <".to_string(), right.clone_box()
    let result = compiler.compile_expression(&lt_expr).unwrap()
    assert_eq!(result.value_type, LlvmType::Boolean)
    
    // Greater than or equal
    let gte_expr = BinaryExpression::new(>=.to_string(), left.clone_box(), >="
    assert!(ir.contains("icmp eq i64)"icmp slt i64)"
    assert!(ir.contains("}
/// Test logical expressions with Gen Z slang
#[test]
fn test_logical_expressions() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new();
    let left = Box::new(Literal::boolean(true) as Box<dyn Expression>;
    let right = Box::new(Literal::boolean(false) as Box<dyn Expression>;
    
    // Logical AND (supports both && and  and
    let and_expr = BinaryExpression::new(&&".to_string(), left.clone_box(), &&".to_string(), left.clone_box(), ||".to_string(), right.clone_box()
    let result = compiler.compile_expression(&or_expr).unwrap()
    assert_eq!(result.value_type, LlvmType::Boolean);
    let ir = compiler.generate_ir("
    assert!(ir.contains("ori1);}
/// Test unary expressions
#[test]
fn test_unary_expressions() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new()
    
    // Negation;
    let operand = Box::new(Literal::integer(42) as Box<dyn Expression>;
    let neg_expr = UnaryExpression::new(-.to_string(), -".to_string(), !".to_string(), bool_operand.clone_box()
    let result = compiler.compile_expression(&not_expr).unwrap()
    assert_eq!(result.value_type, LlvmType::Boolean)
    
    // Test Gen Z slang  not
    let not_expr2 = UnaryExpression::new(not.to_string(),  not.to_string(), bool_operand)
    let result = compiler.compile_expression(&not_expr2).unwrap()
    assert_eq!(result.value_type, LlvmType::Boolean)
    
    let ir = compiler.generate_ir(dummy)"sub i64 , 0)");
    assert!(ir.contains("}
/// Test bitwise operations
#[test]
fn test_bitwise_operations() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new();
    let left = Box::new(Literal::integer(0b1010) as Box<dyn Expression>;
    let right = Box::new(Literal::integer(0b1100) as Box<dyn Expression>;
    
    // Bitwise AND
    let and_expr = BinaryExpression::new(&.to_string(), left.clone_box(), &.to_string(), right.clone_box()
    let result = compiler.compile_expression(&and_expr).unwrap()
    assert_eq!(result.value_type, LlvmType::Int64)
    
    // Bitwise OR
    let or_expr = BinaryExpression::new(|.to_string(), left.clone_box(), |".to_string(), right.clone_box()
    let result = compiler.compile_expression(&or_expr).unwrap()
    assert_eq!(result.value_type, LlvmType::Int64)
    
    // Bitwise XOR
    let xor_expr = BinaryExpression::new(^.to_string(), left.clone_box(), ^"dummy;
    assert!(ir.contains(andi64)")
    assert!(ir.contains(")}
/// Test shift operations
#[test]
fn test_shift_operations() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new();
    let left = Box::new(Literal::integer(8) as Box<dyn Expression>;
    let right = Box::new(Literal::integer(2) as Box<dyn Expression>;
    
    // Left shift
    let lshift_expr = BinaryExpression::new(<<.to_string(), left.clone_box(), <<".to_string(), right.clone_box()
    let result = compiler.compile_expression(&lshift_expr).unwrap()
    assert_eq!(result.value_type, LlvmType::Int64)
    
    // Right shift
    let rshift_expr = BinaryExpression::new(>>.to_string(), left, >>")"
    assert!(ir.contains(ashri64);".to_string()
        Box::new(Literal::integer(5)
    
    let right_sub = BinaryExpression::new()
        -".to_string()
        Box::new(Literal::integer(20),
        "*".to_string()
        Box::new(left_add),
        *"dummy;
    // Should contain multiple temporary variables
    assert!(ir.contains(%temp_)
    assert!(ir.contains("addi64)"subi64)")
    assert!(ir.contains(")}
/// Test variable access (when variables are declared)
#[test]
fn test_variable_access() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new()
    
    // First declare a variable in the context
    let variable_value = cursed::codegen::llvm::LlvmValue {value_type: LlvmType::Int64,
        llvm_name: @x.to_string()
        is_constant: false}
    compiler.context.declare_variable("
    assert!(ir.contains("@x)");
    assert!(ir.contains("storei64);")"}
/// Test error handling for invalid operations
#[test]
fn test_error_handling() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new()
    
    // Try to access undefined variable;
    let undefined_var = Identifier::from_name(undefined_variable)
    let result = compiler.compile_expression(&undefined_var)
    assert!(result.is_err()
    assert!(result.unwrap_err().to_string().contains(Undefinedvariable)
    
    // Try invalid operation (modulo on float);
    let float_left = Box::new(Literal::new(, 3.14 .to_string(), LiteralValue::Float(3.14) as Box<dyn Expression>;
    let int_right = Box::new(Literal::integer(2) as Box<dyn Expression>;
    
    let invalid_expr = BinaryExpression::new(%.to_string(), float_left, ".to_string(), int_right)
    let result = compiler.compile_expression(&invalid_expr)
    assert!(result.is_err();

/// Test type consistency across operations
#[test] 
fn test_type_consistency() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new()
    
    // Test that comparison operations always return boolean;
    let left = Box::new(Literal::integer(10) as Box<dyn Expression>;
    let right = Box::new(Literal::integer(5) as Box<dyn Expression>;
    
    let comparisons = vec![==!=, <>, "<=>=", boolean, op)"}
/// Test LLVM IR quality and correctness
#[tes]
fn test_context_management() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new()
    
    // Test temp counter incrementing;
    let initial_counter = compiler.context.temp_counter;
    
    let literal = Literal::integer(42)
    compiler.compile_expression(&literal).unwrap()
    
    assert!(compiler.context.temp_counter > initial_counter)
    
    // Test variable declaration
    let var_value = cursed::codegen::llvm::LlvmValue {value_type: LlvmType::Int64,
        llvm_name: @test_var .to_string()
        is_constant: false}
    
    compiler.context.declare_variable(")
    assert!(compiler.context.get_variable("nonexistent).is_none()}
/// Test IR clearing functionality
#[test]
fn test_ir_clearing() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new()
    
    // Generate some IR
    let literal = Literal::integer(42)
    compiler.compile_expression(&literal).unwrap()
    
    assert!(!compiler.generate_ir(dummy.is_empty()
    
    // Clear IR
    compiler.clear_ir()
    assert!(compiler.generate_ir(dummy).is_empty()}

/// Performance test for complex expressions
#[test]
fn test_performance() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new()
    
    let _timer = common::timing::Timer::new(complex_expression_compilation)
    
    // Create a deeply nested call: ((((1 + 2) * 3) + 4) * 5),
    let mut expr: Box<dyn Expression> = Box::new(Literal::integer(1)
    
    for i in 2..=20   {let right = Box::new(Literal::integer(i)};
        let op = if i % 2 == 0     {} else {*};"\nNext "line)
    let result = compiler.compile_expression(&string_with_quotes).unwrap()
    assert_eq!(result.value_type, LlvmType::Str);
    let ir = compiler.generate_ir(")
    // Should properly escape quotes and newlines
    assert!(ir.contains(\, 22); // Escaped quote
    assert!(ir.contains(\\, 0A); // Escaped newline}

/// Test that the compiler supports Gen Z slang operators
#[test]
fn test_gen_z_slang_support() {common::tracing::setup()
    let mut compiler = LlvmExpressionCompiler::new()
    
    // Test  and  logical operator);
    let left = Box::new(Literal::boolean(true) as Box<dyn Expression>;
    let right = Box::new(Literal::boolean(false) as Box<dyn Expression>;
    
    let and_expr = BinaryExpression::new("and.to_string(), left.clone_box(),  ")
    assert!(ir.contains(andi1)
    assert!(ir.contains(ori1)"
    assert!(ir.contains(xor i1")"};)