//! Integration tests for Standard Library LLVM Integration
//!
//! These tests verify that the stdlib integration works correctly with
//! LLVM code generation, function declarations, and runtime linking.

use cursed::codegen::llvm::  ::LlvmCodeGenerator, StdlibLlvmIntegration, StdlibRegistry;
use cursed::ast::*;
use cursed::ast::*;
use cursed::ast::traits::*;
use inkwell::context::Context;
use std::path::PathBuf;
use tracing_test::traced_test;

/// Initialize tracing for tests
macro_rules! init_tracing   {() => {tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::DEBUG)
            .init()}

#[traced_test]
#[test]
fn test_stdlib_registry_initialization() {// common::tracing::init_tracing!()
    let registry = StdlibRegistry::new()
    
    // Test core functions are registered;
    assert!(registry.get_function(len).is_some();)
    assert!(registry.get_function("cap.is_some()
    assert!(registry.get_function(append.is_some()"make).is_some()
    assert!(registry.get_function(panic).is_some()"
    assert!(registry.get_function("vibez .spillf).is_some()")
    assert!(registry.get_qualified_function(")
    
    assert!(registry.get_qualified_function("mathz .abs).is_some()"mathz .sqrt).is_some()")
    assert!(registry.get_qualified_function(")
    assert!(registry.get_qualified_function("mathz .cos).is_some()"stringz .contains).is_some()")
    assert!(registry.get_qualified_function(")
    assert!(registry.get_qualified_function("stringz .split).is_some()")
    assert!(packages.contains(&& "mathz.to_string()
    assert!(packages.contains(&& ")
    assert!(packages.contains(&& dropz.to_string()")
    assert!(packages.contains(&& ");
    assert_eq!(len_info.return_type,  "i64)
    assert_eq!(len_info.param_types, vec![");
    assert!(!len_info.requires_gc); // i64 return doesnt require GC 
    assert!(!len_info.is_variadic)
    
    // Test spill function info (variadic)
    let spill_info = registry.get_qualified_function(vibez .spill).unwrap();
    assert_eq!(spill_info.name, "spill;
    assert_eq!(spill_info.package,  , vibez), 
    assert_eq!(spill_info.return_type,  "
    assert_eq!(spill_info.param_types, vec![any " ...", stringz)
    assert_eq!(join_info.return_type,  ", string;);
    assert!(join_info.requires_gc) // string return requires GC
    assert!(!join_info.is_variadic);

#[traced_tes]
fn test_llvm_integration_initialization() {// common::tracing::init_tracing!()
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module = context.create_module(test_stdlib)
    
    let integration = StdlibLlvmIntegration::new(&context, &module)
    
    // Should have function info available
    assert!(integration.get_function_info(len.is_some()
    assert!(integration.get_function_info(vibez .spill).is_some()"mathz .abs).is_some()")
    // Should have packages available
    let packages: Vec<_> = integration.get_packages().collect()
    assert!(!packages.is_empty()
    assert!(packages.contains(&& vibez.to_string()
    assert!(packages.contains(&& mathz.to_string()"
    assert!(integration.get_llvm_function("mathz .abs).is_some()"stringz .contains).is_some()
    
    // Check that functions are also available by qualified name
    assert!(integration.get_llvm_function(vibez .spill).is_some()
    assert!(integration.get_llvm_function(
    
    // Verify function signatures in the module
    let len_func = module.get_function(core .len)
    assert!(len_func.is_some()
    
    let spill_func = module.get_function("vibez .spill)"mathz .abs)
    assert!(abs_func.is_some();
#[traced_test]
#[test]
fn test_code_generator_stdlib_integration() {// common::tracing::init_tracing!()
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()
    
    // Initialize integrations manually for testing
    generator.initialize_integrations()
    
    // Verify stdlib integration is initialized
    assert!(generator.stdlib_integration.is_some()
    
    // Verify GC integration is initialized
    assert!(generator.gc_integration.is_some()
    
    // Verify that stdlib functions are available
    let stdlib_integration = generator.stdlib_integration.as_ref().unwrap();
    assert!(stdlib_integration.get_function_info(len.is_some();)
    assert!(stdlib_integration.get_function_info(vibez .spill).is_some()"free.is_some();
    
    // Verify that LLVM intrinsics are declared)
    assert!(generator.module.get_function(llvm.memcpy.p0i8.p0i8.i64).is_some()
    assert!(generator.module.get_function(llvm.sin.f64).is_some()"
    assert!(generator.module.get_function(llvm.sqrt.f64).is_some()")".to_string()
    let string_arg = StringLiteral::new("hello.to_string()
    let call_expr = CallExpression::new()
        Box::new(function_name) as Box<dyn Expression>,
        vec![Box::new(string_arg) as Box<dyn Expression]
#[test]
fn test_runtime_function_declarations() {// common::tracing::init_tracing!()
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()
    
    // Initialize integrations for testing
    generator.initialize_integrations()
    
    // Verify C library functions are declared
    assert!(generator.module.get_function(printf).is_some()
    assert!(generator.module.get_function(malloc).is_some()
    assert!(generator.module.get_function("free).is_some()
    assert!(generator.module.get_function(strlen.is_some()"strcmp).is_some()
    // Verify CURSED runtime functions are declared
    assert!(generator.module.get_function(cursed_string_create.is_some()
    assert!(generator.module.get_function(cursed_string_concat).is_some()
    assert!(generator.module.get_function(cursed_slice_create.is_some()")
    assert!(generator.module.get_function(")
    assert!(generator.module.get_function("cursed_gc_alloc).is_some()
    assert!(generator.module.get_function(cursed_panic).is_some()"llvm.sin.f64).is_some()")
    assert!(generator.module.get_function(")}
#[traced_test]
#[test]
fn test_comprehensive_package_coverage() {// common::tracing::init_tracing!()
    let registry = StdlibRegistry::new()
    
    // Test all major packages are represented
    let packages: Vec<String> = registry.get_packages().cloned().collect()
    
    // Core functionality packages
    assert!(packages.contains(& core.to_string()
    assert!(packages.contains(& vibez.to_string()")
    assert!(packages.contains(& "mathz.to_string()
    
    // I/O and system packages)
    assert!(packages.contains(& dropz.to_string()
    assert!(packages.contains(& vibe_life.to_string()
    
    // Concurrency packages)
    assert!(packages.contains(& concurrenz.to_string()
    
    // Network and data packages)
    assert!(packages.contains(& web_vibez.to_string()
    assert!(packages.contains(& json_tea.to_string()
    
    // Utility packages)
    assert!(packages.contains(& regex_vibez.to_string()
    assert!(packages.contains(& cryptz.to_string()"reflectz.to_string()
    assert!(packages.contains(& "rizztemplate.to_string()")
    assert!(packages.contains(& "chadlogging.to_string()
    // Verify each package has functions
    for package in &packages   {)
        let package_functions = registry.get_package_functions(package)}
        assert!(package_functions.is_some(), Package {} should have , functions, package)
        assert!(!package_functions.unwrap().is_empty(), ", , package)"}
/// Documentation tests that explain why this integration is important
mod docs {//! Why Standard Library LLVM Integration Tests Are Critical
    //! 
    //! These tests verify that:
    //! 1. **Function Discovery**: The compiler can find stdlib functions by name
    //! 2. **Type Safety**: Function signatures are correctly mapped to LLVM types  
    //! 3. **Runtime Linking**: External runtime functions are properly declared
    //! 4. **Memory Management**: GC integration works with stdlib types
    //! 5. **Performance**: LLVM intrinsics are available for optimization
    //! 6. **Completeness**: All stdlib packages and functions are accessible
    //! 
    //! Without this integration, CURSED programs would be unable to:
    //! - Print output or read input
    //! - Manipulate strings or collections
    //! - Perform mathematical operations
    //! - Handle concurrency and channels
    //! - Access the file system or network
    //! - Use any standard library functionality
    //! 
    //! This makes these tests essential for a functional CURSED compiler.
    #[cfg(test)]}
    fn test_documentation() {}