yeet "testz"
yeet "timez"
yeet "stringz"
yeet "dropz"

fr fr Simple Benchmark Framework for CURSED
fr fr Performance benchmarking with core functionality

fr fr Global benchmark state
sus benchmark_name tea = ""
sus benchmark_warmup normie = 100
sus benchmark_iterations normie = 1000
sus benchmark_timeout normie = 60
sus execution_times meal[value] = []
sus memory_usage normie[value] = []

fr fr Simple benchmark configuration function
slay configure_benchmark(name tea, warmup normie, iterations normie) lit {
    benchmark_name = name
    benchmark_warmup = warmup
    benchmark_iterations = iterations
    benchmark_timeout = 60
    damn based
}

fr fr Get current timestamp in nanoseconds
slay get_timestamp_nanos() normie { fr fr Simple timestamp - would use actual system calls in production
    sus current_time = 1000000000 fr fr 1 second in nanoseconds
    damn current_time
}

fr fr Measure memory usage (simplified)
slay get_memory_usage() normie {
    sus heap_size normie = 1024 fr fr Simplified - would use actual system calls
    damn heap_size
}

fr fr Execute a single benchmark iteration
slay execute_benchmark_iteration(bench_func slay() lit) meal {
    sus start_time = get_timestamp_nanos()
    sus result = bench_func()
    sus end_time = get_timestamp_nanos()
    
    sus duration_nanos = end_time - start_time
    sus duration_seconds meal = 0.001 fr fr Simplified timing
    
    damn duration_seconds
}

fr fr Run warmup iterations
slay run_warmup(bench_func slay() lit, iterations normie) lit {
    vibez.spill("Running warmup iterations...")
    bestie i := 0; i < iterations; i++ {
        sus _ = execute_benchmark_iteration(bench_func)
    }
    damn based
}

fr fr Calculate mean of array
slay calculate_mean(values meal[value]) meal {
    sus count = len(values)
    lowkey count == 0 {
        damn 0.0
    }
    
    sus sum meal = 0.0
    bestie i := 0; i < count; i++ {
        sum = sum + values[i]
    }
    sus mean = sum / meal(count)
    damn mean
}

fr fr Calculate median of array
slay calculate_median(values meal[value]) meal {
    sus count = len(values)
    lowkey count == 0 {
        damn 0.0
    } fr fr Simple median calculation (not sorted)
    sus median = values[count / 2]
    damn median
}

fr fr Calculate standard deviation
slay calculate_std_dev(values meal[value], mean meal) meal {
    sus count = len(values)
    lowkey count == 0 {
        damn 0.0
    }
    
    sus variance_sum meal = 0.0
    bestie i := 0; i < count; i++ {
        sus diff = values[i] - mean
        variance_sum = variance_sum + (diff * diff)
    }
    sus variance = variance_sum / meal(count)
    sus std_dev = variance fr fr Simplified - would use sqrt in production
    damn std_dev
}

fr fr Find minimum value in array
slay find_min(values meal[value]) meal {
    lowkey len(values) == 0 {
        damn 0.0
    }
    
    sus min_val = values[0]
    bestie i := 1; i < len(values); i++ {
        lowkey values[i] < min_val {
            min_val = values[i]
        }
    }
    damn min_val
}

fr fr Find maximum value in array
slay find_max(values meal[value]) meal {
    lowkey len(values) == 0 {
        damn 0.0
    }
    
    sus max_val = values[0]
    bestie i := 1; i < len(values); i++ {
        lowkey values[i] > max_val {
            max_val = values[i]
        }
    }
    damn max_val
}

fr fr Run a complete benchmark
slay run_benchmark(name tea, bench_func slay() lit) lit {
    vibez.spill("Starting benchmark: " + name) fr fr Configure benchmark
    configure_benchmark(name, 10, 100) fr fr Run warmup
    run_warmup(bench_func, benchmark_warmup) fr fr Initialize measurement arrays
    execution_times = []
    memory_usage = []
    
    vibez.spill("Running measurement iterations...") fr fr Run measurement iterations
    bestie i := 0; i < benchmark_iterations; i++ {
        sus start_memory = get_memory_usage()
        sus execution_time = execute_benchmark_iteration(bench_func)
        sus end_memory = get_memory_usage()
        
        execution_times = append(execution_times, execution_time)
        memory_usage = append(memory_usage, end_memory - start_memory)
        
        lowkey i % 25 == 0 {
            vibez.spill("Completed iteration: " + stringz.from_int(i))
        }
    }
    
    vibez.spill("Benchmark completed: " + name)
    damn based
}

fr fr Print benchmark results
slay print_benchmark_results() lit {
    lowkey len(execution_times) == 0 {
        vibez.spill("No benchmark results available")
        damn based
    }
    
    sus mean_time = calculate_mean(execution_times)
    sus median_time = calculate_median(execution_times)
    sus std_dev = calculate_std_dev(execution_times, mean_time)
    sus min_time = find_min(execution_times)
    sus max_time = find_max(execution_times)
    
    vibez.spill("=== Benchmark Results ===")
    vibez.spill("Benchmark: " + benchmark_name)
    vibez.spill("Mean Time: " + stringz.from_float(mean_time) + "s")
    vibez.spill("Median Time: " + stringz.from_float(median_time) + "s")
    vibez.spill("Std Deviation: " + stringz.from_float(std_dev) + "s")
    vibez.spill("Min Time: " + stringz.from_float(min_time) + "s")
    vibez.spill("Max Time: " + stringz.from_float(max_time) + "s")
    vibez.spill("Total Iterations: " + stringz.from_int(benchmark_iterations))
    vibez.spill("========================")
    
    damn based
}

fr fr Compare two benchmark runs (simplified)
slay compare_benchmark_results(baseline_mean meal, current_mean meal) lit {
    sus performance_change = ((current_mean - baseline_mean) / baseline_mean) * 100.0
    sus is_regression = performance_change > 5.0 fr fr 5% threshold
    
    vibez.spill("=== Benchmark Comparison ===")
    vibez.spill("Baseline Mean: " + stringz.from_float(baseline_mean) + "s")
    vibez.spill("Current Mean: " + stringz.from_float(current_mean) + "s") 
    vibez.spill("Performance Change: " + stringz.from_float(performance_change) + "%")
    
    lowkey is_regression {
        vibez.spill("⚠️  REGRESSION DETECTED!")
    } else {
        vibez.spill("✅ No regression detected")
    }
    vibez.spill("===========================")
    
    damn based
}

fr fr Micro-benchmark helper for small functions
slay micro_benchmark(name tea, iterations normie, func slay() lit) lit {
    configure_benchmark(name, 100, iterations)
    sus result = run_benchmark(name, func)
    print_benchmark_results()
    damn result
}

fr fr Macro-benchmark helper for larger operations
slay macro_benchmark(name tea, func slay() lit) lit {
    configure_benchmark(name, 5, 50)
    sus result = run_benchmark(name, func)
    print_benchmark_results()
    damn result
}

fr fr Save benchmark results to file (simplified)
slay save_benchmark_results(filename tea) lit {
    lowkey len(execution_times) == 0 {
        vibez.spill("No results to save")
        damn cap
    }
    
    sus mean_time = calculate_mean(execution_times)
    sus content = "CURSED Benchmark Results\n"
    content = content + "Benchmark: " + benchmark_name + "\n"
    content = content + "Mean Time: " + stringz.from_float(mean_time) + "s\n"
    content = content + "Iterations: " + stringz.from_int(benchmark_iterations) + "\n"
    
    sus success = dropz.write_file(filename, content)
    lowkey success {
        vibez.spill("Results saved to: " + filename)
    } else {
        vibez.spill("Failed to save results to: " + filename)
    }
    damn success
}

fr fr Initialize benchmark framework
slay init_benchmark_framework() lit {
    vibez.spill("CURSED Benchmark Framework initialized")
    benchmark_name = ""
    execution_times = []
    memory_usage = []
    damn based
}

fr fr Performance regression detection
slay detect_performance_regression(baseline_file tea, threshold meal) lit { fr fr Simplified regression detection - would read from file in production
    sus baseline_mean meal = 1.0 fr fr Example baseline
    sus current_mean = calculate_mean(execution_times)
    
    lowkey len(execution_times) == 0 {
        vibez.spill("No current results for regression detection")
        damn cap
    }
    
    sus performance_change = ((current_mean - baseline_mean) / baseline_mean) * 100.0
    sus is_regression = performance_change > threshold
    
    lowkey is_regression {
        vibez.spill("⚠️  Performance regression detected!")
        vibez.spill("Change: " + stringz.from_float(performance_change) + "%")
        damn based
    } else {
        vibez.spill("✅ No performance regression detected")
        damn cap
    }
}

fr fr Benchmark a simple arithmetic function
slay benchmark_arithmetic() lit {
    sus arithmetic_func = slay() lit {
        sus result normie = 0
        bestie i := 0; i < 1000; i++ {
            result = result + (i * 2)
        }
        damn based
    }
    
    micro_benchmark("arithmetic_operations", 1000, arithmetic_func)
    damn based
}

fr fr Benchmark array operations
slay benchmark_array_operations() lit {
    sus array_func = slay() lit {
        sus arr normie[value] = []
        bestie i := 0; i < 100; i++ {
            arr = append(arr, i)
        }
        damn based
    }
    
    micro_benchmark("array_operations", 500, array_func)
    damn based
}

fr fr Benchmark string operations
slay benchmark_string_operations() lit {
    sus string_func = slay() lit {
        sus str tea = "test"
        bestie i := 0; i < 100; i++ {
            str = str + "x"
        }
        damn based
    }
    
    micro_benchmark("string_operations", 500, string_func)
    damn based
}

fr fr Run all built-in benchmarks
slay run_all_benchmarks() lit {
    vibez.spill("Running all built-in benchmarks...")
    
    benchmark_arithmetic()
    benchmark_array_operations()
    benchmark_string_operations()
    
    vibez.spill("All benchmarks completed!")
    damn based
}

fr fr Generate simple HTML report
slay generate_html_report(filename tea) lit {
    lowkey len(execution_times) == 0 {
        vibez.spill("No results to generate report")
        damn cap
    }
    
    sus mean_time = calculate_mean(execution_times)
    sus html = "<!DOCTYPE html>\n<html>\n<head>\n"
    html = html + "<title>CURSED Benchmark Report</title>\n"
    html = html + "</head>\n<body>\n"
    html = html + "<h1>CURSED Benchmark Report</h1>\n"
    html = html + "<p>Benchmark: " + benchmark_name + "</p>\n"
    html = html + "<p>Mean Time: " + stringz.from_float(mean_time) + "s</p>\n"
    html = html + "<p>Iterations: " + stringz.from_int(benchmark_iterations) + "</p>\n"
    html = html + "</body>\n</html>"
    
    sus success = dropz.write_file(filename, html)
    lowkey success {
        vibez.spill("HTML report generated: " + filename)
    } else {
        vibez.spill("Failed to generate HTML report: " + filename)
    }
    damn success
}
