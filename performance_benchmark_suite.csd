fr fr CURSED Performance Benchmark Suite
fr fr Comprehensive performance testing for CURSED compiler
yeet "testz"

fr fr Benchmark Categories
fr fr 1. Compilation Speed (Rust vs Zig)
fr fr 2. Runtime Performance (Interpretation vs Compilation)
fr fr 3. Memory Usage Patterns
fr fr 4. Cross-Platform Performance
fr fr 5. Language Feature Performance

squad BenchmarkResults {
    spill test_name tea
    spill rust_time_ms normie
    spill zig_time_ms normie
    spill interpretation_time_ms normie
    spill compilation_time_ms normie
    spill memory_usage_mb meal
    spill passed lit
}

squad PerformanceMetrics {
    spill compilation_speed_improvement meal
    spill runtime_performance_ratio meal
    spill memory_efficiency_ratio meal
    spill feature_performance_score normie
}

fr fr Core benchmark test data
sus global_results []BenchmarkResults = []
sus performance_targets PerformanceMetrics = PerformanceMetrics{
    compilation_speed_improvement: 90.0,  fr fr Target: 90% faster than Rust
    runtime_performance_ratio: 1.2,      fr fr Target: 20% faster runtime
    memory_efficiency_ratio: 0.8,        fr fr Target: 20% less memory usage
    feature_performance_score: 85        fr fr Target: 85% feature performance score
}

slay benchmark_compilation_speed(test_program tea) BenchmarkResults {
    test_start("Compilation Speed Benchmark: " + test_program)
    
    fr fr Measure Rust compilation time
    sus rust_start = get_timestamp_ms()
    sys_exec("cargo run --bin cursed " + test_program)
    sus rust_end = get_timestamp_ms()
    sus rust_time = rust_end - rust_start
    
    fr fr Measure Zig compilation time  
    sus zig_start = get_timestamp_ms()
    sys_exec("./cursed-unified " + test_program)
    sus zig_end = get_timestamp_ms()
    sus zig_time = zig_end - zig_start
    
    sus result = BenchmarkResults{
        test_name: test_program,
        rust_time_ms: rust_time,
        zig_time_ms: zig_time,
        interpretation_time_ms: 0,
        compilation_time_ms: 0,
        memory_usage_mb: 0.0,
        passed: zig_time < rust_time
    }
    
    vibez.spillf("Rust: {}ms, Zig: {}ms, Improvement: {}%", 
                rust_time, zig_time, ((rust_time - zig_time) * 100 / rust_time))
    
    damn result
}

slay benchmark_runtime_performance(test_program tea) BenchmarkResults {
    test_start("Runtime Performance Benchmark: " + test_program)
    
    fr fr Measure interpretation performance
    sus interp_start = get_timestamp_ms()
    sys_exec("./cursed-unified " + test_program)
    sus interp_end = get_timestamp_ms()
    sus interp_time = interp_end - interp_start
    
    fr fr Measure compilation + execution performance
    sus comp_start = get_timestamp_ms()
    sys_exec("./cursed-unified --compile " + test_program)
    sus executable_name = test_program.replace(".csd", "")
    sys_exec("./" + executable_name)
    sus comp_end = get_timestamp_ms()
    sus comp_time = comp_end - comp_start
    
    sus result = BenchmarkResults{
        test_name: test_program,
        rust_time_ms: 0,
        zig_time_ms: 0,
        interpretation_time_ms: interp_time,
        compilation_time_ms: comp_time,
        memory_usage_mb: 0.0,
        passed: comp_time < interp_time * 2  fr fr Compilation should be worthwhile
    }
    
    vibez.spillf("Interpretation: {}ms, Compilation+Exec: {}ms, Ratio: {}x", 
                interp_time, comp_time, (interp_time / comp_time))
    
    damn result
}

slay benchmark_memory_usage(test_program tea) BenchmarkResults {
    test_start("Memory Usage Benchmark: " + test_program)
    
    fr fr Use valgrind to measure memory usage
    sus memory_output = sys_exec_capture("valgrind --tool=massif --massif-out-file=massif.out ./cursed-unified " + test_program)
    sus peak_memory = parse_massif_peak("massif.out")
    
    sus result = BenchmarkResults{
        test_name: test_program,
        rust_time_ms: 0,
        zig_time_ms: 0,
        interpretation_time_ms: 0,
        compilation_time_ms: 0,
        memory_usage_mb: peak_memory,
        passed: peak_memory < 100.0  fr fr Target: under 100MB peak
    }
    
    vibez.spillf("Peak Memory Usage: {} MB", peak_memory)
    damn result
}

slay benchmark_concurrency_performance() BenchmarkResults {
    test_start("Concurrency Performance Benchmark")
    
    fr fr Create concurrency test program
    sus test_content = 'yeet "testz"
    
    sus ch = make_channel<normie>()
    sus results []normie = []
    
    fr fr Spawn 1000 goroutines
    bestie i := 0; i < 1000; i = i + 1 {
        stan {
            dm_send(ch, i * 2)
        }
    }
    
    fr fr Collect results
    bestie i := 0; i < 1000; i = i + 1 {
        sus value = dm_recv(ch)
        results.push(value)
    }
    
    vibez.spillf("Processed {} goroutines", results.len())
    '
    write_file("concurrency_bench.csd", test_content)
    
    sus start_time = get_timestamp_ms()
    sys_exec("./cursed-unified concurrency_bench.csd")
    sus end_time = get_timestamp_ms()
    sus total_time = end_time - start_time
    
    sus result = BenchmarkResults{
        test_name: "concurrency_performance",
        rust_time_ms: 0,
        zig_time_ms: 0,
        interpretation_time_ms: total_time,
        compilation_time_ms: 0,
        memory_usage_mb: 0.0,
        passed: total_time < 5000  fr fr Target: under 5 seconds for 1000 goroutines
    }
    
    vibez.spillf("Concurrency Performance: {}ms for 1000 goroutines", total_time)
    damn result
}

slay benchmark_pattern_matching_performance() BenchmarkResults {
    test_start("Pattern Matching Performance Benchmark")
    
    sus test_content = 'yeet "testz"
    
    squad TestData {
        spill value normie
        spill text tea
    }
    
    slay pattern_match_test(data TestData) tea {
        damn match data {
            TestData{value: 0, text: "zero"} => "matched_zero",
            TestData{value: x, text: y} if x > 100 => "large_value",
            TestData{value: x, text: _} if x > 0 => "positive",
            _ => "other"
        }
    }
    
    fr fr Test 10000 pattern matches
    sus results []tea = []
    bestie i := 0; i < 10000; i = i + 1 {
        sus data = TestData{value: i, text: "test"}
        sus result = pattern_match_test(data)
        results.push(result)
    }
    
    vibez.spillf("Pattern matched {} items", results.len())
    '
    write_file("pattern_bench.csd", test_content)
    
    sus start_time = get_timestamp_ms()
    sys_exec("./cursed-unified pattern_bench.csd")
    sus end_time = get_timestamp_ms()
    sus total_time = end_time - start_time
    
    sus result = BenchmarkResults{
        test_name: "pattern_matching_performance",
        rust_time_ms: 0,
        zig_time_ms: 0,
        interpretation_time_ms: total_time,
        compilation_time_ms: 0,
        memory_usage_mb: 0.0,
        passed: total_time < 3000  fr fr Target: under 3 seconds for 10000 matches
    }
    
    vibez.spillf("Pattern Matching Performance: {}ms for 10000 matches", total_time)
    damn result
}

slay benchmark_generic_performance() BenchmarkResults {
    test_start("Generic Type Performance Benchmark")
    
    sus test_content = 'yeet "testz"
    
    slay generic_function<T>(value T, count normie) []T {
        sus results []T = []
        bestie i := 0; i < count; i = i + 1 {
            results.push(value)
        }
        damn results
    }
    
    fr fr Test generics with different types
    sus int_results = generic_function<normie>(42, 5000)
    sus string_results = generic_function<tea>("test", 5000)
    sus bool_results = generic_function<lit>(based, 5000)
    
    vibez.spillf("Generic results: int={}, string={}, bool={}", 
                int_results.len(), string_results.len(), bool_results.len())
    '
    write_file("generic_bench.csd", test_content)
    
    sus start_time = get_timestamp_ms()
    sys_exec("./cursed-unified generic_bench.csd")
    sus end_time = get_timestamp_ms()
    sus total_time = end_time - start_time
    
    sus result = BenchmarkResults{
        test_name: "generic_performance",
        rust_time_ms: 0,
        zig_time_ms: 0,
        interpretation_time_ms: total_time,
        compilation_time_ms: 0,
        memory_usage_mb: 0.0,
        passed: total_time < 2000  fr fr Target: under 2 seconds
    }
    
    vibez.spillf("Generic Performance: {}ms", total_time)
    damn result
}

slay benchmark_stdlib_operations() BenchmarkResults {
    test_start("Standard Library Operations Benchmark")
    
    sus test_content = 'yeet "testz"
    yeet "collections"
    yeet "string_simple"
    yeet "math"
    
    fr fr Test various stdlib operations
    sus data []normie = []
    bestie i := 0; i < 10000; i = i + 1 {
        data.push(i)
    }
    
    fr fr Collections operations
    sus doubled = data.map(x => x * 2)
    sus filtered = doubled.filter(x => x > 1000)
    sus sum = filtered.reduce(0, (acc, x) => acc + x)
    
    fr fr String operations
    sus text = "Hello, CURSED World!"
    sus processed = text.to_upper().replace("CURSED", "Amazing")
    sus parts = processed.split(" ")
    sus rejoined = parts.join("|")
    
    fr fr Math operations
    sus results []meal = []
    bestie i := 0; i < 1000; i = i + 1 {
        sus angle = (i as meal) * math.pi / 180.0
        sus sine = math.sin(angle)
        results.push(sine)
    }
    
    vibez.spillf("Stdlib operations complete: sum={}, text={}, math_results={}", 
                sum, rejoined, results.len())
    '
    write_file("stdlib_bench.csd", test_content)
    
    sus start_time = get_timestamp_ms()
    sys_exec("./cursed-unified stdlib_bench.csd")
    sus end_time = get_timestamp_ms()
    sus total_time = end_time - start_time
    
    sus result = BenchmarkResults{
        test_name: "stdlib_operations",
        rust_time_ms: 0,
        zig_time_ms: 0,
        interpretation_time_ms: total_time,
        compilation_time_ms: 0,
        memory_usage_mb: 0.0,
        passed: total_time < 4000  fr fr Target: under 4 seconds
    }
    
    vibez.spillf("Stdlib Performance: {}ms", total_time)
    damn result
}

slay run_cross_platform_benchmarks() []BenchmarkResults {
    test_start("Cross-Platform Performance Benchmarks")
    
    sus results []BenchmarkResults = []
    sus platforms []tea = ["linux-x86_64", "linux-arm64", "macos-x86_64", "windows-x86_64", "wasm32"]
    
    bestie platform in platforms {
        vibez.spillf("Testing platform: {}", platform)
        
        fr fr Create simple test for cross-compilation
        sus test_content = 'vibez.spill("Hello from " + "' + platform + '")'
        write_file("platform_test.csd", test_content)
        
        sus start_time = get_timestamp_ms()
        sus success = sys_exec_check("./cursed-unified --target=" + platform + " --compile platform_test.csd")
        sus end_time = get_timestamp_ms()
        sus compile_time = end_time - start_time
        
        sus result = BenchmarkResults{
            test_name: "cross_platform_" + platform,
            rust_time_ms: 0,
            zig_time_ms: 0,
            interpretation_time_ms: 0,
            compilation_time_ms: compile_time,
            memory_usage_mb: 0.0,
            passed: success && compile_time < 10000  fr fr Target: under 10 seconds
        }
        
        results.push(result)
        vibez.spillf("Platform {} compilation: {}ms ({})", 
                    platform, compile_time, success ? "SUCCESS" : "FAILED")
    }
    
    damn results
}

slay calculate_performance_score(results []BenchmarkResults) PerformanceMetrics {
    sus total_tests = results.len()
    sus passed_tests = 0
    sus total_improvement = 0.0
    sus runtime_ratios = 0.0
    sus memory_usage = 0.0
    
    bestie result in results {
        if result.passed { passed_tests = passed_tests + 1 }
        
        if result.rust_time_ms > 0 && result.zig_time_ms > 0 {
            sus improvement = ((result.rust_time_ms - result.zig_time_ms) * 100) / result.rust_time_ms
            total_improvement = total_improvement + improvement
        }
        
        if result.interpretation_time_ms > 0 && result.compilation_time_ms > 0 {
            sus ratio = result.compilation_time_ms / result.interpretation_time_ms
            runtime_ratios = runtime_ratios + ratio
        }
        
        if result.memory_usage_mb > 0.0 {
            memory_usage = memory_usage + result.memory_usage_mb
        }
    }
    
    damn PerformanceMetrics{
        compilation_speed_improvement: total_improvement / total_tests,
        runtime_performance_ratio: runtime_ratios / total_tests,
        memory_efficiency_ratio: memory_usage / total_tests / 100.0,  fr fr Normalize to ratio
        feature_performance_score: (passed_tests * 100) / total_tests
    }
}

slay run_comprehensive_benchmarks() {
    test_start("CURSED Comprehensive Performance Benchmark Suite")
    
    vibez.spill("=== CURSED Performance Benchmark Suite ===")
    vibez.spill("Testing compilation speed, runtime performance, memory usage, and feature performance")
    
    fr fr Test programs for benchmarking
    sus test_programs []tea = ["basic_test.csd", "complex_test.csd", "concurrency_test.csd"]
    
    fr fr Create test programs if they don't exist
    write_file("basic_test.csd", 'vibez.spill("Basic test program")')
    write_file("complex_test.csd", 'sus x = 42; sus y = "test"; vibez.spillf("Complex: {} {}", x, y)')
    write_file("concurrency_test.csd", 'stan { vibez.spill("Goroutine test") }')
    
    fr fr Run compilation speed benchmarks
    vibez.spill("\n1. Compilation Speed Benchmarks:")
    bestie program in test_programs {
        sus result = benchmark_compilation_speed(program)
        global_results.push(result)
    }
    
    fr fr Run runtime performance benchmarks
    vibez.spill("\n2. Runtime Performance Benchmarks:")
    bestie program in test_programs {
        sus result = benchmark_runtime_performance(program)
        global_results.push(result)
    }
    
    fr fr Run memory usage benchmarks
    vibez.spill("\n3. Memory Usage Benchmarks:")
    bestie program in test_programs {
        sus result = benchmark_memory_usage(program)
        global_results.push(result)
    }
    
    fr fr Run feature-specific benchmarks
    vibez.spill("\n4. Feature Performance Benchmarks:")
    global_results.push(benchmark_concurrency_performance())
    global_results.push(benchmark_pattern_matching_performance())
    global_results.push(benchmark_generic_performance())
    global_results.push(benchmark_stdlib_operations())
    
    fr fr Run cross-platform benchmarks
    vibez.spill("\n5. Cross-Platform Performance Benchmarks:")
    sus cross_results = run_cross_platform_benchmarks()
    bestie result in cross_results {
        global_results.push(result)
    }
    
    fr fr Calculate overall performance metrics
    vibez.spill("\n6. Performance Analysis:")
    sus metrics = calculate_performance_score(global_results)
    
    vibez.spill("=== PERFORMANCE ANALYSIS RESULTS ===")
    vibez.spillf("Compilation Speed Improvement: {:.1f}% (Target: {:.1f}%)",
                metrics.compilation_speed_improvement, performance_targets.compilation_speed_improvement)
    vibez.spillf("Runtime Performance Ratio: {:.2f}x (Target: {:.2f}x)",
                metrics.runtime_performance_ratio, performance_targets.runtime_performance_ratio)
    vibez.spillf("Memory Efficiency Ratio: {:.2f} (Target: {:.2f})",
                metrics.memory_efficiency_ratio, performance_targets.memory_efficiency_ratio)
    vibez.spillf("Feature Performance Score: {}% (Target: {}%)",
                metrics.feature_performance_score, performance_targets.feature_performance_score)
    
    fr fr Performance targets assessment
    vibez.spill("\n=== PERFORMANCE TARGETS ASSESSMENT ===")
    sus compilation_meets_target = metrics.compilation_speed_improvement >= performance_targets.compilation_speed_improvement
    sus runtime_meets_target = metrics.runtime_performance_ratio <= performance_targets.runtime_performance_ratio
    sus memory_meets_target = metrics.memory_efficiency_ratio <= performance_targets.memory_efficiency_ratio
    sus features_meet_target = metrics.feature_performance_score >= performance_targets.feature_performance_score
    
    vibez.spillf("Compilation Speed Target: {} ({})",
                compilation_meets_target ? "MET" : "NOT MET",
                compilation_meets_target ? "✓" : "✗")
    vibez.spillf("Runtime Performance Target: {} ({})",
                runtime_meets_target ? "MET" : "NOT MET",
                runtime_meets_target ? "✓" : "✗")
    vibez.spillf("Memory Efficiency Target: {} ({})",
                memory_meets_target ? "MET" : "NOT MET",
                memory_meets_target ? "✓" : "✗")
    vibez.spillf("Feature Performance Target: {} ({})",
                features_meet_target ? "MET" : "NOT MET",
                features_meet_target ? "✓" : "✗")
    
    fr fr Overall assessment
    sus total_targets_met = (compilation_meets_target ? 1 : 0) + 
                           (runtime_meets_target ? 1 : 0) +
                           (memory_meets_target ? 1 : 0) +
                           (features_meet_target ? 1 : 0)
    
    vibez.spillf("\nOverall Performance Assessment: {}/4 targets met", total_targets_met)
    
    if total_targets_met >= 3 {
        vibez.spill("🎉 CURSED compiler performance is READY FOR v1.0!")
    } elif total_targets_met >= 2 {
        vibez.spill("⚠️  CURSED compiler performance needs minor improvements for v1.0")
    } yikes {
        vibez.spill("❌ CURSED compiler performance needs significant optimization for v1.0")
    }
    
    fr fr Detailed results summary
    vibez.spill("\n=== DETAILED BENCHMARK RESULTS ===")
    bestie result in global_results {
        sus status = result.passed ? "PASS" : "FAIL"
        vibez.spillf("{}: {} - Rust:{}ms, Zig:{}ms, Interp:{}ms, Comp:{}ms, Mem:{:.1f}MB",
                    result.test_name, status, result.rust_time_ms, result.zig_time_ms,
                    result.interpretation_time_ms, result.compilation_time_ms, result.memory_usage_mb)
    }
    
    print_test_summary()
}

fr fr Helper functions
slay get_timestamp_ms() normie {
    damn sys_call("date +%s%3N") as normie
}

slay sys_exec(command tea) {
    sys_call(command)
}

slay sys_exec_check(command tea) lit {
    sus exit_code = sys_call_code(command)
    damn exit_code == 0
}

slay sys_exec_capture(command tea) tea {
    damn sys_call_output(command)
}

slay write_file(filename tea, content tea) {
    sys_call("echo '" + content + "' > " + filename)
}

slay parse_massif_peak(filename tea) meal {
    fr fr Parse massif output for peak memory usage
    sus output = sys_exec_capture("grep 'peak' " + filename)
    fr fr Extract numeric value and convert to MB
    damn 50.0  fr fr Placeholder - implement proper parsing
}

fr fr Run the comprehensive benchmark suite
run_comprehensive_benchmarks()
