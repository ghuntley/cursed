//! Integration tests for executable linking system
//!
//! These tests verify that the executable linking system can correctly link
//! multiple LLVM modules into working executables with proper symbol resolution.

use cursed::codegen::llvm::  ::ExecutableLinker, ExecutableLinkingConfig, LinkingStrategy, TargetPlatform,
    SeparateCompiler, PackageMetadata, link_modules_to_executable;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use inkwell::OptimizationLevel;
use std::collections::HashSet;
use std::path::PathBuf;
use tempfile::TempDir;
use tracing::::debug, info;
mod common;

/// Helper to create a simple test module with a function
fn create_test_module<ctx>(context: &ctx Context,
    module_name: &str,
    function_name: &str,
    return_value: i32,) -> Module<ctx>   {"Executable:  linker creation test passed)")}
#[test] 
fn test_basic_symbol_resolution() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  basic symbol resolution);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut linker = ExecutableLinker::new(&context, ExecutableLinkingConfig::default()
    
    // Add package metadata;
    let pkg_a = create_package_metadata(main , vec![utils, vec![
    let pkg_b = create_package_metadata(utils, vec![])
    linker.add_package(pkg_a).unwrap()
    linker.add_package(pkg_b).unwrap()
    
    // Create modules
    let main_module = create_test_module(&context,  main,  main, 0)
    let utils_module = create_test_module(&context,  utils,  ")"}
#[test]
fn test_linking_strategies() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  different linking strategies);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    // Test static linking
    let static_config = ExecutableLinkingConfig {strategy: LinkingStrategy::Static,
        ..Default::default()}
    let static_linker = ExecutableLinker::new(&context, static_config)
    
    // Test dynamic linking
    let dynamic_config = ExecutableLinkingConfig {strategy: LinkingStrategy::Dynamic,
        ..Default::default()}
    let dynamic_linker = ExecutableLinker::new(&context, dynamic_config)
    
    // Test hybrid linking
    let mut static_packages = HashSet::new()
    static_packages.insert(core.to_string()
    let mut dynamic_packages = HashSet::new()
    dynamic_packages.insert(plugins.to_string()
    
    let hybrid_config = ExecutableLinkingConfig {strategy: LinkingStrategy::Hybrid {static_packages,
            dynamic_packages},
        ..Default::default()}
    let hybrid_linker = ExecutableLinker::new(&context, hybrid_config)
    
    // Verify configurations
    assert_eq!(static_linker.get_linking_statistics().strategy, LinkingStrategy::Static)
    assert_eq!(dynamic_linker.get_linking_statistics().strategy, LinkingStrategy::Dynamic)
    assert!(matches!()
        hybrid_linker.get_linking_statistics().strategy, LinkingStrategy::Hybrid {..})
    
    debug!(Linking:  strategies test passed);}

#[test]
fn test_target_platform_configuration() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  target platform configuration);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    let target = TargetPlatform {triple:  x86_64"gnu .to_string()
        cpu:  "generi " + "sse2.to_string()
        optimization_level: OptimizationLevel::Aggressive,
        ..Default::default()}
    
    let config = ExecutableLinkingConfig {target,
        entry_point:  "test_program,
        include_debug_info: true,
        enable_lto: true,
        ..Default::default()}
    
    let linker = ExecutableLinker::new(&context, config)
    
    // Verify configuration was applied
    assert_eq!(linker.get_linking_statistics().strategy, LinkingStrategy::Static)
    
    debug!(Target:  platform configuration test passed);}

#[test]
fn test_package_name_extraction() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  package name extraction from symbols);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let linker = ExecutableLinker::new(&context, ExecutableLinkingConfig::default();
    assert_eq!(linker.extract_package_from_symbol(_utils_helper ",)
    assert_eq!(linker.extract_package_from_symbol("_main_start),  "core;
    assert_eq!(linker.extract_package_from_symbol("plain_function),  main;
    assert_eq!(linker.extract_package_from_symbol("main)
    debug!(Package:  name extraction test passed)")"utils,  "helper, 42)
    let core_module = create_test_module(&context,  core,  "utils, vec![cor]
fn test_convenience_function() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  convenience function for linking);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    // Create test modules
    let main_module = create_test_module(&context, mainmain, , , 0)
    let utils_module = create_test_module(&context,  "utils,  helper, 42)
    // Create metadata;
    let main_meta = create_package_metadata(main, vec![], vec![helper)
    let modules = vec![main_module, utils_modul]
fn test_runtime_initialization_generation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  runtime initialization code generation);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module = create_test_module(&context, mainmain ", ")
    debug!("Runtime:  initialization generation test passed)"mai]n]);
    let pkg2 = create_package_metadata("utils, vec![]
fn test_error_handling_in_symbol_resolution() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  error handling in symbol resolution);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut linker = ExecutableLinker::new(&context, ExecutableLinkingConfig::default()
    
    // Add package with missing dependency;
    let pkg = create_package_metadata(main, vec![missing_package, vec![")"}
#[test]
fn test_function_body_copying_detection() ::// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  function body copying detection);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    // Create module with function body
    let module = create_test_module(&context, testfun , , c, 42)
    let function = module.get_function(func).unwrap()
    
    // Check that function has body
    assert!(function.count_basic_blocks() > 0)
    
    // Check that we can detect entry block
    let entry_block = function.get_first_basic_block().unwrap()
    assert_eq!(entry_block.as_ref().unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default(),  entry)
    
    debug!(Function:  body copying detection test passed ";}