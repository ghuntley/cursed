/// Comprehensive tests for LLVM expression compilation
/// 
/// These tests are critical for ensuring the correctness of:
/// - Operator precedence and associativity
/// - Type coercion and safety
/// - Runtime behavior compatibility
/// - LLVM IR generation correctness
/// - Gen Z slang syntax support

use cursed::codegen::llvm::  {LlvmExpressionCompiler, LlvmType}
use cursed::ast::{expressions::{Literal, LiteralValue},}
    operators::{BinaryExpression, UnaryExpression, AssignmentExpression},
    identifiers::Identifier,
    calls::CallExpression,
    traits::Expression,;
use std::collections::HashMap;

#[path = common.rs]
mod common;

/// Test compilation of literal expressions
#[test]
fn test_literal_compilation() {common::tracing::setup(})
    let mut compiler = LlvmExpressionCompiler::new();
    let int_val = Box::new(Literal::integer(42) as Box<dyn Expression>;)
    let float_val = Box::new(Literal::new(, 3.14 .to_string(), LiteralValue::Float(3.14) as Box<dyn Expression>;))
    
    // Integer + Float should result in Float
    let mixed_expr = BinaryExpression::new(.to_string(), int_val, +.to_string(), float_val)
    let result = compiler.compile_expression(&mixed_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Float64)}

/// Test comparison expressions
#[test]
fn test_comparison_expressions() {common::tracing::setup(})
    let mut compiler = LlvmExpressionCompiler::new();
    let left = Box::new(Literal::integer(10) as Box<dyn Expression>;)
    let right = Box::new(Literal::integer(5) as Box<dyn Expression>;)
    
    // Equality
    let eq_expr = BinaryExpression::new(==.to_string(), left.clone_box(), ==.to_string(), right.clone_box();)
    let result = compiler.compile_expression(&eq_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Boolean)
    
    // Less than
    let lt_expr = BinaryExpression::new(<.to_string(), left.clone_box(), <".to_string(), right.clone_box();)
    let gte_expr = BinaryExpression::new(>=.to_string(), left.clone_box(), >="")
    assert!(ir.contains(, " eq i64)"icmp slt i64)"
    let and_expr = BinaryExpression::new(&&".to_string(), left.clone_box(), &&.to_string(), left.clone_box(), ||.to_string(), right.clone_box()")
    let ir = compiler.generate_ir(")
    let neg_expr = UnaryExpression::new(-.to_string(), -".to_string(), !)
    let ir = compiler.generate_ir(dummy)", " i64 , 0)
    let or_expr = BinaryExpression::new(|.to_string(), left.clone_box(), |".to_string(), right.clone_box()")
    let xor_expr = BinaryExpression::new(^.to_string(), left.clone_box(), ^, ";")
    assert!(ir.contains(andi64)"")
    let lshift_expr = BinaryExpression::new(<<.to_string(), left.clone_box(), <<.to_string(), right.clone_box()")
    let rshift_expr = BinaryExpression::new(>>.to_string(), left, >>")
    assert!(ir.contains(ashri64);".to_string()")
        -.to_string()""
        *"
        *", ;"
    assert!(ir.contains("addi64), ")
    compiler.context.declare_variable("")
    assert!(ir.contains(@x)")
    let invalid_expr = BinaryExpression::new(%.to_string(), float_left, ".to_string(), int_right)
    let comparisons = vec![==!=, <>, "<=>=", boolean, op)]"
    compiler.context.declare_variable(")
    assert!(compiler.context.get_variable(", ".is_none()}))
        let op = if i % 2 == 0     {} else {*};"\\nNext ", fixed
    let ir = compiler.generate_ir("")
    let and_expr = BinaryExpression::new(, ".to_string(), left.clone_box(),  ")
    assert!(ir.contains(ori1)"")
    assert!(ir.contains(xor i1fixed"))