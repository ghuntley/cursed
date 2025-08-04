#!/bin/bash

# CURSED Build Performance Benchmark
# Simple and effective build performance testing

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

# Test configurations
TEST_ITERATIONS=${TEST_ITERATIONS:-3}
CROSS_TARGETS=("x86_64-linux" "aarch64-linux" "x86_64-macos" "aarch64-macos" "x86_64-windows")
OPTIMIZE_MODES=("Debug" "ReleaseFast" "ReleaseSmall")

# Logging
log_info() {
    echo -e "${BLUE}[BENCHMARK]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[BENCHMARK]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[BENCHMARK]${NC} $1"
}

log_error() {
    echo -e "${RED}[BENCHMARK]${NC} $1"
}

# Benchmark single build command
benchmark_single() {
    local description="$1"
    local command="$2"
    local times=()
    
    log_info "🚀 Benchmarking: $description"
    
    for ((i=1; i<=TEST_ITERATIONS; i++)); do
        log_info "  Iteration $i/$TEST_ITERATIONS"
        
        # Clean between iterations
        rm -rf zig-out zig-cache 2>/dev/null || true
        
        # Time the build
        local start_time
        start_time=$(date +%s.%N)
        
        if eval "$command" >/dev/null 2>&1; then
            local end_time
            end_time=$(date +%s.%N)
            local build_time
            build_time=$(echo "$end_time - $start_time" | awk '{print $1}')
            times+=("$build_time")
            log_info "    ✅ Completed in ${build_time}s"
        else
            log_error "    ❌ Build failed"
            times+=("999")
        fi
    done
    
    # Calculate statistics
    local total=0
    local min=999999
    local max=0
    
    for time in "${times[@]}"; do
        if (( $(echo "$time < 999" | awk '{print ($1 == 1)}') )); then
            total=$(echo "$total + $time" | awk '{print $1}')
            if (( $(echo "$time < $min" | awk '{print ($1 == 1)}') )); then
                min="$time"
            fi
            if (( $(echo "$time > $max" | awk '{print ($1 == 1)}') )); then
                max="$time"
            fi
        fi
    done
    
    local avg
    avg=$(echo "$total / ${#times[@]}" | awk '{print $1}')
    
    log_success "  📊 Results: avg=${avg}s, min=${min}s, max=${max}s"
    echo "$description,$avg,$min,$max" >> benchmark_results.csv
}

# Benchmark cross-compilation performance
benchmark_cross_compilation() {
    log_info "🌍 Benchmarking cross-compilation performance"
    
    echo "Target,Average Time (s),Min Time (s),Max Time (s)" > cross_benchmark_results.csv
    
    for target in "${CROSS_TARGETS[@]}"; do
        local command="zig build -Dtarget=$target -Doptimize=ReleaseFast"
        benchmark_single "Cross-compile to $target" "$command"
        echo "$target,$avg,$min,$max" >> cross_benchmark_results.csv
    done
    
    log_success "✅ Cross-compilation benchmark completed"
}

# Benchmark optimization modes
benchmark_optimization_modes() {
    log_info "⚡ Benchmarking optimization modes"
    
    echo "Optimization Mode,Average Time (s),Min Time (s),Max Time (s)" > optimization_benchmark_results.csv
    
    for mode in "${OPTIMIZE_MODES[@]}"; do
        local command="zig build -Doptimize=$mode"
        benchmark_single "Optimize mode: $mode" "$command"
        echo "$mode,$avg,$min,$max" >> optimization_benchmark_results.csv
    done
    
    log_success "✅ Optimization mode benchmark completed"
}

# Benchmark parallel builds
benchmark_parallel_builds() {
    log_info "🔄 Benchmarking parallel vs sequential builds"
    
    echo "Build Type,Average Time (s),Min Time (s),Max Time (s)" > parallel_benchmark_results.csv
    
    # Sequential build (single target)
    benchmark_single "Sequential build" "zig build"
    echo "Sequential,$avg,$min,$max" >> parallel_benchmark_results.csv
    
    # Simulate parallel cross-compilation
    local start_time
    start_time=$(date +%s.%N)
    
    # Build 3 targets in background
    zig build -Dtarget=x86_64-linux -Doptimize=ReleaseFast &
    local pid1=$!
    zig build -Dtarget=aarch64-linux -Doptimize=ReleaseFast &
    local pid2=$!
    zig build -Dtarget=x86_64-macos -Doptimize=ReleaseFast &
    local pid3=$!
    
    # Wait for all to complete
    wait $pid1 $pid2 $pid3
    
    local end_time
    end_time=$(date +%s.%N)
    local parallel_time
    parallel_time=$(echo "$end_time - $start_time" | awk '{print $1}')
    
    log_success "  📊 Parallel build completed in ${parallel_time}s"
    echo "Parallel (3 targets),$parallel_time,$parallel_time,$parallel_time" >> parallel_benchmark_results.csv
}

# Compare with Rust build performance
benchmark_vs_rust() {
    log_info "⚖️ Comparing Zig vs Rust build performance"
    
    # Benchmark Zig
    log_info "Testing Zig build performance..."
    benchmark_single "Zig build" "zig build"
    local zig_time="$avg"
    
    # Simulate Rust build (if Cargo.toml exists)
    if [[ -f "Cargo.toml" ]]; then
        log_info "Testing Rust build performance..."
        benchmark_single "Rust build" "cargo build"
        local rust_time="$avg"
        
        # Calculate improvement
        local improvement
        improvement=$(echo "($rust_time - $zig_time) * 100 / $rust_time" | awk '{print $1}')
        
        log_success "🚀 Performance comparison:"
        log_success "  Zig: ${zig_time}s"
        log_success "  Rust: ${rust_time}s"
        log_success "  Zig is ${improvement}% faster"
        
        echo "Build System,Time (s),Improvement (%)" > vs_rust_results.csv
        echo "Zig,$zig_time,0" >> vs_rust_results.csv
        echo "Rust,$rust_time,$improvement" >> vs_rust_results.csv
    else
        log_warning "Cargo.toml not found, skipping Rust comparison"
    fi
}

# Generate comprehensive report
generate_report() {
    local report_file="build_performance_report.md"
    
    log_info "📊 Generating comprehensive performance report"
    
    {
        echo "# CURSED Build Performance Report"
        echo "Generated: $(date)"
        echo ""
        echo "## System Information"
        echo "- OS: $(uname -s)"
        echo "- Architecture: $(uname -m)"
        echo "- CPU cores: $(nproc 2>/dev/null || echo "unknown")"
        echo "- Zig version: $(zig version)"
        echo "- Test iterations: $TEST_ITERATIONS"
        echo ""
        
        if [[ -f "optimization_benchmark_results.csv" ]]; then
            echo "## Optimization Mode Performance"
            echo "| Mode | Avg Time (s) | Min Time (s) | Max Time (s) |"
            echo "|------|--------------|--------------|--------------|"
            tail -n +2 optimization_benchmark_results.csv | while IFS=',' read -r mode avg min max; do
                echo "| $mode | $avg | $min | $max |"
            done
            echo ""
        fi
        
        if [[ -f "cross_benchmark_results.csv" ]]; then
            echo "## Cross-Compilation Performance"
            echo "| Target | Avg Time (s) | Min Time (s) | Max Time (s) |"
            echo "|--------|--------------|--------------|--------------|"
            tail -n +2 cross_benchmark_results.csv | while IFS=',' read -r target avg min max; do
                echo "| $target | $avg | $min | $max |"
            done
            echo ""
        fi
        
        if [[ -f "parallel_benchmark_results.csv" ]]; then
            echo "## Parallel vs Sequential Performance"
            echo "| Build Type | Time (s) |"
            echo "|------------|----------|"
            tail -n +2 parallel_benchmark_results.csv | while IFS=',' read -r type avg min max; do
                echo "| $type | $avg |"
            done
            echo ""
        fi
        
        if [[ -f "vs_rust_results.csv" ]]; then
            echo "## Zig vs Rust Performance Comparison"
            echo "| Build System | Time (s) | Performance |"
            echo "|--------------|----------|-------------|"
            tail -n +2 vs_rust_results.csv | while IFS=',' read -r system time improvement; do
                if [[ "$improvement" == "0" ]]; then
                    echo "| $system | $time | baseline |"
                else
                    echo "| $system | $time | ${improvement}% slower |"
                fi
            done
            echo ""
        fi
        
        echo "## Key Findings"
        echo ""
        
        # Calculate best optimization mode
        if [[ -f "optimization_benchmark_results.csv" ]]; then
            local fastest_mode
            fastest_mode=$(tail -n +2 optimization_benchmark_results.csv | sort -t',' -k2 -n | head -1 | cut -d',' -f1)
            echo "- **Fastest optimization mode**: $fastest_mode"
        fi
        
        # Calculate total cross-compilation time
        if [[ -f "cross_benchmark_results.csv" ]]; then
            local total_cross_time
            total_cross_time=$(tail -n +2 cross_benchmark_results.csv | cut -d',' -f2 | awk '{sum+=$1} END {print sum}')
            echo "- **Total cross-compilation time**: ${total_cross_time}s for ${#CROSS_TARGETS[@]} targets"
        fi
        
        echo ""
        echo "## Recommendations"
        echo ""
        echo "1. **Use ReleaseFast for production builds** - provides good balance of speed and optimization"
        echo "2. **Enable parallel cross-compilation** - significantly reduces total build time"
        echo "3. **Implement build caching** - can provide 80%+ speedup for incremental builds"
        echo "4. **Monitor memory usage** - ensure sufficient RAM for parallel builds"
        
    } > "$report_file"
    
    log_success "📊 Report generated: $report_file"
}

# Main benchmark suite
run_full_benchmark() {
    log_info "🚀 Starting comprehensive CURSED build performance benchmark"
    
    # Clean up previous results
    rm -f *_benchmark_results.csv benchmark_results.csv
    
    # Create results header
    echo "Test,Average Time (s),Min Time (s),Max Time (s)" > benchmark_results.csv
    
    # Run all benchmarks
    benchmark_optimization_modes
    benchmark_cross_compilation
    benchmark_parallel_builds
    benchmark_vs_rust
    
    # Generate report
    generate_report
    
    log_success "🎉 Comprehensive benchmark completed!"
    log_info "📊 Check build_performance_report.md for detailed results"
}

# Quick benchmark (reduced iterations)
run_quick_benchmark() {
    log_info "⚡ Running quick performance benchmark"
    
    TEST_ITERATIONS=1
    
    # Test basic build
    benchmark_single "Quick Zig build" "zig build"
    
    # Test one cross-compilation target
    benchmark_single "Quick cross-compile" "zig build -Dtarget=x86_64-linux -Doptimize=ReleaseFast"
    
    log_success "✅ Quick benchmark completed"
}

# Memory usage benchmark
benchmark_memory_usage() {
    log_info "🧠 Benchmarking memory usage"
    
    if command -v /usr/bin/time >/dev/null 2>&1; then
        log_info "Testing memory usage with different optimization modes..."
        
        echo "Optimization,Max Memory (KB),Time (s)" > memory_benchmark_results.csv
        
        for mode in "${OPTIMIZE_MODES[@]}"; do
            log_info "  Testing $mode mode..."
            local output
            output=$(/usr/bin/time -f "%M,%e" zig build -Doptimize="$mode" 2>&1)
            local memory
            memory=$(echo "$output" | tail -1 | cut -d',' -f1)
            local time
            time=$(echo "$output" | tail -1 | cut -d',' -f2)
            
            echo "$mode,$memory,$time" >> memory_benchmark_results.csv
            log_info "    Memory: ${memory}KB, Time: ${time}s"
        done
        
        log_success "✅ Memory benchmark completed"
    else
        log_warning "GNU time not available for memory benchmarking"
    fi
}

# Main function
main() {
    local command="${1:-help}"
    
    cd "$PROJECT_ROOT"
    
    case "$command" in
        "full")
            run_full_benchmark
            ;;
        "quick")
            run_quick_benchmark
            ;;
        "cross")
            benchmark_cross_compilation
            ;;
        "optimization")
            benchmark_optimization_modes
            ;;
        "parallel")
            benchmark_parallel_builds
            ;;
        "memory")
            benchmark_memory_usage
            ;;
        "vs-rust")
            benchmark_vs_rust
            ;;
        "report")
            generate_report
            ;;
        "help"|*)
            echo "CURSED Build Performance Benchmark"
            echo "Usage: $0 <command>"
            echo ""
            echo "Commands:"
            echo "  full         Run comprehensive benchmark suite"
            echo "  quick        Run quick performance test"
            echo "  cross        Benchmark cross-compilation"
            echo "  optimization Benchmark optimization modes"
            echo "  parallel     Benchmark parallel builds"
            echo "  memory       Benchmark memory usage"
            echo "  vs-rust      Compare with Rust build performance"
            echo "  report       Generate performance report"
            echo "  help         Show this help"
            echo ""
            echo "Environment variables:"
            echo "  TEST_ITERATIONS  Number of test iterations (default: 3)"
            ;;
    esac
}

# Run main function
main "$@"
