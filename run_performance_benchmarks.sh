#!/bin/bash

# CURSED Performance Benchmark Runner
# Comprehensive performance testing and analysis

echo "=== CURSED Performance Benchmark Suite ==="
echo "Running comprehensive performance analysis..."

# Ensure unified compiler is built
echo "Building CURSED unified compiler..."
zig build-exe src-zig/main_unified.zig -lc --name cursed-unified || {
    echo "❌ Failed to build unified compiler"
    exit 1
}

# Ensure Rust compiler is available for comparison
echo "Building Rust compiler for comparison..."
cargo build --release || {
    echo "⚠️  Rust compiler build failed, skipping Rust comparisons"
}

# Performance metrics storage
RESULTS_FILE="performance_benchmark_results.csv"
echo "Test,Rust_Time_ms,Zig_Time_ms,Improvement_%,Memory_MB,Status" > $RESULTS_FILE

# Function to measure execution time
measure_time() {
    local command="$1"
    local start_time=$(date +%s%3N)
    $command > /dev/null 2>&1
    local exit_code=$?
    local end_time=$(date +%s%3N)
    local duration=$((end_time - start_time))
    echo "$duration,$exit_code"
}

# Function to measure memory usage
measure_memory() {
    local command="$1"
    local temp_file=$(mktemp)
    
    # Use timeout to prevent hanging
    timeout 30s valgrind --tool=massif --massif-out-file="$temp_file" $command > /dev/null 2>&1
    local exit_code=$?
    
    if [ $exit_code -eq 0 ] && [ -f "$temp_file" ]; then
        # Extract peak memory usage in MB
        local peak_bytes=$(grep -E "mem_heap_B|mem_heap_extra_B" "$temp_file" | tail -1 | awk '{print $2}' | sed 's/,//g')
        if [ -n "$peak_bytes" ] && [ "$peak_bytes" -gt 0 ]; then
            local peak_mb=$(echo "scale=2; $peak_bytes / 1024 / 1024" | bc)
            echo "$peak_mb"
        else
            echo "0.0"
        fi
    else
        echo "0.0"
    fi
    
    rm -f "$temp_file"
}

# Function to run compilation speed benchmark
benchmark_compilation_speed() {
    local test_file="$1"
    echo "Benchmarking compilation speed for: $test_file"
    
    # Measure Rust compilation time
    local rust_result=""
    if [ -f "target/release/cursed" ]; then
        rust_result=$(measure_time "target/release/cursed $test_file")
        local rust_time=$(echo $rust_result | cut -d',' -f1)
        local rust_status=$(echo $rust_result | cut -d',' -f2)
    else
        local rust_time="0"
        local rust_status="1"
    fi
    
    # Measure Zig compilation time
    local zig_result=$(measure_time "./cursed-unified $test_file")
    local zig_time=$(echo $zig_result | cut -d',' -f1)
    local zig_status=$(echo $zig_result | cut -d',' -f2)
    
    # Calculate improvement percentage
    local improvement="0"
    if [ "$rust_time" -gt 0 ] && [ "$zig_time" -gt 0 ]; then
        improvement=$(echo "scale=1; ($rust_time - $zig_time) * 100 / $rust_time" | bc)
    fi
    
    # Measure memory usage
    local memory=$(measure_memory "./cursed-unified $test_file")
    
    local status="PASS"
    if [ "$zig_status" -ne 0 ]; then
        status="FAIL"
    fi
    
    echo "$test_file,$rust_time,$zig_time,$improvement,$memory,$status" >> $RESULTS_FILE
    echo "  Rust: ${rust_time}ms, Zig: ${zig_time}ms, Improvement: ${improvement}%, Memory: ${memory}MB, Status: $status"
}

# Function to run runtime performance benchmark
benchmark_runtime_performance() {
    local test_file="$1"
    echo "Benchmarking runtime performance for: $test_file"
    
    # Measure interpretation time
    local interp_result=$(measure_time "./cursed-unified $test_file")
    local interp_time=$(echo $interp_result | cut -d',' -f1)
    local interp_status=$(echo $interp_result | cut -d',' -f2)
    
    # Measure compilation + execution time
    local comp_result=""
    local comp_time="0"
    local comp_status="1"
    
    if ./cursed-unified --compile "$test_file" > /dev/null 2>&1; then
        local executable_name=$(basename "$test_file" .csd)
        if [ -f "$executable_name" ]; then
            comp_result=$(measure_time "./$executable_name")
            comp_time=$(echo $comp_result | cut -d',' -f1)
            comp_status=$(echo $comp_result | cut -d',' -f2)
            rm -f "$executable_name"  # Cleanup
        fi
    fi
    
    # Calculate performance ratio
    local ratio="1.0"
    if [ "$interp_time" -gt 0 ] && [ "$comp_time" -gt 0 ]; then
        ratio=$(echo "scale=2; $comp_time / $interp_time" | bc)
    fi
    
    local status="PASS"
    if [ "$interp_status" -ne 0 ] || [ "$comp_status" -ne 0 ]; then
        status="FAIL"
    fi
    
    echo "runtime_$test_file,$interp_time,$comp_time,$ratio,0.0,$status" >> $RESULTS_FILE
    echo "  Interpretation: ${interp_time}ms, Compilation+Exec: ${comp_time}ms, Ratio: ${ratio}x, Status: $status"
}

# Function to run cross-platform benchmark
benchmark_cross_platform() {
    echo "Benchmarking cross-platform compilation..."
    
    local platforms=("linux-x86_64" "linux-arm64" "macos-x86_64" "windows-x86_64" "wasm32")
    
    for platform in "${platforms[@]}"; do
        echo "Testing platform: $platform"
        
        local test_content='vibez.spill("Hello from '"$platform"'")'
        echo "$test_content" > "platform_test_$platform.csd"
        
        # Try cross-compilation (may not be supported for all targets)
        local start_time=$(date +%s%3N)
        local success="FAIL"
        
        if timeout 10s ./cursed-unified --target="$platform" --compile "platform_test_$platform.csd" > /dev/null 2>&1; then
            success="PASS"
        fi
        
        local end_time=$(date +%s%3N)
        local duration=$((end_time - start_time))
        
        echo "cross_platform_$platform,0,$duration,0,0.0,$success" >> $RESULTS_FILE
        echo "  Platform $platform: ${duration}ms ($success)"
        
        rm -f "platform_test_$platform.csd"
    done
}

# Function to analyze results and generate report
generate_performance_report() {
    echo ""
    echo "=== PERFORMANCE ANALYSIS REPORT ==="
    
    # Parse CSV results
    local total_tests=0
    local passed_tests=0
    local total_improvement=0
    local rust_times=0
    local zig_times=0
    local memory_usage=0
    
    while IFS=',' read -r test rust_time zig_time improvement memory status; do
        if [ "$test" != "Test" ]; then  # Skip header
            total_tests=$((total_tests + 1))
            if [ "$status" = "PASS" ]; then
                passed_tests=$((passed_tests + 1))
            fi
            
            if [ "$rust_time" -gt 0 ]; then
                rust_times=$((rust_times + rust_time))
            fi
            if [ "$zig_time" -gt 0 ]; then
                zig_times=$((zig_times + zig_time))
            fi
            if [ "$improvement" != "0" ]; then
                total_improvement=$(echo "$total_improvement + $improvement" | bc)
            fi
            if [ "$memory" != "0.0" ]; then
                memory_usage=$(echo "$memory_usage + $memory" | bc)
            fi
        fi
    done < $RESULTS_FILE
    
    # Calculate averages
    local avg_improvement="0"
    local avg_memory="0"
    local success_rate="0"
    
    if [ $total_tests -gt 0 ]; then
        avg_improvement=$(echo "scale=1; $total_improvement / $total_tests" | bc)
        avg_memory=$(echo "scale=1; $memory_usage / $total_tests" | bc)
        success_rate=$(echo "scale=1; $passed_tests * 100 / $total_tests" | bc)
    fi
    
    echo "Overall Performance Metrics:"
    echo "  Total Tests: $total_tests"
    echo "  Success Rate: ${success_rate}%"
    echo "  Average Compilation Speed Improvement: ${avg_improvement}%"
    echo "  Average Memory Usage: ${avg_memory}MB"
    echo "  Total Rust Time: ${rust_times}ms"
    echo "  Total Zig Time: ${zig_times}ms"
    
    # Performance targets assessment
    echo ""
    echo "Performance Targets Assessment:"
    
    local compilation_target_met="❌"
    if [ $(echo "$avg_improvement >= 80" | bc) -eq 1 ]; then
        compilation_target_met="✅"
    fi
    
    local memory_target_met="❌"
    if [ $(echo "$avg_memory <= 100" | bc) -eq 1 ]; then
        memory_target_met="✅"
    fi
    
    local success_target_met="❌"
    if [ $(echo "$success_rate >= 80" | bc) -eq 1 ]; then
        success_target_met="✅"
    fi
    
    echo "  Compilation Speed (>80% improvement): $compilation_target_met"
    echo "  Memory Usage (<100MB average): $memory_target_met"
    echo "  Test Success Rate (>80%): $success_target_met"
    
    # Overall readiness assessment
    local targets_met=0
    if [ "$compilation_target_met" = "✅" ]; then targets_met=$((targets_met + 1)); fi
    if [ "$memory_target_met" = "✅" ]; then targets_met=$((targets_met + 1)); fi
    if [ "$success_target_met" = "✅" ]; then targets_met=$((targets_met + 1)); fi
    
    echo ""
    if [ $targets_met -ge 2 ]; then
        echo "🎉 CURSED compiler performance is READY for v1.0! ($targets_met/3 targets met)"
    else
        echo "⚠️  CURSED compiler needs optimization for v1.0 ($targets_met/3 targets met)"
    fi
    
    # Optimization recommendations
    echo ""
    echo "=== OPTIMIZATION RECOMMENDATIONS ==="
    
    if [ "$compilation_target_met" = "❌" ]; then
        echo "- 🔧 Optimize compilation pipeline for faster builds"
        echo "- 🔧 Implement more aggressive caching strategies"
        echo "- 🔧 Parallelize compilation stages"
    fi
    
    if [ "$memory_target_met" = "❌" ]; then
        echo "- 🔧 Optimize memory allocation patterns"
        echo "- 🔧 Implement better garbage collection"
        echo "- 🔧 Reduce runtime memory overhead"
    fi
    
    if [ "$success_target_met" = "❌" ]; then
        echo "- 🔧 Fix failing test cases"
        echo "- 🔧 Improve error handling and recovery"
        echo "- 🔧 Enhance cross-platform compatibility"
    fi
    
    echo ""
    echo "Detailed results saved to: $RESULTS_FILE"
}

# Main benchmark execution
echo ""
echo "1. Compilation Speed Benchmarks:"
benchmark_compilation_speed "basic_test.csd"
benchmark_compilation_speed "complex_test.csd"
benchmark_compilation_speed "computation_intensive_test.csd"

echo ""
echo "2. Runtime Performance Benchmarks:"
benchmark_runtime_performance "basic_test.csd"
benchmark_runtime_performance "complex_test.csd"
benchmark_runtime_performance "concurrency_test.csd"

echo ""
echo "3. Feature-Specific Performance Benchmarks:"
benchmark_compilation_speed "memory_allocation_test.csd"
benchmark_compilation_speed "concurrency_stress_test.csd"
benchmark_compilation_speed "pattern_matching_performance_test.csd"
benchmark_compilation_speed "generic_performance_test.csd"
benchmark_compilation_speed "stdlib_benchmark_test.csd"

echo ""
echo "4. Cross-Platform Performance Benchmarks:"
benchmark_cross_platform

# Generate comprehensive report
generate_performance_report

echo ""
echo "=== PERFORMANCE BENCHMARK COMPLETE ==="
echo "Results file: $RESULTS_FILE"
