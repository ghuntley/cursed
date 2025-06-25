/// Comprehensive tests for LTO module cloning optimization
/// 
/// Tests the real module cloning implementation that replaces the previous
/// placeholder functionality in the LLVM LTO integration.

use cursed::codegen::llvm::lto_integration::{LlvmLtoIntegration, LtoConfig, LtoLevel};
use cursed::optimization::lto::{LtoConfig as OptLtoConfig, LtoLevel as OptLtoLevel};
use cursed::error::Result;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;
use inkwell::module::Linkage;
use inkwell::attributes::AttributeLoc;
use std::collections::HashMap;

#[path = "common.rs"]
pub mod common;

/// Test basic module cloning functionality
#[test]
fn test_basic_module_cloning() {
    common::tracing::setup();

    let context = Context::create();
    let config = create_test_lto_config();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();

    // Create source module with basic components
    let source_module = context.create_module("basic_test");
    
    // Add function declaration
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = source_module.add_function("test_func", fn_type, None);
    
    // Add global variable
    let global = source_module.add_global(i32_type, None, "test_global");
    global.set_initializer(&i32_type.const_int(42, false));
    global.set_constant(true);

    // Perform cloning
    let cloned_module = integration.clone_module_for_lto(&source_module).unwrap();

    // Verify module name
    assert_eq!(cloned_module.get_name().to_str().unwrap(), "basic_test_lto");

    // Verify function was cloned
    let cloned_function = cloned_module.get_function("test_func");
    assert!(cloned_function.is_some());
    
    // Verify global was cloned
    let cloned_global = cloned_module.get_global("test_global");
    assert!(cloned_global.is_some());
    let global_val = cloned_global.unwrap();
    assert!(global_val.is_constant());
    
    tracing::info!("Basic module cloning test completed successfully");
}

/// Test cloning module with complex function bodies
#[test]
fn test_function_body_cloning() {
    common::tracing::setup();

    let context = Context::create();
    let config = create_test_lto_config();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();

    // Create source module with function that has body
    let source_module = context.create_module("function_body_test");
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = source_module.add_function("add_numbers", fn_type, None);

    // Create function body with multiple basic blocks
    let entry_block = context.append_basic_block(function, "entry");
    let then_block = context.append_basic_block(function, "then");
    let else_block = context.append_basic_block(function, "else");
    let merge_block = context.append_basic_block(function, "merge");

    let builder = context.create_builder();

    // Entry block: check if first parameter is positive
    builder.position_at_end(entry_block);
    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    let zero = i32_type.const_int(0, false);
    let cmp = builder.build_int_compare(inkwell::IntPredicate::SGT, param1, zero, "is_positive").unwrap();
    builder.build_conditional_branch(cmp, then_block, else_block).unwrap();

    // Then block: add parameters
    builder.position_at_end(then_block);
    let sum = builder.build_int_add(param1, param2, "sum").unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();

    // Else block: subtract parameters
    builder.position_at_end(else_block);
    let diff = builder.build_int_sub(param1, param2, "diff").unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();

    // Merge block: phi node and return
    builder.position_at_end(merge_block);
    let phi = builder.build_phi(i32_type, "result").unwrap();
    phi.add_incoming(&[(&sum, then_block), (&diff, else_block)]);
    builder.build_return(Some(&phi.as_basic_value())).unwrap();

    // Clone the module
    let cloned_module = integration.clone_module_for_lto(&source_module).unwrap();

    // Verify cloned function structure
    let cloned_function = cloned_module.get_function("add_numbers").unwrap();
    assert_eq!(cloned_function.count_basic_blocks(), 4);
    assert_eq!(cloned_function.count_params(), 2);

    tracing::info!("Function body cloning test completed successfully");
}

/// Test cloning with various global variable types and attributes
#[test]
fn test_global_variable_cloning() {
    common::tracing::setup();

    let context = Context::create();
    let config = create_test_lto_config();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();

    // Create source module with various global types
    let source_module = context.create_module("globals_test");
    let i32_type = context.i32_type();
    let f64_type = context.f64_type();
    let bool_type = context.bool_type();

    // Integer global with initializer
    let int_global = source_module.add_global(i32_type, None, "int_constant");
    int_global.set_initializer(&i32_type.const_int(123, false));
    int_global.set_constant(true);
    int_global.set_linkage(Linkage::Internal);

    // Float global variable (mutable)
    let float_global = source_module.add_global(f64_type, None, "float_var");
    float_global.set_initializer(&f64_type.const_float(3.14159));
    float_global.set_constant(false);
    float_global.set_linkage(Linkage::External);

    // Boolean global
    let bool_global = source_module.add_global(bool_type, None, "flag");
    bool_global.set_initializer(&bool_type.const_int(1, false));
    bool_global.set_constant(true);

    // Clone the module
    let cloned_module = integration.clone_module_for_lto(&source_module).unwrap();

    // Verify integer global
    let cloned_int_global = cloned_module.get_global("int_constant").unwrap();
    assert!(cloned_int_global.is_constant());
    assert_eq!(cloned_int_global.get_linkage(), Linkage::Internal);

    // Verify float global
    let cloned_float_global = cloned_module.get_global("float_var").unwrap();
    assert!(!cloned_float_global.is_constant());
    assert_eq!(cloned_float_global.get_linkage(), Linkage::External);

    // Verify boolean global
    let cloned_bool_global = cloned_module.get_global("flag").unwrap();
    assert!(cloned_bool_global.is_constant());

    tracing::info!("Global variable cloning test completed successfully");
}

/// Test function attribute preservation during cloning
#[test]
fn test_function_attribute_preservation() {
    common::tracing::setup();

    let context = Context::create();
    let config = create_test_lto_config();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();

    // Create source module with attributed function
    let source_module = context.create_module("attributes_test");
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = source_module.add_function("attributed_func", fn_type, None);

    // Add function attributes
    let inline_attr = context.create_enum_attribute(
        inkwell::attributes::Attribute::get_named_enum_kind_id("alwaysinline"),
        0
    );
    function.add_enum_attribute(AttributeLoc::Function, inline_attr);

    let nounwind_attr = context.create_enum_attribute(
        inkwell::attributes::Attribute::get_named_enum_kind_id("nounwind"),
        0
    );
    function.add_enum_attribute(AttributeLoc::Function, nounwind_attr);

    // Clone the module
    let cloned_module = integration.clone_module_for_lto(&source_module).unwrap();

    // Verify attributes were preserved
    let cloned_function = cloned_module.get_function("attributed_func").unwrap();
    let function_attrs = cloned_function.get_enum_attributes(AttributeLoc::Function);
    
    // Should have at least the attributes we added
    assert!(function_attrs.len() >= 2);

    tracing::info!("Function attribute preservation test completed successfully");
}

/// Test LTO-specific optimizations during cloning
#[test]
fn test_lto_optimizations() {
    common::tracing::setup();

    let context = Context::create();
    let config = create_test_lto_config();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();

    // Create source module with optimization candidates
    let source_module = context.create_module("optimization_test");
    let i32_type = context.i32_type();

    // Small function that should get inlining hint
    let small_fn_type = i32_type.fn_type(&[], false);
    let small_function = source_module.add_function("small_func", small_fn_type, None);
    small_function.set_linkage(Linkage::Internal);

    let entry = context.append_basic_block(small_function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry);
    let const_val = i32_type.const_int(42, false);
    builder.build_return(Some(&const_val)).unwrap();

    // Global constant that should be optimized
    let global_const = source_module.add_global(i32_type, None, "local_constant");
    global_const.set_constant(true);
    global_const.set_initializer(&i32_type.const_int(100, false));
    global_const.set_linkage(Linkage::External);

    // Clone the module (this applies LTO optimizations)
    let cloned_module = integration.clone_module_for_lto(&source_module).unwrap();

    // Verify optimization was applied
    let optimized_function = cloned_module.get_function("small_func").unwrap();
    let optimized_global = cloned_module.get_global("local_constant").unwrap();

    // Function should exist and be properly cloned
    assert_eq!(optimized_function.count_basic_blocks(), 1);

    // Global should be properly cloned
    assert!(optimized_global.is_constant());

    tracing::info!("LTO optimizations test completed successfully");
}

/// Test cloning of empty modules
#[test]
fn test_empty_module_cloning() {
    common::tracing::setup();

    let context = Context::create();
    let config = create_test_lto_config();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();

    // Create completely empty module
    let empty_module = context.create_module("empty");

    // Clone should succeed even for empty modules
    let cloned_module = integration.clone_module_for_lto(&empty_module).unwrap();

    assert_eq!(cloned_module.get_name().to_str().unwrap(), "empty_lto");
    assert_eq!(cloned_module.get_functions().count(), 0);
    assert_eq!(cloned_module.get_globals().count(), 0);

    tracing::info!("Empty module cloning test completed successfully");
}

/// Test module validation after cloning
#[test]
fn test_cloned_module_validation() {
    common::tracing::setup();

    let context = Context::create();
    let config = create_test_lto_config();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();

    // Create valid source module
    let source_module = context.create_module("validation_test");
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = source_module.add_function("valid_func", fn_type, None);

    // Create valid function body
    let entry = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry);
    let param = function.get_nth_param(0).unwrap();
    builder.build_return(Some(&param)).unwrap();

    // Clone and validate
    let cloned_module = integration.clone_module_for_lto(&source_module).unwrap();

    // Explicit validation should pass
    let validation_result = integration.validate_cloned_module(&cloned_module);
    assert!(validation_result.is_ok());

    tracing::info!("Cloned module validation test completed successfully");
}

/// Test instruction cloning with various opcodes
#[test]
fn test_instruction_cloning_comprehensive() {
    common::tracing::setup();

    let context = Context::create();
    let config = create_test_lto_config();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();

    // Create module with function containing various instructions
    let source_module = context.create_module("instruction_test");
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = source_module.add_function("complex_func", fn_type, None);

    // Create function with arithmetic operations
    let entry = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry);

    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();

    // Various arithmetic operations
    let sum = builder.build_int_add(param1, param2, "sum").unwrap();
    let diff = builder.build_int_sub(sum, param1, "diff").unwrap();
    let product = builder.build_int_mul(diff, param2, "product").unwrap();

    builder.build_return(Some(&product)).unwrap();

    // Clone the module
    let cloned_module = integration.clone_module_for_lto(&source_module).unwrap();

    // Verify function was cloned properly
    let cloned_function = cloned_module.get_function("complex_func").unwrap();
    assert_eq!(cloned_function.count_basic_blocks(), 1);
    assert_eq!(cloned_function.count_params(), 2);

    tracing::info!("Comprehensive instruction cloning test completed successfully");
}

/// Test metadata and debug information preservation
#[test]
fn test_metadata_preservation() {
    common::tracing::setup();

    let context = Context::create();
    let config = create_test_lto_config();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();

    // Create source module with metadata
    let source_module = context.create_module("metadata_test");
    source_module.set_source_file_name("test_source.cursed");

    // Add some named metadata
    let version_metadata = source_module.add_named_metadata("llvm.ident");
    let metadata_string = context.metadata_string("CURSED Compiler 1.0");
    let metadata_node = context.metadata_node(&[metadata_string.into()]);
    version_metadata.add_operand(&metadata_node);

    // Clone the module
    let cloned_module = integration.clone_module_for_lto(&source_module).unwrap();

    // Verify source filename was copied
    assert!(cloned_module.get_source_file_name().to_str().is_ok());

    // Verify named metadata was copied
    let cloned_metadata = cloned_module.get_named_metadata();
    assert!(cloned_metadata.any(|nm| nm.get_name().to_str().unwrap_or("") == "llvm.ident"));

    tracing::info!("Metadata preservation test completed successfully");
}

/// Test performance of module cloning with large modules
#[test]
fn test_cloning_performance() {
    common::tracing::setup();

    let context = Context::create();
    let config = create_test_lto_config();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();

    // Create large module for performance testing
    let source_module = context.create_module("performance_test");
    let i32_type = context.i32_type();

    // Add many functions
    for i in 0..100 {
        let fn_type = i32_type.fn_type(&[], false);
        let function = source_module.add_function(&format!("func_{}", i), fn_type, None);
        
        let entry = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(entry);
        let const_val = i32_type.const_int(i as u64, false);
        builder.build_return(Some(&const_val)).unwrap();
    }

    // Add many globals
    for i in 0..50 {
        let global = source_module.add_global(i32_type, None, &format!("global_{}", i));
        global.set_initializer(&i32_type.const_int(i as u64, false));
        global.set_constant(true);
    }

    // Time the cloning operation
    let start_time = std::time::Instant::now();
    let cloned_module = integration.clone_module_for_lto(&source_module).unwrap();
    let clone_duration = start_time.elapsed();

    // Verify all content was cloned
    assert_eq!(cloned_module.get_functions().count(), 100);
    assert_eq!(cloned_module.get_globals().count(), 50);

    // Performance should be reasonable (less than 5 seconds for this size)
    assert!(clone_duration.as_secs() < 5, "Cloning took too long: {:?}", clone_duration);

    tracing::info!("Cloning performance test completed in {:?}", clone_duration);
}

/// Test error handling in cloning edge cases
#[test]
fn test_cloning_error_handling() {
    common::tracing::setup();

    let context = Context::create();
    let config = create_test_lto_config();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();

    // Test with module containing functions without names (should handle gracefully)
    let source_module = context.create_module("error_test");
    let i32_type = context.i32_type();

    // Add a properly named function (should work)
    let fn_type = i32_type.fn_type(&[], false);
    let _normal_function = source_module.add_function("normal_func", fn_type, None);

    // Clone should succeed even with edge cases
    let cloned_result = integration.clone_module_for_lto(&source_module);
    assert!(cloned_result.is_ok());

    let cloned_module = cloned_result.unwrap();
    assert!(cloned_module.get_function("normal_func").is_some());

    tracing::info!("Cloning error handling test completed successfully");
}

/// Helper function to create test LTO configuration
fn create_test_lto_config() -> LtoConfig {
    LtoConfig {
        level: LtoLevel::Thin,
        inline_threshold: 250,
        import_inline_threshold: 100,
        enable_split_lto_unit: true,
        whole_program_vtables: false,
        enable_new_pass_manager: true,
        debug_pass_manager: false,
        verify_each: false,
        disable_verify: false,
        enable_machine_function_splitter: false,
        passes: Vec::new(),
    }
}
