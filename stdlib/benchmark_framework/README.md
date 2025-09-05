# CURSED Benchmark Framework

A comprehensive benchmarking and performance analysis framework for the CURSED programming language. This framework provides tools for measuring, analyzing, and comparing the performance of CURSED code with statistical rigor and professional reporting capabilities.

## Features

- **Performance Measurement**: Accurate timing, memory usage, and CPU utilization tracking
- **Statistical Analysis**: Mean, median, standard deviation, percentiles, and variance calculation
- **Benchmark Comparison**: Compare performance across different implementations or versions
- **Regression Detection**: Automatic detection of performance regressions with configurable thresholds
- **Multiple Report Formats**: Text, JSON, and HTML report generation
- **Micro and Macro Benchmarking**: Support for both small function and large system benchmarks
- **Memory Profiling**: Track memory allocations and usage patterns
- **CLI Integration**: Command-line tool for automated benchmarking workflows

## Quick Start

### Basic Benchmark

```cursed
yeet "benchmark_framework"

# Initialize the framework
init_benchmark_framework()

# Create a simple benchmark
sus config = create_benchmark("fibonacci_test", 100, 1000)

# Define the function to benchmark
sus fib_func = slay() lit {
    sus n normie = 30
    sus result = fibonacci(n)
    damn based
}

# Run the benchmark
sus result = run_benchmark(config, fib_func)

# Display results
vibez.spill(format_benchmark_result(result))
```

### Micro-benchmarking

```cursed
yeet "benchmark_framework"

# Quick micro-benchmark for small operations
sus quick_func = slay() lit {
    sus x = 42 * 58 + 100
    damn based
}

sus result = micro_benchmark("arithmetic_test", 10000, quick_func)
vibez.spill(format_benchmark_result(result))
```

### Macro-benchmarking

```cursed
yeet "benchmark_framework"

# Benchmark larger operations
sus large_func = slay() lit {
    sus data []normie = []
    bestie i := 0; i < 100000; i++ {
        data = append(data, i * 2)
    }
    sus sorted_data = sort_array(data)
    damn based
}

sus result = macro_benchmark("sorting_test", large_func)
vibez.spill(format_benchmark_result(result))
```

## API Reference

### Core Functions

#### `create_benchmark(name tea, warmup normie, iterations normie) BenchmarkConfig`
Creates a new benchmark configuration.

**Parameters:**
- `name`: Benchmark identifier
- `warmup`: Number of warmup iterations
- `iterations`: Number of measurement iterations

**Returns:** `BenchmarkConfig` structure

#### `run_benchmark(config BenchmarkConfig, func slay() lit) BenchmarkResult`
Executes a complete benchmark with the given configuration.

**Parameters:**
- `config`: Benchmark configuration
- `func`: Function to benchmark

**Returns:** `BenchmarkResult` with timing and statistical data

#### `calculate_statistics(values []meal) StatisticalAnalysis`
Performs statistical analysis on measurement data.

**Parameters:**
- `values`: Array of measured values

**Returns:** `StatisticalAnalysis` with mean, median, std dev, etc.

### Comparison Functions

#### `compare_benchmarks(baseline BenchmarkResult, current BenchmarkResult) ComparisonResult`
Compares two benchmark results to detect performance changes.

**Parameters:**
- `baseline`: Previous benchmark result
- `current`: Current benchmark result

**Returns:** `ComparisonResult` with performance change analysis

#### `detect_regressions(current []BenchmarkResult, previous []BenchmarkResult) []ComparisonResult`
Detects performance regressions across multiple benchmarks.

**Parameters:**
- `current`: Current benchmark results
- `previous`: Previous benchmark results

**Returns:** Array of detected regressions

### Utility Functions

#### `micro_benchmark(name tea, iterations normie, func slay() lit) BenchmarkResult`
Quick micro-benchmark for small operations.

#### `macro_benchmark(name tea, func slay() lit) BenchmarkResult`
Macro-benchmark for larger operations with default settings.

#### `profile_memory_allocations(name tea, func slay() lit) BenchmarkResult`
Benchmark with detailed memory allocation tracking.

### Reporting Functions

#### `format_benchmark_result(result BenchmarkResult) tea`
Formats benchmark result as human-readable text.

#### `generate_html_report(results []BenchmarkResult) tea`
Generates comprehensive HTML report.

#### `save_results_to_file(results []BenchmarkResult, filename tea) lit`
Saves benchmark results to file for later analysis.

## Data Structures

### BenchmarkConfig
```cursed
be_like BenchmarkConfig = {
    name tea,
    warmup_iterations normie,
    measurement_iterations normie,
    timeout_seconds normie,
    memory_tracking lit,
    cpu_tracking lit
}
```

### BenchmarkResult
```cursed
be_like BenchmarkResult = {
    name tea,
    execution_times []meal,
    memory_usage []normie,
    cpu_usage []meal,
    mean_time meal,
    median_time meal,
    std_dev meal,
    min_time meal,
    max_time meal,
    total_iterations normie
}
```

### StatisticalAnalysis
```cursed
be_like StatisticalAnalysis = {
    mean meal,
    median meal,
    std_deviation meal,
    variance meal,
    min_value meal,
    max_value meal,
    percentile_95 meal,
    percentile_99 meal
}
```

## Command-Line Usage

The `cursed_bench` CLI tool provides command-line access to benchmarking functionality:

### Basic Usage
```bash
# Run a single benchmark
cursed_bench run program.💀 --name "my_benchmark" --iterations 1000

# Save results to file
cursed_bench run program.💀 --output results.json

# Generate HTML report
cursed_bench report results.json --output report.html

# Compare with baseline
cursed_bench compare baseline.json current.json
```

### Advanced Options
```bash
# Custom iterations and warmup
cursed_bench run program.💀 --warmup 500 --iterations 2000

# Enable memory and CPU tracking
cursed_bench run program.💀 --memory --cpu

# Set timeout
cursed_bench run program.💀 --timeout 120
```

## Examples

### Performance Regression Testing

```cursed
yeet "benchmark_framework"

# Run current implementation
sus current_config = create_benchmark("algorithm_v2", 50, 500)
sus current_result = run_benchmark(current_config, new_algorithm)

# Load previous results
sus previous_results = load_previous_results("baseline.json")

# Detect regressions
sus regressions = detect_regressions([current_result], previous_results)

lowkey len(regressions) > 0 {
    vibez.spill("⚠️  Performance regressions detected!")
    bestie i := 0; i < len(regressions); i++ {
        vibez.spill(format_comparison_result(regressions[i]))
    }
} else {
    vibez.spill("✅ No performance regressions detected")
}
```

### Memory Usage Analysis

```cursed
yeet "benchmark_framework"

sus memory_test = slay() lit {
    sus large_array []normie = []
    bestie i := 0; i < 1000000; i++ {
        large_array = append(large_array, i)
    }
    damn based
}

sus memory_result = profile_memory_allocations("memory_stress_test", memory_test)

vibez.spill("Memory usage statistics:")
vibez.spill("Average memory: " + normie_to_tea(average(memory_result.memory_usage)) + " bytes")
vibez.spill("Peak memory: " + normie_to_tea(max(memory_result.memory_usage)) + " bytes")
```

### Batch Benchmarking

```cursed
yeet "benchmark_framework"

# Register multiple benchmarks
sus sorting_config = create_benchmark("quicksort", 20, 100)
sus search_config = create_benchmark("binary_search", 50, 200)
sus hash_config = create_benchmark("hash_lookup", 100, 500)

register_benchmark(sorting_config)
register_benchmark(search_config)
register_benchmark(hash_config)

# Run all benchmarks
sus all_results = run_all_benchmarks()

# Generate comprehensive report
sus html_report = generate_html_report(all_results)
save_results_to_file(all_results, "benchmark_results.json")
dropz.write_file("benchmark_report.html", html_report)
```

## Performance Best Practices

### Accurate Measurements
1. **Use adequate warmup iterations** to stabilize performance
2. **Run sufficient measurement iterations** for statistical significance
3. **Control system load** during benchmarking
4. **Use consistent hardware** for comparative benchmarks

### Statistical Significance
1. **Monitor standard deviation** - high values indicate inconsistent performance
2. **Use percentiles** to understand performance distribution
3. **Set appropriate regression thresholds** (typically 3-10%)
4. **Run multiple benchmark sessions** to verify results

### Memory Benchmarking
1. **Enable GC before measurements** to get consistent baseline
2. **Monitor both allocation and usage** patterns
3. **Consider memory fragmentation** in long-running benchmarks
4. **Profile both heap and stack** usage when possible

## Integration with Testing

The benchmark framework integrates seamlessly with the CURSED testing framework:

```cursed
yeet "testz"
yeet "benchmark_framework"

test_start("Performance regression tests")

sus benchmark_func = slay() lit {
    # Your implementation here
    damn based
}

sus result = micro_benchmark("performance_test", 1000, benchmark_func)

# Assert performance requirements
assert_true(result.mean_time < 0.001)  # Must complete in < 1ms
assert_true(result.std_dev < 0.0001)   # Must be consistent

print_test_summary()
```

## Advanced Features

### Custom Statistical Analysis
```cursed
# Define custom analysis function
slay custom_analysis(values []meal) CustomStats {
    sus stats = calculate_statistics(values)
    # Add custom metrics
    damn CustomStats{
        standard_stats: stats,
        efficiency_score: calculate_efficiency(values),
        stability_index: calculate_stability(values)
    }
}
```

### Benchmark Profiling Integration
```cursed
# Profile specific code sections
sus profiler = create_profiler()
profiler.start("critical_section")
critical_algorithm()
profiler.end("critical_section")

sus profile_data = profiler.get_results()
vibez.spill("Critical section time: " + meal_to_tea(profile_data.critical_section))
```

## Contributing

To contribute to the benchmark framework:

1. Run the comprehensive test suite:
   ```bash
   cargo run --bin cursed stdlib/benchmark_framework/test_benchmark_framework.💀
   ```

2. Test both interpretation and compilation modes:
   ```bash
   cargo run --bin cursed stdlib/benchmark_framework/test_benchmark_framework.💀
   cargo run --bin cursed -- compile stdlib/benchmark_framework/test_benchmark_framework.💀
   ./test_benchmark_framework
   ```

3. Verify CLI tool functionality:
   ```bash
   cargo build --bin cursed_bench
   ./target/debug/cursed_bench run examples/benchmark_example.💀
   ```

## Performance Characteristics

The benchmark framework itself is optimized for minimal overhead:

- **Timing precision**: Nanosecond resolution where supported
- **Memory overhead**: < 1MB for typical benchmark runs
- **Statistical calculations**: O(n log n) for sorting-based operations
- **Report generation**: Streaming for large datasets

## License

This benchmark framework is part of the CURSED programming language project and follows the same licensing terms.
