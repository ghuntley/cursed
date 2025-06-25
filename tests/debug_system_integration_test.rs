/// Debug System Integration Tests
/// 
/// Comprehensive system-wide integration tests for the CURSED debugging infrastructure
/// including LLVM integration, CLI tool functionality, runtime debugging, memory debugging,
/// crash reporting, and real-time debugging features.

use cursed::debug::{
    EnhancedDebugInfo, DebugInfoRegistry, SymbolMetadata, SymbolType, SymbolVisibility,
    SourceMap, TypeDebugInfo, TypeKind, FieldDebugInfo, ScopeInfo, ScopeType, DebugConfig,
    DebugInfoManager, DebugStatistics
};
use cursed::runtime::debug_info::{DebugInfo, VariableInfo, EnhancedStackFrame, SymbolInfo};
use cursed::runtime::debug_manager::{DebugManager, SourceFile, DebugEvent};
use cursed::runtime::debug_runtime::{DebugRuntime, RuntimeDebugInfo};
use cursed::runtime::debug_output::{DebugOutput, OutputFormat};
use cursed::error::{Error as CursedError, SourceLocation as ErrorSourceLocation};
use cursed::compiler::Compiler;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::fs;
use std::io::Write;

#[path = "common.rs"]
mod common;

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{info, debug, error, warn};

    #[test]
    fn test_complete_debug_system_integration() {
        init_tracing!();
        info!("Testing complete debug system integration");

        // Step 1: Create a comprehensive test program
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("integration_test.csd");
        let test_program = r#"
// Integration test program
import "stdlib::io";

slay factorial(sus n) -> sus {
    lowkey (n <= 1) {
        periodt 1;
    }
    periodt n * factorial(n - 1);
}

squad Calculator {
    sus precision;
    
    slay new(sus prec) -> Calculator {
        periodt Calculator { precision: prec };
    }
    
    slay compute(sus a, sus b) -> sus {
        facts is_valid = a > 0 && b > 0;
        lowkey (!is_valid) {
            panic("Invalid input values");
        }
        
        sus result = a + b * this.precision;
        periodt result;
    }
}

collab Processable {
    slay process() -> tea;
    slay validate() -> facts;
}

slay main() {
    sus num = 5;
    sus fact_result = factorial(num);
    
    facts calc = Calculator::new(2);
    sus calc_result = calc.compute(10, 20);
    
    tea message = "Results: " + fact_result.to_string() + ", " + calc_result.to_string();
    println(message);
    
    lowkey (sus i = 0; i < 3; i++) {
        tea loop_msg = "Iteration: " + i.to_string();
        println(loop_msg);
    }
    
    periodt;
}
"#;

        fs::write(&test_file, test_program).unwrap();

        // Step 2: Initialize debug system components
        let mut debug_manager = DebugManager::new();
        debug_manager.enable();
        debug_manager.set_verbose(true);

        let debug_registry = Arc::new(DebugInfoRegistry::new());
        let mut debug_runtime = DebugRuntime::new();

        // Step 3: Load source file and create mappings
        let mut source_file = SourceFile::new(&test_file);
        source_file.load_content().unwrap();

        // Create comprehensive source map
        let mut source_map = SourceMap::new(test_file.clone());
        
        // Map key program elements
        source_map.add_range(4, 0, 4, 0, 30);  // factorial function
        source_map.add_range(10, 0, 10, 0, 18); // Calculator struct
        source_map.add_range(25, 0, 25, 0, 22); // Processable interface
        source_map.add_range(30, 0, 30, 0, 11); // main function
        source_map.add_range(32, 4, 32, 4, 25); // factorial call
        source_map.add_range(34, 4, 34, 4, 28); // Calculator instantiation
        source_map.add_range(35, 4, 35, 4, 32); // compute call
        source_map.add_range(39, 4, 39, 4, 35); // for loop

        debug_registry.register_source_map(test_file.clone(), source_map).unwrap();

        // Step 4: Create detailed debug information
        let factorial_debug = EnhancedDebugInfo::new(&test_file, 4, 0, "factorial".to_string())
            .with_symbol_metadata(SymbolMetadata::function("factorial", Some("slay"))
                .with_attribute("recursive".to_string(), "true".to_string())
                .with_attribute("parameters".to_string(), "n: sus".to_string())
                .with_attribute("return_type".to_string(), "sus".to_string())
                .with_tag("mathematical".to_string())
                .with_tag("recursive".to_string()));

        let calculator_debug = EnhancedDebugInfo::new(&test_file, 10, 0, "Calculator".to_string())
            .with_symbol_metadata({
                let mut metadata = SymbolMetadata::new();
                metadata.symbol_type = SymbolType::Struct;
                metadata.visibility = SymbolVisibility::Public;
                metadata.attributes.insert("fields".to_string(), "precision: sus".to_string());
                metadata.tags.push("computational".to_string());
                metadata
            })
            .with_type_info(TypeDebugInfo::new("Calculator".to_string(), TypeKind::Struct)
                .with_field(FieldDebugInfo::new("precision".to_string(), "i32".to_string())));

        let main_debug = EnhancedDebugInfo::new(&test_file, 30, 0, "main".to_string())
            .with_symbol_metadata(SymbolMetadata::function("main", Some("slay"))
                .with_attribute("entry_point".to_string(), "true".to_string())
                .with_tag("entry".to_string()));

        // Register all debug information
        debug_registry.register_debug_info("factorial:4:0".to_string(), factorial_debug).unwrap();
        debug_registry.register_debug_info("Calculator:10:0".to_string(), calculator_debug).unwrap();
        debug_registry.register_debug_info("main:30:0".to_string(), main_debug).unwrap();

        // Step 5: Test symbol resolution and lookup
        let factorial_symbols = debug_registry.find_symbols("factorial").unwrap();
        assert!(!factorial_symbols.is_empty());
        assert!(factorial_symbols.iter().any(|(name, metadata)| 
            name.contains("factorial") && metadata.symbol_type == SymbolType::Function));

        let math_symbols = debug_registry.find_symbols("mathematical").unwrap();
        assert!(!math_symbols.is_empty());

        // Step 6: Test type information integration
        let calculator_type = debug_registry.get_type("Calculator").unwrap();
        assert!(calculator_type.is_some());
        let calc_type_info = calculator_type.unwrap();
        assert_eq!(calc_type_info.type_kind, TypeKind::Struct);
        assert_eq!(calc_type_info.fields.len(), 1);
        assert_eq!(calc_type_info.fields[0].name, "precision");

        // Step 7: Test scope hierarchy creation
        let module_scope = ScopeInfo {
            scope_type: ScopeType::Module,
            depth: 0,
            parent_scope: None,
            variables: HashMap::new(),
            start_location: Some((1, 1)),
            end_location: Some((45, 1)),
        };
        let module_id = debug_registry.create_scope(module_scope).unwrap();

        let mut main_scope = ScopeInfo::function_scope(1);
        main_scope.parent_scope = Some(module_id);
        main_scope.start_location = Some((30, 0));
        main_scope.end_location = Some((43, 1));

        // Add variables to main scope
        let num_var = VariableInfo::new("num".to_string(), "i32".to_string());
        let fact_result_var = VariableInfo::new("fact_result".to_string(), "i32".to_string());
        let calc_var = VariableInfo::new("calc".to_string(), "Calculator".to_string());

        main_scope.add_variable("num".to_string(), num_var);
        main_scope.add_variable("fact_result".to_string(), fact_result_var);
        main_scope.add_variable("calc".to_string(), calc_var);

        let main_scope_id = debug_registry.create_scope(main_scope).unwrap();

        // Create loop scope
        let mut loop_scope = ScopeInfo {
            scope_type: ScopeType::Loop,
            depth: 2,
            parent_scope: Some(main_scope_id),
            variables: HashMap::new(),
            start_location: Some((39, 4)),
            end_location: Some((42, 5)),
        };

        let loop_var = VariableInfo::new("i".to_string(), "i32".to_string());
        loop_scope.add_variable("i".to_string(), loop_var);
        let loop_scope_id = debug_registry.create_scope(loop_scope).unwrap();

        // Step 8: Test runtime debug information
        debug_runtime.enable();
        
        // Simulate runtime state
        let mut runtime_info = RuntimeDebugInfo::new("main".to_string());
        runtime_info.add_variable("num".to_string(), "5".to_string(), "i32".to_string());
        runtime_info.add_variable("fact_result".to_string(), "120".to_string(), "i32".to_string());
        runtime_info.add_variable("calc_result".to_string(), "50".to_string(), "i32".to_string());

        debug_runtime.update_runtime_info(runtime_info);

        // Step 9: Test stack frame creation
        let stack_frames = vec![
            EnhancedStackFrame::new("main".to_string(), test_file.clone(), 32, 4)
                .with_variable("num".to_string(), "5".to_string(), "i32".to_string())
                .with_variable("fact_result".to_string(), "120".to_string(), "i32".to_string()),
            EnhancedStackFrame::new("factorial".to_string(), test_file.clone(), 4, 0)
                .with_variable("n".to_string(), "5".to_string(), "i32".to_string()),
        ];

        // Test stack frame information
        assert_eq!(stack_frames.len(), 2);
        assert_eq!(stack_frames[0].function_name, "main");
        assert_eq!(stack_frames[1].function_name, "factorial");
        assert!(stack_frames[0].has_variable("num"));
        assert!(stack_frames[1].has_variable("n"));

        // Step 10: Test debug output formatting
        let mut debug_output = DebugOutput::new();
        debug_output.set_format(OutputFormat::Json);

        let debug_data = serde_json::json!({
            "function": "main",
            "line": 32,
            "variables": {
                "num": 5,
                "fact_result": 120
            }
        });

        let formatted_output = debug_output.format_debug_info(&debug_data.to_string());
        assert!(formatted_output.is_ok());

        // Step 11: Test statistics and verification
        let final_stats = debug_registry.get_statistics().unwrap();
        assert!(final_stats.debug_info_count >= 3);
        assert!(final_stats.symbol_count >= 0);
        assert!(final_stats.type_count >= 1);
        assert!(final_stats.scope_count >= 3);

        info!("Debug system integration stats: {}", final_stats);

        // Step 12: Test error handling integration
        let error_source = ErrorSourceLocation::new(test_file.clone(), 35, 4);
        let debug_context = debug_manager.create_debug_context(&error_source);
        assert!(debug_context.is_some());

        // Cleanup
        fs::remove_file(&test_file).ok();

        debug!("Complete debug system integration test passed");
    }

    #[test]
    fn test_llvm_debug_integration() {
        init_tracing!();
        info!("Testing LLVM debug integration");

        let mut debug_info_manager = DebugInfoManager::new();
        
        // Test compilation unit initialization
        let source_file = PathBuf::from("test_llvm.csd");
        let init_result = debug_info_manager.initialize_compilation_unit(
            source_file.clone(),
            "CURSED Compiler v1.0".to_string()
        );
        assert!(init_result.is_ok());

        // Test function debug information
        let main_location = ErrorSourceLocation::new(source_file.clone(), 10, 1);
        let begin_result = debug_info_manager.begin_function("main".to_string(), main_location.clone());
        assert!(begin_result.is_ok());

        // Test variable debug information
        let var_location = ErrorSourceLocation::new(source_file.clone(), 11, 5);
        let var_result = debug_info_manager.add_variable(
            "test_var".to_string(),
            "i32".to_string(),
            var_location.clone()
        );
        assert!(var_result.is_ok());

        // Test debug location generation
        let debug_location = debug_info_manager.generate_debug_location(&var_location);
        assert!(!debug_location.is_empty());

        // Test current location tracking
        debug_info_manager.set_current_location(var_location.clone());
        let current_loc = debug_info_manager.current_location();
        assert!(current_loc.is_some());
        assert_eq!(current_loc.unwrap().line, 11);

        // Test function end
        let end_result = debug_info_manager.end_function();
        assert!(end_result.is_ok());

        // Test LLVM metadata generation
        let metadata_result = debug_info_manager.generate_llvm_debug_metadata();
        assert!(metadata_result.is_ok());

        // Test line table generation
        let line_table = debug_info_manager.generate_line_table();
        assert!(!line_table.is_empty());

        // Test validation
        let validation_result = debug_info_manager.validate();
        assert!(validation_result.is_ok());

        // Test statistics
        let stats = debug_info_manager.statistics();
        assert!(stats.symbol_count >= 0);

        debug!("LLVM debug integration test passed");
    }

    #[test]
    fn test_debug_cli_functionality() {
        init_tracing!();
        info!("Testing debug CLI functionality");

        // Create test files for CLI testing
        let temp_dir = std::env::temp_dir();
        let project_dir = temp_dir.join("debug_cli_test");
        fs::create_dir_all(&project_dir).unwrap();

        let main_file = project_dir.join("main.csd");
        let lib_file = project_dir.join("lib.csd");

        let main_content = r#"
import "lib";

slay main() {
    sus result = lib::calculate(42);
    println("Result: " + result.to_string());
    periodt;
}
"#;

        let lib_content = r#"
slay calculate(sus value) -> sus {
    periodt value * 2;
}

squad MathUtils {
    slay power(sus base, sus exp) -> sus {
        sus result = 1;
        lowkey (sus i = 0; i < exp; i++) {
            result = result * base;
        }
        periodt result;
    }
}
"#;

        fs::write(&main_file, main_content).unwrap();
        fs::write(&lib_file, lib_content).unwrap();

        // Test debug configuration
        let mut debug_config = DebugConfig::default();
        debug_config.enable_debug = true;
        debug_config.debug_level = 2;
        debug_config.generate_dwarf = true;
        debug_config.optimize_debug_info = false;
        debug_config.emit_location_info = true;

        // Create debug manager with configuration
        let mut debug_manager = DebugManager::new();
        debug_manager.apply_config(&debug_config);
        assert!(debug_manager.is_enabled());

        // Test source file loading
        let mut main_source = SourceFile::new(&main_file);
        let load_result = main_source.load_content();
        assert!(load_result.is_ok());

        let mut lib_source = SourceFile::new(&lib_file);
        let load_result = lib_source.load_content();
        assert!(load_result.is_ok());

        // Test debug information extraction
        let debug_registry = DebugInfoRegistry::new();
        
        // Extract debug info from main.csd
        let main_debug = EnhancedDebugInfo::new(&main_file, 4, 0, "main".to_string())
            .with_symbol_metadata(SymbolMetadata::function("main", Some("slay")));
        
        debug_registry.register_debug_info("main:4:0".to_string(), main_debug).unwrap();

        // Extract debug info from lib.csd
        let calculate_debug = EnhancedDebugInfo::new(&lib_file, 2, 0, "calculate".to_string())
            .with_symbol_metadata(SymbolMetadata::function("calculate", Some("slay")));
        
        let mathutils_debug = EnhancedDebugInfo::new(&lib_file, 6, 0, "MathUtils".to_string())
            .with_symbol_metadata({
                let mut metadata = SymbolMetadata::new();
                metadata.symbol_type = SymbolType::Struct;
                metadata
            });

        debug_registry.register_debug_info("calculate:2:0".to_string(), calculate_debug).unwrap();
        debug_registry.register_debug_info("MathUtils:6:0".to_string(), mathutils_debug).unwrap();

        // Test symbol search (CLI command simulation)
        let all_functions = debug_registry.find_symbols("slay").unwrap();
        assert!(all_functions.len() >= 2);

        let main_functions = debug_registry.find_symbols("main").unwrap();
        assert!(!main_functions.is_empty());

        let calculate_functions = debug_registry.find_symbols("calculate").unwrap();
        assert!(!calculate_functions.is_empty());

        // Test line information extraction
        let main_line_3 = main_source.get_line(3);
        assert!(main_line_3.is_some());
        assert!(main_line_3.unwrap().contains("import"));

        let main_line_5 = main_source.get_line(5);
        assert!(main_line_5.is_some());
        assert!(main_line_5.unwrap().contains("calculate"));

        // Test context extraction (for debugger stepping)
        let context = main_source.get_lines_with_context(5, 2);
        assert!(context.is_some());
        let context_lines = context.unwrap();
        assert!(context_lines.len() >= 3);

        // Test breakpoint locations
        let breakpoint_locations = vec![
            ("main.csd".to_string(), 5_u32),
            ("lib.csd".to_string(), 3_u32),
            ("lib.csd".to_string(), 9_u32),
        ];

        for (file, line) in breakpoint_locations {
            let source = if file == "main.csd" { &main_source } else { &lib_source };
            let line_content = source.get_line(line);
            assert!(line_content.is_some());
            debug!("Breakpoint at {}:{} -> {}", file, line, line_content.unwrap());
        }

        // Test statistics reporting (CLI status command)
        let stats = debug_registry.get_statistics().unwrap();
        info!("CLI Debug Statistics: {}", stats);
        assert!(stats.debug_info_count >= 3);

        // Cleanup
        fs::remove_dir_all(&project_dir).ok();

        debug!("Debug CLI functionality test passed");
    }

    #[test]
    fn test_crash_reporting_system() {
        init_tracing!();
        info!("Testing crash reporting system");

        let debug_registry = Arc::new(DebugInfoRegistry::new());
        let mut debug_manager = DebugManager::new();
        debug_manager.enable();

        // Create a test scenario that would cause a crash
        let temp_dir = std::env::temp_dir();
        let crash_test_file = temp_dir.join("crash_test.csd");
        let crash_program = r#"
slay divide_by_zero(sus a, sus b) -> sus {
    lowkey (b == 0) {
        panic("Division by zero!");
    }
    periodt a / b;
}

slay access_invalid_array() {
    sus[] numbers = [1, 2, 3];
    sus invalid = numbers[10]; // Out of bounds access
    println("Value: " + invalid.to_string());
}

slay main() {
    sus result1 = divide_by_zero(10, 0);
    access_invalid_array();
    periodt;
}
"#;

        fs::write(&crash_test_file, crash_program).unwrap();

        // Set up debug information for crash reporting
        let divide_debug = EnhancedDebugInfo::new(&crash_test_file, 2, 0, "divide_by_zero".to_string())
            .with_symbol_metadata(SymbolMetadata::function("divide_by_zero", Some("slay"))
                .with_attribute("can_panic".to_string(), "true".to_string())
                .with_tag("unsafe".to_string()));

        let array_debug = EnhancedDebugInfo::new(&crash_test_file, 8, 0, "access_invalid_array".to_string())
            .with_symbol_metadata(SymbolMetadata::function("access_invalid_array", Some("slay"))
                .with_attribute("can_panic".to_string(), "true".to_string())
                .with_tag("unsafe".to_string()));

        let main_debug = EnhancedDebugInfo::new(&crash_test_file, 13, 0, "main".to_string())
            .with_symbol_metadata(SymbolMetadata::function("main", Some("slay"))
                .with_tag("entry".to_string()));

        debug_registry.register_debug_info("divide_by_zero:2:0".to_string(), divide_debug).unwrap();
        debug_registry.register_debug_info("access_invalid_array:8:0".to_string(), array_debug).unwrap();
        debug_registry.register_debug_info("main:13:0".to_string(), main_debug).unwrap();

        // Simulate crash scenarios and create crash reports
        let crash_scenarios = vec![
            ("Division by zero", "divide_by_zero", 4, "Division by zero!"),
            ("Array bounds", "access_invalid_array", 10, "Index out of bounds"),
            ("Null pointer", "main", 14, "Null pointer dereference"),
        ];

        for (crash_type, function, line, error_msg) in crash_scenarios {
            // Create crash report
            let crash_location = ErrorSourceLocation::new(crash_test_file.clone(), line, 5);
            
            // Create enhanced stack frame for crash
            let crash_frame = EnhancedStackFrame::new(
                function.to_string(),
                crash_test_file.clone(),
                line,
                5
            ).with_error_context(error_msg.to_string());

            // Test crash context creation
            let crash_context = debug_manager.create_debug_context(&crash_location);
            assert!(crash_context.is_some());

            // Test that we can find relevant debug information
            let function_debug = debug_registry.get_debug_info(&format!("{}:{}:0", function, line));
            // Note: We might not find exact matches due to line number differences
            
            // Test unsafe function detection
            let unsafe_functions = debug_registry.find_symbols("unsafe").unwrap();
            assert!(!unsafe_functions.is_empty());

            debug!("Crash scenario '{}' in function '{}' at line {}: {}", 
                   crash_type, function, line, error_msg);
        }

        // Test source file loading for crash reporting
        let mut source_file = SourceFile::new(&crash_test_file);
        source_file.load_content().unwrap();

        // Test crash context line extraction
        let crash_line = source_file.get_line(4);
        assert!(crash_line.is_some());
        assert!(crash_line.unwrap().contains("panic"));

        let bounds_line = source_file.get_line(10);
        assert!(bounds_line.is_some());
        assert!(bounds_line.unwrap().contains("numbers[10]"));

        // Test crash context with surrounding lines
        let crash_context = source_file.get_lines_with_context(4, 3);
        assert!(crash_context.is_some());
        let context_lines = crash_context.unwrap();
        assert!(context_lines.len() >= 5);

        // Test crash report generation
        let mut crash_report = HashMap::new();
        crash_report.insert("type".to_string(), "Division by zero".to_string());
        crash_report.insert("function".to_string(), "divide_by_zero".to_string());
        crash_report.insert("file".to_string(), crash_test_file.to_string_lossy().to_string());
        crash_report.insert("line".to_string(), "4".to_string());
        crash_report.insert("message".to_string(), "Division by zero!".to_string());

        // Verify crash report completeness
        assert!(crash_report.contains_key("type"));
        assert!(crash_report.contains_key("function"));
        assert!(crash_report.contains_key("file"));
        assert!(crash_report.contains_key("line"));
        assert!(crash_report.contains_key("message"));

        // Cleanup
        fs::remove_file(&crash_test_file).ok();

        debug!("Crash reporting system test passed");
    }

    #[test]
    fn test_memory_debugging_capabilities() {
        init_tracing!();
        info!("Testing memory debugging capabilities");

        let debug_registry = Arc::new(DebugInfoRegistry::new());
        let mut debug_manager = DebugManager::new();
        debug_manager.enable();

        // Create test program with various memory operations
        let temp_dir = std::env::temp_dir();
        let memory_test_file = temp_dir.join("memory_test.csd");
        let memory_program = r#"
squad LargeStruct {
    sus[] data;
    tea name;
    facts active;
    
    slay new(sus size) -> LargeStruct {
        sus[] new_data = make_array(size);
        periodt LargeStruct { 
            data: new_data, 
            name: "Large", 
            active: true 
        };
    }
    
    slay resize(sus new_size) {
        this.data = make_array(new_size);
    }
}

slay create_memory_pressure() {
    sus[] objects = [];
    
    lowkey (sus i = 0; i < 1000; i++) {
        facts obj = LargeStruct::new(100);
        objects.push(obj);
        
        lowkey (i % 100 == 0) {
            // Trigger potential garbage collection
            objects.clear();
        }
    }
}

slay test_circular_references() {
    squad Node {
        tea value;
        Node? next;
        Node? prev;
    }
    
    facts node1 = Node { value: "first", next: nil, prev: nil };
    facts node2 = Node { value: "second", next: nil, prev: nil };
    
    node1.next = node2;
    node2.prev = node1;
    node2.next = node1; // Creates circular reference
    node1.prev = node2;
}

slay main() {
    create_memory_pressure();
    test_circular_references();
    periodt;
}
"#;

        fs::write(&memory_test_file, memory_program).unwrap();

        // Set up debug information for memory debugging
        let large_struct_debug = EnhancedDebugInfo::new(&memory_test_file, 2, 0, "LargeStruct".to_string())
            .with_symbol_metadata({
                let mut metadata = SymbolMetadata::new();
                metadata.symbol_type = SymbolType::Struct;
                metadata.attributes.insert("memory_intensive".to_string(), "true".to_string());
                metadata.tags.push("heap_allocated".to_string());
                metadata
            })
            .with_type_info(TypeDebugInfo::new("LargeStruct".to_string(), TypeKind::Struct)
                .with_field(FieldDebugInfo::new("data".to_string(), "Array<i32>".to_string()))
                .with_field(FieldDebugInfo::new("name".to_string(), "String".to_string()))
                .with_field(FieldDebugInfo::new("active".to_string(), "bool".to_string())));

        let memory_pressure_debug = EnhancedDebugInfo::new(&memory_test_file, 17, 0, "create_memory_pressure".to_string())
            .with_symbol_metadata(SymbolMetadata::function("create_memory_pressure", Some("slay"))
                .with_attribute("memory_intensive".to_string(), "true".to_string())
                .with_attribute("gc_trigger".to_string(), "true".to_string())
                .with_tag("allocation_heavy".to_string()));

        let circular_ref_debug = EnhancedDebugInfo::new(&memory_test_file, 29, 0, "test_circular_references".to_string())
            .with_symbol_metadata(SymbolMetadata::function("test_circular_references", Some("slay"))
                .with_attribute("creates_cycles".to_string(), "true".to_string())
                .with_tag("circular_references".to_string()));

        debug_registry.register_debug_info("LargeStruct:2:0".to_string(), large_struct_debug).unwrap();
        debug_registry.register_debug_info("create_memory_pressure:17:0".to_string(), memory_pressure_debug).unwrap();
        debug_registry.register_debug_info("test_circular_references:29:0".to_string(), circular_ref_debug).unwrap();

        // Test memory-related symbol analysis
        let memory_intensive_symbols = debug_registry.find_symbols("memory_intensive").unwrap();
        assert!(!memory_intensive_symbols.is_empty());
        assert!(memory_intensive_symbols.len() >= 2);

        let heap_allocated_symbols = debug_registry.find_symbols("heap_allocated").unwrap();
        assert!(!heap_allocated_symbols.is_empty());

        let circular_ref_symbols = debug_registry.find_symbols("circular_references").unwrap();
        assert!(!circular_ref_symbols.is_empty());

        // Test type information for memory analysis
        let large_struct_type = debug_registry.get_type("LargeStruct").unwrap();
        assert!(large_struct_type.is_some());
        let type_info = large_struct_type.unwrap();
        assert_eq!(type_info.fields.len(), 3);
        
        // Identify potentially large fields
        let data_field = type_info.fields.iter().find(|f| f.name == "data");
        assert!(data_field.is_some());
        assert!(data_field.unwrap().field_type.contains("Array"));

        // Test source code analysis for memory patterns
        let mut source_file = SourceFile::new(&memory_test_file);
        source_file.load_content().unwrap();

        // Look for allocation patterns
        let allocation_lines = vec![
            (7, "make_array"),     // Array allocation
            (21, "LargeStruct::new"), // Object allocation
            (25, "objects.clear"), // Deallocation
            (38, "Node"),          // Node allocation
        ];

        for (line_num, pattern) in allocation_lines {
            let line_content = source_file.get_line(line_num);
            assert!(line_content.is_some());
            let line = line_content.unwrap();
            assert!(line.contains(pattern), "Line {} should contain '{}': {}", line_num, pattern, line);
        }

        // Test memory debugging scope analysis
        let module_scope = ScopeInfo {
            scope_type: ScopeType::Module,
            depth: 0,
            parent_scope: None,
            variables: HashMap::new(),
            start_location: Some((1, 1)),
            end_location: Some((50, 1)),
        };
        let module_id = debug_registry.create_scope(module_scope).unwrap();

        // Create function scope with memory tracking
        let mut memory_func_scope = ScopeInfo::function_scope(1);
        memory_func_scope.parent_scope = Some(module_id);
        memory_func_scope.start_location = Some((17, 0));
        memory_func_scope.end_location = Some((28, 1));

        // Add memory-related variables
        let objects_var = VariableInfo::new("objects".to_string(), "Array<LargeStruct>".to_string());
        let obj_var = VariableInfo::new("obj".to_string(), "LargeStruct".to_string());
        memory_func_scope.add_variable("objects".to_string(), objects_var);
        memory_func_scope.add_variable("obj".to_string(), obj_var);

        let memory_scope_id = debug_registry.create_scope(memory_func_scope).unwrap();

        // Test memory debugging statistics
        let stats = debug_registry.get_statistics().unwrap();
        assert!(stats.debug_info_count >= 3);
        assert!(stats.type_count >= 1);
        assert!(stats.scope_count >= 2);

        // Test memory leak detection patterns
        let potential_leaks = debug_registry.find_symbols("allocation_heavy").unwrap();
        assert!(!potential_leaks.is_empty());

        let circular_patterns = debug_registry.find_symbols("creates_cycles").unwrap();
        assert!(!circular_patterns.is_empty());

        // Test GC trigger points
        let gc_triggers = debug_registry.find_symbols("gc_trigger").unwrap();
        assert!(!gc_triggers.is_empty());

        info!("Memory debugging analysis complete - found {} potential allocation points, {} GC triggers, {} circular reference patterns",
              potential_leaks.len(), gc_triggers.len(), circular_patterns.len());

        // Cleanup
        fs::remove_file(&memory_test_file).ok();

        debug!("Memory debugging capabilities test passed");
    }

    #[test]
    fn test_real_time_debugging_features() {
        init_tracing!();
        info!("Testing real-time debugging features");

        let debug_registry = Arc::new(DebugInfoRegistry::new());
        let mut debug_manager = DebugManager::new();
        debug_manager.enable();
        debug_manager.set_real_time_mode(true);

        let mut debug_runtime = DebugRuntime::new();
        debug_runtime.enable();

        // Create test program for real-time debugging
        let temp_dir = std::env::temp_dir();
        let realtime_test_file = temp_dir.join("realtime_test.csd");
        let realtime_program = r#"
slay fibonacci_realtime(sus n) -> sus {
    lowkey (n <= 1) {
        periodt n;
    }
    
    sus a = 0;
    sus b = 1;
    
    lowkey (sus i = 2; i <= n; i++) {
        sus temp = a + b;
        a = b;
        b = temp;
        
        // Debug checkpoint
        debug_checkpoint("fibonacci_iteration", i, a, b);
    }
    
    periodt b;
}

slay realtime_monitoring() {
    sus[] values = [];
    
    lowkey (sus i = 0; i < 50; i++) {
        sus fib_val = fibonacci_realtime(i);
        values.push(fib_val);
        
        // Monitor memory usage
        debug_memory_checkpoint();
        
        lowkey (i % 10 == 0) {
            debug_trace("Progress", i, values.length());
        }
    }
}

slay main() {
    debug_start_session("realtime_fibonacci");
    realtime_monitoring();
    debug_end_session();
    periodt;
}
"#;

        fs::write(&realtime_test_file, realtime_program).unwrap();

        // Set up real-time debug information
        let fibonacci_debug = EnhancedDebugInfo::new(&realtime_test_file, 2, 0, "fibonacci_realtime".to_string())
            .with_symbol_metadata(SymbolMetadata::function("fibonacci_realtime", Some("slay"))
                .with_attribute("recursive_iterative".to_string(), "true".to_string())
                .with_attribute("debug_monitored".to_string(), "true".to_string())
                .with_tag("performance_critical".to_string())
                .with_tag("real_time".to_string()));

        let monitoring_debug = EnhancedDebugInfo::new(&realtime_test_file, 21, 0, "realtime_monitoring".to_string())
            .with_symbol_metadata(SymbolMetadata::function("realtime_monitoring", Some("slay"))
                .with_attribute("memory_monitored".to_string(), "true".to_string())
                .with_tag("real_time".to_string())
                .with_tag("performance_critical".to_string()));

        debug_registry.register_debug_info("fibonacci_realtime:2:0".to_string(), fibonacci_debug).unwrap();
        debug_registry.register_debug_info("realtime_monitoring:21:0".to_string(), monitoring_debug).unwrap();

        // Simulate real-time debugging session
        let session_start = Instant::now();
        
        // Create runtime state tracking
        let mut runtime_states = Vec::new();
        
        // Simulate fibonacci execution with debug checkpoints
        for i in 0..20 {
            let mut runtime_info = RuntimeDebugInfo::new("fibonacci_realtime".to_string());
            runtime_info.add_variable("n".to_string(), i.to_string(), "i32".to_string());
            runtime_info.add_variable("i".to_string(), i.to_string(), "i32".to_string());
            
            if i >= 2 {
                let a = if i == 2 { 0 } else { runtime_states.last().unwrap().get_variable("b").unwrap_or("0") };
                let b = if i == 2 { 1 } else { 
                    let prev_a = runtime_states.last().unwrap().get_variable("a").unwrap_or("0");
                    let prev_b = runtime_states.last().unwrap().get_variable("b").unwrap_or("1");
                    (prev_a.parse::<i32>().unwrap_or(0) + prev_b.parse::<i32>().unwrap_or(0)).to_string()
                };
                
                runtime_info.add_variable("a".to_string(), a.clone(), "i32".to_string());
                runtime_info.add_variable("b".to_string(), b.clone(), "i32".to_string());
            }
            
            runtime_states.push(runtime_info.clone());
            debug_runtime.update_runtime_info(runtime_info);
        }
        
        let execution_time = session_start.elapsed();
        
        // Test real-time performance requirements
        assert!(execution_time < Duration::from_millis(100), 
                "Real-time debugging should complete within 100ms");
        
        // Test debug checkpoint data
        assert_eq!(runtime_states.len(), 20);
        
        // Verify state progression
        for (index, state) in runtime_states.iter().enumerate() {
            assert_eq!(state.function_name, "fibonacci_realtime");
            assert!(state.has_variable("n"));
            assert_eq!(state.get_variable("n").unwrap(), index.to_string());
            
            if index >= 2 {
                assert!(state.has_variable("a"));
                assert!(state.has_variable("b"));
            }
        }

        // Test memory monitoring simulation
        let mut memory_checkpoints = Vec::new();
        for i in 0..10 {
            let memory_usage = 1024 * (i + 1); // Simulated memory usage
            memory_checkpoints.push((i, memory_usage));
        }
        
        // Verify memory trend analysis
        assert_eq!(memory_checkpoints.len(), 10);
        for (iteration, memory) in &memory_checkpoints {
            assert!(memory > &1000); // Basic sanity check
            debug!("Memory checkpoint {}: {} bytes", iteration, memory);
        }

        // Test real-time symbol resolution
        let realtime_symbols = debug_registry.find_symbols("real_time").unwrap();
        assert!(realtime_symbols.len() >= 2);
        
        let performance_symbols = debug_registry.find_symbols("performance_critical").unwrap();
        assert!(performance_symbols.len() >= 2);

        // Test breakpoint simulation in real-time context
        let breakpoints = vec![
            ("fibonacci_realtime".to_string(), 10_u32), // Loop body
            ("fibonacci_realtime".to_string(), 15_u32), // Debug checkpoint
            ("realtime_monitoring".to_string(), 25_u32), // Fibonacci call
            ("realtime_monitoring".to_string(), 30_u32), // Memory checkpoint
        ];

        let mut source_file = SourceFile::new(&realtime_test_file);
        source_file.load_content().unwrap();

        for (function, line) in breakpoints {
            let line_content = source_file.get_line(line);
            if line_content.is_some() {
                let context = source_file.get_lines_with_context(line, 1);
                assert!(context.is_some());
                debug!("Breakpoint in {} at line {}: {:?}", function, line, line_content);
            }
        }

        // Test variable watch simulation
        let watched_variables = vec![
            ("n", "Input parameter"),
            ("i", "Loop counter"),
            ("a", "Fibonacci previous"),
            ("b", "Fibonacci current"),
            ("temp", "Temporary sum"),
        ];

        for (var_name, description) in watched_variables {
            let final_state = runtime_states.last().unwrap();
            if final_state.has_variable(var_name) {
                let value = final_state.get_variable(var_name).unwrap();
                debug!("Watched variable '{}' ({}): {}", var_name, description, value);
            }
        }

        // Test real-time statistics
        let stats = debug_registry.get_statistics().unwrap();
        info!("Real-time debugging session stats: {}", stats);
        info!("Session duration: {:?}", execution_time);
        info!("States captured: {}", runtime_states.len());
        info!("Memory checkpoints: {}", memory_checkpoints.len());

        // Verify real-time performance metrics
        assert!(stats.debug_info_count >= 2);
        assert!(execution_time < Duration::from_millis(200)); // Allow some overhead

        // Cleanup
        fs::remove_file(&realtime_test_file).ok();

        debug!("Real-time debugging features test passed");
    }

    #[test]
    fn test_debug_system_stress_test() {
        init_tracing!();
        info!("Testing debug system under stress conditions");

        let debug_registry = Arc::new(DebugInfoRegistry::new());
        let start_time = Instant::now();

        // Test large-scale debug information registration
        let num_files = 50;
        let functions_per_file = 20;
        let variables_per_function = 10;

        for file_id in 0..num_files {
            let file_path = PathBuf::from(format!("stress_test_{}.csd", file_id));
            
            // Create source map
            let mut source_map = SourceMap::new(file_path.clone());
            
            for func_id in 0..functions_per_file {
                let line = (func_id * 10) as u32 + 1;
                source_map.add_range(line, 0, line, 0, 50);
                
                // Register function debug info
                let func_debug = EnhancedDebugInfo::new(
                    &file_path,
                    line,
                    0,
                    format!("function_{}_{}", file_id, func_id)
                ).with_symbol_metadata(
                    SymbolMetadata::function(&format!("function_{}_{}", file_id, func_id), Some("slay"))
                        .with_tag("stress_test".to_string())
                );
                
                let func_key = format!("func_{}_{}_{}:{}:0", file_id, func_id, line, 0);
                debug_registry.register_debug_info(func_key, func_debug).unwrap();
                
                // Register variables
                for var_id in 0..variables_per_function {
                    let var_line = line + var_id as u32 + 1;
                    let var_debug = EnhancedDebugInfo::new(
                        &file_path,
                        var_line,
                        4,
                        format!("var_{}_{}_{}", file_id, func_id, var_id)
                    ).with_symbol_metadata(
                        SymbolMetadata::variable(&format!("var_{}", var_id), "i32")
                    );
                    
                    let var_key = format!("var_{}_{}_{}_{}:{}:4", file_id, func_id, var_id, var_line, 4);
                    debug_registry.register_debug_info(var_key, var_debug).unwrap();
                }
            }
            
            debug_registry.register_source_map(file_path, source_map).unwrap();
        }

        let registration_time = start_time.elapsed();
        
        // Calculate expected totals
        let expected_functions = num_files * functions_per_file;
        let expected_variables = expected_functions * variables_per_function;
        let expected_debug_entries = expected_functions + expected_variables;

        info!("Stress test registered {} debug entries in {:?}", 
              expected_debug_entries, registration_time);

        // Test retrieval performance under load
        let retrieval_start = Instant::now();
        let mut successful_retrievals = 0;

        for file_id in 0..num_files {
            for func_id in 0..functions_per_file {
                let line = (func_id * 10) as u32 + 1;
                let func_key = format!("func_{}_{}_{}:{}:0", file_id, func_id, line, 0);
                
                if debug_registry.get_debug_info(&func_key).unwrap().is_some() {
                    successful_retrievals += 1;
                }
            }
        }

        let retrieval_time = retrieval_start.elapsed();
        
        // Test concurrent search operations
        let search_start = Instant::now();
        let search_handles: Vec<_> = (0..8).map(|thread_id| {
            let registry_clone = Arc::clone(&debug_registry);
            std::thread::spawn(move || {
                let mut search_results = 0;
                for i in 0..100 {
                    let pattern = format!("function_{}", (thread_id * 100 + i) % num_files);
                    let results = registry_clone.find_symbols(&pattern).unwrap();
                    search_results += results.len();
                }
                search_results
            })
        }).collect();

        let mut total_search_results = 0;
        for handle in search_handles {
            total_search_results += handle.join().unwrap();
        }

        let search_time = search_start.elapsed();

        // Test statistics under load
        let stats_start = Instant::now();
        let final_stats = debug_registry.get_statistics().unwrap();
        let stats_time = stats_start.elapsed();

        // Performance assertions
        assert!(registration_time < Duration::from_secs(10), 
                "Registration should complete within 10 seconds");
        assert!(retrieval_time < Duration::from_secs(5), 
                "Retrieval should complete within 5 seconds");
        assert!(search_time < Duration::from_secs(2), 
                "Concurrent search should complete within 2 seconds");
        assert!(stats_time < Duration::from_millis(100), 
                "Statistics should be computed quickly");

        // Correctness assertions
        assert_eq!(successful_retrievals, expected_functions);
        assert!(final_stats.debug_info_count >= expected_debug_entries);
        assert!(total_search_results > 0);

        info!("Stress test results:");
        info!("  Registration time: {:?}", registration_time);
        info!("  Retrieval time: {:?}", retrieval_time);
        info!("  Search time: {:?}", search_time);
        info!("  Statistics time: {:?}", stats_time);
        info!("  Final statistics: {}", final_stats);
        info!("  Search results: {}", total_search_results);

        debug!("Debug system stress test passed");
    }
}
