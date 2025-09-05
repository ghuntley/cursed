#!/bin/bash

# Performance Regression Detection for CURSED Compiler
# Monitors compilation and execution performance over time

set -euo pipefail

echo "📊 CURSED Performance Regression Detection"
echo "=========================================="

# Configuration
COMPILER_BINARY="${COMPILER_BINARY:-target/release/cursed}"
BENCHMARK_DIR="${BENCHMARK_DIR:-/tmp/performance_benchmarks}"
BASELINE_FILE="${BASELINE_FILE:-ci/performance_baseline.json}"
REGRESSION_THRESHOLD="${REGRESSION_THRESHOLD:-1.5}"
TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-300}"

# Cleanup and setup
cleanup() {
    echo "🧹 Cleaning up benchmark directory..."
    rm -rf "$BENCHMARK_DIR"
}
trap cleanup EXIT

mkdir -p "$BENCHMARK_DIR"
cd "$BENCHMARK_DIR"

# Create benchmark test programs
create_benchmark_programs() {
    echo "📝 Creating benchmark test programs..."
    
    # Benchmark 1: Simple arithmetic
    cat > arithmetic_benchmark.💀 << 'EOF'
// Arithmetic operations benchmark
sus iterations normie = 10000
sus result normie = 0

bestie i := 0; i < iterations; i++ {
    result += i * 2 + 1
}

vibez.spill(result)
EOF

    # Benchmark 2: Function calls
    cat > function_benchmark.💀 << 'EOF'
// Function call benchmark
slay expensive_function(n normie) normie {
    sus sum normie = 0
    bestie i := 0; i < n; i++ {
        sum += i * i
    }
    damn sum
}

sus result normie = expensive_function(1000)
vibez.spill(result)
EOF

    # Benchmark 3: Array operations
    cat > array_benchmark.💀 << 'EOF'
// Array operations benchmark
sus size normie = 1000
sus numbers [1000]normie

bestie i := 0; i < size; i++ {
    numbers[i] = i * 2
}

sus sum normie = 0
bestie i := 0; i < size; i++ {
    sum += numbers[i]
}

vibez.spill(sum)
EOF

    # Benchmark 4: String operations
    cat > string_benchmark.💀 << 'EOF'
// String operations benchmark
yeet "stringz"

sus base tea = "Hello"
sus result tea = base

bestie i := 0; i < 100; i++ {
    result = result + " World"
}

vibez.spill(stringz.length(result))
EOF

    # Benchmark 5: Recursive function
    cat > recursive_benchmark.💀 << 'EOF'
// Recursive function benchmark
slay fibonacci(n normie) normie {
    if n <= 1 {
        damn n
    } else {
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

sus result normie = fibonacci(25)
vibez.spill(result)
EOF

    # Benchmark 6: Complex computation
    cat > complex_benchmark.💀 << 'EOF'
// Complex computation benchmark
slay prime_check(n normie) lit {
    if n <= 1 {
        damn cap
    }
    if n <= 3 {
        damn based
    }
    if n % 2 == 0 || n % 3 == 0 {
        damn cap
    }
    
    sus i normie = 5
    bestie i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            damn cap
        }
        i += 6
    }
    damn based
}

sus count normie = 0
bestie i := 2; i < 1000; i++ {
    if prime_check(i) {
        count++
    }
}

vibez.spill(count)
EOF
}

# Run single benchmark
run_benchmark() {
    local benchmark_name=$1
    local benchmark_file="${benchmark_name}.💀"
    
    echo "⚡ Running benchmark: $benchmark_name"
    
    # Measure compilation time
    echo "  🔨 Measuring compilation time..."
    start_time=$(date +%s%N)
    
    if timeout $TIMEOUT_SECONDS "$OLDPWD/$COMPILER_BINARY" -- compile "$benchmark_file" -o "$benchmark_name" 2>/dev/null; then
        end_time=$(date +%s%N)
        compile_time=$((end_time - start_time))
        compile_seconds=$(echo "scale=4; $compile_time / 1000000000" | bc)
        echo "    ✅ Compilation time: ${compile_seconds}s"
    else
        echo "    ❌ Compilation failed"
        return 1
    fi
    
    # Measure execution time
    echo "  🏃 Measuring execution time..."
    start_time=$(date +%s%N)
    
    if timeout $TIMEOUT_SECONDS "./$benchmark_name" > "${benchmark_name}_output.txt" 2>&1; then
        end_time=$(date +%s%N)
        exec_time=$((end_time - start_time))
        exec_seconds=$(echo "scale=4; $exec_time / 1000000000" | bc)
        echo "    ✅ Execution time: ${exec_seconds}s"
    else
        echo "    ❌ Execution failed"
        return 1
    fi
    
    # Measure memory usage (if available)
    echo "  💾 Measuring memory usage..."
    if command -v /usr/bin/time >/dev/null 2>&1; then
        /usr/bin/time -v "./$benchmark_name" 2> "${benchmark_name}_memory.txt" >/dev/null
        max_memory=$(grep "Maximum resident set size" "${benchmark_name}_memory.txt" | awk '{print $6}')
        echo "    📊 Peak memory: ${max_memory}KB"
    else
        max_memory="N/A"
        echo "    ⚠️  Memory measurement not available"
    fi
    
    # Store results
    cat > "${benchmark_name}_results.json" << EOF
{
    "benchmark": "$benchmark_name",
    "compile_time": $compile_seconds,
    "execution_time": $exec_seconds,
    "memory_usage": "$max_memory",
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "output": "$(cat ${benchmark_name}_output.txt)"
}
EOF
    
    echo "    📈 Results saved to ${benchmark_name}_results.json"
    return 0
}

# Compare with baseline
compare_with_baseline() {
    local benchmark_name=$1
    local current_results="${benchmark_name}_results.json"
    
    if [ ! -f "$OLDPWD/$BASELINE_FILE" ]; then
        echo "  ⚠️  No baseline found, creating initial baseline"
        return 0
    fi
    
    echo "  📊 Comparing with baseline..."
    
    # Extract current metrics
    current_compile_time=$(jq -r '.compile_time' "$current_results")
    current_exec_time=$(jq -r '.execution_time' "$current_results")
    
    # Extract baseline metrics
    baseline_compile_time=$(jq -r ".benchmarks.${benchmark_name}.compile_time // 0" "$OLDPWD/$BASELINE_FILE")
    baseline_exec_time=$(jq -r ".benchmarks.${benchmark_name}.execution_time // 0" "$OLDPWD/$BASELINE_FILE")
    
    if [ "$baseline_compile_time" != "0" ] && [ "$baseline_exec_time" != "0" ]; then
        # Calculate performance ratios
        compile_ratio=$(echo "scale=2; $current_compile_time / $baseline_compile_time" | bc)
        exec_ratio=$(echo "scale=2; $current_exec_time / $baseline_exec_time" | bc)
        
        echo "    📈 Compilation performance: ${compile_ratio}x baseline"
        echo "    📈 Execution performance: ${exec_ratio}x baseline"
        
        # Check for regressions
        if (( $(echo "$compile_ratio > $REGRESSION_THRESHOLD" | bc -l) )); then
            echo "    ⚠️  Compilation regression detected (>${REGRESSION_THRESHOLD}x)"
            echo "REGRESSION" > "${benchmark_name}_regression_compile.flag"
        fi
        
        if (( $(echo "$exec_ratio > $REGRESSION_THRESHOLD" | bc -l) )); then
            echo "    ⚠️  Execution regression detected (>${REGRESSION_THRESHOLD}x)"
            echo "REGRESSION" > "${benchmark_name}_regression_exec.flag"
        fi
    else
        echo "    ℹ️  No baseline data for comparison"
    fi
}

# Generate performance report
generate_performance_report() {
    echo "📊 Generating performance report..."
    
    # Aggregate results
    cat > performance_report.json << 'EOF'
{
    "report_timestamp": "",
    "benchmarks": {},
    "summary": {
        "total_benchmarks": 0,
        "successful_benchmarks": 0,
        "failed_benchmarks": 0,
        "regressions_detected": 0,
        "average_compile_time": 0,
        "average_execution_time": 0
    }
}
EOF
    
    # Update timestamp
    jq --arg timestamp "$(date -u +%Y-%m-%dT%H:%M:%SZ)" '.report_timestamp = $timestamp' performance_report.json > temp.json && mv temp.json performance_report.json
    
    # Process each benchmark result
    total_benchmarks=0
    successful_benchmarks=0
    failed_benchmarks=0
    regressions_detected=0
    total_compile_time=0
    total_exec_time=0
    
    for result_file in *_results.json; do
        if [ -f "$result_file" ]; then
            benchmark_name=$(jq -r '.benchmark' "$result_file")
            total_benchmarks=$((total_benchmarks + 1))
            
            # Add to report
            jq --slurpfile benchmark "$result_file" ".benchmarks.${benchmark_name} = \$benchmark[0]" performance_report.json > temp.json && mv temp.json performance_report.json
            
            # Check if benchmark was successful
            if [ -f "${benchmark_name}" ]; then
                successful_benchmarks=$((successful_benchmarks + 1))
                
                # Add to totals for averages
                compile_time=$(jq -r '.compile_time' "$result_file")
                exec_time=$(jq -r '.execution_time' "$result_file")
                total_compile_time=$(echo "$total_compile_time + $compile_time" | bc)
                total_exec_time=$(echo "$total_exec_time + $exec_time" | bc)
            else
                failed_benchmarks=$((failed_benchmarks + 1))
            fi
            
            # Check for regressions
            if [ -f "${benchmark_name}_regression_compile.flag" ] || [ -f "${benchmark_name}_regression_exec.flag" ]; then
                regressions_detected=$((regressions_detected + 1))
            fi
        fi
    done
    
    # Calculate averages
    if [ $successful_benchmarks -gt 0 ]; then
        avg_compile_time=$(echo "scale=4; $total_compile_time / $successful_benchmarks" | bc)
        avg_exec_time=$(echo "scale=4; $total_exec_time / $successful_benchmarks" | bc)
    else
        avg_compile_time=0
        avg_exec_time=0
    fi
    
    # Update summary
    jq --argjson total $total_benchmarks \
       --argjson successful $successful_benchmarks \
       --argjson failed $failed_benchmarks \
       --argjson regressions $regressions_detected \
       --argjson avg_compile $avg_compile_time \
       --argjson avg_exec $avg_exec_time \
       '.summary.total_benchmarks = $total |
        .summary.successful_benchmarks = $successful |
        .summary.failed_benchmarks = $failed |
        .summary.regressions_detected = $regressions |
        .summary.average_compile_time = $avg_compile |
        .summary.average_execution_time = $avg_exec' \
       performance_report.json > temp.json && mv temp.json performance_report.json
    
    # Copy report to main directory
    cp performance_report.json "$OLDPWD/"
    
    echo "📋 Performance report generated: performance_report.json"
}

# Update baseline if no regressions
update_baseline() {
    echo "📊 Updating performance baseline..."
    
    # Check if any regressions were detected
    if ls *_regression_*.flag 1> /dev/null 2>&1; then
        echo "⚠️  Regressions detected, not updating baseline"
        return 1
    fi
    
    # Create or update baseline
    if [ -f "$OLDPWD/$BASELINE_FILE" ]; then
        echo "🔄 Updating existing baseline..."
        cp "$OLDPWD/$BASELINE_FILE" baseline_backup.json
    else
        echo "🆕 Creating new baseline..."
    fi
    
    # Use current performance report as new baseline
    cp performance_report.json "$OLDPWD/$BASELINE_FILE"
    echo "✅ Baseline updated successfully"
}

# Main execution
echo "🚀 Starting performance regression detection..."

# Build compiler if not exists
if [ ! -f "$OLDPWD/$COMPILER_BINARY" ]; then
    echo "🏗️  Building compiler..."
    cd "$OLDPWD"
    cargo build --release --bin cursed
    cd "$BENCHMARK_DIR"
fi

# Create benchmark programs
create_benchmark_programs

# Run all benchmarks
echo "🧪 Running performance benchmarks..."
benchmarks=("arithmetic_benchmark" "function_benchmark" "array_benchmark" "string_benchmark" "recursive_benchmark" "complex_benchmark")

for benchmark in "${benchmarks[@]}"; do
    if run_benchmark "$benchmark"; then
        compare_with_baseline "$benchmark"
    else
        echo "❌ Benchmark $benchmark failed"
    fi
done

# Generate comprehensive report
generate_performance_report

# Display summary
echo ""
echo "📊 Performance Regression Detection Summary"
echo "=========================================="
echo "Total benchmarks: $(jq -r '.summary.total_benchmarks' performance_report.json)"
echo "Successful: $(jq -r '.summary.successful_benchmarks' performance_report.json)"
echo "Failed: $(jq -r '.summary.failed_benchmarks' performance_report.json)"
echo "Regressions detected: $(jq -r '.summary.regressions_detected' performance_report.json)"
echo "Average compile time: $(jq -r '.summary.average_compile_time' performance_report.json)s"
echo "Average execution time: $(jq -r '.summary.average_execution_time' performance_report.json)s"

# Update baseline if appropriate
if [ "$(jq -r '.summary.regressions_detected' performance_report.json)" = "0" ]; then
    update_baseline
fi

# Exit with appropriate code
regressions=$(jq -r '.summary.regressions_detected' performance_report.json)
if [ "$regressions" -gt 0 ]; then
    echo ""
    echo "⚠️  Performance regressions detected!"
    echo "🔍 Review the performance report and investigate regressions"
    exit 1
else
    echo ""
    echo "✅ No performance regressions detected"
    echo "🚀 Performance is within acceptable thresholds"
    exit 0
fi
