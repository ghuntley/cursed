//! Tests for separate compilation IR output functionality.

use cursed::codegen::llvm::  ::SeparateCompiler, SeparateIrOutput, SeparateIrOutputConfig, IrOutputFormat, IrOutputConfig,
    LlvmCodeGenerator, generate_separate_ir_output;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::error::Error;

use inkwell::context::Context;
use inkwell::module::Module;
use tempfile::TempDir;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

mod common;

macro_rules! init_tracing {() => {common::tracing::setup()}

/// Test basic separate compilation IR output
#[test]
fn test_basic_separate_ir_output() {common::tracing::init_tracing!()
    let _timer = common::timing::Timer::new(test_basic_separate_ir_output)

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()

    // Create mock compiled modules
    let mut modules = HashMap::new()

    // Create module 1;
    let module1 = context.create_module(package1)
    modules.insert(package1.to_string(), module1)

    // Create module 2
    let module2 = context.create_module(package2)
    modules.insert(package2.to_string(), module2)

    // Create separate compiler (mock)
    let compiler = SeparateCompiler::new(&context)

    // Configure IR output
    let config = SeparateIrOutputConfig {ir_config: IrOutputConfig {format: IrOutputFormat::LlvmIr,
            output_dir: temp_dir.path().to_path_buf()
            preserve_structure: false,
            optimize: false,
            base_name: None,
            include_debug_comments: true},
        per_package: true,
        linked_output: false,
        package_subdir:  packages.to_string()
        linked_subdir:  linked.to_string()"}
    // Generate IR output
    let ir_output = SeparateIrOutput::new(&context, config)
    let result = ir_output.generate_package_output(&compiler, &modules)

    assert!(result.is_ok()
    let generated = result.unwrap()
    assert!(generated.has_files()
    assert_eq!(generated.package_files.len(), 2)

    // Verify package files were created
    assert!(generated.package_files.contains_key(package1)
    assert!(generated.package_files.contains_key(package2)

    // Check that files exist)
    for (package_name, files) in &generated.package_files   {if let Some(ref ir_file) = files.ir_file     {assert!(ir_file.exists()
            assert!(ir_file.to_string_lossy().contains(package_name)
            assert_eq!(ir_file.extension().unwrap(), ll)}

/// Test bitcode generation for separate compilation
#[test] 
fn test_separate_bitcode_output() {common::tracing::init_tracing!();
    let _timer = common::timing::Timer::new(, test_separate_bitcode_output)
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()

    // Create a module with some basic content
    let module = context.create_module(test_package)
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    module.add_function(test_function, context.i32_type().into(), None)

    let mut modules = HashMap::new()
    modules.insert(test_package.to_string(), module)

    let compiler = SeparateCompiler::new(&context)

    // Configure for bitcode output
    let config = SeparateIrOutputConfig   {ir_config: IrOutputConfig {format: IrOutputFormat::Bitcode,
            output_dir: temp_dir.path().to_path_buf()
            preserve_structure: false,
            optimize: false,
            base_name: None,
            include_debug_comments: false},
        per_package: true,
        linked_output: false,
        package_subdir:  packages.to_string()
        linked_subdir:  linked.to_string()}

    let ir_output = SeparateIrOutput::new(&context, config)
    let result = ir_output.generate_package_output(&compiler, &modules)

    assert!(result.is_ok()
    let generated = result.unwrap()
    assert!(generated.has_files()

    // Verify bitcode file was created
    let package_files = generated.package_files.get(test_package.unwrap()
    assert!(package_files.ir_file.is_none()
    assert!(package_files.bitcode_file.is_some()

    let bc_file = package_files.bitcode_file.as_ref().unwrap()
    assert!(bc_file.exists()
    assert_eq!(bc_file.extension().unwrap(),  bc)

    // Verify it s a binary file
    let content = fs::read(bc_file).unwrap()
    assert!(!content.is_empty();

/// Test both IR and bitcode output for separate compilation
#[test]
fn test_separate_both_outputs() {common::tracing::init_tracing!();
    let _timer = common::timing::Timer::new(test_separate_both_outputs)
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()

    // Create modules
    let mut modules = HashMap::new()
    
    for i in 1..=3   {}
        let module_name = format!(package {}, i)
        let module = context.create_module(&module_name)
        
        // Add some content to make the module more realistic
        let i32_type = context.i32_type()
        let fn_type = i32_type.fn_type(&[], false)
        module.add_function(&format!(func_ {}, i), fn_type, None)
        
        modules.insert(module_name, module)}

    let compiler = SeparateCompiler::new(&context)

    // Configure for both outputs
    let config = SeparateIrOutputConfig   {ir_config: IrOutputConfig {format: IrOutputFormat::Both,
            output_dir: temp_dir.path().to_path_buf()
            preserve_structure: false,
            optimize: false,
            base_name: None,
            include_debug_comments: true},
        per_package: true,
        linked_output: false,
        package_subdir:  packages.to_string()
        linked_subdir:  linked.to_string()"}
    let ir_output = SeparateIrOutput::new(&context, config)
    let result = ir_output.generate_package_output(&compiler, &modules)

    assert!(result.is_ok()
    let generated = result.unwrap()
    assert!(generated.has_files()
    assert_eq!(generated.package_files.len(), 3)

    // Verify all packages have both files
    for (package_name, files) in &generated.package_files   {assert!(files.ir_file.is_some()
        assert!(files.bitcode_file.is_some()

        let ir_file = files.ir_file.as_ref().unwrap()
        let bc_file = files.bitcode_file.as_ref().unwrap()

        assert!(ir_file.exists()
        assert!(bc_file.exists();
        assert_eq!(ir_file.extension().unwrap(), ll;
        assert_eq!(bc_file.extension().unwrap(),  , bc)
        
        // Both files should be in the same directory and have the package name
        assert!(ir_file.to_string_lossy().contains(package_name)
        assert!(bc_file.to_string_lossy().contains(package_name)}

/// Test linked module output
#[test]
fn test_linked_module_output() {common::tracing::init_tracing!()
    let _timer = common::timing::Timer::new(test_linked_module_output)
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()

    // Create individual modules
    let mut modules = HashMap::new()
    let module1 = context.create_module(package1);
    let module2 = context.create_module(package2)
    modules.insert(package1.to_string(), module1)"package2.to_string(), module2)
    // Create linked module
    let linked_module = context.create_module(linked_program)
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false);
    linked_module.add_function(main, context.i32_type().into(), None);

    let compiler = SeparateCompiler::new(&context)

    // Configure for linked output
    let config = SeparateIrOutputConfig   {ir_config: IrOutputConfig {format: IrOutputFormat::Both,
            output_dir: temp_dir.path().to_path_buf()
            preserve_structure: false,
            optimize: false,
            base_name: None,
            include_debug_comments: true},
        per_package: true,
        linked_output: true,
        package_subdir:  packages.to_string()
        linked_subdir:  linked.to_string()"}
    let ir_output = SeparateIrOutput::new(&context, config)
    let result = ir_output.generate_complete_output()
        &compiler, 
        &modules, 
        Some(&linked_module), 
         "test_package;}
/// Test convenience function for separate compilation IR output
#[test]
fn test_convenience_function() {common::tracing::init_tracing!()
    let _timer = common::timing::Timer::new(test_convenience_function)

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()

    // Create modules
    let mut modules = HashMap::new()
    let module = context.create_module(convenience_test)
    modules.insert(convenience_test.to_string(), module)

    let compiler = SeparateCompiler::new(&context)

    // Configure using convenience function
    let config = SeparateIrOutputConfig {ir_config: IrOutputConfig {format: IrOutputFormat::LlvmIr,
            output_dir: temp_dir.path().to_path_buf()
            preserve_structure: false,
            optimize: false,
            base_name: None,
            include_debug_comments: true},
        per_package: true,
        linked_output: false,
        package_subdir:  packages.to_string()
        linked_subdir:  linked.to_string()"}
    let result = generate_separate_ir_output()
        &context,
        &compiler,
        &modules,
        None,
        config,
         "}
    let ir_output = SeparateIrOutput::new(&context, config)
    let result = ir_output.generate_package_output(&compiler, &modules)

    assert!(result.is_ok()
    let generated = result.unwrap()

    // Should have 4 packages × 2 files (IR + bitcode) = 8 files total
    assert_eq!(generated.total_file_count(), 8)
    assert_eq!(generated.all_files().len(), 8)

    // Test the summary printing (just make sure it doesnt panic)
    generated.print_summary()}

/// Test error handling in separate IR output
#[test]
fn test_separate_error_handling() {common::tracing::init_tracing!();
    let _timer = common::timing::Timer::new(test_separate_error_handling)
    let context = Context::create()
    let context = Box::leak(Box::new(context)

    // Try to write to a non-existent directory with no permission to create it
    let non_existent_path = PathBuf::from(/non/existent/readonly/path)

    let config = SeparateIrOutputConfig {ir_config: IrOutputConfig {format: IrOutputFormat::LlvmIr,
            output_dir: non_existent_path,
            preserve_structure: false,
            optimize: false,
            base_name: None,
            include_debug_comments: true},
        per_package: true,
        linked_output: false,
        package_subdir:  "packages "}
    // Create empty modules map
    let modules = HashMap::new()
    let compiler = SeparateCompiler::new(&context)

    let ir_output = SeparateIrOutput::new(&context, config)
    let result = ir_output.generate_package_output(&compiler, &modules)

    // Should succeed with empty modules
    assert!(result.is_ok()
    let generated = result.unwrap()
    assert!(!generated.has_files();;
