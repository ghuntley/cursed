/// Integration tests for enhanced debugging system
///
/// Validates integration between debug information system, runtime debugging,
/// error context enhancement, and LLVM debug integration.

use cursed::debug::enhanced_debug::*;
use cursed::runtime::debug_runtime::*;
use cursed::error::debug_context::*;
use cursed::error::Error as CursedError;
use cursed::stdlib::value::Value;
use std::path::PathBuf;
use std::collections::HashMap;

#[test]
fn test_enhanced_debug_info_integration() {
    // Create enhanced debug info
    let debug_info = EnhancedDebugInfo::new("test.csd", 42, 10, "test_function".to_string())
        .with_symbol_metadata(SymbolMetadata::function("test_function", Some("slay")))
        .with_type_info(TypeDebugInfo::new("TestType".to_string(), TypeKind::Struct));

    assert_eq!(debug_info.debug_info.line, 42);
    assert_eq!(debug_info.debug_info.column, 10);
    assert!(debug_info.is_user_code());
    assert_eq!(debug_info.location_string(), "test.csd:42:10");
}

#[test]
fn test_debug_registry_operations() {
    let registry = DebugInfoRegistry::new();

    // Register debug information
    let debug_info = EnhancedDebugInfo::new("test.csd", 42, 10, "test_function".to_string());
    let location_key = "test.csd:42:10".to_string();
    
    let result = registry.register_debug_info(location_key.clone(), debug_info);
    assert!(result.is_ok());

    // Retrieve debug information
    let retrieved = registry.get_debug_info(&location_key);
    assert!(retrieved.is_ok());
    assert!(retrieved.unwrap().is_some());

    // Register symbol metadata
    let metadata = SymbolMetadata::function("test_function", Some("slay"));
    let symbol_result = registry.register_symbol("module::test_function".to_string(), metadata);
    assert!(symbol_result.is_ok());

    // Search symbols
    let matches = registry.find_symbols("test");
    assert!(matches.is_ok());
    assert!(!matches.unwrap().is_empty());
}

#[test]
fn test_runtime_debugger_integration() {
    let debugger = RuntimeDebugger::new(true);

    // Enter function scope
    let frame_id = debugger.enter_function("test_func", std::path::Path::new("test.csd"), 42);
    assert!(frame_id.is_ok());

    // Register variables
    let var_result = debugger.register_variable(
        "test_var".to_string(),
        Value::String("test value".to_string()),
        "tea".to_string(),
        10,
    );
    assert!(var_result.is_ok());

    // Inspect variable
    let inspection = debugger.inspect_variable("test_var");
    assert!(inspection.is_ok());
    let inspection = inspection.unwrap();
    assert!(inspection.is_some());

    let inspection = inspection.unwrap();
    assert_eq!(inspection.name, "test_var");
    assert_eq!(inspection.var_type, "tea");
    assert!(inspection.contents.contains("test value"));

    // Exit function scope
    let exit_result = debugger.exit_function(frame_id.unwrap());
    assert!(exit_result.is_ok());
}

#[test]
fn test_enhanced_error_context_integration() {
    let runtime_debugger = RuntimeDebugger::new(true);
    let registry = DebugInfoRegistry::new();

    // Create error with enhanced debug context
    let error = CursedError::Runtime("Test runtime error".to_string());
    let debug_context = DebugContext::new(error)
        .with_runtime_debugger(std::sync::Arc::new(runtime_debugger))
        .with_debug_registry(std::sync::Arc::new(registry))
        .with_symbol_metadata(SymbolMetadata::function("error_func", Some("slay")))
        .with_annotation("context".to_string(), "Test error context".to_string());

    // Generate error report
    let mut debug_context = debug_context;
    let report = debug_context.generate_error_report();

    assert!(report.contains("Error:"));
    assert!(report.contains("Test runtime error"));
    assert!(report.contains("Additional information"));
    assert!(report.contains("Test error context"));
    assert!(report.contains("Gen Z Keyword: slay"));
}

#[test]
fn test_source_mapping_integration() {
    let mut source_map = SourceMap::new(PathBuf::from("test.csd"));

    // Add mapping ranges
    source_map.add_range(10, 5, 8, 3, 15);  // generated -> original
    source_map.add_range(15, 0, 12, 0, 20);

    // Test mapping
    let mapped1 = source_map.map_to_original(10, 10);
    assert_eq!(mapped1, Some((8, 8)));

    let mapped2 = source_map.map_to_original(15, 5);
    assert_eq!(mapped2, Some((12, 5)));

    // Test out of range
    let mapped3 = source_map.map_to_original(100, 0);
    assert_eq!(mapped3, None);
}

#[test]
fn test_variable_inspection_comprehensive() {
    let inspector = VariableInspector::new();

    // Test different value types
    let runtime_var = RuntimeVariable::new(
        "complex_var".to_string(),
        Value::Object({
            let mut obj = HashMap::new();
            obj.insert("field1".to_string(), Value::Integer(42));
            obj.insert("field2".to_string(), Value::String("test".to_string()));
            obj.insert("field3".to_string(), Value::Array(vec![Value::Integer(1), Value::Integer(2)]));
            obj
        }),
        "Object".to_string(),
        25,
    );

    let inspection = inspector.inspect_variable(&runtime_var);
    assert!(inspection.is_ok());

    let inspection = inspection.unwrap();
    assert_eq!(inspection.name, "complex_var");
    assert!(inspection.size_estimate > 0);
    assert!(inspection.type_info.is_complex);
    assert!(inspection.contents.contains("field1"));
    assert!(inspection.contents.contains("field2"));
}

#[test]
fn test_breakpoint_simulation() {
    let debugger = RuntimeDebugger::new(true);

    // Set breakpoint
    let bp_id = debugger.set_breakpoint(PathBuf::from("test.csd"), 42);
    assert!(bp_id.is_ok());

    // Check breakpoint triggering
    let check = debugger.check_breakpoint(std::path::Path::new("test.csd"), 42);
    assert!(check.is_ok());
    assert!(check.unwrap().is_some());

    // Check non-matching location
    let no_match = debugger.check_breakpoint(std::path::Path::new("test.csd"), 50);
    assert!(no_match.is_ok());
    assert!(no_match.unwrap().is_none());

    // Remove breakpoint
    let removed = debugger.remove_breakpoint(bp_id.unwrap());
    assert!(removed.is_ok());
    assert!(removed.unwrap());
}

#[test]
fn test_scope_management_complex() {
    let debugger = RuntimeDebugger::new(true);

    // Create nested function scopes
    let frame1 = debugger.enter_function("outer_func", std::path::Path::new("test.csd"), 10);
    assert!(frame1.is_ok());

    let _ = debugger.register_variable(
        "outer_var".to_string(),
        Value::Integer(1),
        "sus".to_string(),
        11,
    );

    let frame2 = debugger.enter_function("inner_func", std::path::Path::new("test.csd"), 20);
    assert!(frame2.is_ok());

    let _ = debugger.register_variable(
        "inner_var".to_string(),
        Value::Integer(2),
        "sus".to_string(),
        21,
    );

    // Get stack trace
    let stack_trace = debugger.get_stack_trace();
    assert!(stack_trace.is_ok());
    let frames = stack_trace.unwrap();
    assert_eq!(frames.len(), 2);
    assert_eq!(frames[0].function_name, "outer_func");
    assert_eq!(frames[1].function_name, "inner_func");

    // Get scope variables (should only see inner scope)
    let scope_vars = debugger.get_scope_variables();
    assert!(scope_vars.is_ok());
    let vars = scope_vars.unwrap();
    assert!(vars.contains_key("inner_var"));

    // Exit inner scope
    let _ = debugger.exit_function(frame2.unwrap());

    // Get scope variables (should see outer scope)
    let scope_vars = debugger.get_scope_variables();
    assert!(scope_vars.is_ok());
    let vars = scope_vars.unwrap();
    assert!(vars.contains_key("outer_var"));

    // Exit outer scope
    let _ = debugger.exit_function(frame1.unwrap());
}

#[test]
fn test_debug_report_generation() {
    let debugger = RuntimeDebugger::new(true);

    // Setup debugging context
    let _ = debugger.enter_function("report_func", std::path::Path::new("test.csd"), 30);
    let _ = debugger.register_variable(
        "report_var".to_string(),
        Value::String("test".to_string()),
        "tea".to_string(),
        31,
    );
    let _ = debugger.set_breakpoint(PathBuf::from("test.csd"), 35);

    // Generate debug report
    let report = debugger.generate_debug_report();
    assert!(report.is_ok());

    let report = report.unwrap();
    assert!(!report.stack_trace.is_empty());
    assert!(!report.scope_variables.is_empty());
    assert!(!report.active_breakpoints.is_empty());

    // Check report formatting
    let report_string = format!("{}", report);
    assert!(report_string.contains("Stack Trace"));
    assert!(report_string.contains("Scope Variables"));
    assert!(report_string.contains("Active Breakpoints"));
    assert!(report_string.contains("Performance"));
}

#[test]
fn test_type_debug_info_creation() {
    let type_info = TypeDebugInfo::new("TestStruct".to_string(), TypeKind::Struct)
        .with_field(FieldDebugInfo::new("field1".to_string(), "sus".to_string()))
        .with_field(FieldDebugInfo::new("field2".to_string(), "tea".to_string()))
        .with_type_parameter("T".to_string());

    assert_eq!(type_info.type_name, "TestStruct");
    assert_eq!(type_info.type_kind, TypeKind::Struct);
    assert_eq!(type_info.fields.len(), 2);
    assert_eq!(type_info.type_parameters.len(), 1);
    assert_eq!(type_info.fields[0].name, "field1");
    assert_eq!(type_info.fields[1].field_type, "tea");
}

#[test]
fn test_error_chain_visualization() {
    // Create error chain
    let root_error = CursedError::Runtime("Root cause".to_string());
    let intermediate_error = CursedError::ParseError {
        message: "Parse failed".to_string(),
        source_location: None,
    };
    let final_error = CursedError::panic_error("Final panic".to_string());

    let mut debug_context = DebugContext::new(final_error)
        .with_error_chain(intermediate_error)
        .with_error_chain(root_error)
        .with_annotation("context".to_string(), "Error chain test".to_string());

    let report = debug_context.generate_error_report();

    assert!(report.contains("Error chain"));
    assert!(report.contains("Root cause"));
    assert!(report.contains("Parse failed"));
    assert!(report.contains("Final panic"));
    assert!(report.contains("[CRITICAL]")); // Panic severity
    assert!(report.contains("[ERROR]"));    // Other error severity
}

#[test]
fn test_performance_monitoring() {
    let debugger = RuntimeDebugger::new(true);

    // Perform multiple function calls to generate performance data
    for i in 0..5 {
        let frame_id = debugger.enter_function(
            &format!("perf_func_{}", i),
            std::path::Path::new("test.csd"),
            i * 10,
        );
        assert!(frame_id.is_ok());

        // Simulate some work with a small delay
        std::thread::sleep(std::time::Duration::from_millis(1));

        let _ = debugger.exit_function(frame_id.unwrap());
    }

    // Generate report with performance data
    let report = debugger.generate_debug_report();
    assert!(report.is_ok());

    let report = report.unwrap();
    assert!(report.performance_data.total_function_calls >= 5);
    assert!(!report.performance_data.function_call_counts.is_empty());
}

#[test]
fn test_disabled_debugging_performance() {
    // Test that disabled debugging has minimal overhead
    let debugger = RuntimeDebugger::new(false);

    // These operations should succeed but do nothing
    let frame_id = debugger.enter_function("no_debug", std::path::Path::new("test.csd"), 1);
    assert_eq!(frame_id.unwrap(), 0); // Should return dummy ID

    let var_result = debugger.register_variable(
        "no_debug_var".to_string(),
        Value::Integer(42),
        "sus".to_string(),
        2,
    );
    assert!(var_result.is_ok());

    let inspection = debugger.inspect_variable("no_debug_var");
    assert_eq!(inspection.unwrap(), None); // Should return None when disabled

    let _ = debugger.exit_function(0);
}

#[test]
fn test_gen_z_keyword_integration() {
    let registry = DebugInfoRegistry::new();

    // Register function with Gen Z keyword
    let metadata = SymbolMetadata::function("calculate", Some("slay"))
        .with_attribute("description".to_string(), "Calculates values like a boss".to_string())
        .with_tag("math".to_string());

    let result = registry.register_symbol("math::calculate".to_string(), metadata);
    assert!(result.is_ok());

    // Test variable with Gen Z type mapping
    let var_metadata = SymbolMetadata::variable("count", "i32");
    assert_eq!(var_metadata.attributes.get("gen_z_type"), Some(&"sus".to_string()));

    let bool_metadata = SymbolMetadata::variable("is_valid", "bool");
    assert_eq!(bool_metadata.attributes.get("gen_z_type"), Some(&"facts".to_string()));

    let float_metadata = SymbolMetadata::variable("energy", "f64");
    assert_eq!(float_metadata.attributes.get("gen_z_type"), Some(&"vibes".to_string()));

    let string_metadata = SymbolMetadata::variable("message", "String");
    assert_eq!(string_metadata.attributes.get("gen_z_type"), Some(&"tea".to_string()));
}
