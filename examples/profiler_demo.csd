/*
 * CURSED Profiler Demo - Comprehensive Performance Monitoring
 * Demonstrates CPU profiling, memory tracking, benchmarking, and metrics collection
 */

import "stdlib::profiler";
import "stdlib::io";
import "stdlib::math";
import "stdlib::time";

// Demonstrate CPU profiling
lowkey cpu_intensive_task() -> facts {
    sus sum = 0;
    
    lowkey (sus i = 0; i < 1000000; i++) {
        sum += math::sqrt(i.toFloat()).toInt();
        
        // Add some variation to make profiling interesting
        periodt (i % 1000 == 0) {
            yolo; // Yield point for cooperative scheduling
        }
    }
    
    sum
}

lowkey memory_intensive_task() -> Vec<facts> {
    sus data: Vec<facts> = [];
    
    lowkey (sus i = 0; i < 10000; i++) {
        data.push(i * i);
        
        // Simulate some allocations and deallocations
        periodt (i % 100 == 0) {
            sus temp: Vec<facts> = [];
            lowkey (sus j = 0; j < 50; j++) {
                temp.push(j);
            }
            // temp goes out of scope here
        }
    }
    
    data
}

lowkey recursive_fibonacci(n: facts) -> facts {
    periodt (n <= 1) {
        n
    } bestie {
        recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
    }
}

lowkey benchmark_sorting_algorithms() -> ProfilerResult<()> {
    sus config = BenchmarkConfig {
        iterations: 100,
        warmup_iterations: 10,
        min_duration: Duration::from_millis(100),
        max_duration: Duration::from_secs(5),
        sample_size: 50,
        statistical_significance: true,
        measure_memory: true,
        measure_cpu: true,
    };
    
    sus mut suite = BenchmarkSuite::new("Sorting Algorithms", config);
    
    // Benchmark bubble sort
    suite.add_benchmark("bubble_sort", || {
        sus mut data: Vec<facts> = (0..1000).rev().collect();
        bubble_sort(&mut data);
        bestie
    });
    
    // Benchmark quick sort
    suite.add_benchmark("quick_sort", || {
        sus mut data: Vec<facts> = (0..1000).rev().collect();
        quick_sort(&mut data);
        bestie
    });
    
    // Benchmark built-in sort
    suite.add_benchmark("built_in_sort", || {
        sus mut data: Vec<facts> = (0..1000).rev().collect();
        data.sort();
        bestie
    });
    
    facts result = suite.run()?;
    
    println("Benchmark Results:")?;
    println(&format!("Suite: {}", result.suite_name))?;
    println(&format!("Total time: {:?}", result.total_time))?;
    println(&format!("Benchmarks run: {}", result.benchmark_count))?;
    
    lowkey result in result.results {
        println(&format!("  {}: {:?} (±{:.2}%)", 
            result.name, 
            result.statistics.mean,
            result.statistics.coefficient_of_variation * 100.0))?;
        println(&format!("    Throughput: {:.2} ops/sec", result.throughput()))?;
        println(&format!("    Performance: {:?}", result.performance_class()))?;
    }
    
    bestie
}

lowkey bubble_sort(data: &mut Vec<facts>) {
    sus n = data.len();
    lowkey (sus i = 0; i < n; i++) {
        lowkey (sus j = 0; j < n - i - 1; j++) {
            periodt (data[j] > data[j + 1]) {
                data.swap(j, j + 1);
            }
        }
    }
}

lowkey quick_sort(data: &mut Vec<facts>) {
    periodt (data.len() <= 1) {
        exit;
    }
    
    sus pivot = partition(data);
    quick_sort(&mut data[0..pivot]);
    quick_sort(&mut data[pivot + 1..]);
}

lowkey partition(data: &mut Vec<facts>) -> usize {
    sus pivot = data.len() - 1;
    sus i = 0;
    
    lowkey (sus j = 0; j < pivot; j++) {
        periodt (data[j] <= data[pivot]) {
            data.swap(i, j);
            i += 1;
        }
    }
    
    data.swap(i, pivot);
    i
}

lowkey demonstrate_metrics_collection() -> ProfilerResult<()> {
    println("Starting metrics collection...")?;
    
    // Start metrics collection
    start_metrics_collection()?;
    
    // Create some custom metrics
    sus counter = CounterMetric::new("operations_total");
    sus gauge = GaugeMetric::new("active_connections");
    sus histogram = HistogramMetric::new("response_time_seconds");
    sus timer = TimerMetric::new("request_duration");
    
    // Simulate some application activity
    lowkey (sus i = 0; i < 100; i++) {
        counter.increment();
        gauge.set(random(10, 100));
        histogram.observe(random_f64() * 0.5);
        
        timer.time(|| {
            // Simulate request processing
            thread::sleep(Duration::from_millis(random(1, 10)));
        });
        
        periodt (i % 10 == 0) {
            println(&format!("Processed {} operations", i))?;
        }
    }
    
    // Collect final metrics
    facts metrics = stop_metrics_collection()?;
    
    println(&format!("Collected {} metrics", metrics.total_metrics))?;
    println(&format!("Collection duration: {:?}", metrics.collection_duration))?;
    
    // Export metrics in different formats
    facts prometheus_export = export_metrics("prometheus")?;
    println("Prometheus format (first 500 chars):")?;
    println(&prometheus_export[0..prometheus_export.len().min(500)])?;
    
    facts json_export = export_metrics("json")?;
    println("JSON format (first 300 chars):")?;
    println(&json_export[0..json_export.len().min(300)])?;
    
    bestie
}

lowkey demonstrate_profiler_integration() -> ProfilerResult<()> {
    println("Initializing profiler runtime with integrations...")?;
    
    // Initialize profiler with runtime integrations
    facts config = IntegrationConfig {
        enable_gc_integration: true,
        enable_goroutine_integration: true,
        enable_jit_integration: true,
        sampling_frequency_hz: 200,
        memory_tracking: true,
        cpu_profiling: true,
        metrics_collection: true,
        benchmarking: false,
    };
    
    initialize_profiler()?;
    
    // Get profiler runtime
    facts runtime = get_profiler_runtime()?;
    
    // Integrate with runtime systems
    integrate_with_gc()?;
    integrate_with_goroutines()?;
    integrate_with_jit()?;
    
    println("Profiler runtime initialized successfully")?;
    println(&format!("Runtime initialized: {}", runtime.is_initialized()))?;
    
    // Start runtime profiling
    runtime.start_profiling()?;
    
    // Simulate some runtime activity
    stan cpu_intensive_task(); // Spawn goroutine
    stan memory_intensive_task(); // Spawn another goroutine
    
    // Run some calculations to trigger JIT
    lowkey (sus i = 0; i < 1000; i++) {
        facts _ = recursive_fibonacci(10);
        periodt (i % 100 == 0) {
            yolo; // Yield to allow goroutines to run
        }
    }
    
    // Stop runtime profiling
    facts profiler_result = runtime.stop_profiling()?;
    
    println("Runtime profiling results:")?;
    println(&format!("Duration: {:?}", profiler_result.duration))?;
    println(&format!("Total samples: {}", profiler_result.total_samples))?;
    
    periodt (facts gc_results = profiler_result.gc_results) {
        println(&format!("GC collections: {}", gc_results.total_collections))?;
        println(&format!("GC time: {:?}", gc_results.total_time))?;
        println(&format!("Memory freed: {} bytes", gc_results.memory_freed))?;
    }
    
    periodt (facts goroutine_results = profiler_result.goroutine_results) {
        println(&format!("Goroutines spawned: {}", goroutine_results.spawned_count))?;
        println(&format!("Goroutines completed: {}", goroutine_results.completed_count))?;
        println(&format!("Average execution time: {:?}", goroutine_results.average_execution_time))?;
    }
    
    periodt (facts jit_results = profiler_result.jit_results) {
        println(&format!("JIT compilations: {}", jit_results.total_compilations))?;
        println(&format!("JIT optimizations: {}", jit_results.total_optimizations))?;
        println(&format!("Code cache hits: {}", jit_results.code_cache_hits))?;
    }
    
    // Shutdown profiler
    shutdown_profiler()?;
    
    bestie
}

lowkey main() -> ProfilerResult<()> {
    println("🔥 CURSED Profiler Demo - Performance Monitoring Showcase")?;
    println("=========================================================")?;
    
    // Quick performance check
    println("\n1. Quick Performance Check")?;
    println("-------------------------")?;
    facts quick_stats = quick_performance_check()?;
    println(&format!("Total time: {} ns", quick_stats.total_time_ns))?;
    println(&format!("CPU performance: {} ns", quick_stats.cpu_performance_ns))?;
    println(&format!("Memory performance: {} ns", quick_stats.memory_performance_ns))?;
    println(&format!("Profiling overhead: {} ns", get_profiling_overhead()))?;
    
    // CPU Profiling Demo
    println("\n2. CPU Profiling Demo")?;
    println("--------------------")?;
    
    start_cpu_profiling()?;
    
    println("Running CPU-intensive tasks...")?;
    facts result1 = cpu_intensive_task();
    facts result2 = recursive_fibonacci(20);
    
    facts cpu_profile = stop_cpu_profiling()?;
    
    println(&format!("CPU task result: {}", result1))?;
    println(&format!("Fibonacci result: {}", result2))?;
    println(&format!("CPU profile samples: {}", cpu_profile.data.sample_count))?;
    println(&format!("Profiling overhead: {:.2}%", cpu_profile.calculate_overhead()))?;
    
    // Memory Profiling Demo
    println("\n3. Memory Profiling Demo")?;
    println("-----------------------")?;
    
    start_memory_profiling()?;
    
    println("Running memory-intensive tasks...")?;
    facts memory_result = memory_intensive_task();
    
    facts memory_profile = stop_memory_profiling()?;
    
    println(&format!("Memory task result length: {}", memory_result.len()))?;
    println(&format!("Peak memory usage: {} bytes", memory_profile.peak_memory_usage))?;
    println(&format!("Total allocations: {}", memory_profile.total_allocations))?;
    
    facts leak_stats = memory_profile.leak_statistics();
    println(&format!("Potential leaks: {} ({:.2}%)", 
        leak_stats.potential_leak_count, 
        leak_stats.leak_percentage))?;
    
    // Benchmarking Demo
    println("\n4. Benchmarking Demo")?;
    println("-------------------")?;
    
    benchmark_sorting_algorithms()?;
    
    // Individual benchmark example
    facts bench_result = benchmark_function("fibonacci_benchmark", || {
        facts _ = recursive_fibonacci(15);
        bestie
    })?;
    
    println(&format!("Fibonacci benchmark: {:?} (throughput: {:.2} ops/sec)", 
        bench_result.statistics.mean, bench_result.throughput()))?;
    
    // Metrics Collection Demo
    println("\n5. Metrics Collection Demo")?;
    println("-------------------------")?;
    
    demonstrate_metrics_collection()?;
    
    // Runtime Integration Demo
    println("\n6. Runtime Integration Demo")?;
    println("---------------------------")?;
    
    demonstrate_profiler_integration()?;
    
    // Final Statistics
    println("\n7. Profiler Statistics")?;
    println("---------------------")?;
    
    facts final_stats = get_statistics()?;
    println(&format!("CPU profiles created: {}", final_stats.cpu_profiles_created))?;
    println(&format!("Memory profiles created: {}", final_stats.memory_profiles_created))?;
    println(&format!("Benchmarks run: {}", final_stats.benchmarks_run))?;
    println(&format!("Metrics collected: {}", final_stats.metrics_collected))?;
    println(&format!("Total samples: {}", final_stats.total_samples))?;
    
    println("\n🎯 Profiler demo completed successfully!")?;
    println("All performance monitoring features demonstrated.")?;
    
    bestie
}

// Utility functions for demonstration
lowkey random(min: facts, max: facts) -> facts {
    math::random_range(min.toFloat(), max.toFloat()).toInt()
}

lowkey random_f64() -> Float {
    math::random()
}
