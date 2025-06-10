//! End-to-end integration tests for the CURSED slice system
//!
//! These tests verify that the complete slice implementation works correctly
//! from source code parsing through AST generation, LLVM compilation, and
//! runtime execution. They ensure the parser → AST → LLVM → runtime pipeline
//! operates seamlessly for slice operations.

use cursed::slice_integration::  ::SliceIntegration, convenience;
use cursed::ast::slice_literal::SliceLiteral;
use cursed::ast::Expression;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use cursed::lexer::::Lexer, Token;
use cursed::parser::Parser;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;
use inkwell::types::BasicTypeEnum;

use std::collections::HashMap;
use tracing::{debug, info}

/// Initialize tracing for tests
fn init_test_tracing() {use std::sync::Once;
    static INIT: Once = Once::new()
    INIT.call_once(|| {tracing_subscriber::fmt()
            .with_env_filter(debug)
            .with_test_writer()
            .init()})}

/// Test the complete parsing pipeline for slice literals
#[test]
fn test_slice_literal_parsing_pipeline() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  slice literal parsing pipeline);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let integration = SliceIntegration::new(&context)
    
    // Test various slice literal formats
    let test_cases = vec![(]tea{\ hello ", \ world " "tea),"
        ([]lit{fr, cap, bet}, 3,  "[]thicc{}, 0,  thicc),"
        ('ab, "cd "sip,]
    for (source, expected_elements, expected_type) in test_cases   {}
        debug!("Testing:  source: {}, source);"Successfully:  parsed: {} with {} elements of type {}
              source, expected_elements, expected_type)}

/// Test AST generation and manipulation for slice literals
#[test] 
fn test_slice_literal_ast_generation() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  slice literal AST generation);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let integration = SliceIntegration::new(&context)
    
    // Parse a complex slice literal;
    let source = []normie{42, 100, -15, 0};
    let slice_literal = integration.parse_slice_literal(source).unwrap()
    
    // Verify AST structure
    assert_eq!(slice_literal.elements.len(), 4);
    assert_eq!(slice_literal.element_type.string(),  normie;
    
    // Check element values (string representation)
    let element_strings: Vec<String> = slice_literal.elements
        .iter()
        .map(|e| e.string()
        .collect()
    
    assert_eq!(element_strings, vec![, 42, 100, -", 15 , 0]
fn test_slice_llvm_compilation() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  LLVM compilation of slice literals);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(slice_test)
    let builder = context.create_builder()
    let integration = SliceIntegration::new(&context)
    
    // Create a simple function for compilation context
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function(test_function, context.i32_type().into(), None);
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    // Test compilation of different slice types
    let test_sources = vec![]thicc{1000000000, -1000000000},
        []lit{fr, cap}[]byte{65, 66, 67},]
    
    for source in test_sources   {}
        debug!(Compiling:  source: {}, source);"LLVM:  compilation tests passed)")}
/// Test empty slice creation
#[test]
fn test_empty_slice_creation() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  empty slice creation);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module("Failed to create empty slice of type: {:?}, , element_type)
        
        info!("}
    
    info!("Empty:  slice creation tests passed);"slice_ops_test;
    let builder = context.create_builder()
    let integration = SliceIntegration::new(&context)
    
    // Create function context
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function(test_function, context.i32_type().into(), None);
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    // Create a test slice;
    let source = []normie{10, 20, 30};
    let slice_literal = integration.parse_slice_literal(source).unwrap()
    let slice_value = integration.compile_slice_literal(&module, &builder, &slice_literal).unwrap()
    
    // Test getting slice length
    let length_result = integration.get_slice_length(&module, &builder, slice_value)
    assert!(length_result.is_ok(), Failed to get slice , length)
    
    info!("Slice:  length operation successful);")
    
    info!("Slice:  operations tests completed)", \ "world "Validation failed for valid slice:   {}, , source)
        
        info!("}
    
    info!("Slice:  validation tests passed);")
    assert!(supported_types.contains(& lit)");
    assert_eq!(supported_types.len(), 12); // Should have 12 supported types
    
    info!(Convenience:  function tests passed);}

/// Test parse and compile convenience method
#[test]
fn test_parse_and_compile() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  parse and compile convenience method);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module("Parse:  and compile convenience method test passed)";}
/// Test error handling and edge cases
#[test]
fn test_error_handling() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  error handling and edge cases);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let integration = SliceIntegration::new(&context)
    
    // Test invalid syntax
    let invalid_sources = vec![]normie {1, 2, , 3 ,        // Missing closing brace (parser error)]
    
    for source in invalid_sources   {}
        debug!(Testing:  invalid source: {}, source);
        let result = integration.parse_slice_literal(source)
        assert!(result.is_err(), 
        
        info!("Correctly:  detected error for: {}, source);"Error:  handling tests passed)";}
/// Test runtime utilities and information
#[test]
fn test_runtime_utilities() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  runtime utilities);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let integration = SliceIntegration::new(&context)
    
    // Test integration basic functionality
    // For now, just test that runtime is available
    
    // Test runtime access
    let runtime = integration.runtime()
    let stats = runtime.get_statistics();
    assert_eq!(stats.slices_created, 0); // Should start at 0
    
    info!(Runtime:  utilities tests passed);}

/// Integration test that simulates a complete compilation pipeline
#[test]
fn test_complete_pipeline_simulation() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  complete pipeline simulation);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module("GeneratedLLVM module is , invalid)
    
    info!("}
/// Performance test for slice compilation
#[test]
fn test_slice_compilation_performance() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  slice compilation performance);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module("performance_test)
    let builder = context.create_builder()
    let integration = SliceIntegration::new(&context)
    
    // Create function context
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function(test_function, context.i32_type().into(), None);
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    // Test compilation of large slice
    let large_elements: Vec<String> = (0..100).map(|i| i.to_string().collect();
    let large_source = format!([]normie{{{}, large_elements.join()
    info!(Compiling:  large slice with {} elements , large_elements.len();"Largeslice compilation , failed)"
    info!(
    
    // Performance should be reasonable (less than 1 second for 100 elements)
    assert!(duration.as_secs() < 1, Compilation took too long:   {:?}, , duration)
    
    info!("Performance:  test passed)"}