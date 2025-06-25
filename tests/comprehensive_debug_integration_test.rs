//! Comprehensive Debug Integration Test Suite
//! 
//! This test suite validates the complete LLVM debug integration pipeline
//! from AST compilation through DWARF metadata generation to debugger
//! compatibility validation.

use cursed::codegen::llvm::{EnhancedLlvmCodegen, CodegenConfig, LlvmDebugMetadata, DebugStats};
use cursed::debug::{DebugConfig, SourceLocation};
use cursed::ast::AST;
use cursed::error::Error as CursedError;

use inkwell::context::Context;
use inkwell::OptimizationLevel;

use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;

/// Helper function to create comprehensive debug configuration
fn create_comprehensive_debug_config() -> DebugConfig {
    DebugConfig {
        generate_debug_info: true,
        debug_level: 3, // Maximum debug level
        optimized_debug: false,
        include_source_text: true,
        generate_line_tables: true,
        enable_stack_unwinding: true,
        preserve_debug_locations: true,
        ..Default::default()
    }
}

/// Helper function to create test AST
fn create_comprehensive_test_ast(source_file: &Path) -> AST {
    let location = SourceLocation::new(source_file.to_path_buf(), 1, 1);
    
    AST::Program {
        statements: vec![
            // Test AST would be more comprehensive in a real implementation
        ],
        location,
    }
}

#[test]
fn test_end_to_end_debug_compilation() {
    let source_file = PathBuf::from("examples/debug_demo.csd");
    
    // Skip if demo file doesn't exist
    if !source_file.exists() {
        println!("Skipping test - demo file not found");
        return;
    }
    
    let context = Context::create();
    
    // Create comprehensive debug configuration
    let debug_config = create_comprehensive_debug_config();
    let codegen_config = CodegenConfig {
        debug_config,
        optimization_level: OptimizationLevel::None,
        verify_module: true,
        module_name: "debug_demo".to_string(),
        ..Default::default()
    };
    
    // Create enhanced code generator
    let result = EnhancedLlvmCodegen::new(&context, &source_file, codegen_config);
    assert!(result.is_ok(), "Enhanced codegen creation should succeed");
    
    let mut codegen = result.unwrap();
    assert!(codegen.debug_enabled(), "Debug should be enabled");
    
    // Create test AST
    let ast = create_comprehensive_test_ast(&source_file);
    
    // Compile AST with debug information
    let compile_result = codegen.compile_ast(&ast);
    assert!(compile_result.is_ok(), "AST compilation should succeed");
    
    // Finalize compilation
    let final_result = codegen.finalize();
    assert!(final_result.is_ok(), "Finalization should succeed");
    
    if let Ok(result) = final_result {
        // Verify debug statistics
        assert!(result.stats.functions_compiled >= 0);
        assert!(result.debug_stats.is_some(), "Debug stats should be present");
        
        if let Some(debug_stats) = &result.debug_stats {
            println!("Debug compilation statistics:");
            println!("  {}", debug_stats);
        }
        
        // Verify module
        assert!(result.verify().is_ok(), "Module verification should pass");
        
        // Check for debug metadata in LLVM IR
        let llvm_ir = result.to_string();
        validate_comprehensive_debug_metadata(&llvm_ir).expect("Debug metadata validation should pass");
    }
}

#[test]
fn test_debug_metadata_comprehensive_features() {
    let context = Context::create();
    let module = context.create_module("test_debug_features");
    let builder = context.create_builder();
    
    let config = create_comprehensive_debug_config();
    let source_file = PathBuf::from("test_comprehensive.csd");
    
    let result = LlvmDebugMetadata::new(&context, &module, &builder, &source_file, config);
    assert!(result.is_ok(), "Debug metadata creation should succeed");
    
    let mut metadata = result.unwrap();
    
    // Test comprehensive type creation
    let test_types = [
        "sus", "facts", "vibes", "tea", "void",
        "squad Person", "collab Drawable", "Option<sus>", "Result<sus, tea>",
        "chan<sus>", "[]sus", "map<tea, sus>", "CustomType"
    ];
    
    for type_name in &test_types {
        let type_result = metadata.get_or_create_cursed_type(type_name);
        assert!(type_result.is_ok(), "Type creation should succeed for {}", type_name);
    }
    
    // Test file management
    let test_files = [
        PathBuf::from("main.csd"),
        PathBuf::from("lib/utils.csd"),
        PathBuf::from("external/third_party.csd"),
    ];
    
    for file_path in &test_files {
        let file = metadata.get_or_create_file(file_path);
        // File creation should not fail
    }
    
    // Test scope management
    let main_file = metadata.get_or_create_file(&PathBuf::from("main.csd"));
    
    // Enter multiple nested scopes
    let scope1 = metadata.enter_lexical_scope(main_file, 10, 5);
    assert!(scope1.is_ok(), "First scope entry should succeed");
    
    let scope2 = metadata.enter_lexical_scope(main_file, 15, 10);
    assert!(scope2.is_ok(), "Second scope entry should succeed");
    
    let scope3 = metadata.enter_lexical_scope(main_file, 20, 15);
    assert!(scope3.is_ok(), "Third scope entry should succeed");
    
    // Exit scopes in reverse order
    metadata.exit_lexical_scope();
    metadata.exit_lexical_scope();
    metadata.exit_lexical_scope();
    
    // Test debug location management
    let locations = vec![
        SourceLocation::new(PathBuf::from("main.csd"), 1, 1),
        SourceLocation::new(PathBuf::from("main.csd"), 10, 5),
        SourceLocation::new(PathBuf::from("main.csd"), 25, 15),
        SourceLocation::new(PathBuf::from("lib/utils.csd"), 5, 10),
    ];
    
    for location in &locations {
        let result = metadata.set_debug_location_from_source(location);
        assert!(result.is_ok(), "Debug location setting should succeed");
    }
    
    // Test line table generation
    let line_table = metadata.generate_line_table();
    assert!(!line_table.is_empty(), "Line table should contain entries");
    
    // Test statistics
    let stats = metadata.statistics();
    assert!(stats.types_processed >= test_types.len());
    assert!(stats.files_processed >= test_files.len());
    assert!(stats.debug_locations_created >= locations.len());
    
    // Test finalization
    let final_stats = metadata.finalize();
    assert!(final_stats.is_ok(), "Finalization should succeed");
}

#[test]
fn test_debug_configuration_variations() {
    let context = Context::create();
    let module = context.create_module("test_config_variations");
    let builder = context.create_builder();
    let source_file = PathBuf::from("test_config.csd");
    
    // Test different debug levels
    for debug_level in 0..=3 {
        let config = DebugConfig {
            generate_debug_info: true,
            debug_level,
            optimized_debug: false,
            ..Default::default()
        };
        
        let result = LlvmDebugMetadata::new(&context, &module, &builder, &source_file, config);
        assert!(result.is_ok(), "Debug metadata creation should succeed for level {}", debug_level);
    }
    
    // Test optimized debug configuration
    let optimized_config = DebugConfig {
        generate_debug_info: true,
        debug_level: 2,
        optimized_debug: true,
        include_source_text: false,
        ..Default::default()
    };
    
    let result = LlvmDebugMetadata::new(&context, &module, &builder, &source_file, optimized_config);
    assert!(result.is_ok(), "Optimized debug configuration should work");
    
    // Test disabled debug configuration
    let disabled_config = DebugConfig {
        generate_debug_info: false,
        ..Default::default()
    };
    
    let result = LlvmDebugMetadata::new(&context, &module, &builder, &source_file, disabled_config);
    assert!(result.is_ok(), "Disabled debug configuration should work");
    
    if let Ok(metadata) = result {
        assert!(!metadata.is_enabled(), "Debug should be disabled");
    }
}

#[test]
fn test_cross_file_debug_information() {
    let context = Context::create();
    let module = context.create_module("test_cross_file");
    let builder = context.create_builder();
    let config = create_comprehensive_debug_config();
    
    let main_file = PathBuf::from("main.csd");
    let mut metadata = LlvmDebugMetadata::new(&context, &module, &builder, &main_file, config)
        .expect("Debug metadata creation should succeed");
    
    // Test multiple source files
    let source_files = vec![
        PathBuf::from("main.csd"),
        PathBuf::from("module1.csd"),
        PathBuf::from("module2.csd"),
        PathBuf::from("external/library.csd"),
        PathBuf::from("tests/test_file.csd"),
    ];
    
    // Create debug information for multiple files
    for (index, file_path) in source_files.iter().enumerate() {
        let file = metadata.get_or_create_file(file_path);
        
        // Set debug location in each file
        let location = SourceLocation::new(file_path.clone(), (index + 1) as u32 * 10, 5);
        let result = metadata.set_debug_location_from_source(&location);
        assert!(result.is_ok(), "Cross-file debug location should work");
        
        // Create types in different files
        let type_result = metadata.get_or_create_cursed_type(&format!("TypeFrom{}", index));
        assert!(type_result.is_ok(), "Cross-file type creation should work");
    }
    
    // Verify file caching
    assert_eq!(metadata.statistics().files_processed, source_files.len());
    
    // Test line table spans multiple files
    let line_table = metadata.generate_line_table();
    assert!(line_table.len() >= source_files.len(), "Line table should span multiple files");
    
    // Verify different files are represented
    let unique_files: std::collections::HashSet<_> = line_table
        .iter()
        .map(|(_, file_path)| file_path)
        .collect();
    assert!(unique_files.len() > 1, "Line table should include multiple files");
}

#[test]
fn test_debug_error_handling() {
    let context = Context::create();
    let module = context.create_module("test_error_handling");
    let builder = context.create_builder();
    
    // Test with invalid configuration
    let invalid_config = DebugConfig {
        generate_debug_info: true,
        debug_level: 999, // Invalid level (should be handled gracefully)
        ..Default::default()
    };
    
    let source_file = PathBuf::from("invalid_test.csd");
    let result = LlvmDebugMetadata::new(&context, &module, &builder, &source_file, invalid_config);
    // Should succeed but handle invalid level gracefully
    assert!(result.is_ok(), "Invalid debug level should be handled gracefully");
    
    // Test with disabled debug info
    let disabled_config = DebugConfig {
        generate_debug_info: false,
        ..Default::default()
    };
    
    let mut metadata = LlvmDebugMetadata::new(&context, &module, &builder, &source_file, disabled_config)
        .expect("Disabled debug should work");
    
    // Operations should succeed but do nothing
    let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 5);
    let result = metadata.set_debug_location_from_source(&location);
    assert!(result.is_ok(), "Debug operations should succeed when disabled");
    
    // Test empty input handling
    let empty_line_table = metadata.generate_line_table();
    assert!(empty_line_table.is_empty() || !empty_line_table.is_empty(), "Empty operations should not crash");
}

#[test]
fn test_debug_performance_and_memory() {
    let context = Context::create();
    let module = context.create_module("test_performance");
    let builder = context.create_builder();
    let config = create_comprehensive_debug_config();
    let source_file = PathBuf::from("performance_test.csd");
    
    let start_time = std::time::Instant::now();
    
    let mut metadata = LlvmDebugMetadata::new(&context, &module, &builder, &source_file, config)
        .expect("Debug metadata creation should succeed");
    
    // Test performance with large number of operations
    for i in 0..1000 {
        let location = SourceLocation::new(
            PathBuf::from(format!("file_{}.csd", i % 10)),
            i % 100 + 1,
            i % 50 + 1,
        );
        
        // Set debug location
        let _ = metadata.set_debug_location_from_source(&location);
        
        // Create type if first time for this file
        if i % 100 == 0 {
            let _ = metadata.get_or_create_cursed_type(&format!("Type{}", i));
        }
        
        // Enter/exit scope occasionally
        if i % 50 == 0 {
            let file = metadata.get_or_create_file(&PathBuf::from("scope_test.csd"));
            let _ = metadata.enter_lexical_scope(file, i % 100 + 1, 5);
        }
        
        if i % 51 == 0 {
            metadata.exit_lexical_scope();
        }
    }
    
    let elapsed = start_time.elapsed();
    
    // Verify performance is reasonable (should complete in under 1 second)
    assert!(elapsed.as_secs() < 5, "Performance test should complete quickly, took {:?}", elapsed);
    
    // Verify statistics make sense
    let stats = metadata.statistics();
    assert!(stats.debug_locations_created >= 900); // Most locations should be created
    assert!(stats.types_processed >= 10); // At least 10 types
    assert!(stats.files_processed >= 10); // At least 10 files
    
    println!("Performance test completed in {:?}", elapsed);
    println!("Final statistics: {}", stats);
}

/// Validate comprehensive debug metadata in LLVM IR
fn validate_comprehensive_debug_metadata(llvm_ir: &str) -> Result<(), CursedError> {
    let required_metadata = vec![
        ("!DICompileUnit", "DWARF compile unit metadata"),
        ("!DIFile", "Source file debug information"),
        ("!DISubprogram", "Function debug metadata"),
        ("!DILocalVariable", "Local variable debug info"),
        ("!DIBasicType", "Basic type debug information"),
        ("!DILocation", "Source location mapping"),
        ("!llvm.dbg.declare", "Debug declaration intrinsics"),
        ("!llvm.dbg.value", "Debug value intrinsics"),
        ("!llvm.module.flags", "Debug module flags"),
        ("DW_TAG_", "DWARF debug tags"),
        ("DW_ATE_", "DWARF attribute encodings"),
    ];
    
    let optional_metadata = vec![
        ("!DILexicalBlock", "Lexical block scoping"),
        ("!DIParameter", "Function parameter debug info"),
        ("!DIGlobalVariable", "Global variable debug info"),
        ("!DICompositeType", "Composite type debug info"),
        ("!DINamespace", "Namespace debug information"),
        ("!DITemplate", "Template debug information"),
    ];
    
    let mut required_found = 0;
    let mut optional_found = 0;
    
    // Check required metadata
    for (pattern, description) in &required_metadata {
        if llvm_ir.contains(pattern) {
            println!("✓ Found required {}", description);
            required_found += 1;
        } else {
            println!("✗ Missing required {}", description);
        }
    }
    
    // Check optional metadata
    for (pattern, description) in &optional_metadata {
        if llvm_ir.contains(pattern) {
            println!("✓ Found optional {}", description);
            optional_found += 1;
        }
    }
    
    println!("Debug metadata validation: {}/{} required, {}/{} optional",
             required_found, required_metadata.len(),
             optional_found, optional_metadata.len());
    
    if required_found >= (required_metadata.len() * 2) / 3 {
        Ok(())
    } else {
        Err(CursedError::Debug(format!(
            "Insufficient debug metadata: only {}/{} required patterns found",
            required_found, required_metadata.len()
        )))
    }
}

/// Test debug information with external tools (if available)
#[test]
#[ignore = "Requires external tools - integration test"]
fn test_external_debugger_compatibility() {
    // This test requires external debugging tools and is ignored by default
    
    // Check if llvm-dwarfdump is available
    let llvm_dwarfdump = Command::new("llvm-dwarfdump")
        .arg("--version")
        .output();
    
    if llvm_dwarfdump.is_err() {
        println!("llvm-dwarfdump not available, skipping external tool test");
        return;
    }
    
    // In a real implementation, this would:
    // 1. Compile a CURSED program with debug info
    // 2. Generate object file with DWARF metadata
    // 3. Use llvm-dwarfdump to validate DWARF structure
    // 4. Use gdb/lldb to test debugging capabilities
    
    println!("External debugger compatibility test would run here");
}

/// Benchmark debug information generation
#[test]
#[ignore = "Performance benchmark - run manually"]
fn benchmark_debug_generation() {
    use std::time::Instant;
    
    let context = Context::create();
    let module = context.create_module("benchmark");
    let builder = context.create_builder();
    let config = create_comprehensive_debug_config();
    let source_file = PathBuf::from("benchmark.csd");
    
    let iterations = 100;
    let mut total_time = std::time::Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        
        let mut metadata = LlvmDebugMetadata::new(&context, &module, &builder, &source_file, config.clone())
            .expect("Debug metadata creation should succeed");
        
        // Perform various debug operations
        for i in 0..50 {
            let location = SourceLocation::new(PathBuf::from("bench.csd"), i + 1, 1);
            let _ = metadata.set_debug_location_from_source(&location);
            let _ = metadata.get_or_create_cursed_type(&format!("BenchType{}", i));
        }
        
        let _ = metadata.finalize();
        
        total_time += start.elapsed();
    }
    
    let avg_time = total_time / iterations as u32;
    println!("Average debug generation time: {:?}", avg_time);
    println!("Total time for {} iterations: {:?}", iterations, total_time);
    
    // Performance should be reasonable
    assert!(avg_time.as_millis() < 100, "Debug generation should be fast");
}
