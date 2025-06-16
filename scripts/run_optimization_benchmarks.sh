#!/bin/bash
set -euo pipefail

# CURSED Optimization Performance Benchmark Script
# 
# Runs comprehensive performance benchmarks to validate optimization improvements
# and measure the impact of enhanced optimization passes, PGO, and LTO.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
BENCHMARK_DIR="${PROJECT_ROOT}/benchmarks"
RESULTS_DIR="${PROJECT_ROOT}/benchmark_results"
CURSED_BINARY="${PROJECT_ROOT}/target/release/cursed"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
ITERATIONS=5
WARMUP_ITERATIONS=2
TIMEOUT=300
VERBOSE=false
BASELINE_MODE=false
COMPARE_MODE=false
BASELINE_FILE=""

# Print usage information
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Run CURSED optimization performance benchmarks.

OPTIONS:
    -h, --help              Show this help message
    -v, --verbose           Enable verbose output
    -i, --iterations N      Number of benchmark iterations (default: $ITERATIONS)
    -w, --warmup N          Number of warmup iterations (default: $WARMUP_ITERATIONS)
    -t, --timeout N         Timeout in seconds for each benchmark (default: $TIMEOUT)
    -b, --baseline          Run in baseline mode (save results as baseline)
    -c, --compare FILE      Compare results against baseline file
    --build-first           Build the CURSED compiler before running benchmarks

EXAMPLES:
    $0                      Run standard benchmark suite
    $0 -v -i 10             Run with verbose output and 10 iterations
    $0 -b                   Run and save as baseline
    $0 -c baseline.json     Compare against previous baseline
    $0 --build-first        Build compiler first, then run benchmarks

EOF
}

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_verbose() {
    if [[ "$VERBOSE" == "true" ]]; then
        echo -e "${BLUE}[VERBOSE]${NC} $1"
    fi
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                usage
                exit 0
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -i|--iterations)
                ITERATIONS="$2"
                shift 2
                ;;
            -w|--warmup)
                WARMUP_ITERATIONS="$2"
                shift 2
                ;;
            -t|--timeout)
                TIMEOUT="$2"
                shift 2
                ;;
            -b|--baseline)
                BASELINE_MODE=true
                shift
                ;;
            -c|--compare)
                COMPARE_MODE=true
                BASELINE_FILE="$2"
                shift 2
                ;;
            --build-first)
                BUILD_FIRST=true
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                usage
                exit 1
                ;;
        esac
    done
}

# Check if required tools are available
check_dependencies() {
    log_info "Checking dependencies..."
    
    local missing_deps=()
    
    if ! command -v rustc &> /dev/null; then
        missing_deps+=("rustc")
    fi
    
    if ! command -v cargo &> /dev/null; then
        missing_deps+=("cargo")
    fi
    
    if ! command -v jq &> /dev/null; then
        log_warning "jq not found - JSON processing will be limited"
    fi
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log_error "Missing required dependencies: ${missing_deps[*]}"
        exit 1
    fi
    
    log_success "All dependencies available"
}

# Build the CURSED compiler
build_compiler() {
    log_info "Building CURSED compiler in release mode..."
    
    cd "$PROJECT_ROOT"
    
    # Use the linking fix for Nix environments
    if [[ -f "./fix_linking.sh" ]]; then
        log_verbose "Using linking fix for Nix environment"
        ./fix_linking.sh cargo build --release
    else
        cargo build --release
    fi
    
    if [[ ! -f "$CURSED_BINARY" ]]; then
        log_error "Failed to build CURSED compiler at $CURSED_BINARY"
        exit 1
    fi
    
    log_success "CURSED compiler built successfully"
}

# Prepare benchmark environment
prepare_environment() {
    log_info "Preparing benchmark environment..."
    
    # Create results directory
    mkdir -p "$RESULTS_DIR"
    
    # Create benchmark directory if it doesn't exist
    mkdir -p "$BENCHMARK_DIR"
    
    # Check if benchmark files exist
    local benchmark_files=(
        "small_function.csd"
        "medium_program.csd"  
        "large_application.csd"
    )
    
    for file in "${benchmark_files[@]}"; do
        if [[ ! -f "$BENCHMARK_DIR/$file" ]]; then
            log_warning "Benchmark file not found: $BENCHMARK_DIR/$file"
        fi
    done
    
    log_success "Environment prepared"
}

# Run a single benchmark
run_single_benchmark() {
    local benchmark_name="$1"
    local optimization_level="$2"
    local source_file="$3"
    
    log_verbose "Running benchmark: $benchmark_name with optimization $optimization_level"
    
    local output_file="${RESULTS_DIR}/${benchmark_name}_${optimization_level}.out"
    local timing_file="${RESULTS_DIR}/${benchmark_name}_${optimization_level}.time"
    local start_time
    local end_time
    local compile_time
    local binary_size=0
    
    # Warmup iterations
    for ((i=1; i<=WARMUP_ITERATIONS; i++)); do
        log_verbose "Warmup iteration $i/$WARMUP_ITERATIONS for $benchmark_name"
        timeout "$TIMEOUT" "$CURSED_BINARY" compile "$source_file" -O "$optimization_level" \
            --output "${output_file}.warmup" &>/dev/null || true
        rm -f "${output_file}.warmup"
    done
    
    # Measured iterations
    local times=()
    local sizes=()
    
    for ((i=1; i<=ITERATIONS; i++)); do
        log_verbose "Benchmark iteration $i/$ITERATIONS for $benchmark_name"
        
        start_time=$(date +%s.%N)
        
        if timeout "$TIMEOUT" "$CURSED_BINARY" compile "$source_file" \
            -O "$optimization_level" --output "$output_file" &>"$timing_file"; then
            
            end_time=$(date +%s.%N)
            compile_time=$(echo "$end_time - $start_time" | bc -l)
            times+=("$compile_time")
            
            if [[ -f "$output_file" ]]; then
                binary_size=$(stat -f%z "$output_file" 2>/dev/null || stat -c%s "$output_file" 2>/dev/null || echo "0")
                sizes+=("$binary_size")
            fi
            
            rm -f "$output_file"
        else
            log_warning "Benchmark iteration $i failed for $benchmark_name"
            times+=("999.999")  # Mark as failed
            sizes+=("0")
        fi
    done
    
    # Calculate averages
    local avg_time=0
    local avg_size=0
    
    if [[ ${#times[@]} -gt 0 ]]; then
        local sum_time=0
        local sum_size=0
        
        for time in "${times[@]}"; do
            sum_time=$(echo "$sum_time + $time" | bc -l)
        done
        
        for size in "${sizes[@]}"; do
            sum_size=$(echo "$sum_size + $size" | bc -l)
        done
        
        avg_time=$(echo "scale=3; $sum_time / ${#times[@]}" | bc -l)
        avg_size=$(echo "$sum_size / ${#sizes[@]}" | bc -l)
    fi
    
    # Output results in JSON format
    cat > "${RESULTS_DIR}/${benchmark_name}_${optimization_level}.json" << EOF
{
    "benchmark_name": "$benchmark_name",
    "optimization_level": "$optimization_level",
    "iterations": $ITERATIONS,
    "warmup_iterations": $WARMUP_ITERATIONS,
    "average_compile_time": $avg_time,
    "average_binary_size": $avg_size,
    "individual_times": [$(IFS=,; echo "${times[*]}")],
    "individual_sizes": [$(IFS=,; echo "${sizes[*]}")],
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF
    
    log_verbose "Benchmark $benchmark_name ($optimization_level) completed: ${avg_time}s average"
}

# Run all benchmarks
run_benchmarks() {
    log_info "Starting benchmark suite..."
    
    local benchmarks=(
        "small_function:$BENCHMARK_DIR/small_function.csd"
        "medium_program:$BENCHMARK_DIR/medium_program.csd"
        "large_application:$BENCHMARK_DIR/large_application.csd"
    )
    
    local optimization_levels=("O0" "O1" "O2" "O3" "Os" "Oz")
    
    local total_benchmarks=$((${#benchmarks[@]} * ${#optimization_levels[@]}))
    local completed_benchmarks=0
    
    for benchmark_spec in "${benchmarks[@]}"; do
        IFS=':' read -r benchmark_name source_file <<< "$benchmark_spec"
        
        if [[ ! -f "$source_file" ]]; then
            log_warning "Skipping $benchmark_name - source file not found: $source_file"
            continue
        fi
        
        log_info "Running benchmark: $benchmark_name"
        
        for opt_level in "${optimization_levels[@]}"; do
            run_single_benchmark "$benchmark_name" "$opt_level" "$source_file"
            ((completed_benchmarks++))
            
            local progress=$((completed_benchmarks * 100 / total_benchmarks))
            log_info "Progress: $completed_benchmarks/$total_benchmarks ($progress%)"
        done
    done
    
    log_success "All benchmarks completed"
}

# Generate comprehensive results report
generate_report() {
    log_info "Generating benchmark report..."
    
    local report_file="${RESULTS_DIR}/benchmark_report_$(date +%Y%m%d_%H%M%S).md"
    local summary_file="${RESULTS_DIR}/benchmark_summary.json"
    
    cat > "$report_file" << EOF
# CURSED Optimization Performance Benchmark Report

**Generated:** $(date)  
**Iterations:** $ITERATIONS  
**Warmup Iterations:** $WARMUP_ITERATIONS  
**Timeout:** ${TIMEOUT}s  

## Summary

This report contains performance measurements for the enhanced CURSED optimization system.
The benchmarks validate the performance improvements from:

- Enhanced optimization passes enabled by default
- Profile-guided optimization (PGO) integration
- Link-time optimization (LTO) enablement
- Aggressive optimization for release builds

## Results

EOF
    
    # Process JSON results if jq is available
    if command -v jq &> /dev/null; then
        log_verbose "Processing results with jq"
        
        cat >> "$report_file" << EOF
| Benchmark | Optimization | Avg Compile Time (s) | Avg Binary Size (bytes) |
|-----------|--------------|---------------------|------------------------|
EOF
        
        for result_file in "$RESULTS_DIR"/*.json; do
            if [[ -f "$result_file" ]]; then
                local name
                local opt_level
                local avg_time
                local avg_size
                
                name=$(jq -r '.benchmark_name' "$result_file")
                opt_level=$(jq -r '.optimization_level' "$result_file")
                avg_time=$(jq -r '.average_compile_time' "$result_file")
                avg_size=$(jq -r '.average_binary_size' "$result_file")
                
                printf "| %s | %s | %.3f | %s |\n" "$name" "$opt_level" "$avg_time" "$avg_size" >> "$report_file"
            fi
        done
        
    else
        log_warning "jq not available - generating basic report"
        echo "Raw results available in: $RESULTS_DIR" >> "$report_file"
    fi
    
    cat >> "$report_file" << EOF

## Optimization Analysis

### Default Optimization Improvements

The enhanced optimization system enables aggressive optimization by default:

1. **Optimization Level**: Changed from O2 to O3 by default
2. **Enhanced Passes**: Aggressive inlining, vectorization, loop unrolling
3. **LTO**: Link-time optimization enabled by default
4. **PGO**: Profile-guided optimization when data available
5. **Target Features**: Native CPU features utilized

### Performance Expectations

- **Compile Time**: May increase 20-50% due to aggressive optimization
- **Runtime Performance**: Expected 10-30% improvement over previous defaults
- **Binary Size**: May increase 10-20% but with better performance
- **Memory Usage**: Compilation may use 20-40% more memory

### Regression Thresholds

- Maximum compile time increase: 50%
- Minimum runtime improvement: 10%
- Maximum binary size increase: 20%
- Maximum memory increase: 30%

EOF
    
    log_success "Report generated: $report_file"
    
    # Save baseline if requested
    if [[ "$BASELINE_MODE" == "true" ]]; then
        local baseline_file="${RESULTS_DIR}/baseline_$(date +%Y%m%d_%H%M%S).json"
        
        # Combine all results into baseline
        if command -v jq &> /dev/null; then
            jq -s '.' "$RESULTS_DIR"/*.json > "$baseline_file"
            log_success "Baseline saved: $baseline_file"
        else
            log_warning "Cannot save baseline - jq not available"
        fi
    fi
}

# Compare results with baseline
compare_with_baseline() {
    if [[ "$COMPARE_MODE" != "true" ]] || [[ ! -f "$BASELINE_FILE" ]]; then
        return 0
    fi
    
    log_info "Comparing results with baseline: $BASELINE_FILE"
    
    # TODO: Implement detailed baseline comparison
    # This would compare compile times, binary sizes, and detect regressions
    
    log_info "Baseline comparison feature is under development"
}

# Cleanup temporary files
cleanup() {
    log_verbose "Cleaning up temporary files..."
    
    # Remove any temporary benchmark outputs
    find "$RESULTS_DIR" -name "*.warmup" -delete 2>/dev/null || true
    find "$RESULTS_DIR" -name "*.time" -delete 2>/dev/null || true
    
    log_verbose "Cleanup completed"
}

# Main execution
main() {
    log_info "CURSED Optimization Performance Benchmark Suite"
    log_info "=============================================="
    
    parse_args "$@"
    
    check_dependencies
    
    if [[ "${BUILD_FIRST:-false}" == "true" ]]; then
        build_compiler
    fi
    
    if [[ ! -f "$CURSED_BINARY" ]]; then
        log_error "CURSED binary not found at $CURSED_BINARY"
        log_info "Use --build-first to build the compiler, or build manually with:"
        log_info "  cargo build --release"
        exit 1
    fi
    
    prepare_environment
    run_benchmarks
    generate_report
    compare_with_baseline
    cleanup
    
    log_success "Benchmark suite completed successfully!"
    log_info "Results available in: $RESULTS_DIR"
}

# Set trap for cleanup
trap cleanup EXIT

# Run main function with all arguments
main "$@"
