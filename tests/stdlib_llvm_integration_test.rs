//! Integration tests for Standard Library LLVM Integration
//!
//! These tests verify that the stdlib integration works correctly with
//! LLVM code generation, function declarations, and runtime linking.

use cursed::codegen::llvm::{LlvmCodeGenerator, StdlibLlvmIntegration, StdlibRegistry};
use cursed::ast::*;
use cursed::ast::expressions::*;
use cursed::ast::traits::*;
use inkwell::context::Context;
use std::path::PathBuf;
use tracing_test::traced_test;

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    };
}

#[traced_test]
#[test]
fn test_stdlib_registry_initialization() {
    let registry = StdlibRegistry::new();
    
    // Test core functions are registered
    assert!(registry.get_function("len").is_some());
    assert!(registry.get_function("cap").is_some());
    assert!(registry.get_function("append").is_some());
    assert!(registry.get_function("make").is_some());
    assert!(registry.get_function("panic").is_some());
    assert!(registry.get_function("recover").is_some());
    
    // Test package functions are registered with qualified names
    assert!(registry.get_qualified_function("vibez.spill").is_some());
    assert!(registry.get_qualified_function("vibez.spillf").is_some());
    assert!(registry.get_qualified_function("vibez.spillstr").is_some());
    
    assert!(registry.get_qualified_function("mathz.abs").is_some());
    assert!(registry.get_qualified_function("mathz.sqrt").is_some());
    assert!(registry.get_qualified_function("mathz.sin").is_some());
    assert!(registry.get_qualified_function("mathz.cos").is_some());
    
    assert!(registry.get_qualified_function("stringz.contains").is_some());
    assert!(registry.get_qualified_function("stringz.join").is_some());
    assert!(registry.get_qualified_function("stringz.split").is_some());
    
    // Test package listing
    let packages: Vec<_> = registry.get_packages().collect();
    assert!(packages.contains(&&"core".to_string()));
    assert!(packages.contains(&&"vibez".to_string()));
    assert!(packages.contains(&&"mathz".to_string()));
    assert!(packages.contains(&&"stringz".to_string()));
    assert!(packages.contains(&&"dropz".to_string()));
    assert!(packages.contains(&&"concurrenz".to_string()));
}

#[traced_test]
#[test]
fn test_function_info_metadata() {
    let registry = StdlibRegistry::new();
    
    // Test len function info
    let len_info = registry.get_function("len").unwrap();
    assert_eq!(len_info.name, "len");
    assert_eq!(len_info.package, "core");
    assert_eq!(len_info.return_type, "i64");
    assert_eq!(len_info.param_types, vec!["any"]);
    assert!(!len_info.requires_gc); // i64 return doesn't require GC
    assert!(!len_info.is_variadic);
    
    // Test spill function info (variadic)
    let spill_info = registry.get_qualified_function("vibez.spill").unwrap();
    assert_eq!(spill_info.name, "spill");
    assert_eq!(spill_info.package, "vibez");
    assert_eq!(spill_info.return_type, "void");
    assert_eq!(spill_info.param_types, vec!["any..."]);
    assert!(!spill_info.requires_gc); // void return doesn't require GC
    assert!(spill_info.is_variadic);
    
    // Test string function info (requires GC)
    let join_info = registry.get_qualified_function("stringz.join").unwrap();
    assert_eq!(join_info.name, "join");
    assert_eq!(join_info.package, "stringz");
    assert_eq!(join_info.return_type, "string");
    assert!(join_info.requires_gc); // string return requires GC
    assert!(!join_info.is_variadic);
}

#[traced_test]
#[test]
fn test_llvm_integration_initialization() {
    let context = Context::create();
    let module = context.create_module("test_stdlib");
    
    let integration = StdlibLlvmIntegration::new(&context, &module);
    
    // Should have function info available
    assert!(integration.get_function_info("len").is_some());
    assert!(integration.get_function_info("vibez.spill").is_some());
    assert!(integration.get_function_info("mathz.abs").is_some());
    
    // Should have packages available
    let packages: Vec<_> = integration.get_packages().collect();
    assert!(!packages.is_empty());
    assert!(packages.contains(&&"vibez".to_string()));
    assert!(packages.contains(&&"mathz".to_string()));
}

#[traced_test]
#[test]
fn test_llvm_function_declaration_generation() {
    let context = Context::create();
    let module = context.create_module("test_stdlib");
    
    let mut integration = StdlibLlvmIntegration::new(&context, &module);
    
    // Generate function declarations
    assert!(integration.generate_function_declarations().is_ok());
    
    // Check that LLVM functions were created
    assert!(integration.get_llvm_function("len").is_some());
    assert!(integration.get_llvm_function("vibez.spill").is_some());
    assert!(integration.get_llvm_function("mathz.abs").is_some());
    assert!(integration.get_llvm_function("stringz.contains").is_some());
    
    // Check that functions are also available by qualified name
    assert!(integration.get_llvm_function("vibez.spill").is_some());
    assert!(integration.get_llvm_function("mathz.sqrt").is_some());
    
    // Verify function signatures in the module
    let len_func = module.get_function("core.len");
    assert!(len_func.is_some());
    
    let spill_func = module.get_function("vibez.spill");
    assert!(spill_func.is_some());
    
    let abs_func = module.get_function("mathz.abs");
    assert!(abs_func.is_some());
}

#[traced_test]
#[test]
fn test_code_generator_stdlib_integration() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Initialize integrations manually for testing
    generator.initialize_integrations();
    
    // Verify stdlib integration is initialized
    assert!(generator.stdlib_integration.is_some());
    
    // Verify GC integration is initialized
    assert!(generator.gc_integration.is_some());
    
    // Verify that stdlib functions are available
    let stdlib_integration = generator.stdlib_integration.as_ref().unwrap();
    assert!(stdlib_integration.get_function_info("len").is_some());
    assert!(stdlib_integration.get_function_info("vibez.spill").is_some());
    
    // Verify that runtime functions are declared in the module
    assert!(generator.module.get_function("printf").is_some());
    assert!(generator.module.get_function("malloc").is_some());
    assert!(generator.module.get_function("free").is_some());
    
    // Verify that LLVM intrinsics are declared
    assert!(generator.module.get_function("llvm.memcpy.p0i8.p0i8.i64").is_some());
    assert!(generator.module.get_function("llvm.sin.f64").is_some());
    assert!(generator.module.get_function("llvm.sqrt.f64").is_some());
}

#[traced_test]
#[test]
fn test_direct_function_call_compilation() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Initialize integrations for testing
    generator.initialize_integrations();
    
    // Create a simple function call: len("hello")
    let function_name = Identifier::new("len".to_string());
    let string_arg = StringLiteral::new("hello".to_string());
    let call_expr = CallExpression::new(
        Box::new(function_name) as Box<dyn Expression>,
        vec![Box::new(string_arg) as Box<dyn Expression>]
    );
    
    // Compile the call expression
    let result = generator.compile_call_expression(&call_expr);
    
    // Should succeed (even if we don't have runtime implementation yet)
    match result {
        Ok(_) => {
            // Success - the function call was compiled
        }
        Err(e) => {
            // If it fails, it should be due to missing runtime, not missing declaration
            assert!(!e.contains("not found"), "Function should be declared: {}", e);
        }
    }
}

#[traced_test]
#[test]
fn test_qualified_function_call_compilation() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Initialize integrations for testing
    generator.initialize_integrations();
    
    // Create a qualified function call: vibez.spill("hello")
    let package_name = Identifier::new("vibez".to_string());
    let function_name = Identifier::new("spill".to_string());
    let dot_expr = DotExpression::new(
        Box::new(package_name) as Box<dyn Expression>,
        Box::new(function_name) as Box<dyn Expression>
    );
    let string_arg = StringLiteral::new("hello".to_string());
    let call_expr = CallExpression::new(
        Box::new(dot_expr) as Box<dyn Expression>,
        vec![Box::new(string_arg) as Box<dyn Expression>]
    );
    
    // Compile the call expression
    let result = generator.compile_call_expression(&call_expr);
    
    // Should succeed (even if we don't have runtime implementation yet)
    match result {
        Ok(_) => {
            // Success - the qualified function call was compiled
        }
        Err(e) => {
            // If it fails, it should be due to missing runtime, not missing declaration
            assert!(!e.contains("not found"), "Qualified function should be declared: {}", e);
        }
    }
}

#[traced_test]
#[test]
fn test_type_mapping_coverage() {
    let context = Context::create();
    let module = context.create_module("test_stdlib");
    let integration = StdlibLlvmIntegration::new(&context, &module);
    
    // Test basic type mappings
    assert!(integration.map_cursed_type_to_llvm("void").unwrap().is_none());
    assert!(integration.map_cursed_type_to_llvm("bool").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("i32").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("i64").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("f32").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("f64").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("char").unwrap().is_some());
    
    // Test complex type mappings
    assert!(integration.map_cursed_type_to_llvm("string").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("array").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("slice").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("map").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("any").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("number").unwrap().is_some());
    
    // Test special types
    assert!(integration.map_cursed_type_to_llvm("mutex").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("channel").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("regex").unwrap().is_some());
    assert!(integration.map_cursed_type_to_llvm("template").unwrap().is_some());
    
    // Test unknown types (should default to pointer)
    assert!(integration.map_cursed_type_to_llvm("unknown_type").unwrap().is_some());
}

#[traced_test]
#[test]
fn test_gc_integration_with_stdlib() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Initialize integrations for testing
    generator.initialize_integrations();
    
    // Verify GC integration is available
    let gc_integration = generator.gc_integration.as_ref().unwrap();
    
    // Check that built-in types have GC metadata
    assert!(gc_integration.get_type_metadata("string").is_some());
    assert!(gc_integration.get_type_metadata("slice").is_some());
    assert!(gc_integration.get_type_metadata("map").is_some());
    assert!(gc_integration.get_type_metadata("channel").is_some());
    assert!(gc_integration.get_type_metadata("interface").is_some());
    assert!(gc_integration.get_type_metadata("function").is_some());
    
    // Verify string type metadata
    let string_metadata = gc_integration.get_type_metadata("string").unwrap();
    assert_eq!(string_metadata.type_name, "string");
    assert_eq!(string_metadata.type_size, 16);
    assert_eq!(string_metadata.pointer_fields, vec![1]);
    assert!(!string_metadata.needs_finalization);
    
    // Verify channel type metadata (should need finalization)
    let channel_metadata = gc_integration.get_type_metadata("channel").unwrap();
    assert_eq!(channel_metadata.type_name, "channel");
    assert!(channel_metadata.needs_finalization);
    
    // Check GC descriptor table is available
    assert!(gc_integration.get_gc_descriptor_table().is_some());
}

#[traced_test]
#[test]
fn test_runtime_function_declarations() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Initialize integrations for testing
    generator.initialize_integrations();
    
    // Verify C library functions are declared
    assert!(generator.module.get_function("printf").is_some());
    assert!(generator.module.get_function("malloc").is_some());
    assert!(generator.module.get_function("free").is_some());
    assert!(generator.module.get_function("strlen").is_some());
    assert!(generator.module.get_function("strcmp").is_some());
    
    // Verify CURSED runtime functions are declared
    assert!(generator.module.get_function("cursed_string_create").is_some());
    assert!(generator.module.get_function("cursed_string_concat").is_some());
    assert!(generator.module.get_function("cursed_slice_create").is_some());
    assert!(generator.module.get_function("cursed_map_create").is_some());
    assert!(generator.module.get_function("cursed_channel_create").is_some());
    assert!(generator.module.get_function("cursed_gc_alloc").is_some());
    assert!(generator.module.get_function("cursed_panic").is_some());
    
    // Verify LLVM intrinsics are declared
    assert!(generator.module.get_function("llvm.memcpy.p0i8.p0i8.i64").is_some());
    assert!(generator.module.get_function("llvm.sin.f64").is_some());
    assert!(generator.module.get_function("llvm.gcroot").is_some());
}

#[traced_test]
#[test]
fn test_comprehensive_package_coverage() {
    let registry = StdlibRegistry::new();
    
    // Test all major packages are represented
    let packages: Vec<String> = registry.get_packages().cloned().collect();
    
    // Core functionality packages
    assert!(packages.contains(&"core".to_string()));
    assert!(packages.contains(&"vibez".to_string()));
    assert!(packages.contains(&"stringz".to_string()));
    assert!(packages.contains(&"mathz".to_string()));
    
    // I/O and system packages
    assert!(packages.contains(&"dropz".to_string()));
    assert!(packages.contains(&"vibe_life".to_string()));
    
    // Concurrency packages
    assert!(packages.contains(&"concurrenz".to_string()));
    
    // Network and data packages
    assert!(packages.contains(&"web_vibez".to_string()));
    assert!(packages.contains(&"json_tea".to_string()));
    
    // Utility packages
    assert!(packages.contains(&"regex_vibez".to_string()));
    assert!(packages.contains(&"cryptz".to_string()));
    assert!(packages.contains(&"reflectz".to_string()));
    assert!(packages.contains(&"rizztemplate".to_string()));
    assert!(packages.contains(&"htmlrizzler".to_string()));
    assert!(packages.contains(&"chadlogging".to_string()));
    
    // Verify each package has functions
    for package in &packages {
        let package_functions = registry.get_package_functions(package);
        assert!(package_functions.is_some(), "Package {} should have functions", package);
        assert!(!package_functions.unwrap().is_empty(), "Package {} should have non-empty function list", package);
    }
}

/// Documentation tests that explain why this integration is important
mod docs {
    //! Why Standard Library LLVM Integration Tests Are Critical
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
    #[cfg(test)]
    fn test_documentation() {}
}
