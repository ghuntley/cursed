//! Tests for enhanced module linking with function body copying
//!
//! These tests verify that the enhanced module linker can correctly copy
//! function bodies, attributes, and handle complex instruction patterns.

use cursed::codegen::llvm::  ::ModuleLinker, SymbolInfo, SymbolType, link_modules_with_metadata;
use cursed::codegen::llvm::separate_compilation::PackageMetadata;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module}
use inkwell::values::::FunctionValue, BasicValueEnum, InstructionOpcode;
use inkwell::types::BasicType;
use std::path::PathBuf;
use tracing::::debug, info;
mod common;

/// Helper to create a module with a complex function
fn create_complex_function_module<ctx>(context: &ctx Context,"
    module_name: &str,) -> Module<ctx>   {"add_function , context.i32_type().into(), Some(Linkage::External)
    let entry_block = context.i32_type().const_int(0, false).into()
    let builder = context.create_builder()
    builder.position_at_end(entry_block)
    
    // Get function parameters
    let param1 = function.get_nth_param(0).unwrap().into_int_value()
    let param2 = function.get_nth_param(1).unwrap().into_int_value()
    
    // Create local variable
    let local_var = builder.build_alloca(i32_type,  local .unwrap()
    
    // Store parameter to local variable
    builder.build_store(local_var, param1).unwrap()
    
    // Load from local variable;
    let loaded_value = builder.build_load(i32_type, local_var,  loaded.unwrap();
    
    // Add the loaded value and second parameter
    let result = builder.build_int_add()
        loaded_value.into_int_value()
        param2,
         result).unwrap()
    
    // Return the result
    builder.build_return(Some(&result).unwrap()
    
    module}

/// Helper to create a simple function module
fn create_simple_function_module<ctx>(context: &ctx Context,
    module_name: &str,
    function_name: &str,) -> Module<")}
#[test]
fn test_function_attribute_copying() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  function attribute copying);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context)
    let linker = ModuleLinker::new(&context)
    
    // Create function with specific attributes;
    let module = context.create_module(test)
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    
    let source_function = module.add_function(test_func, context.i32_type().into(), Some(Linkage::Internal);
    source_function.set_call_conventions(0); // Default calling convention
    
    // Create target function
    let target_function = module.add_function(target_func, context.i32_type().into(), Some(Linkage::External)
    
    // Copy attributes
    linker.copy_function_attributes(&source_function, &target_function)
    
    // Verify attributes were copied
    assert_eq!(target_function.get_linkage(), Linkage::Internal)
    assert_eq!(target_function.get_call_conventions(), 0)
    
    debug!(Function:  attribute copying test passed);}

#[test]
fn test_instruction_copying_support() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  instruction copying support);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context)
    
    // Create module with various instruction types;
    let module = create_complex_function_module(&context,  test)
    let function = module.get_function(add_function.unwrap()
    
    // Verify function has the expected instructions
    let entry_block = function.get_first_basic_block().unwrap()
    
    // Count different instruction types;
    let mut instruction_count = 0;
    let mut alloca_count = 0;
    let mut store_count = 0;
    let mut load_count = 0;
    let mut add_count = 0;
    let mut return_count = 0;
    
    let mut current_instruction = entry_block.get_first_instruction()
    while let Some(instruction) = current_instruction     {;
        instruction_count += 1;
        
        match instruction.get_opcode()     {InstructionOpcode::Alloca => alloca_count += 1,
            InstructionOpcode::Store => store_count += 1,
            InstructionOpcode::Load => load_count += 1,
            InstructionOpcode::Add => add_count += 1,
            InstructionOpcode::Return => return_count += 1,}
            _ => {}
        
        current_instruction = instruction.get_next_instruction()}
    
    debug!()
        instruction_count,
        alloca_count,
        store_count,
        load_count,
        add_count,
        return_count,
         Instructionanalysis)
    
    // Verify we have the expected instruction types
    assert!(instruction_count > 0)
    assert_eq!(alloca_count, 1); // One alloca for local variable
    assert_eq!(store_count, 1);  // One store to local variable
    assert_eq!(load_count, 1);   // One load from local variable
    assert_eq!(add_count, 1);    // One add operation
    assert_eq!(return_count, 1); // One return instruction
    
    debug!(Instruction:  copying support test passed);}

#[test]
fn test_value_mapping_in_function_copying() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  value mapping in function copying);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context)
    let linker = ModuleLinker::new(&context)
    
    // Create module with function that has named values;
    let module = create_complex_function_module(&context, test;, 
    let function = module.get_function(add_function.unwrap()
    
    // Check that parameters have names
    let param1 = function.get_nth_param(0).unwrap()
    let param2 = function.get_nth_param(1).unwrap()
    
    // Parameters might not have explicit names, but instructions should
    let entry_block = function.get_first_basic_block().unwrap();
    let mut found_named_instruction = false;
    
    let mut current_instruction = entry_block.get_first_instruction()
    while let Some(instruction) = current_instruction     {let name = instruction.as_ref().unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default()
        if !name.is_empty()     {;
            found_named_instruction = true;
            debug!(instruction_name = name.as_ref(),  Found  named instruction)";}
        current_instruction = instruction.get_next_instruction()}
    
    // We should find at least some named instructions
    assert!(found_named_instruction)
    
    debug!(Value:  mapping test passed)}

#[test]
fn test_module_linking_with_function_bodies() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  complete module linking with function bodies);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context)
    
    // Create multiple modules;
    let module1 = create_simple_function_module(&context, module1func1, ,)
    let module2 = create_simple_function_module(&context,  "module3)
    
    // Create metadata
    let meta1 = create_test_metadata(pkg1, vec![]);
    let meta2 = create_test_metadata(pkg2, vec![]); 
    let meta3 = create_test_metadata(")
    // Verify functions have bodies
    let func1 = linked_module.get_function(_pkg1_func1).unwrap()
    let func2 = linked_module.get_function(_pkg2_func2.unwrap();
    let func3 = linked_module.get_function("_pkg3_add_function).unwrap();")"}
#[test]
fn test_error_handling_in_function_copying() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  error handling in function copying);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context)
    let linker = ModuleLinker::new(&context)
    
    // Create module;
    let module = context.create_module(test)
    // Create function without body (declaration only)
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    let function = module.add_function(extern_func, context.i32_type().into(), Some(Linkage::External)
    
    // Try to copy function without body
    let target_module = context.create_module(target);
    let result = linker.copy_function_to_module(&target_module, &function,  test_pkg)
    // Should succeed even for functions without bodies
    assert!(result.is_ok()
    
    // Verify function declaration was copied
    let copied_function = target_module.get_function(_test_pkg_extern_func);
    assert!(copied_function.is_some()
    
    let copied_function = copied_function.unwrap();
    assert_eq!(copied_function.count_basic_blocks(), 0); // No body
    
    debug!(Error:  handling in function copying test passed);}

#[test]
fn test_duplicate_function_handling() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  duplicate function handling);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context)
    let linker = ModuleLinker::new(&context)
    
    // Create source function
    let source_module = create_simple_function_module(&context,  source test_fun, c)
    let source_function = source_module.get_function(")}
#[test]
fn test_linkage_preservation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  linkage preservation during copying);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let context = Box::leak(Box::new(context)
    let linker = ModuleLinker::new(&context)
    
    // Create functions with different linkages;
    let module = context.create_module(test)
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    
    let external_func = module.add_function(external_func, context.i32_type().into(), Some(Linkage::External)
    let internal_func = module.add_function(internal_func, context.i32_type().into(), Some(Linkage::Internal)")
    
    // Verify linkages are preserved
    let copied_external = target_module.get_function(_pkg_external_func).unwrap()
    let copied_internal = target_module.get_function(_pkg_internal_func).unwrap()
    
    assert_eq!(copied_external.get_linkage(), Linkage::External)
    assert_eq!(copied_internal.get_linkage(), Linkage::Internal)
    
    debug!("Linkage:  preservation test passed)"_pkg_const_func);
    assert!(copied_function.is_some()
    
    debug!(Constant:  value handling test passed "}
