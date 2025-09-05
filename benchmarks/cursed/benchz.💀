fr fr CURSED Benchmarking Framework (benchz) - Comprehensive Performance Suite

yeet "testz"
yeet "vibez"

fr fr Benchmark configuration and state
sus benchmark_iterations normie = 1000
sus warmup_iterations normie = 100
sus benchmark_results []BenchmarkResult = []
sus compilation_results []CompilationBenchmark = []
sus memory_results []MemoryBenchmark = []
sus current_benchmark_suite tea = ""

squad BenchmarkResult {
    spill name tea
    spill duration_ns normie
    spill iterations normie
    spill memory_used normie
    spill ops_per_sec meal
    spill min_time_ns normie
    spill max_time_ns normie
    spill avg_time_ns normie
    spill percentile_95_ns normie
}

squad CompilationBenchmark {
    spill source_file tea
    spill source_size normie
    spill compilation_time_ms normie
    spill interpretation_time_ms normie
    spill llvm_compilation_time_ms normie
    spill binary_size normie
    spill optimization_level tea
}

squad MemoryBenchmark {
    spill test_name tea
    spill initial_memory normie
    spill peak_memory normie
    spill final_memory normie
    spill gc_cycles normie
    spill allocation_count normie
    spill deallocation_count normie
}

squad PerformanceProfile {
    spill cpu_usage_percent meal
    spill memory_usage_mb meal
    spill io_operations normie
    spill cache_hits normie
    spill cache_misses normie
}

fr fr Core benchmarking functions
slay benchmark_suite_start(suite_name tea) lit {
    current_benchmark_suite = suite_name
    vibez.spill("\n🚀 Starting benchmark suite: ", suite_name)
    vibez.spill("════════════════════════════════════════")
    damn based
}

slay benchmark_precise(name tea, test_function slay()) BenchmarkResult {
    vibez.spill("⏱️ Benchmarking: ", name)
    
    fr fr Warmup phase
    bestie i := 0; i < warmup_iterations; i = i + 1 {
        test_function()
    }
    
    fr fr Collect timing data
    sus times []normie = []
    sus start_mem normie = get_memory_usage()
    
    bestie i := 0; i < benchmark_iterations; i = i + 1 {
        sus iter_start normie = get_time_ns()
        test_function()
        sus iter_end normie = get_time_ns()
        times.push(iter_end - iter_start)
    }
    
    sus end_mem normie = get_memory_usage()
    
    fr fr Calculate statistics
    sus total_time normie = 0
    sus min_time normie = times[0]
    sus max_time normie = times[0]
    
    bestie time in times {
        total_time = total_time + time
        lowkey time < min_time {
            min_time = time
        }
        lowkey time > max_time {
            max_time = time
        }
    }
    
    sus avg_time normie = total_time / benchmark_iterations
    sus percentile_95 normie = calculate_percentile(times, 95)
    sus memory_used normie = end_mem - start_mem
    sus ops_per_sec meal = (benchmark_iterations * 1000000000.0) / total_time
    
    sus result BenchmarkResult = BenchmarkResult{
        name: name,
        duration_ns: total_time,
        iterations: benchmark_iterations,
        memory_used: memory_used,
        ops_per_sec: ops_per_sec,
        min_time_ns: min_time,
        max_time_ns: max_time,
        avg_time_ns: avg_time,
        percentile_95_ns: percentile_95
    }
    
    benchmark_results.push(result)
    
    vibez.spill("📊 ", name)
    vibez.spill("   Avg: ", avg_time, "ns")
    vibez.spill("   Min: ", min_time, "ns") 
    vibez.spill("   Max: ", max_time, "ns")
    vibez.spill("   95%: ", percentile_95, "ns")
    vibez.spill("   Ops/sec: ", ops_per_sec)
    vibez.spill("")
    
    damn result
}

slay benchmark_compilation(source_file tea) CompilationBenchmark {
    vibez.spill("🔧 Benchmarking compilation: ", source_file)
    
    sus source_size normie = get_file_size(source_file)
    
    fr fr Interpretation benchmark
    sus interp_start normie = get_time_ns()
    run_interpreter(source_file)
    sus interp_end normie = get_time_ns()
    sus interpretation_time normie = (interp_end - interp_start) / 1000000
    
    fr fr LLVM compilation benchmark
    sus llvm_start normie = get_time_ns()
    sus binary_file tea = compile_to_llvm(source_file)
    sus llvm_end normie = get_time_ns()
    sus llvm_time normie = (llvm_end - llvm_start) / 1000000
    
    sus binary_size normie = get_file_size(binary_file)
    
    sus result CompilationBenchmark = CompilationBenchmark{
        source_file: source_file,
        source_size: source_size,
        compilation_time_ms: 0,
        interpretation_time_ms: interpretation_time,
        llvm_compilation_time_ms: llvm_time,
        binary_size: binary_size,
        optimization_level: "O2"
    }
    
    compilation_results.push(result)
    
    vibez.spill("📊 Compilation results for ", source_file)
    vibez.spill("   Source size: ", source_size, " bytes")
    vibez.spill("   Interpretation: ", interpretation_time, "ms")
    vibez.spill("   LLVM compilation: ", llvm_time, "ms")
    vibez.spill("   Binary size: ", binary_size, " bytes")
    vibez.spill("   Speedup: ", interpretation_time / llvm_time, "x")
    vibez.spill("")
    
    damn result
}

slay benchmark_memory(test_name tea, test_function slay()) MemoryBenchmark {
    vibez.spill("🧠 Memory benchmarking: ", test_name)
    
    fr fr Force GC to get baseline
    force_gc()
    sus initial_mem normie = get_memory_usage()
    sus initial_gc_cycles normie = get_gc_cycle_count()
    sus initial_allocations normie = get_allocation_count()
    
    fr fr Run test and monitor memory
    sus peak_mem normie = initial_mem
    bestie i := 0; i < benchmark_iterations; i = i + 1 {
        test_function()
        sus current_mem normie = get_memory_usage()
        lowkey current_mem > peak_mem {
            peak_mem = current_mem
        }
    }
    
    force_gc()
    sus final_mem normie = get_memory_usage()
    sus final_gc_cycles normie = get_gc_cycle_count()
    sus final_allocations normie = get_allocation_count()
    
    sus result MemoryBenchmark = MemoryBenchmark{
        test_name: test_name,
        initial_memory: initial_mem,
        peak_memory: peak_mem,
        final_memory: final_mem,
        gc_cycles: final_gc_cycles - initial_gc_cycles,
        allocation_count: final_allocations - initial_allocations,
        deallocation_count: 0  fr fr Would need runtime support
    }
    
    memory_results.push(result)
    
    vibez.spill("📊 Memory usage for ", test_name)
    vibez.spill("   Initial: ", initial_mem, " bytes")
    vibez.spill("   Peak: ", peak_mem, " bytes")
    vibez.spill("   Final: ", final_mem, " bytes")
    vibez.spill("   GC cycles: ", final_gc_cycles - initial_gc_cycles)
    vibez.spill("   Allocations: ", final_allocations - initial_allocations)
    vibez.spill("")
    
    damn result
}

fr fr Comparison and analysis functions
slay compare_benchmarks(name1 tea, name2 tea) lit {
    sus result1 BenchmarkResult = find_benchmark(name1)
    sus result2 BenchmarkResult = find_benchmark(name2)
    
    lowkey result1.name == "" || result2.name == "" {
        vibez.spill("❌ Could not find benchmarks for comparison")
        damn based
    }
    
    vibez.spill("📈 Comparing ", name1, " vs ", name2)
    vibez.spill("────────────────────────────────────")
    
    sus speedup meal = result1.ops_per_sec / result2.ops_per_sec
    sus time_ratio meal = result2.avg_time_ns / result1.avg_time_ns
    
    vibez.spill("Speedup: ", speedup, "x")
    vibez.spill("Time ratio: ", time_ratio, "x")
    
    lowkey result1.memory_used < result2.memory_used {
        vibez.spill("Memory: ", name1, " uses less memory")
    } highkey result1.memory_used > result2.memory_used {
        vibez.spill("Memory: ", name2, " uses less memory")
    } highkey {
        vibez.spill("Memory: Similar usage")
    }
    
    damn based
}

slay analyze_performance_trends() lit {
    lowkey benchmark_results.len() < 2 {
        vibez.spill("⚠️ Need at least 2 benchmarks for trend analysis")
        damn based
    }
    
    vibez.spill("📊 Performance Trend Analysis")
    vibez.spill("════════════════════════════════")
    
    sus total_ops meal = 0.0
    sus fastest_name tea = ""
    sus fastest_ops meal = 0.0
    sus slowest_name tea = ""
    sus slowest_ops meal = 999999999.0
    
    bestie result in benchmark_results {
        total_ops = total_ops + result.ops_per_sec
        
        lowkey result.ops_per_sec > fastest_ops {
            fastest_ops = result.ops_per_sec
            fastest_name = result.name
        }
        
        lowkey result.ops_per_sec < slowest_ops {
            slowest_ops = result.ops_per_sec
            slowest_name = result.name
        }
    }
    
    sus avg_ops meal = total_ops / benchmark_results.len()
    
    vibez.spill("Average performance: ", avg_ops, " ops/sec")
    vibez.spill("Fastest: ", fastest_name, " (", fastest_ops, " ops/sec)")
    vibez.spill("Slowest: ", slowest_name, " (", slowest_ops, " ops/sec)")
    vibez.spill("Performance range: ", fastest_ops / slowest_ops, "x")
    
    damn based
}

fr fr Utility functions
slay calculate_percentile(values []normie, percentile normie) normie {
    fr fr Simple percentile calculation (would need sorting in real implementation)
    sus index normie = (values.len() * percentile) / 100
    lowkey index >= values.len() {
        index = values.len() - 1
    }
    damn values[index]
}

slay find_benchmark(name tea) BenchmarkResult {
    bestie result in benchmark_results {
        lowkey result.name == name {
            damn result
        }
    }
    fr fr Return empty result if not found
    damn BenchmarkResult{name: "", duration_ns: 0, iterations: 0, memory_used: 0, ops_per_sec: 0.0, min_time_ns: 0, max_time_ns: 0, avg_time_ns: 0, percentile_95_ns: 0}
}

fr fr Mock runtime functions (would be implemented in runtime)
slay get_file_size(filename tea) normie {
    fr fr Mock implementation
    damn 1024
}

slay run_interpreter(filename tea) lit {
    fr fr Mock implementation
    damn based
}

slay compile_to_llvm(filename tea) tea {
    fr fr Mock implementation
    damn "output_binary"
}

slay force_gc() lit {
    fr fr Mock implementation
    damn based
}

slay get_gc_cycle_count() normie {
    fr fr Mock implementation
    damn 1
}

slay get_allocation_count() normie {
    fr fr Mock implementation
    damn 1000
}

fr fr Reporting functions
slay generate_benchmark_report() lit {
    vibez.spill("\n📊 COMPREHENSIVE BENCHMARK REPORT")
    vibez.spill("═══════════════════════════════════════════")
    vibez.spill("Suite: ", current_benchmark_suite)
    vibez.spill("Timestamp: ", get_current_timestamp())
    vibez.spill("")
    
    fr fr Performance benchmarks
    lowkey benchmark_results.len() > 0 {
        vibez.spill("🚀 PERFORMANCE BENCHMARKS")
        vibez.spill("───────────────────────────────")
        bestie result in benchmark_results {
            vibez.spill("• ", result.name)
            vibez.spill("  Iterations: ", result.iterations)
            vibez.spill("  Avg time: ", result.avg_time_ns, "ns")
            vibez.spill("  Ops/sec: ", result.ops_per_sec)
            vibez.spill("  Memory: ", result.memory_used, " bytes")
            vibez.spill("")
        }
    }
    
    fr fr Compilation benchmarks
    lowkey compilation_results.len() > 0 {
        vibez.spill("🔧 COMPILATION BENCHMARKS")
        vibez.spill("───────────────────────────────")
        bestie result in compilation_results {
            vibez.spill("• ", result.source_file)
            vibez.spill("  Source: ", result.source_size, " bytes")
            vibez.spill("  Interpretation: ", result.interpretation_time_ms, "ms")
            vibez.spill("  LLVM compilation: ", result.llvm_compilation_time_ms, "ms")
            vibez.spill("  Binary size: ", result.binary_size, " bytes")
            vibez.spill("")
        }
    }
    
    fr fr Memory benchmarks
    lowkey memory_results.len() > 0 {
        vibez.spill("🧠 MEMORY BENCHMARKS")
        vibez.spill("───────────────────────────────")
        bestie result in memory_results {
            vibez.spill("• ", result.test_name)
            vibez.spill("  Peak memory: ", result.peak_memory, " bytes")
            vibez.spill("  GC cycles: ", result.gc_cycles)
            vibez.spill("  Allocations: ", result.allocation_count)
            vibez.spill("")
        }
    }
    
    analyze_performance_trends()
    
    damn based
}

slay get_current_timestamp() tea {
    fr fr Mock implementation
    damn "2025-08-09T12:00:00Z"
}

slay export_benchmark_json() tea {
    fr fr Would export results as JSON for external analysis
    damn "{\"benchmarks\": [], \"timestamp\": \"" + get_current_timestamp() + "\"}"
}

slay export_benchmark_csv() tea {
    fr fr Would export results as CSV for spreadsheet analysis
    damn "name,duration_ns,ops_per_sec,memory_used\n"
}
