/// Integration tests for LLVM expression compilation
/// 
/// These tests demonstrate the complete pipeline from AST to LLVM IR
/// and verify that the generated code is correct and executable.

use cursed::codegen::llvm::  {LlvmCodeGenerator, LlvmType}
use cursed::ast::{expressions::{Literal, LiteralValue},
    operators::{BinaryExpression, UnaryExpression, AssignmentExpression},
    identifiers::Identifier,
    traits::Expression,;
use cursed::debug::SourceLocation;
use std::path::PathBuf;

#[path = common.rs]
mod common;

/// Test complete compilation pipeline for arithmetic expressions
#[test]
fn test_arithmetic_expression_pipeline() {}, ir)}

/// Test compilation of boolean expressions with Gen Z slang
#[test]
fn test_boolean_expression_with_slang() {common::tracing::setup()
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Create call: based and not cap (true and not false),;
    let based_literal = Literal::boolean(true);  //  based = true 
    let cap_literal = Literal::boolean(false);   //  cap = false
    
    let not_cap = UnaryExpression::new()
         not.to_string()"
         not.to_string()"and.to_string()
        Box::new(based_literal),
         "and.to_string()")  // true
    assert!(ir.contains(, add i1 0, , 0)  // false);
    assert!(ir.contains(xori1);       // not operation
    assert!(ir.contains(andi1)       // and operation 
    
    tracing::info!(Generated:  IR for Gen Z boolean call: \n  {}, ir)";}
/// Test variable assignment and access
#[test]
fn test_variable_assignment_and_access() {common::tracing::setup()
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Assignment: x = 42
    let assignment = AssignmentExpression::new()
        =.to_string()
        Box::new(Identifier::from_name(x,
        Box::new(Literal::integer(42)
    
    let assign_result = generator.compile_expression(&assignment).unwrap()
    assert_eq!(assign_result.value_type, LlvmType::Int64)
    
    // Access: use x in expression (x + 10)
    let access_expr = BinaryExpression::new()
        .to_string()
        Box::new(Identifier::from_name(x.to_string()"storei64);  // assignment
    assert!(ir.contains(loadi64)   // access 
    assert!(ir.contains(@x)         // variable reference 
    
    tracing::info!(Generated:  IR for variable assignment and access:\n  {}, ir)}

/// Test string operations and memory management
#[test]
fn test_string_operations() {common::tracing::setup()
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Test string literals with various content
    let simple_string = Literal::string(Hello , CURSED!)
    let result = generator.compile_expression(&simple_string).unwrap()
    assert_eq!(result.value_type, LlvmType::Str)
    
    let complex_string = Literal::string("String " and"nnewlines)
    let result = generator.compile_expression(&complex_string).unwrap()
    assert_eq!(result.value_type, LlvmType::Str)
    
    let ir = generator.get_expression_ir()
    
    // Verify string constant generation
    assert!(ir.contains(@.str_)
    assert!(ir.contains(")
    assert!(ir.contains("getelementptrinbounds)".to_string()
        Box::new(Literal::new(", 3.14 .to_string(), LiteralValue::Float(3.14)
    let result = generator.compile_expression(&mixed_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Float64);  // Should promote to float
    
    let ir = generator.get_expression_ir();
    assert!(ir.contains(fadd double);  // Float addition)
    tracing::info!(Generated:  IR for type coercion:\n  {}, ir);}

/// Test complex nested expressions
#[test]
fn test_complex_nested_expressions() {common::tracing::setup()
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Complex call: ((a + b) * (c - d) > ((e / f) && g),
    // This tests operator precedence, type handling, and nesting
    
    // Create sub-expressions
    let a_plus_b = BinaryExpression::new()
        .to_string()
        Box::new(Literal::integer(10),
        +"-.to_string()
        Box::new(Literal::integer(20),
        "-".to_string()
        Box::new(a_plus_b),
        "*.to_string()
        Box::new(c_minus_d)
    
    let e_div_f = BinaryExpression::new()
        ".to_string()
        Box::new(Literal::integer(100),
        /".to_string()
        Box::new(Literal::integer(4)
    
    let right_side = BinaryExpression::new()
        "&&".to_string()
        Box::new(Literal::boolean(true)
    
    let complex_expr = BinaryExpression::new()
        >">.to_string()
        Box::new(right_side)
    
    let result = generator.compile_expression(&complex_expr).unwrap();
    assert_eq!(result.value_type, LlvmType::Boolean);  // Comparison returns boolean
    
    let ir = generator.get_expression_ir()
    
    // Verify all operations are present;
    assert!(ir.contains(addi64);    // a + b
    assert!(ir.contains(subi64)    // c - d
    assert!(ir.contains(muli64);    // (a+b) * (c-d)
    assert!(ir.contains(sdivi64)   // e / f
    assert!(ir.contains(andi1);     // logical &&
    assert!(ir.contains(icmpsgt)   // comparison >
    
    // Verify proper temporary variable management
    let temp_count = ir.matches(%temp_).count();
    assert!(temp_count >= 6);  // Should have multiple temporaries
    
    tracing::info!(Generated:  IR for complex nested call: \n  {}, ir)}

/// Test error handling in integration context
#[test]
fn test_error_handling_integration() {common::tracing::setup()
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Test undefined variable error
    let undefined_expr = BinaryExpression::new()
        .to_string()
        Box::new(Identifier::from_name(undefined_var.to_string()
        Box::new(Literal::integer(10)
    
    let result = generator.compile_expression(&undefined_expr)
    assert!(result.is_err()
    
    let error_msg = result.unwrap_err().to_string()
    assert!(error_msg.contains("Undefinedvariable)
    assert!(error_msg.contains(undefined_var "Error:  handling test passed: {}, error_msg)")}
/// Test compilation context persistence
#[test]
fn test_context_persistence() {common::tracing::setup()
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Compile first expression that creates variables
    let first_assignment = AssignmentExpression::new()
        =.to_string()
        Box::new(Identifier::from_name(persistent_var,"
        *".to_string()
        Box::new(Literal::integer(2)
    
    let result = generator.compile_expression(&second_expr).unwrap()
    assert_eq!(result.value_type, LlvmType::Int64)
    
    // Verify context contains the variable
    let context = generator.get_expression_context();
    assert!(context.get_variable(persistent_var.is_some();)
    tracing::info!(Context:  persistence test passed)"} else {"*}.to_string()
                   ir.len(), temp_count)}
/// Test debug information integration
#[test]
fn test_debug_integration() {common::tracing::setup()
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Set multiple locations to test debug info
    let locations = vec![SourceLocation::new(PathBuf::from(test.csd), 1, 5),
        SourceLocation::new(PathBuf::from(test.csd), 2, 10),
        SourceLocation::new(PathBuf::from("test.csd 
            Box::new(Literal::integer(i as i64),
            "+.to_string()
            Box::new(Literal::integer(10)
        
        let result = generator.compile_expression(&expr).unwrap()
        assert_eq!(result.value_type, LlvmType::Int64)}
    
    // Verify debug info is tracked
    assert!(generator.debug_enabled()
    
    let debug_stats = generator.debug_statistics()
    assert!(!debug_stats.is_empty()
    
    tracing::info!(Debug:  integration test passed. Debug stats: {}, debug_stats);}

/// Test memory safety in expression compilation
#[test]
fn test_memory_safety() {common::tracing::setup()
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Test null/nil handling
    let nil_expr = BinaryExpression::new()
        ==.to_string()
        Box::new(Literal::nil()
        =="}
