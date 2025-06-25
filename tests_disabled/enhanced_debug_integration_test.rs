/// Enhanced Debug Integration Tests
/// 
/// Comprehensive integration tests for the CURSED debugging system including
/// end-to-end workflows, LLVM integration, cross-platform compatibility,
/// and advanced debugging features.

use cursed::debug::{
    EnhancedDebugInfo, DebugInfoRegistry, SymbolMetadata, SymbolType, SymbolVisibility,
    SourceMap, TypeDebugInfo, TypeKind, FieldDebugInfo, ScopeInfo, ScopeType, DebugConfig
};
use cursed::runtime::debug_info::{DebugInfo, VariableInfo, EnhancedStackFrame, SymbolInfo};
use cursed::runtime::debug_manager::{DebugManager, SourceFile};
use cursed::runtime::debug_runtime::{DebugRuntime, RuntimeDebugInfo};
use cursed::error::{Error as CursedError, SourceLocation as ErrorSourceLocation};
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

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
    fn test_end_to_end_debug_workflow() {
        init_tracing!();
        info!("Testing end-to-end debug workflow");

        let registry = Arc::new(DebugInfoRegistry::new());
        let mut debug_manager = DebugManager::new();
        debug_manager.enable();

        // Step 1: Register source files
        let source_path = PathBuf::from("test_program.csd");
        let source_content = r#"
slay main() {
    sus x = 42;
    facts is_valid = true;
    tea message = "Hello, World!";
    periodt;
}
"#;
        
        // Create temporary source file
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_program.csd");
        std::fs::write(&test_file, source_content).unwrap();

        let mut source_file = SourceFile::new(&test_file);
        source_file.load_content().unwrap();

        // Step 2: Create enhanced debug information
        let main_debug = EnhancedDebugInfo::new(&test_file, 2, 1, "main".to_string())
            .with_symbol_metadata(SymbolMetadata::function("main", Some("slay")))
            .with_scope_info(ScopeInfo::function_scope(0));

        let x_debug = EnhancedDebugInfo::new(&test_file, 3, 5, "x".to_string())
            .with_symbol_metadata(SymbolMetadata::variable("x", "i32"));

        let is_valid_debug = EnhancedDebugInfo::new(&test_file, 4, 5, "is_valid".to_string())
            .with_symbol_metadata(SymbolMetadata::variable("is_valid", "bool"));

        let message_debug = EnhancedDebugInfo::new(&test_file, 5, 5, "message".to_string())
            .with_symbol_metadata(SymbolMetadata::variable("message", "String"));

        // Step 3: Register debug information
        registry.register_debug_info("main:2:1".to_string(), main_debug).unwrap();
        registry.register_debug_info("x:3:5".to_string(), x_debug).unwrap();
        registry.register_debug_info("is_valid:4:5".to_string(), is_valid_debug).unwrap();
        registry.register_debug_info("message:5:5".to_string(), message_debug).unwrap();

        // Step 4: Create source map
        let mut source_map = SourceMap::new(test_file.clone());
        source_map.add_range(2, 1, 2, 1, 10); // main function
        source_map.add_range(3, 5, 3, 5, 8);  // x variable
        source_map.add_range(4, 5, 4, 5, 15); // is_valid variable
        source_map.add_range(5, 5, 5, 5, 25); // message variable

        registry.register_source_map(test_file.clone(), source_map).unwrap();

        // Step 5: Test symbol resolution
        let symbols = registry.find_symbols("main").unwrap();
        assert!(!symbols.is_empty());
        assert!(symbols.iter().any(|(name, _)| name.contains("main")));

        // Step 6: Test type information
        let struct_type = TypeDebugInfo::new("TestStruct".to_string(), TypeKind::Struct)
            .with_field(FieldDebugInfo::new("x".to_string(), "i32".to_string()))
            .with_field(FieldDebugInfo::new("is_valid".to_string(), "bool".to_string()));

        registry.register_type("TestStruct".to_string(), struct_type).unwrap();

        // Step 7: Verify end-to-end retrieval
        let retrieved_main = registry.get_debug_info("main:2:1").unwrap();
        assert!(retrieved_main.is_some());
        let main_info = retrieved_main.unwrap();
        assert_eq!(main_info.debug_info.function_name, "main");
        assert!(main_info.is_user_code());

        let retrieved_type = registry.get_type("TestStruct").unwrap();
        assert!(retrieved_type.is_some());
        let type_info = retrieved_type.unwrap();
        assert_eq!(type_info.fields.len(), 2);

        // Step 8: Test statistics
        let stats = registry.get_statistics().unwrap();
        assert!(stats.debug_info_count >= 4);
        assert!(stats.symbol_count >= 0);
        assert!(stats.type_count >= 1);

        // Cleanup
        std::fs::remove_file(&test_file).ok();

        debug!("End-to-end debug workflow test passed");
    }

    #[test]
    fn test_complex_source_mapping() {
        init_tracing!();
        info!("Testing complex source mapping scenarios");

        let mut source_map = SourceMap::new(PathBuf::from("complex.csd"));
        
        // Add overlapping and adjacent ranges
        source_map.add_range(10, 0, 8, 2, 20);   // Line 10, cols 0-19 -> Line 8, cols 2-21
        source_map.add_range(10, 20, 8, 22, 15); // Line 10, cols 20-34 -> Line 8, cols 22-36
        source_map.add_range(11, 5, 9, 0, 30);   // Line 11, cols 5-34 -> Line 9, cols 0-29
        source_map.add_range(12, 0, 10, 5, 25);  // Line 12, cols 0-24 -> Line 10, cols 5-29
        
        // Test precise mapping within ranges
        assert_eq!(source_map.map_to_original(10, 0), Some((8, 2)));   // Start of first range
        assert_eq!(source_map.map_to_original(10, 10), Some((8, 12))); // Middle of first range
        assert_eq!(source_map.map_to_original(10, 19), Some((8, 21))); // End of first range
        assert_eq!(source_map.map_to_original(10, 20), Some((8, 22))); // Start of second range
        assert_eq!(source_map.map_to_original(10, 30), Some((8, 32))); // Middle of second range
        
        // Test range boundaries
        assert_eq!(source_map.map_to_original(11, 5), Some((9, 0)));   // Start of third range
        assert_eq!(source_map.map_to_original(11, 34), Some((9, 29))); // End of third range
        
        // Test outside ranges
        assert_eq!(source_map.map_to_original(10, 35), None); // After second range
        assert_eq!(source_map.map_to_original(11, 4), None);  // Before third range
        assert_eq!(source_map.map_to_original(13, 0), None);  // Different line

        debug!("Complex source mapping test passed");
    }

    #[test]
    fn test_advanced_symbol_metadata() {
        init_tracing!();
        info!("Testing advanced symbol metadata features");

        let registry = DebugInfoRegistry::new();
        
        // Create complex function metadata with Gen Z annotations
        let advanced_func = SymbolMetadata::function("compute_fibonacci", Some("slay"))
            .with_attribute("async".to_string(), "true".to_string())
            .with_attribute("visibility".to_string(), "public".to_string())
            .with_attribute("parameters".to_string(), "n: sus".to_string())
            .with_attribute("return_type".to_string(), "sus".to_string())
            .with_tag("mathematical".to_string())
            .with_tag("recursive".to_string())
            .with_tag("pure_function".to_string());

        registry.register_symbol("math::compute_fibonacci".to_string(), advanced_func).unwrap();

        // Create interface metadata
        let mut interface_metadata = SymbolMetadata::new();
        interface_metadata.symbol_type = SymbolType::Interface;
        interface_metadata.visibility = SymbolVisibility::Public;
        interface_metadata.attributes.insert("methods".to_string(), "calculate,validate".to_string());
        interface_metadata.tags.push("contract".to_string());

        registry.register_symbol("Calculator".to_string(), interface_metadata).unwrap();

        // Create struct metadata with field information
        let struct_metadata = SymbolMetadata::new()
            .with_attribute("fields".to_string(), "x,y,z".to_string())
            .with_attribute("size".to_string(), "24".to_string())
            .with_tag("geometric".to_string());

        registry.register_symbol("Point3D".to_string(), struct_metadata).unwrap();

        // Test retrieval and validation
        let func_symbol = registry.get_symbol("math::compute_fibonacci").unwrap().unwrap();
        assert_eq!(func_symbol.symbol_type, SymbolType::Function);
        assert_eq!(func_symbol.attributes.get("async"), Some(&"true".to_string()));
        assert!(func_symbol.tags.contains(&"mathematical".to_string()));
        assert!(func_symbol.tags.contains(&"recursive".to_string()));

        let interface_symbol = registry.get_symbol("Calculator").unwrap().unwrap();
        assert_eq!(interface_symbol.symbol_type, SymbolType::Interface);
        assert_eq!(interface_symbol.visibility, SymbolVisibility::Public);
        assert!(interface_symbol.tags.contains(&"contract".to_string()));

        // Test pattern-based symbol search
        let math_symbols = registry.find_symbols("math").unwrap();
        assert_eq!(math_symbols.len(), 1);
        assert!(math_symbols[0].0.contains("fibonacci"));

        let recursive_symbols = registry.find_symbols("recursive").unwrap();
        assert!(!recursive_symbols.is_empty());

        debug!("Advanced symbol metadata test passed");
    }

    #[test]
    fn test_complex_type_hierarchies() {
        init_tracing!();
        info!("Testing complex type hierarchies");

        let registry = DebugInfoRegistry::new();
        
        // Create base interface
        let drawable_interface = TypeDebugInfo::new("Drawable".to_string(), TypeKind::Interface);
        registry.register_type("Drawable".to_string(), drawable_interface).unwrap();

        // Create struct implementing interface
        let circle_struct = TypeDebugInfo::new("Circle".to_string(), TypeKind::Struct)
            .with_field(FieldDebugInfo::new("radius".to_string(), "f64".to_string()))
            .with_field(FieldDebugInfo::new("center_x".to_string(), "f64".to_string()))
            .with_field(FieldDebugInfo::new("center_y".to_string(), "f64".to_string()));

        registry.register_type("Circle".to_string(), circle_struct).unwrap();

        // Create generic type
        let vector_generic = TypeDebugInfo::new("Vector".to_string(), TypeKind::Generic)
            .with_type_parameter("T".to_string())
            .with_field(FieldDebugInfo::new("data".to_string(), "Array<T>".to_string()))
            .with_field(FieldDebugInfo::new("length".to_string(), "usize".to_string()))
            .with_field(FieldDebugInfo::new("capacity".to_string(), "usize".to_string()));

        registry.register_type("Vector<T>".to_string(), vector_generic).unwrap();

        // Create specialized generic type
        let int_vector = TypeDebugInfo::new("Vector<i32>".to_string(), TypeKind::Array)
            .with_field(FieldDebugInfo::new("data".to_string(), "Array<i32>".to_string()))
            .with_field(FieldDebugInfo::new("length".to_string(), "usize".to_string()))
            .with_field(FieldDebugInfo::new("capacity".to_string(), "usize".to_string()));

        registry.register_type("Vector<i32>".to_string(), int_vector).unwrap();

        // Create function type
        let function_type = TypeDebugInfo::new("FunctionPointer".to_string(), TypeKind::Function)
            .with_field(FieldDebugInfo::new("parameters".to_string(), "(i32, bool)".to_string()))
            .with_field(FieldDebugInfo::new("return_type".to_string(), "String".to_string()));

        registry.register_type("FunctionPointer".to_string(), function_type).unwrap();

        // Verify type hierarchy
        let drawable = registry.get_type("Drawable").unwrap().unwrap();
        assert_eq!(drawable.type_kind, TypeKind::Interface);

        let circle = registry.get_type("Circle").unwrap().unwrap();
        assert_eq!(circle.type_kind, TypeKind::Struct);
        assert_eq!(circle.fields.len(), 3);
        assert_eq!(circle.fields[0].name, "radius");

        let vector_generic = registry.get_type("Vector<T>").unwrap().unwrap();
        assert_eq!(vector_generic.type_kind, TypeKind::Generic);
        assert_eq!(vector_generic.type_parameters.len(), 1);
        assert_eq!(vector_generic.type_parameters[0], "T");

        let int_vector = registry.get_type("Vector<i32>").unwrap().unwrap();
        assert_eq!(int_vector.type_kind, TypeKind::Array);
        assert_eq!(int_vector.fields[0].field_type, "Array<i32>");

        let func_type = registry.get_type("FunctionPointer").unwrap().unwrap();
        assert_eq!(func_type.type_kind, TypeKind::Function);

        debug!("Complex type hierarchies test passed");
    }

    #[test]
    fn test_nested_scope_management() {
        init_tracing!();
        info!("Testing nested scope management");

        let registry = DebugInfoRegistry::new();
        
        // Create module scope (depth 0)
        let module_scope = ScopeInfo {
            scope_type: ScopeType::Module,
            depth: 0,
            parent_scope: None,
            variables: HashMap::new(),
            start_location: Some((1, 1)),
            end_location: Some((100, 1)),
        };
        let module_id = registry.create_scope(module_scope).unwrap();
        
        // Create function scope (depth 1)
        let mut function_scope = ScopeInfo::function_scope(1);
        function_scope.parent_scope = Some(module_id);
        function_scope.start_location = Some((10, 1));
        function_scope.end_location = Some((50, 1));
        
        let param_var = VariableInfo::new("param".to_string(), "i32".to_string());
        function_scope.add_variable("param".to_string(), param_var);
        
        let function_id = registry.create_scope(function_scope).unwrap();
        
        // Create block scope (depth 2)
        let mut block_scope = ScopeInfo {
            scope_type: ScopeType::Block,
            depth: 2,
            parent_scope: Some(function_id),
            variables: HashMap::new(),
            start_location: Some((20, 5)),
            end_location: Some((40, 5)),
        };
        
        let local_var = VariableInfo::new("local".to_string(), "String".to_string());
        block_scope.add_variable("local".to_string(), local_var);
        
        let block_id = registry.create_scope(block_scope).unwrap();
        
        // Create loop scope (depth 3)
        let mut loop_scope = ScopeInfo {
            scope_type: ScopeType::Loop,
            depth: 3,
            parent_scope: Some(block_id),
            variables: HashMap::new(),
            start_location: Some((25, 9)),
            end_location: Some((35, 9)),
        };
        
        let loop_var = VariableInfo::new("i".to_string(), "i32".to_string());
        loop_scope.add_variable("i".to_string(), loop_var);
        
        let loop_id = registry.create_scope(loop_scope).unwrap();
        
        // Verify scope hierarchy
        let retrieved_module = registry.get_scope(module_id).unwrap().unwrap();
        assert_eq!(retrieved_module.scope_type, ScopeType::Module);
        assert_eq!(retrieved_module.depth, 0);
        assert_eq!(retrieved_module.parent_scope, None);
        
        let retrieved_function = registry.get_scope(function_id).unwrap().unwrap();
        assert_eq!(retrieved_function.scope_type, ScopeType::Function);
        assert_eq!(retrieved_function.depth, 1);
        assert_eq!(retrieved_function.parent_scope, Some(module_id));
        assert!(retrieved_function.has_variable("param"));
        
        let retrieved_block = registry.get_scope(block_id).unwrap().unwrap();
        assert_eq!(retrieved_block.scope_type, ScopeType::Block);
        assert_eq!(retrieved_block.depth, 2);
        assert_eq!(retrieved_block.parent_scope, Some(function_id));
        assert!(retrieved_block.has_variable("local"));
        
        let retrieved_loop = registry.get_scope(loop_id).unwrap().unwrap();
        assert_eq!(retrieved_loop.scope_type, ScopeType::Loop);
        assert_eq!(retrieved_loop.depth, 3);
        assert_eq!(retrieved_loop.parent_scope, Some(block_id));
        assert!(retrieved_loop.has_variable("i"));

        debug!("Nested scope management test passed");
    }

    #[test]
    fn test_debug_performance_characteristics() {
        init_tracing!();
        info!("Testing debug performance characteristics");

        let registry = Arc::new(DebugInfoRegistry::new());
        let start_time = Instant::now();
        
        // Test large-scale debug information registration
        let num_entries = 1000;
        for i in 0..num_entries {
            let debug_info = EnhancedDebugInfo::new(
                format!("file_{}.csd", i / 100),
                (i % 1000) as u32 + 1,
                (i % 80) as u32 + 1,
                format!("function_{}", i)
            );
            
            let location_key = format!("file_{}:{}:{}", i / 100, i % 1000 + 1, i % 80 + 1);
            registry.register_debug_info(location_key, debug_info).unwrap();
            
            // Add symbol metadata
            let symbol_metadata = SymbolMetadata::function(&format!("function_{}", i), Some("slay"));
            registry.register_symbol(format!("module::function_{}", i), symbol_metadata).unwrap();
        }
        
        let registration_time = start_time.elapsed();
        info!("Registered {} debug entries in {:?}", num_entries, registration_time);
        
        // Test retrieval performance
        let retrieval_start = Instant::now();
        let mut retrieved_count = 0;
        
        for i in 0..num_entries {
            let location_key = format!("file_{}:{}:{}", i / 100, i % 1000 + 1, i % 80 + 1);
            if registry.get_debug_info(&location_key).unwrap().is_some() {
                retrieved_count += 1;
            }
        }
        
        let retrieval_time = retrieval_start.elapsed();
        info!("Retrieved {} debug entries in {:?}", retrieved_count, retrieval_time);
        
        // Test search performance
        let search_start = Instant::now();
        let search_results = registry.find_symbols("function_1").unwrap();
        let search_time = search_start.elapsed();
        info!("Found {} symbols matching pattern in {:?}", search_results.len(), search_time);
        
        // Performance assertions
        assert!(registration_time < Duration::from_secs(5), "Registration should complete within 5 seconds");
        assert!(retrieval_time < Duration::from_secs(2), "Retrieval should complete within 2 seconds");
        assert!(search_time < Duration::from_millis(100), "Search should complete within 100ms");
        assert_eq!(retrieved_count, num_entries);
        assert!(!search_results.is_empty());
        
        // Test statistics performance
        let stats_start = Instant::now();
        let stats = registry.get_statistics().unwrap();
        let stats_time = stats_start.elapsed();
        info!("Retrieved statistics in {:?}: {}", stats_time, stats);
        
        assert!(stats_time < Duration::from_millis(50), "Statistics retrieval should be fast");
        assert!(stats.debug_info_count >= num_entries);
        assert!(stats.symbol_count >= num_entries);

        debug!("Debug performance characteristics test passed");
    }

    #[test]
    fn test_concurrent_debug_operations() {
        init_tracing!();
        info!("Testing concurrent debug operations");

        let registry = Arc::new(DebugInfoRegistry::new());
        let num_threads = 4;
        let entries_per_thread = 100;
        
        let mut handles = Vec::new();
        
        // Spawn threads to perform concurrent operations
        for thread_id in 0..num_threads {
            let registry_clone = Arc::clone(&registry);
            
            let handle = std::thread::spawn(move || {
                // Each thread adds debug info
                for i in 0..entries_per_thread {
                    let debug_info = EnhancedDebugInfo::new(
                        format!("thread_{}_file.csd", thread_id),
                        (i + 1) as u32,
                        1,
                        format!("thread_{}_function_{}", thread_id, i)
                    );
                    
                    let location_key = format!("thread_{}:{}:1", thread_id, i + 1);
                    registry_clone.register_debug_info(location_key, debug_info).unwrap();
                    
                    // Add symbols
                    let symbol_metadata = SymbolMetadata::function(&format!("func_{}", i), Some("slay"));
                    let symbol_key = format!("thread_{}::func_{}", thread_id, i);
                    registry_clone.register_symbol(symbol_key, symbol_metadata).unwrap();
                    
                    // Perform some reads
                    if i % 10 == 0 {
                        let search_results = registry_clone.find_symbols(&format!("thread_{}", thread_id)).unwrap();
                        assert!(!search_results.is_empty());
                    }
                }
                
                // Return thread statistics
                (thread_id, entries_per_thread)
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        let mut total_entries = 0;
        for handle in handles {
            let (thread_id, entries) = handle.join().unwrap();
            total_entries += entries;
            debug!("Thread {} completed with {} entries", thread_id, entries);
        }
        
        // Verify final state
        let final_stats = registry.get_statistics().unwrap();
        assert!(final_stats.debug_info_count >= total_entries);
        assert!(final_stats.symbol_count >= total_entries);
        
        // Test concurrent reads
        let read_handles: Vec<_> = (0..num_threads).map(|thread_id| {
            let registry_clone = Arc::clone(&registry);
            
            std::thread::spawn(move || {
                let mut read_count = 0;
                for i in 0..entries_per_thread {
                    let location_key = format!("thread_{}:{}:1", thread_id, i + 1);
                    if registry_clone.get_debug_info(&location_key).unwrap().is_some() {
                        read_count += 1;
                    }
                }
                read_count
            })
        }).collect();
        
        let mut total_reads = 0;
        for handle in read_handles {
            total_reads += handle.join().unwrap();
        }
        
        assert_eq!(total_reads, total_entries);
        info!("Concurrent operations completed successfully: {} total entries, {} total reads", total_entries, total_reads);

        debug!("Concurrent debug operations test passed");
    }

    #[test]
    fn test_debug_source_file_integration() {
        init_tracing!();
        info!("Testing debug source file integration");

        // Create a complex test file
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("complex_debug_test.csd");
        let complex_content = r#"
// Main module
slay main() {
    sus count = 0;
    lowkey (sus i = 0; i < 10; i++) {
        count = count + i;
        lowkey (count > 5) {
            facts found = true;
            tea message = "Found value: " + count.to_string();
            print(message);
        }
    }
    periodt;
}

squad Point {
    sus x;
    sus y;
    
    slay new(sus x_val, sus y_val) -> Point {
        periodt Point { x: x_val, y: y_val };
    }
    
    slay distance_from_origin() -> f64 {
        periodt sqrt((this.x * this.x) + (this.y * this.y));
    }
}

collab Drawable {
    slay draw();
    slay area() -> f64;
}
"#;

        std::fs::write(&test_file, complex_content).unwrap();
        
        let mut source_file = SourceFile::new(&test_file);
        let load_result = source_file.load_content();
        assert!(load_result.is_ok());
        
        // Test line retrieval for various parts
        let main_line = source_file.get_line(3);
        assert!(main_line.is_some());
        assert!(main_line.unwrap().contains("slay main"));
        
        let loop_line = source_file.get_line(5);
        assert!(loop_line.is_some());
        assert!(loop_line.unwrap().contains("lowkey"));
        
        let struct_line = source_file.get_line(15);
        assert!(struct_line.is_some());
        assert!(struct_line.unwrap().contains("squad Point"));
        
        let interface_line = source_file.get_line(27);
        assert!(interface_line.is_some());
        assert!(interface_line.unwrap().contains("collab Drawable"));
        
        // Test context retrieval
        let main_context = source_file.get_lines_with_context(3, 2);
        assert!(main_context.is_some());
        let context_lines = main_context.unwrap();
        assert!(context_lines.len() >= 3);
        
        // Find the main function line
        let main_found = context_lines.iter().any(|(_, line)| line.contains("slay main"));
        assert!(main_found);
        
        // Test struct context
        let struct_context = source_file.get_lines_with_context(15, 3);
        assert!(struct_context.is_some());
        let struct_lines = struct_context.unwrap();
        let struct_found = struct_lines.iter().any(|(_, line)| line.contains("squad Point"));
        assert!(struct_found);
        
        // Create debug information for the source file
        let registry = DebugInfoRegistry::new();
        
        // Create source map for the file
        let mut source_map = SourceMap::new(test_file.clone());
        source_map.add_range(3, 0, 3, 0, 11);   // main function
        source_map.add_range(4, 4, 4, 4, 15);   // count variable
        source_map.add_range(5, 4, 5, 4, 35);   // for loop
        source_map.add_range(15, 0, 15, 0, 12); // Point struct
        source_map.add_range(27, 0, 27, 0, 17); // Drawable interface
        
        registry.register_source_map(test_file.clone(), source_map).unwrap();
        
        // Create debug info for key elements
        let main_debug = EnhancedDebugInfo::new(&test_file, 3, 0, "main".to_string())
            .with_symbol_metadata(SymbolMetadata::function("main", Some("slay")));
        
        let point_debug = EnhancedDebugInfo::new(&test_file, 15, 0, "Point".to_string())
            .with_symbol_metadata({
                let mut metadata = SymbolMetadata::new();
                metadata.symbol_type = SymbolType::Struct;
                metadata.visibility = SymbolVisibility::Public;
                metadata
            });
        
        registry.register_debug_info("main:3:0".to_string(), main_debug).unwrap();
        registry.register_debug_info("Point:15:0".to_string(), point_debug).unwrap();
        
        // Test source map integration
        let source_map_retrieved = registry.get_source_map(&test_file).unwrap();
        assert!(source_map_retrieved.is_some());
        
        let sm = source_map_retrieved.unwrap();
        let mapped_main = sm.map_to_original(3, 5);
        assert_eq!(mapped_main, Some((3, 5)));
        
        // Cleanup
        std::fs::remove_file(&test_file).ok();

        debug!("Debug source file integration test passed");
    }

    #[test]
    fn test_cross_platform_compatibility() {
        init_tracing!();
        info!("Testing cross-platform compatibility");

        let registry = DebugInfoRegistry::new();
        
        // Test with different path separators and formats
        let windows_path = PathBuf::from(r"C:\Users\test\project\main.csd");
        let unix_path = PathBuf::from("/home/user/project/main.csd");
        let relative_path = PathBuf::from("./src/main.csd");
        
        // Create debug info for different path formats
        let windows_debug = EnhancedDebugInfo::new(&windows_path, 10, 5, "windows_func".to_string());
        let unix_debug = EnhancedDebugInfo::new(&unix_path, 20, 10, "unix_func".to_string());
        let relative_debug = EnhancedDebugInfo::new(&relative_path, 30, 15, "relative_func".to_string());
        
        // Register debug information
        registry.register_debug_info("windows:10:5".to_string(), windows_debug).unwrap();
        registry.register_debug_info("unix:20:10".to_string(), unix_debug).unwrap();
        registry.register_debug_info("relative:30:15".to_string(), relative_debug).unwrap();
        
        // Create source maps for different platforms
        let mut windows_map = SourceMap::new(windows_path.clone());
        windows_map.add_range(10, 5, 10, 5, 20);
        registry.register_source_map(windows_path, windows_map).unwrap();
        
        let mut unix_map = SourceMap::new(unix_path.clone());
        unix_map.add_range(20, 10, 20, 10, 25);
        registry.register_source_map(unix_path, unix_map).unwrap();
        
        let mut relative_map = SourceMap::new(relative_path.clone());
        relative_map.add_range(30, 15, 30, 15, 30);
        registry.register_source_map(relative_path, relative_map).unwrap();
        
        // Test retrieval across platforms
        let windows_retrieved = registry.get_debug_info("windows:10:5").unwrap();
        assert!(windows_retrieved.is_some());
        assert_eq!(windows_retrieved.unwrap().debug_info.function_name, "windows_func");
        
        let unix_retrieved = registry.get_debug_info("unix:20:10").unwrap();
        assert!(unix_retrieved.is_some());
        assert_eq!(unix_retrieved.unwrap().debug_info.function_name, "unix_func");
        
        let relative_retrieved = registry.get_debug_info("relative:30:15").unwrap();
        assert!(relative_retrieved.is_some());
        assert_eq!(relative_retrieved.unwrap().debug_info.function_name, "relative_func");
        
        // Test statistics across platforms
        let stats = registry.get_statistics().unwrap();
        assert!(stats.debug_info_count >= 3);

        debug!("Cross-platform compatibility test passed");
    }

    #[test]
    fn test_error_recovery_and_resilience() {
        init_tracing!();
        info!("Testing error recovery and resilience");

        let registry = DebugInfoRegistry::new();
        
        // Test graceful handling of invalid data
        let result = registry.get_debug_info("nonexistent:key");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        
        let symbol_result = registry.get_symbol("nonexistent::symbol");
        assert!(symbol_result.is_ok());
        assert!(symbol_result.unwrap().is_none());
        
        let type_result = registry.get_type("NonexistentType");
        assert!(type_result.is_ok());
        assert!(type_result.unwrap().is_none());
        
        let scope_result = registry.get_scope(99999);
        assert!(scope_result.is_ok());
        assert!(scope_result.unwrap().is_none());
        
        // Test search with empty results
        let empty_search = registry.find_symbols("this_pattern_matches_nothing_at_all");
        assert!(empty_search.is_ok());
        assert!(empty_search.unwrap().is_empty());
        
        // Test with malformed but valid data
        let malformed_debug = EnhancedDebugInfo::new("", 0, 0, "".to_string());
        let register_result = registry.register_debug_info("malformed:0:0".to_string(), malformed_debug);
        assert!(register_result.is_ok());
        
        // Test statistics with partial data
        let stats = registry.get_statistics();
        assert!(stats.is_ok());
        let stats_value = stats.unwrap();
        assert!(stats_value.debug_info_count >= 1);
        
        // Test recovery from concurrent access
        let registry_arc = Arc::new(DebugInfoRegistry::new());
        let handles: Vec<_> = (0..10).map(|i| {
            let registry_clone = Arc::clone(&registry_arc);
            std::thread::spawn(move || {
                // Each thread tries to access the same non-existent key
                let result = registry_clone.get_debug_info("concurrent:test:key");
                assert!(result.is_ok());
                assert!(result.unwrap().is_none());
                i
            })
        }).collect();
        
        // All threads should complete successfully
        for handle in handles {
            let thread_id = handle.join().unwrap();
            debug!("Thread {} completed successfully", thread_id);
        }

        debug!("Error recovery and resilience test passed");
    }

    #[test]
    fn test_memory_efficiency() {
        init_tracing!();
        info!("Testing memory efficiency");

        let registry = Arc::new(DebugInfoRegistry::new());
        
        // Get initial statistics
        let initial_stats = registry.get_statistics().unwrap();
        
        // Add a moderate amount of debug information
        let num_entries = 500;
        for i in 0..num_entries {
            let debug_info = EnhancedDebugInfo::new(
                format!("file_{}.csd", i % 10), // Reuse filenames
                (i % 100) as u32 + 1,
                (i % 50) as u32 + 1,
                format!("func_{}", i)
            );
            
            let location_key = format!("entry_{}:{}:{}", i, i % 100 + 1, i % 50 + 1);
            registry.register_debug_info(location_key, debug_info).unwrap();
            
            // Add some symbols (fewer than debug info entries)
            if i % 5 == 0 {
                let symbol_metadata = SymbolMetadata::function(&format!("func_{}", i), Some("slay"));
                registry.register_symbol(format!("mod::func_{}", i), symbol_metadata).unwrap();
            }
            
            // Add some types (even fewer)
            if i % 20 == 0 {
                let type_info = TypeDebugInfo::new(format!("Type_{}", i), TypeKind::Struct);
                registry.register_type(format!("Type_{}", i), type_info).unwrap();
            }
        }
        
        // Get final statistics
        let final_stats = registry.get_statistics().unwrap();
        
        // Verify expected counts
        assert_eq!(final_stats.debug_info_count - initial_stats.debug_info_count, num_entries);
        assert_eq!(final_stats.symbol_count - initial_stats.symbol_count, num_entries / 5);
        assert_eq!(final_stats.type_count - initial_stats.type_count, num_entries / 20);
        
        // Test that we can still retrieve data efficiently
        let start_time = Instant::now();
        let mut found_count = 0;
        
        for i in 0..num_entries {
            let location_key = format!("entry_{}:{}:{}", i, i % 100 + 1, i % 50 + 1);
            if registry.get_debug_info(&location_key).unwrap().is_some() {
                found_count += 1;
            }
        }
        
        let retrieval_time = start_time.elapsed();
        assert_eq!(found_count, num_entries);
        assert!(retrieval_time < Duration::from_millis(500), "Retrieval should remain fast with {} entries", num_entries);
        
        info!("Memory efficiency test: {} entries processed in {:?}", num_entries, retrieval_time);

        debug!("Memory efficiency test passed");
    }
}
