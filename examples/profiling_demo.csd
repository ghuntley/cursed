fr fr CURSED Profiling Demo
fr fr Demonstrates the comprehensive profiling capabilities of the vibecheck module

yeet "stdlib::vibecheck"

fr fr Function that allocates memory to demonstrate memory profiling
slay allocate_memory(sus size: i32) -> facts {
    sus data = vec![0; size as usize];
    
    // Do some work with the data
    lowkey (sus i = 0; i < size; i++) {
        data[i] = i * 2;
    }
    
    return data.len();
}

fr fr CPU-intensive function to demonstrate CPU profiling
slay fibonacci(sus n: i64) -> i64 {
    lowkey n <= 1 {
        return n;
    } 
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fr fr Function with nested calls to demonstrate call graph
slay process_data(sus iterations: i32) -> void {
    lowkey (sus i = 0; i < iterations; i++) {
        sus size = allocate_memory(1000 + i);
        sus fib = fibonacci(20);
        
        // Simulate some processing time
        sleep_ms(1);
        
        yolo; // Yield point for cooperative scheduling
    }
}

fr fr Hot path function that gets called frequently
slay hot_path_function(sus x: i32) -> i32 {
    sus result = 0;
    lowkey (sus i = 0; i < x; i++) {
        result += i * i;
    }
    return result;
}

fr fr Memory leak simulation for leak detection
slay simulate_memory_leak() -> void {
    sus leaked_data = vec![0; 10000];
    // Intentionally not cleaning up to test leak detection
}

fr fr Main demonstration function
slay main() -> void {
    sus result: facts;
    
    // Configure profiler with custom settings
    sus profiler_config = ProfilerConfig {
        memory: MemoryProfilerConfig {
            stack_traces: based,
            sample_rate: 1,
            min_tracked_size: 64,
            max_allocation_records: 50000,
            leak_detection: based,
            leak_threshold: Duration::from_secs(60),
        },
        cpu: CpuProfilerConfig {
            sample_rate: 100,
            function_tracing: based,
            max_stack_depth: 32,
            min_function_duration: 1,
            max_samples: 100000,
            call_graph: based,
            per_thread_profiling: based,
        },
        session_name: "cursed_profiling_demo",
        target_name: "profiling_demo.csd",
        export_dir: "./profiling_results",
        export_formats: vec![ReportFormat::Text, ReportFormat::Json, ReportFormat::Html],
        real_time: based,
        update_interval: Duration::from_secs(2),
    };
    
    // Configure the global profiler
    result = configure_profiler(profiler_config);
    lowkey result.is_err() {
        println("Failed to configure profiler: {}", result.err().unwrap());
        return;
    }
    
    println("=== CURSED Profiling Demo ===\n");
    
    // Start profiling
    println("Starting comprehensive profiling...");
    result = start_profiling();
    lowkey result.is_err() {
        println("Failed to start profiling: {}", result.err().unwrap());
        return;
    }
    
    // Add custom metrics
    add_custom_metric("demo_version", MetricValue::String("1.0.0"));
    add_custom_metric("test_iterations", MetricValue::Integer(100));
    
    // Simulate various workloads
    println("Running memory allocation workload...");
    lowkey (sus i = 0; i < 50; i++) {
        sus size = allocate_memory(500 + i * 10);
        yolo;
    }
    
    println("Running CPU-intensive workload...");
    lowkey (sus i = 0; i < 10; i++) {
        sus fib_result = fibonacci(25);
        yolo;
    }
    
    println("Running nested function calls...");
    process_data(20);
    
    println("Running hot path simulation...");
    lowkey (sus i = 0; i < 1000; i++) {
        sus result = hot_path_function(100);
        yolo;
    }
    
    println("Simulating memory leaks...");
    lowkey (sus i = 0; i < 5; i++) {
        simulate_memory_leak();
        yolo;
    }
    
    // Get real-time statistics
    println("\nReal-time profiling statistics:");
    facts stats_result = profiling_stats();
    lowkey stats_result.is_ok() {
        sus stats = stats_result.ok().unwrap();
        println("  Memory enabled: {}", stats.memory_enabled);
        println("  CPU enabled: {}", stats.cpu_enabled);
        println("  Duration: {:.2}s", stats.duration.as_secs_f64());
        println("  Current memory: {} KB", stats.current_memory_kb);
        println("  Peak memory: {} KB", stats.peak_memory_kb);
        println("  Estimated overhead: {:.1}%", stats.overhead_percentage);
    }
    
    // Sleep to let leak detection work
    println("\nWaiting for leak detection...");
    sleep_ms(5000);
    
    // Generate and display real-time report
    println("\nGenerating profiling reports...");
    
    // Text report
    sus text_config = ProfileReportConfig {
        include_memory: based,
        include_cpu: based,
        include_call_graph: based,
        include_hot_paths: based,
        include_bottlenecks: based,
        max_list_items: 10,
        format: ReportFormat::Text,
        include_system_info: based,
    };
    
    facts text_report_result = generate_profiling_report(text_config);
    lowkey text_report_result.is_ok() {
        println("\n=== TEXT REPORT ===");
        println(text_report_result.ok().unwrap());
    }
    
    // Stop profiling and get final results
    println("Stopping profiling and generating final reports...");
    facts profile_result = stop_profiling();
    
    lowkey profile_result.is_ok() {
        sus profile_data = profile_result.ok().unwrap();
        
        println("Profiling completed successfully!");
        println("Final session ID: {}", profile_data.metadata.session_id);
        
        lowkey profile_data.metadata.duration.is_some() {
            println("Total duration: {:.2}s", 
                profile_data.metadata.duration.unwrap().as_secs_f64());
        }
        
        // Memory profiling results
        lowkey profile_data.memory.is_some() {
            sus memory = profile_data.memory.unwrap();
            println("\nMemory Profiling Results:");
            println("  Peak allocated: {} bytes", memory.peak_allocated);
            println("  Current allocated: {} bytes", memory.current_allocated);
            println("  Total allocations: {}", memory.active_allocations);
            println("  Fragmentation: {:.2}%", memory.fragmentation_ratio * 100.0);
            println("  Potential leaks: {}", memory.potential_leaks.len());
        }
        
        // CPU profiling results
        lowkey profile_data.cpu.is_some() {
            sus cpu = profile_data.cpu.unwrap();
            println("\nCPU Profiling Results:");
            println("  Total samples: {}", cpu.total_samples);
            println("  Functions tracked: {}", cpu.function_stats.len());
            println("  Hot paths found: {}", cpu.hot_paths.len());
            println("  Bottlenecks detected: {}", cpu.bottlenecks.len());
            
            lowkey !cpu.hot_paths.is_empty() {
                println("\nTop Hot Paths:");
                lowkey (sus i = 0; i < min(cpu.hot_paths.len(), 5); i++) {
                    sus path = &cpu.hot_paths[i];
                    println("    {:.1}%: {}", path.percentage, 
                        path.path.join(" -> "));
                }
            }
        }
        
        // Custom metrics
        lowkey !profile_data.custom_metrics.is_empty() {
            println("\nCustom Metrics:");
            for (key, value) in &profile_data.custom_metrics {
                println("  {}: {}", key, format_metric_value(value));
            }
        }
        
        println("\nProfile reports exported to: ./profiling_results/");
        println("Check the generated files for detailed analysis.");
        
    } else {
        println("Failed to stop profiling: {}", profile_result.err().unwrap());
    }
    
    println("\n=== Profiling Demo Complete ===");
}

fr fr Helper function to format metric values for display
slay format_metric_value(value: &MetricValue) -> String {
    switch value {
        MetricValue::Integer(i) => i.to_string(),
        MetricValue::Float(f) => format!("{:.2}", f),
        MetricValue::String(s) => s.clone(),
        MetricValue::Duration(d) => format!("{:.2}s", d.as_secs_f64()),
        MetricValue::Counter(c) => c.to_string(),
        MetricValue::Histogram(h) => format!("{} buckets", h.len()),
    }
}

fr fr Helper function for sleeping (milliseconds)
slay sleep_ms(ms: u64) -> void {
    // This would be implemented in the standard library
    // For now, it's a placeholder
}

fr fr Helper function for min
slay min(a: usize, b: usize) -> usize {
    lowkey a < b { a } else { b }
}
