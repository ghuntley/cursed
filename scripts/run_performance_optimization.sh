#!/bin/bash

# CURSED Compiler Performance Optimization Runner
# Comprehensive performance optimization and benchmarking suite

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_ROOT/zig-out/bin"
BENCHMARK_DIR="$PROJECT_ROOT/benchmarks"
RESULTS_DIR="$PROJECT_ROOT/performance_results"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Logging functions
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_header() { echo -e "${PURPLE}[PERF]${NC} $1"; }

# Performance optimization options
OPTIMIZATION_LEVEL="standard"
ENABLE_PGO="true"
ENABLE_LTO="true"
ENABLE_PROFILING="true"
ENABLE_BENCHMARKS="true"
OUTPUT_FORMAT="text"
TARGET_FILE=""
BENCHMARK_SUITE="all"

# Usage information
show_usage() {
    cat << EOF
🚀 CURSED Compiler Performance Optimization Suite

Usage: $0 [options] <command> [arguments]

Commands:
  optimize <file>         Apply comprehensive optimizations to CURSED program
  profile <file>          Profile CURSED program execution
  benchmark [suite]       Run performance benchmarks
  pgo <action> [file]     Profile-guided optimization (collect|analyze|apply)
  lto <files>             Link-time optimization
  analyze <data>          Analyze performance data
  report <format>         Generate performance reports

Options:
  --level=<level>         Optimization level (basic|standard|aggressive) [default: standard]
  --enable-pgo            Enable profile-guided optimization [default: true]
  --disable-pgo           Disable profile-guided optimization
  --enable-lto            Enable link-time optimization [default: true]
  --disable-lto           Disable link-time optimization
  --enable-profiling      Enable performance profiling [default: true]
  --disable-profiling     Disable performance profiling
  --format=<format>       Output format (text|json|csv|flamegraph|chrome) [default: text]
  --output=<file>         Output file for results
  --benchmark-suite=<suite> Benchmark suite to run (compiler|memory|concurrency|all) [default: all]
  --help                  Show this help message

Examples:
  $0 optimize --level=aggressive my_program.💀
  $0 profile --format=json --output=profile.json my_program.💀
  $0 benchmark compiler
  $0 pgo collect my_program.💀
  $0 lto --level=aggressive *.o
  $0 report --format=html performance_report.html

Environment Variables:
  CURSED_PERF_LEVEL       Default optimization level
  CURSED_PERF_OUTPUT      Default output directory
  CURSED_PARALLEL_JOBS    Number of parallel jobs for benchmarks

EOF
}

# Parse command line arguments
parse_arguments() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --level=*)
                OPTIMIZATION_LEVEL="${1#*=}"
                shift
                ;;
            --enable-pgo)
                ENABLE_PGO="true"
                shift
                ;;
            --disable-pgo)
                ENABLE_PGO="false"
                shift
                ;;
            --enable-lto)
                ENABLE_LTO="true"
                shift
                ;;
            --disable-lto)
                ENABLE_LTO="false"
                shift
                ;;
            --enable-profiling)
                ENABLE_PROFILING="true"
                shift
                ;;
            --disable-profiling)
                ENABLE_PROFILING="false"
                shift
                ;;
            --format=*)
                OUTPUT_FORMAT="${1#*=}"
                shift
                ;;
            --output=*)
                OUTPUT_FILE="${1#*=}"
                shift
                ;;
            --benchmark-suite=*)
                BENCHMARK_SUITE="${1#*=}"
                shift
                ;;
            --help)
                show_usage
                exit 0
                ;;
            -*|--*)
                log_error "Unknown option $1"
                show_usage
                exit 1
                ;;
            *)
                if [[ -z "$COMMAND" ]]; then
                    COMMAND="$1"
                else
                    COMMAND_ARGS+=("$1")
                fi
                shift
                ;;
        esac
    done
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if CURSED compiler is built
    if [[ ! -f "$BUILD_DIR/cursed-zig" ]]; then
        log_error "CURSED compiler not found. Please build it first:"
        log_error "  cd $PROJECT_ROOT && zig build"
        exit 1
    fi
    
    # Check if performance optimization CLI is built
    if [[ ! -f "$BUILD_DIR/cursed-perf" ]]; then
        log_info "Building performance optimization CLI..."
        cd "$PROJECT_ROOT"
        zig build-exe performance_optimization_cli.zig -O ReleaseFast --name cursed-perf || {
            log_error "Failed to build performance optimization CLI"
            exit 1
        }
        mv cursed-perf "$BUILD_DIR/"
    fi
    
    # Create results directory
    mkdir -p "$RESULTS_DIR"
    
    # Check system tools
    command -v valgrind >/dev/null 2>&1 || log_warning "valgrind not found - memory profiling unavailable"
    command -v perf >/dev/null 2>&1 || log_warning "perf not found - advanced CPU profiling unavailable"
    
    log_success "Prerequisites checked"
}

# Build optimized CURSED compiler
build_optimized_compiler() {
    log_info "Building optimized CURSED compiler..."
    
    cd "$PROJECT_ROOT"
    
    # Build with maximum optimizations
    local build_opts=(
        "-O ReleaseFast"
        "--strip"
    )
    
    if [[ "$ENABLE_LTO" == "true" ]]; then
        build_opts+=("-flto")
    fi
    
    zig build "${build_opts[@]}" || {
        log_error "Failed to build optimized compiler"
        exit 1
    }
    
    log_success "Optimized compiler built"
}

# Run comprehensive optimization
run_optimization() {
    local target_file="$1"
    
    log_header "Running comprehensive optimization on $target_file"
    
    if [[ ! -f "$target_file" ]]; then
        log_error "Target file not found: $target_file"
        exit 1
    fi
    
    local optimization_args=(
        "optimize"
        "--level=$OPTIMIZATION_LEVEL"
    )
    
    if [[ "$ENABLE_PGO" == "false" ]]; then
        optimization_args+=("--disable-pgo")
    fi
    
    if [[ "$ENABLE_LTO" == "false" ]]; then
        optimization_args+=("--disable-lto")
    fi
    
    optimization_args+=("$target_file")
    
    log_info "Optimization configuration:"
    log_info "  Level: $OPTIMIZATION_LEVEL"
    log_info "  PGO: $ENABLE_PGO"
    log_info "  LTO: $ENABLE_LTO"
    log_info "  Target: $target_file"
    
    "$BUILD_DIR/cursed-perf" "${optimization_args[@]}" || {
        log_error "Optimization failed"
        exit 1
    }
    
    log_success "Optimization completed"
}

# Run performance profiling
run_profiling() {
    local target_file="$1"
    
    log_header "Running performance profiling on $target_file"
    
    if [[ ! -f "$target_file" ]]; then
        log_error "Target file not found: $target_file"
        exit 1
    fi
    
    local profiling_args=(
        "profile"
        "--format=$OUTPUT_FORMAT"
    )
    
    if [[ -n "${OUTPUT_FILE:-}" ]]; then
        profiling_args+=("--output=$OUTPUT_FILE")
    else
        local timestamp=$(date +%Y%m%d_%H%M%S)
        profiling_args+=("--output=$RESULTS_DIR/profile_$timestamp")
    fi
    
    profiling_args+=("$target_file")
    
    log_info "Profiling configuration:"
    log_info "  Format: $OUTPUT_FORMAT"
    log_info "  Target: $target_file"
    
    "$BUILD_DIR/cursed-perf" "${profiling_args[@]}" || {
        log_error "Profiling failed"
        exit 1
    }
    
    log_success "Profiling completed"
}

# Run benchmarks
run_benchmarks() {
    local suite="${1:-$BENCHMARK_SUITE}"
    
    log_header "Running performance benchmarks: $suite"
    
    local benchmark_args=(
        "benchmark"
        "$suite"
    )
    
    log_info "Benchmark configuration:"
    log_info "  Suite: $suite"
    log_info "  Parallel jobs: ${CURSED_PARALLEL_JOBS:-auto}"
    
    # Set parallel jobs if specified
    if [[ -n "${CURSED_PARALLEL_JOBS:-}" ]]; then
        export NINJA_MAX_JOBS="$CURSED_PARALLEL_JOBS"
    fi
    
    "$BUILD_DIR/cursed-perf" "${benchmark_args[@]}" || {
        log_error "Benchmarks failed"
        exit 1
    }
    
    log_success "Benchmarks completed"
}

# Run PGO operations
run_pgo() {
    local action="$1"
    local target_file="${2:-}"
    
    log_header "Running PGO: $action"
    
    case "$action" in
        collect)
            if [[ -z "$target_file" ]]; then
                log_error "Target file required for PGO collection"
                exit 1
            fi
            
            log_info "Collecting PGO data from $target_file"
            "$BUILD_DIR/cursed-perf" pgo collect "$target_file" || {
                log_error "PGO collection failed"
                exit 1
            }
            ;;
        analyze)
            log_info "Analyzing PGO data"
            "$BUILD_DIR/cursed-perf" pgo analyze || {
                log_error "PGO analysis failed"
                exit 1
            }
            ;;
        apply)
            if [[ -z "$target_file" ]]; then
                log_error "Target file required for PGO application"
                exit 1
            fi
            
            log_info "Applying PGO optimizations to $target_file"
            "$BUILD_DIR/cursed-perf" pgo apply "$target_file" || {
                log_error "PGO application failed"
                exit 1
            }
            ;;
        *)
            log_error "Unknown PGO action: $action"
            log_error "Available actions: collect, analyze, apply"
            exit 1
            ;;
    esac
    
    log_success "PGO $action completed"
}

# Run LTO optimization
run_lto() {
    local object_files=("$@")
    
    log_header "Running Link-Time Optimization"
    
    if [[ ${#object_files[@]} -eq 0 ]]; then
        log_error "No object files specified for LTO"
        exit 1
    fi
    
    local lto_args=(
        "lto"
        "--opt-level=3"
    )
    
    lto_args+=("${object_files[@]}")
    
    log_info "LTO configuration:"
    log_info "  Object files: ${#object_files[@]}"
    log_info "  Optimization level: 3"
    
    "$BUILD_DIR/cursed-perf" "${lto_args[@]}" || {
        log_error "LTO failed"
        exit 1
    }
    
    log_success "LTO completed"
}

# Analyze performance data
analyze_performance_data() {
    local data_file="$1"
    
    log_header "Analyzing performance data: $data_file"
    
    if [[ ! -f "$data_file" ]]; then
        log_error "Data file not found: $data_file"
        exit 1
    fi
    
    "$BUILD_DIR/cursed-perf" analyze "$data_file" || {
        log_error "Performance analysis failed"
        exit 1
    }
    
    log_success "Performance analysis completed"
}

# Generate performance report
generate_performance_report() {
    local format="${1:-$OUTPUT_FORMAT}"
    local output_file="${2:-performance_report}"
    
    log_header "Generating performance report"
    
    "$BUILD_DIR/cursed-perf" report "$format" "$output_file" || {
        log_error "Report generation failed"
        exit 1
    }
    
    log_success "Performance report generated"
}

# Run comprehensive performance suite
run_comprehensive_suite() {
    local target_file="$1"
    
    log_header "Running comprehensive performance optimization suite"
    
    # Step 1: Build optimized compiler
    build_optimized_compiler
    
    # Step 2: Run profiling
    log_info "Step 1/5: Performance profiling"
    run_profiling "$target_file"
    
    # Step 3: Collect PGO data
    log_info "Step 2/5: PGO data collection"
    run_pgo collect "$target_file"
    
    # Step 4: Apply optimizations
    log_info "Step 3/5: Applying optimizations"
    run_optimization "$target_file"
    
    # Step 5: Run benchmarks
    log_info "Step 4/5: Running benchmarks"
    run_benchmarks
    
    # Step 6: Generate comprehensive report
    log_info "Step 5/5: Generating report"
    local timestamp=$(date +%Y%m%d_%H%M%S)
    generate_performance_report html "comprehensive_report_$timestamp"
    
    log_success "Comprehensive performance optimization suite completed"
    log_info "Results available in: $RESULTS_DIR"
}

# Memory leak detection
run_memory_analysis() {
    local target_file="$1"
    
    log_header "Running memory analysis with Valgrind"
    
    if ! command -v valgrind >/dev/null 2>&1; then
        log_warning "Valgrind not available, skipping memory analysis"
        return
    fi
    
    log_info "Compiling $target_file with debug info..."
    "$BUILD_DIR/cursed-zig" "$target_file" --debug -o "${target_file%.💀}_debug" || {
        log_error "Failed to compile with debug info"
        return
    }
    
    local debug_binary="${target_file%.💀}_debug"
    local memory_report="$RESULTS_DIR/memory_analysis_$(date +%Y%m%d_%H%M%S).txt"
    
    log_info "Running Valgrind memory analysis..."
    valgrind --tool=memcheck \
             --leak-check=full \
             --track-origins=yes \
             --show-leak-kinds=all \
             --log-file="$memory_report" \
             "./$debug_binary" || true
    
    log_info "Memory analysis report saved to: $memory_report"
    
    # Clean up debug binary
    rm -f "$debug_binary"
}

# CPU performance analysis
run_cpu_analysis() {
    local target_file="$1"
    
    log_header "Running CPU performance analysis"
    
    if ! command -v perf >/dev/null 2>&1; then
        log_warning "perf not available, skipping CPU analysis"
        return
    fi
    
    log_info "Compiling $target_file for performance analysis..."
    "$BUILD_DIR/cursed-zig" "$target_file" -O ReleaseFast -o "${target_file%.💀}_perf" || {
        log_error "Failed to compile for performance analysis"
        return
    }
    
    local perf_binary="${target_file%.💀}_perf"
    local cpu_report="$RESULTS_DIR/cpu_analysis_$(date +%Y%m%d_%H%M%S).txt"
    
    log_info "Running perf analysis..."
    perf stat -e cycles,instructions,cache-misses,branch-misses \
              -o "$cpu_report" \
              "./$perf_binary" || true
    
    log_info "CPU analysis report saved to: $cpu_report"
    
    # Clean up performance binary
    rm -f "$perf_binary"
}

# Main execution
main() {
    local COMMAND=""
    local COMMAND_ARGS=()
    
    # Parse environment variables
    OPTIMIZATION_LEVEL="${CURSED_PERF_LEVEL:-$OPTIMIZATION_LEVEL}"
    RESULTS_DIR="${CURSED_PERF_OUTPUT:-$RESULTS_DIR}"
    
    # Parse command line arguments
    parse_arguments "$@"
    
    if [[ -z "$COMMAND" ]]; then
        log_error "No command specified"
        show_usage
        exit 1
    fi
    
    # Check prerequisites
    check_prerequisites
    
    # Execute command
    case "$COMMAND" in
        optimize)
            if [[ ${#COMMAND_ARGS[@]} -eq 0 ]]; then
                log_error "No target file specified for optimization"
                exit 1
            fi
            run_optimization "${COMMAND_ARGS[0]}"
            ;;
        profile)
            if [[ ${#COMMAND_ARGS[@]} -eq 0 ]]; then
                log_error "No target file specified for profiling"
                exit 1
            fi
            run_profiling "${COMMAND_ARGS[0]}"
            ;;
        benchmark)
            run_benchmarks "${COMMAND_ARGS[0]:-}"
            ;;
        pgo)
            if [[ ${#COMMAND_ARGS[@]} -eq 0 ]]; then
                log_error "No PGO action specified"
                exit 1
            fi
            run_pgo "${COMMAND_ARGS[@]}"
            ;;
        lto)
            if [[ ${#COMMAND_ARGS[@]} -eq 0 ]]; then
                log_error "No object files specified for LTO"
                exit 1
            fi
            run_lto "${COMMAND_ARGS[@]}"
            ;;
        analyze)
            if [[ ${#COMMAND_ARGS[@]} -eq 0 ]]; then
                log_error "No data file specified for analysis"
                exit 1
            fi
            analyze_performance_data "${COMMAND_ARGS[0]}"
            ;;
        report)
            generate_performance_report "${COMMAND_ARGS[@]}"
            ;;
        comprehensive)
            if [[ ${#COMMAND_ARGS[@]} -eq 0 ]]; then
                log_error "No target file specified for comprehensive suite"
                exit 1
            fi
            run_comprehensive_suite "${COMMAND_ARGS[0]}"
            ;;
        memory)
            if [[ ${#COMMAND_ARGS[@]} -eq 0 ]]; then
                log_error "No target file specified for memory analysis"
                exit 1
            fi
            run_memory_analysis "${COMMAND_ARGS[0]}"
            ;;
        cpu)
            if [[ ${#COMMAND_ARGS[@]} -eq 0 ]]; then
                log_error "No target file specified for CPU analysis"
                exit 1
            fi
            run_cpu_analysis "${COMMAND_ARGS[0]}"
            ;;
        *)
            log_error "Unknown command: $COMMAND"
            show_usage
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
