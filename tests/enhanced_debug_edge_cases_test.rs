/// Edge case tests for enhanced debugging system
///
/// Tests error conditions, edge cases, and boundary conditions for the
/// debugging system to ensure robustness and proper error handling.

use cursed::debug::enhanced_debug::*;
use cursed::runtime::debug_runtime::*;
use cursed::error::debug_context::*;
use cursed::error::Error as CursedError;
use cursed::stdlib::value::Value;
use std::path::PathBuf;
use std::collections::HashMap;

#[test]
fn test_empty_debug_info_handling() {
    let registry = DebugInfoRegistry::new();

    // Test retrieving non-existent debug info
    let result = registry.get_debug_info("non_existent_key");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());

    // Test empty symbol search
    let matches = registry.find_symbols("non_existent_pattern");
    assert!(matches.is_ok());
    assert!(matches.unwrap().is_empty());

    // Test statistics with empty registry
    let stats = registry.get_statistics();
    assert!(stats.is_ok());
    let stats = stats.unwrap();
    assert_eq!(stats.debug_info_count, 0);
    assert_eq!(stats.symbol_count, 0);
}

#[test]
fn test_invalid_file_paths() {
    // Test with invalid file path
    let debug_info = EnhancedDebugInfo::new(
        "/invalid/path/that/does/not/exist.csd",
        42,
        10,
        "test_function".to_string(),
    );

    assert_eq!(debug_info.debug_info.line, 42);
    assert_eq!(debug_info.debug_info.column, 10);
    // Should handle invalid paths gracefully
    assert!(!debug_info.is_user_code()); // No .csd extension detection
}

#[test]
fn test_source_mapping_edge_cases() {
    let mut source_map = SourceMap::new(PathBuf::from("test.csd"));

    // Test mapping with zero-length ranges
    source_map.add_range(10, 5, 8, 3, 0);
    let mapped = source_map.map_to_original(10, 5);
    assert_eq!(mapped, Some((8, 3)));

    // Test mapping beyond range
    let mapped_beyond = source_map.map_to_original(10, 6);
    assert_eq!(mapped_beyond, None);

    // Test mapping with overlapping ranges
    source_map.add_range(10, 0, 8, 0, 20);
    source_map.add_range(10, 10, 8, 10, 5);
    
    let mapped_overlap = source_map.map_to_original(10, 12);
    assert_eq!(mapped_overlap, Some((8, 12)));
}

#[test]
fn test_runtime_debugger_error_conditions() {
    let debugger = RuntimeDebugger::new(true);

    // Test exiting non-existent function
    let result = debugger.exit_function(999);
    assert!(result.is_ok()); // Should handle gracefully

    // Test getting non-existent variable
    let var = debugger.get_variable("non_existent_var");
    assert!(var.is_ok());
    assert_eq!(var.unwrap(), None);

    // Test inspecting non-existent variable
    let inspection = debugger.inspect_variable("non_existent_var");
    assert!(inspection.is_ok());
    assert!(inspection.unwrap().is_none());

    // Test removing non-existent breakpoint
    let removed = debugger.remove_breakpoint(999);
    assert!(removed.is_ok());
    assert!(!removed.unwrap());
}

#[test]
fn test_variable_inspection_complex_types() {
    let inspector = VariableInspector::new();

    // Test with deeply nested structure
    let mut deep_object = HashMap::new();
    for i in 0..5 {
        let mut inner_object = HashMap::new();
        for j in 0..5 {
            inner_object.insert(
                format!("inner_{}", j),
                Value::Array((0..10).map(|k| Value::Integer(i * 100 + j * 10 + k)).collect()),
            );
        }
        deep_object.insert(format!("level_{}", i), Value::Object(inner_object));
    }

    let runtime_var = RuntimeVariable::new(
        "deep_var".to_string(),
        Value::Object(deep_object),
        "DeepObject".to_string(),
        1,
    );

    let inspection = inspector.inspect_variable(&runtime_var);
    assert!(inspection.is_ok());

    let inspection = inspection.unwrap();
    assert!(inspection.size_estimate > 0);
    assert!(inspection.type_info.is_complex);
    // Should handle deep nesting gracefully
    assert!(inspection.contents.len() > 0);
}

#[test]
fn test_variable_inspection_max_depth() {
    let inspector = VariableInspector::new();

    // Create a structure that exceeds max depth
    let mut current_value = Value::Integer(42);
    
    // Create nested structure deeper than max depth (10)
    for _ in 0..15 {
        let mut wrapper = HashMap::new();
        wrapper.insert("nested".to_string(), current_value);
        current_value = Value::Object(wrapper);
    }

    let runtime_var = RuntimeVariable::new(
        "max_depth_var".to_string(),
        current_value,
        "NestedObject".to_string(),
        1,
    );

    let inspection = inspector.inspect_variable(&runtime_var);
    assert!(inspection.is_ok());

    let inspection = inspection.unwrap();
    // Should contain max depth message
    assert!(inspection.contents.contains("max depth reached"));
}

#[test]
fn test_breakpoint_edge_cases() {
    let debugger = RuntimeDebugger::new(true);

    // Set breakpoint at line 0
    let bp_id1 = debugger.set_breakpoint(PathBuf::from("test.csd"), 0);
    assert!(bp_id1.is_ok());

    // Set breakpoint at very high line number
    let bp_id2 = debugger.set_breakpoint(PathBuf::from("test.csd"), u32::MAX);
    assert!(bp_id2.is_ok());

    // Set multiple breakpoints at same location
    let bp_id3 = debugger.set_breakpoint(PathBuf::from("test.csd"), 42);
    let bp_id4 = debugger.set_breakpoint(PathBuf::from("test.csd"), 42);
    assert!(bp_id3.is_ok());
    assert!(bp_id4.is_ok());
    assert_ne!(bp_id3.unwrap(), bp_id4.unwrap());

    // Check breakpoint at same location (should find first one)
    let check = debugger.check_breakpoint(std::path::Path::new("test.csd"), 42);
    assert!(check.is_ok());
    assert!(check.unwrap().is_some());
}

#[test]
fn test_error_context_with_empty_data() {
    let error = CursedError::Runtime("Test error".to_string());
    let mut debug_context = DebugContext::new(error);

    // Generate report with no additional data
    let report = debug_context.generate_error_report();
    
    assert!(report.contains("Error:"));
    assert!(report.contains("Test error"));
    // Should handle empty data gracefully
    assert!(!report.contains("Stack trace"));
    assert!(!report.contains("Variables at error point"));
}

#[test]
fn test_error_context_with_nil_values() {
    let debugger = RuntimeDebugger::new(true);
    
    // Register variables with nil values
    let _ = debugger.enter_function("nil_test", std::path::Path::new("test.csd"), 1);
    let _ = debugger.register_variable(
        "nil_var".to_string(),
        Value::Nil,
        "nil".to_string(),
        2,
    );

    let inspection = debugger.inspect_variable("nil_var");
    assert!(inspection.is_ok());
    
    let inspection = inspection.unwrap();
    assert!(inspection.is_some());
    
    let inspection = inspection.unwrap();
    assert_eq!(inspection.contents, "nil");
    assert_eq!(inspection.size_estimate, 0);
}

#[test]
fn test_scope_management_edge_cases() {
    let debugger = RuntimeDebugger::new(true);

    // Test exiting scope that was never entered
    let result = debugger.exit_function(0);
    assert!(result.is_ok());

    // Test extremely deep nesting
    let mut frame_ids = Vec::new();
    for i in 0..100 {
        let frame_id = debugger.enter_function(
            &format!("deep_func_{}", i),
            std::path::Path::new("test.csd"),
            i as u32,
        );
        assert!(frame_id.is_ok());
        frame_ids.push(frame_id.unwrap());
    }

    // Get stack trace with deep nesting
    let stack_trace = debugger.get_stack_trace();
    assert!(stack_trace.is_ok());
    assert_eq!(stack_trace.unwrap().len(), 100);

    // Exit all frames
    for frame_id in frame_ids.into_iter().rev() {
        let _ = debugger.exit_function(frame_id);
    }
}

#[test]
fn test_symbol_metadata_edge_cases() {
    // Test with empty strings
    let metadata1 = SymbolMetadata::function("", None);
    assert_eq!(metadata1.symbol_type, SymbolType::Function);

    let metadata2 = SymbolMetadata::variable("", "");
    assert_eq!(metadata2.symbol_type, SymbolType::Variable);

    // Test with very long names
    let long_name = "a".repeat(1000);
    let metadata3 = SymbolMetadata::function(&long_name, Some("slay"));
    assert_eq!(metadata3.attributes.get("gen_z_keyword"), Some(&"slay".to_string()));

    // Test with special characters
    let special_name = "func!@#$%^&*()";
    let metadata4 = SymbolMetadata::function(special_name, None);
    assert_eq!(metadata4.symbol_type, SymbolType::Function);
}

#[test]
fn test_type_debug_info_edge_cases() {
    // Test with empty type name
    let type_info1 = TypeDebugInfo::new("".to_string(), TypeKind::Struct);
    assert_eq!(type_info1.type_name, "");
    assert_eq!(type_info1.type_kind, TypeKind::Struct);

    // Test with many fields
    let mut type_info2 = TypeDebugInfo::new("LargeStruct".to_string(), TypeKind::Struct);
    for i in 0..1000 {
        let field = FieldDebugInfo::new(format!("field_{}", i), "sus".to_string());
        type_info2 = type_info2.with_field(field);
    }
    assert_eq!(type_info2.fields.len(), 1000);

    // Test with duplicate field names
    let mut type_info3 = TypeDebugInfo::new("DuplicateStruct".to_string(), TypeKind::Struct);
    for _ in 0..5 {
        let field = FieldDebugInfo::new("same_name".to_string(), "sus".to_string());
        type_info3 = type_info3.with_field(field);
    }
    assert_eq!(type_info3.fields.len(), 5);
    assert!(type_info3.fields.iter().all(|f| f.name == "same_name"));
}

#[test]
fn test_concurrent_debugging_edge_cases() {
    use std::sync::Arc;
    use std::thread;

    let debugger = Arc::new(RuntimeDebugger::new(true));
    let registry = Arc::new(DebugInfoRegistry::new());

    // Test concurrent access to same resources
    let mut handles = Vec::new();
    
    for thread_id in 0..4 {
        let debugger_clone = debugger.clone();
        let registry_clone = registry.clone();
        
        let handle = thread::spawn(move || {
            // All threads try to access same function name
            for i in 0..100 {
                let frame_id = debugger_clone.enter_function(
                    "shared_function", // Same name
                    std::path::Path::new("shared.csd"),
                    i as u32,
                ).unwrap();

                // Register variable with same name
                let _ = debugger_clone.register_variable(
                    "shared_var".to_string(), // Same name
                    Value::Integer((thread_id * 1000 + i) as i64),
                    "sus".to_string(),
                    i as u32,
                );

                // Register debug info with same key
                let debug_info = EnhancedDebugInfo::new(
                    "shared.csd",
                    i as u32,
                    1,
                    "shared_function".to_string(),
                );
                
                let location_key = format!("shared.csd:{}:1", i);
                let _ = registry_clone.register_debug_info(location_key, debug_info);

                let _ = debugger_clone.exit_function(frame_id);
            }
        });
        
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify system remains consistent
    let report = debugger.generate_debug_report();
    assert!(report.is_ok());

    let stats = registry.get_statistics();
    assert!(stats.is_ok());
}

#[test]
fn test_memory_pressure_conditions() {
    let debugger = RuntimeDebugger::new(true);

    // Create large amounts of debug data
    for i in 0..1000 {
        let frame_id = debugger.enter_function(
            &format!("memory_func_{}", i),
            std::path::Path::new("memory.csd"),
            i as u32,
        ).unwrap();

        // Register many variables with large data
        for j in 0..10 {
            let large_string = "x".repeat(1000);
            let _ = debugger.register_variable(
                format!("large_var_{}_{}", i, j),
                Value::String(large_string),
                "tea".to_string(),
                i as u32,
            );
        }

        // Create large object
        let mut large_object = HashMap::new();
        for k in 0..100 {
            large_object.insert(
                format!("field_{}", k),
                Value::String("data".repeat(100)),
            );
        }

        let _ = debugger.register_variable(
            format!("large_object_{}", i),
            Value::Object(large_object),
            "LargeObject".to_string(),
            i as u32,
        );

        let _ = debugger.exit_function(frame_id);
    }

    // Generate report under memory pressure
    let report = debugger.generate_debug_report();
    assert!(report.is_ok());

    // System should remain functional
    let final_vars = debugger.get_scope_variables();
    assert!(final_vars.is_ok());
}

#[test]
fn test_unicode_and_special_characters() {
    let debugger = RuntimeDebugger::new(true);

    // Test with Unicode function names
    let unicode_name = "测试函数_🚀";
    let frame_id = debugger.enter_function(
        unicode_name,
        std::path::Path::new("unicode_测试.csd"),
        42,
    );
    assert!(frame_id.is_ok());

    // Test with Unicode variable names and values
    let _ = debugger.register_variable(
        "变量名".to_string(),
        Value::String("Unicode value: 你好世界 🌍".to_string()),
        "tea".to_string(),
        43,
    );

    // Test inspection with Unicode
    let inspection = debugger.inspect_variable("变量名");
    assert!(inspection.is_ok());
    assert!(inspection.unwrap().is_some());

    // Test with special characters in paths
    let _ = debugger.set_breakpoint(
        PathBuf::from("file with spaces & symbols!@#.csd"),
        100,
    );

    let _ = debugger.exit_function(frame_id.unwrap());
}

#[test]
fn test_error_propagation_in_debug_context() {
    // Test error context with multiple error types
    let errors = vec![
        CursedError::Runtime("Runtime error".to_string()),
        CursedError::ParseError {
            message: "Parse error".to_string(),
            source_location: None,
        },
        CursedError::Type("Type error".to_string()),
        CursedError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")),
        CursedError::panic_error("Panic error".to_string()),
    ];

    for error in errors {
        let mut debug_context = DebugContext::new(error)
            .with_annotation("test".to_string(), "Error propagation test".to_string());

        let report = debug_context.generate_error_report();
        assert!(report.contains("Error:"));
        assert!(report.contains("Error propagation test"));
        
        // Check severity classification
        let severity = debug_context.severity();
        assert!(matches!(severity, ErrorSeverity::Info | ErrorSeverity::Warning | ErrorSeverity::Error | ErrorSeverity::Critical | ErrorSeverity::Fatal));
    }
}

#[test]
fn test_debug_system_recovery_from_errors() {
    let debugger = RuntimeDebugger::new(true);

    // Cause various error conditions and verify recovery
    
    // 1. Try to exit more functions than entered
    for _ in 0..10 {
        let _ = debugger.exit_function(999);
    }

    // System should still work
    let frame_id = debugger.enter_function("recovery_test", std::path::Path::new("test.csd"), 1);
    assert!(frame_id.is_ok());

    // 2. Register variables with problematic data
    let _ = debugger.register_variable(
        "".to_string(), // Empty name
        Value::Nil,
        "".to_string(), // Empty type
        0,
    );

    // 3. Try to access non-existent data
    let _ = debugger.get_variable("non_existent");
    let _ = debugger.inspect_variable("non_existent");

    // System should still generate reports
    let report = debugger.generate_debug_report();
    assert!(report.is_ok());

    let _ = debugger.exit_function(frame_id.unwrap());
}
