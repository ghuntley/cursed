/// Performance tests for enhanced debugging system
///
/// Validates that debugging features have minimal performance overhead
/// and scale appropriately with increased load.

use cursed::debug::enhanced_debug::*;
use cursed::runtime::debug_runtime::*;
use cursed::error::debug_context::*;
use cursed::stdlib::value::Value;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use std::collections::HashMap;

const PERFORMANCE_ITERATIONS: usize = 1000;
const LARGE_SCALE_ITERATIONS: usize = 10000;

#[test]
fn test_debug_registry_performance() {
    let registry = DebugInfoRegistry::new();
    let start_time = Instant::now();

    // Register many debug info entries
    for i in 0..PERFORMANCE_ITERATIONS {
        let debug_info = EnhancedDebugInfo::new(
            "perf_test.csd",
            i as u32,
            1,
            format!("function_{}", i),
        );
        
        let location_key = format!("perf_test.csd:{}:1", i);
        let _ = registry.register_debug_info(location_key, debug_info);
    }

    let registration_time = start_time.elapsed();
    println!("Registry registration time for {} entries: {:?}", PERFORMANCE_ITERATIONS, registration_time);

    // Test retrieval performance
    let retrieval_start = Instant::now();
    
    for i in 0..PERFORMANCE_ITERATIONS {
        let location_key = format!("perf_test.csd:{}:1", i);
        let _ = registry.get_debug_info(&location_key);
    }
    
    let retrieval_time = retrieval_start.elapsed();
    println!("Registry retrieval time for {} entries: {:?}", PERFORMANCE_ITERATIONS, retrieval_time);

    // Performance assertions
    assert!(registration_time < Duration::from_millis(100), "Registration should be under 100ms");
    assert!(retrieval_time < Duration::from_millis(50), "Retrieval should be under 50ms");
}

#[test]
fn test_runtime_debugger_overhead() {
    // Test with debugging enabled
    let enabled_debugger = RuntimeDebugger::new(true);
    let enabled_start = Instant::now();

    for i in 0..PERFORMANCE_ITERATIONS {
        let frame_id = enabled_debugger.enter_function(
            &format!("perf_func_{}", i),
            std::path::Path::new("perf_test.csd"),
            i as u32,
        ).unwrap();

        let _ = enabled_debugger.register_variable(
            format!("var_{}", i),
            Value::Integer(i as i64),
            "sus".to_string(),
            i as u32,
        );

        let _ = enabled_debugger.exit_function(frame_id);
    }

    let enabled_time = enabled_start.elapsed();
    println!("Enabled debugging time for {} operations: {:?}", PERFORMANCE_ITERATIONS, enabled_time);

    // Test with debugging disabled
    let disabled_debugger = RuntimeDebugger::new(false);
    let disabled_start = Instant::now();

    for i in 0..PERFORMANCE_ITERATIONS {
        let frame_id = disabled_debugger.enter_function(
            &format!("perf_func_{}", i),
            std::path::Path::new("perf_test.csd"),
            i as u32,
        ).unwrap();

        let _ = disabled_debugger.register_variable(
            format!("var_{}", i),
            Value::Integer(i as i64),
            "sus".to_string(),
            i as u32,
        );

        let _ = disabled_debugger.exit_function(frame_id);
    }

    let disabled_time = disabled_start.elapsed();
    println!("Disabled debugging time for {} operations: {:?}", PERFORMANCE_ITERATIONS, disabled_time);

    // Overhead should be minimal when disabled
    let overhead_ratio = enabled_time.as_nanos() as f64 / disabled_time.as_nanos() as f64;
    println!("Debug overhead ratio: {:.2}x", overhead_ratio);
    
    assert!(overhead_ratio < 10.0, "Debug overhead should be less than 10x when enabled");
    assert!(disabled_time < Duration::from_millis(10), "Disabled debugging should be very fast");
}

#[test]
fn test_variable_inspection_performance() {
    let inspector = VariableInspector::new();

    // Create complex nested structure
    let complex_value = Value::Object({
        let mut obj = HashMap::new();
        for i in 0..100 {
            obj.insert(
                format!("field_{}", i),
                Value::Array((0..10).map(|j| Value::Integer(i * 10 + j)).collect()),
            );
        }
        obj
    });

    let runtime_var = RuntimeVariable::new(
        "complex_var".to_string(),
        complex_value,
        "ComplexObject".to_string(),
        1,
    );

    let start_time = Instant::now();
    
    // Perform multiple inspections
    for _ in 0..100 {
        let _ = inspector.inspect_variable(&runtime_var);
    }
    
    let inspection_time = start_time.elapsed();
    println!("Variable inspection time for 100 complex inspections: {:?}", inspection_time);

    assert!(inspection_time < Duration::from_millis(500), "Complex variable inspection should be under 500ms");
}

#[test]
fn test_source_mapping_performance() {
    let mut source_map = SourceMap::new(PathBuf::from("large_file.csd"));

    let start_time = Instant::now();

    // Add many mapping ranges
    for i in 0..PERFORMANCE_ITERATIONS {
        source_map.add_range(
            i as u32,
            0,
            (i / 2) as u32,
            0,
            10,
        );
    }

    let mapping_time = start_time.elapsed();
    println!("Source mapping creation time for {} ranges: {:?}", PERFORMANCE_ITERATIONS, mapping_time);

    // Test mapping lookup performance
    let lookup_start = Instant::now();
    
    for i in 0..PERFORMANCE_ITERATIONS {
        let _ = source_map.map_to_original(i as u32, 5);
    }
    
    let lookup_time = lookup_start.elapsed();
    println!("Source mapping lookup time for {} lookups: {:?}", PERFORMANCE_ITERATIONS, lookup_time);

    assert!(mapping_time < Duration::from_millis(50), "Source mapping creation should be under 50ms");
    assert!(lookup_time < Duration::from_millis(100), "Source mapping lookup should be under 100ms");
}

#[test]
fn test_breakpoint_performance() {
    let debugger = RuntimeDebugger::new(true);

    let start_time = Instant::now();

    // Set many breakpoints
    let mut breakpoint_ids = Vec::new();
    for i in 0..PERFORMANCE_ITERATIONS {
        let bp_id = debugger.set_breakpoint(
            PathBuf::from(format!("file_{}.csd", i % 10)),
            i as u32,
        ).unwrap();
        breakpoint_ids.push(bp_id);
    }

    let setup_time = start_time.elapsed();
    println!("Breakpoint setup time for {} breakpoints: {:?}", PERFORMANCE_ITERATIONS, setup_time);

    // Test breakpoint checking performance
    let check_start = Instant::now();
    
    for i in 0..PERFORMANCE_ITERATIONS {
        let _ = debugger.check_breakpoint(
            std::path::Path::new(&format!("file_{}.csd", i % 10)),
            i as u32,
        );
    }
    
    let check_time = check_start.elapsed();
    println!("Breakpoint checking time for {} checks: {:?}", PERFORMANCE_ITERATIONS, check_time);

    // Clean up
    let cleanup_start = Instant::now();
    for bp_id in breakpoint_ids {
        let _ = debugger.remove_breakpoint(bp_id);
    }
    let cleanup_time = cleanup_start.elapsed();
    println!("Breakpoint cleanup time: {:?}", cleanup_time);

    assert!(setup_time < Duration::from_millis(100), "Breakpoint setup should be under 100ms");
    assert!(check_time < Duration::from_millis(200), "Breakpoint checking should be under 200ms");
    assert!(cleanup_time < Duration::from_millis(50), "Breakpoint cleanup should be under 50ms");
}

#[test]
fn test_error_context_generation_performance() {
    let start_time = Instant::now();

    for i in 0..PERFORMANCE_ITERATIONS {
        let error = cursed::error::Error::Runtime(format!("Test error {}", i));
        let mut debug_context = DebugContext::new(error)
            .with_annotation("iteration".to_string(), i.to_string())
            .with_annotation("context".to_string(), "Performance test".to_string());

        // Generate error report
        let _ = debug_context.generate_error_report();
    }

    let generation_time = start_time.elapsed();
    println!("Error context generation time for {} contexts: {:?}", PERFORMANCE_ITERATIONS, generation_time);

    assert!(generation_time < Duration::from_millis(500), "Error context generation should be under 500ms");
}

#[test]
#[ignore = "Large scale performance test"]
fn test_large_scale_debugging_performance() {
    let debugger = RuntimeDebugger::new(true);
    
    let start_time = Instant::now();

    // Simulate large application debugging
    for i in 0..LARGE_SCALE_ITERATIONS {
        // Enter function
        let frame_id = debugger.enter_function(
            &format!("large_func_{}", i % 100), // Reuse function names
            std::path::Path::new(&format!("module_{}.csd", i % 10)),
            (i % 1000) as u32,
        ).unwrap();

        // Register multiple variables
        for j in 0..5 {
            let _ = debugger.register_variable(
                format!("var_{}_{}", i, j),
                Value::Integer((i * 5 + j) as i64),
                "sus".to_string(),
                (i % 1000) as u32 + j as u32,
            );
        }

        // Occasionally set breakpoints
        if i % 100 == 0 {
            let _ = debugger.set_breakpoint(
                PathBuf::from(&format!("module_{}.csd", i % 10)),
                (i % 1000) as u32,
            );
        }

        // Exit function
        let _ = debugger.exit_function(frame_id);
    }

    let total_time = start_time.elapsed();
    println!("Large scale debugging time for {} operations: {:?}", LARGE_SCALE_ITERATIONS, total_time);

    // Generate final debug report
    let report_start = Instant::now();
    let report = debugger.generate_debug_report();
    let report_time = report_start.elapsed();
    
    println!("Debug report generation time: {:?}", report_time);
    assert!(report.is_ok());

    // Performance assertions for large scale
    let avg_time_per_op = total_time.as_nanos() / LARGE_SCALE_ITERATIONS as u128;
    println!("Average time per debug operation: {} ns", avg_time_per_op);
    
    assert!(avg_time_per_op < 100_000, "Average debug operation should be under 100μs");
    assert!(report_time < Duration::from_millis(100), "Report generation should be under 100ms");
}

#[test]
fn test_concurrent_debugging_performance() {
    use std::sync::Arc;
    use std::thread;

    let debugger = Arc::new(RuntimeDebugger::new(true));
    let num_threads = 4;
    let iterations_per_thread = PERFORMANCE_ITERATIONS / num_threads;

    let start_time = Instant::now();

    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let debugger_clone = debugger.clone();
        
        let handle = thread::spawn(move || {
            for i in 0..iterations_per_thread {
                let frame_id = debugger_clone.enter_function(
                    &format!("thread_{}_{}", thread_id, i),
                    std::path::Path::new(&format!("thread_{}.csd", thread_id)),
                    i as u32,
                ).unwrap();

                let _ = debugger_clone.register_variable(
                    format!("thread_var_{}_{}", thread_id, i),
                    Value::Integer((thread_id * 1000 + i) as i64),
                    "sus".to_string(),
                    i as u32,
                );

                let _ = debugger_clone.exit_function(frame_id);
            }
        });
        
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let concurrent_time = start_time.elapsed();
    println!("Concurrent debugging time for {} threads × {} ops: {:?}", 
             num_threads, iterations_per_thread, concurrent_time);

    // Generate report to verify data integrity
    let report = debugger.generate_debug_report();
    assert!(report.is_ok());

    // Concurrent performance should be reasonable
    assert!(concurrent_time < Duration::from_millis(1000), "Concurrent debugging should be under 1s");
}

#[test]
fn test_memory_usage_scaling() {
    let debugger = RuntimeDebugger::new(true);
    let registry = DebugInfoRegistry::new();

    // Measure initial memory baseline
    let initial_stats = registry.get_statistics().unwrap();
    
    // Add a substantial amount of debug information
    for i in 0..PERFORMANCE_ITERATIONS {
        // Register debug info
        let debug_info = EnhancedDebugInfo::new(
            "memory_test.csd",
            i as u32,
            1,
            format!("memory_func_{}", i),
        );
        
        let location_key = format!("memory_test.csd:{}:1", i);
        let _ = registry.register_debug_info(location_key, debug_info);

        // Register symbol
        let metadata = SymbolMetadata::function(&format!("func_{}", i), None);
        let _ = registry.register_symbol(format!("module::func_{}", i), metadata);

        // Add variables to debugger
        let frame_id = debugger.enter_function(
            &format!("memory_func_{}", i),
            std::path::Path::new("memory_test.csd"),
            i as u32,
        ).unwrap();

        let _ = debugger.register_variable(
            format!("memory_var_{}", i),
            Value::String(format!("value_{}", i)),
            "tea".to_string(),
            i as u32,
        );

        let _ = debugger.exit_function(frame_id);
    }

    // Measure final memory usage
    let final_stats = registry.get_statistics().unwrap();
    
    println!("Initial stats: {}", initial_stats);
    println!("Final stats: {}", final_stats);
    
    // Verify that memory usage scales linearly
    assert_eq!(final_stats.debug_info_count, PERFORMANCE_ITERATIONS);
    assert_eq!(final_stats.symbol_count, PERFORMANCE_ITERATIONS);
    
    // Memory scaling should be reasonable (this is a basic sanity check)
    assert!(final_stats.debug_info_count > initial_stats.debug_info_count);
    assert!(final_stats.symbol_count > initial_stats.symbol_count);
}

#[test]
fn test_debug_report_scalability() {
    let debugger = RuntimeDebugger::new(true);

    // Create a complex debugging scenario
    let mut frame_ids = Vec::new();
    
    // Create nested function calls
    for i in 0..50 {
        let frame_id = debugger.enter_function(
            &format!("nested_func_{}", i),
            std::path::Path::new(&format!("nested_{}.csd", i % 5)),
            i as u32 * 10,
        ).unwrap();
        
        frame_ids.push(frame_id);

        // Add multiple variables per function
        for j in 0..10 {
            let _ = debugger.register_variable(
                format!("var_{}", j),
                Value::Integer((i * 10 + j) as i64),
                "sus".to_string(),
                i as u32 * 10 + j as u32,
            );
        }

        // Set some breakpoints
        if i % 10 == 0 {
            let _ = debugger.set_breakpoint(
                PathBuf::from(&format!("nested_{}.csd", i % 5)),
                i as u32 * 10,
            );
        }
    }

    // Generate debug report with complex state
    let report_start = Instant::now();
    let report = debugger.generate_debug_report();
    let report_time = report_start.elapsed();

    assert!(report.is_ok());
    let report = report.unwrap();

    println!("Complex debug report generation time: {:?}", report_time);
    println!("Report contains {} stack frames", report.stack_trace.len());
    println!("Report contains {} scope variables", report.scope_variables.len());
    println!("Report contains {} breakpoints", report.active_breakpoints.len());

    // Clean up (exit functions in reverse order)
    for frame_id in frame_ids.into_iter().rev() {
        let _ = debugger.exit_function(frame_id);
    }

    // Report generation should be fast even with complex state
    assert!(report_time < Duration::from_millis(50), "Complex report generation should be under 50ms");
    assert!(report.stack_trace.len() == 50);
    assert!(!report.scope_variables.is_empty());
}
