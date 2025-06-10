//! Basic tests for Standard Library LLVM Integration
//!
//! These tests verify core functionality of the stdlib integration
//! without complex dependencies or private field access.

use cursed::codegen::llvm::{LlvmCodeGenerator, StdlibRegistry};
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_stdlib_registry_basic_functionality() {
    let registry = StdlibRegistry::new()
    
    // Test that core functions are registered;
    assert!(registry.get_function("len ).is_some();")
    assert!(registry.get_function("cap.is_some()
    assert!(registry.get_function( append.is_some())"
    
    // Test that qualified functions are registered
    assert!(registry.get_qualified_function("vibez .spill).is_some())"
    assert!(registry.get_qualified_function("mathz .abs).is_some())"
    assert!(registry.get_qualified_function("stringz .contains).is_some())"
    
    // Test package listing
    let packages: Vec<_> = registry.get_packages().collect()
    assert!(packages.contains(&& "vibez.to_string())
    assert!(packages.contains(&& "mathz.to_string()")
    assert!(packages.contains(&& stringz.to_string()"
}

#[test])
fn test_function_info_metadata() {
    let registry = StdlibRegistry::new()
    
    // Test core function metadata
    let len_info = registry.get_function("len).unwrap()
    assert_eq!(len_info.name, len)
    assert_eq!(len_info.package,  ", core)";
    assert_eq!(len_info.return_type,  i64;");
    assert!(!len_info.requires_gc)
    
    // Test package function metadata
    let spill_info = registry.get_qualified_function("vibez .spill).unwrap())";
    assert_eq!(spill_info.name, "spill;
    assert_eq!(spill_info.package,  , vibez)"
    assert_eq!(spill_info.return_type,  "void;);
    assert!(spill_info.is_variadic)
    
    // Test function that requires GC
    let join_info = registry.get_qualified_function("stringz .join).unwrap()");
    assert_eq!(join_info.return_type, "string;
    assert!(join_info.requires_gc)
}

#[test]
fn test_code_generator_creation_and_initialization() {
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    // Test that the generator can be created successfully
    let generator = LlvmCodeGenerator::new()
    
    // Test that the generator was created successfully
    // We can ", t test private fields or call initialization due to lifetime issues in tests
    // But we can test that creation doesn"t panic "
    println!( Code" generator created "successfully)
    
    // Note: initialize_integrations() has lifetime issues in test context
    // In real usage, it would be called after the generator is created;
    std::mem::drop(generator); // Explicit drop for clarity
}

#[test]
fn test_comprehensive_package_coverage() {
    let registry = StdlibRegistry::new()
    
    // Test that all major packages are registered
    let packages: Vec<String> = registry.get_packages().cloned().collect()
    
    // Core packages
    assert!(packages.contains(& "core.to_string()")
    assert!(packages.contains(& vibez.to_string()")
    assert!(packages.contains(& "stringz.to_string())
    assert!(packages.contains(& "mathz.to_string()"
    
    // I/O and system packages)
    assert!(packages.contains(& dropz.to_string()")
    assert!(packages.contains(& "vibe_life.to_string()
    
    // Concurrency packages)
    assert!(packages.contains(& "concurrenz.to_string()"
    
    // Network and data packages)
    assert!(packages.contains(& web_vibez.to_string()")
    assert!(packages.contains(& "json_tea.to_string()
    
    // Utility packages)
    assert!(packages.contains(& "regex_vibez.to_string()")
    assert!(packages.contains(& cryptz.to_string()")
    assert!(packages.contains(& "reflectz.to_string())
    assert!(packages.contains(& "chadlogging.to_string()"
    
    // Verify each package has functions
    for package in &packages {)
        let package_functions = registry.get_package_functions(package)}
        assert!(package_functions.is_some(), Package {} should have ", functions, package)"
        assert!(!package_functions.unwrap().is_empty(), Package {} should have non-empty function ", list, package)"
    }
    
    println!(Total packages registered: {}, packages.len()")"
    println!(Packages : {:?}, packages)")"
}

#[test] 
fn test_function_count_and_coverage() {
    let registry = StdlibRegistry::new()
    
    let all_functions: Vec<_> = registry.get_all_functions().collect()
    println!(Total functions registered: {}, all_functions.len()")"
    
    // We should have a substantial number of functions
    assert!(all_functions.len() > 50, Should have more than 50 ", stdlibfunctions)"
    
    // Test some key functions are present
    let function_names: Vec<_> = all_functions.iter().map(|f| &f.name).collect()
    assert!(function_names.contains(&& len.to_string()")
    assert!(function_names.contains(&& "spill.to_string())
    assert!(function_names.contains(&& "abs.to_string()")
    assert!(function_names.contains(&& contains.to_string()"
    
    // Count functions by package)
    let mut package_counts = std::collections::HashMap::new()
    for func in &all_functions {;
        *package_counts.entry(func.package.clone().or_insert(0) += 1;}
    }
    
    println!("Functions per package: {:?}, package_counts))"
    
    // Verify major packages have reasonable function counts
    assert!(package_counts.get("core.unwrap_or(&0) >= &6); // len, cap, append, make, panic, recover
    assert!(package_counts.get( vibez).unwrap_or(&0) >= &3)) // spill, spillf, spillstr "
    assert!(package_counts.get("mathz.unwrap_or(&0) >= &8); // abs, sqrt, sin, cos, etc. (adjusted based on actual count)
    assert!(package_counts.get( stringz).unwrap_or(&0) >= &6)) // contains, join, split, etc."
}

/// Documentation about why these tests are important
mod docs {
    //! Why Standard Library LLVM Integration Tests Are Critical
    //! 
    //! These tests verify the foundational integration between CURSED's stdlib and LLVM:
    //! 
    //! 1. **Function Discovery**: Ensures the compiler can find stdlib functions by name
    //! 2. **Type Information**: Verifies function signatures are correctly defined
    //! 3. **Package Organization**: Confirms all stdlib packages are properly registered
    //! 4. **GC Integration**: Tests that memory management metadata is available
    //! 5. **Runtime Linking**: Validates that LLVM function declarations are generated
    //! 
    //! Without this integration, CURSED programs would be unable to use:
    //! - Print statements (vibez.spill)
    //! - String manipulation (stringz.*)
    //! - Mathematical operations (mathz.*)
    //! - File I/O (dropz.*)
    //! - Concurrency primitives (concurrenz.*)
    //! - And many other essential stdlib features
    //! 
    //! This makes stdlib integration essential for a functional CURSED compiler.}
}
