#!/bin/bash

# CURSED Performance Optimization Test Runner
# Runs comprehensive performance optimization tests with linking fixes and detailed reporting

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LINKING_FIX="$PROJECT_ROOT/fix_linking.sh"
VERBOSE=${VERBOSE:-0}
QUICK=${QUICK:-0}
IGNORED=${IGNORED:-0}
COVERAGE=${COVERAGE:-0}
REPORT_FILE=""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${CYAN}[INFO]${NC} $1"
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

# Print usage information
show_help() {
    cat << EOF
CURSED Performance Optimization Test Runner

USAGE:
    $0 [OPTIONS]

OPTIONS:
    --quick         Run quick performance validation tests only
    --full          Run complete performance test suite including stress tests
    --ignored       Run ignored performance tests (stress and long-running tests)
    --test TYPE     Run specific test type (unit|integration|performance|comprehensive)
    --verbose       Enable verbose output
    --coverage      Generate code coverage report
    --report FILE   Generate detailed test report to specified file
    --help          Show this help message

EXAMPLES:
    $0                              # Run standard performance tests
    $0 --quick                      # Quick validation
    $0 --full --coverage            # Complete suite with coverage
    $0 --test performance --verbose # Performance tests with detailed output
    $0 --report perf_report.md      # Generate markdown report

PERFORMANCE TEST CATEGORIES:
    - Enhanced GC Performance
    - Type Conversion Performance  
    - Channels Performance
    - LLVM Optimization Effectiveness
    - Profile-Guided Optimization (PGO)
    - Compilation Speed Improvements
    - Comprehensive Optimization Pipeline

EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --quick)
                QUICK=1
                shift
                ;;
            --full)
                IGNORED=1
                shift
                ;;
            --ignored)
                IGNORED=1
                shift
                ;;
            --test)
                TEST_TYPE="$2"
                shift 2
                ;;
            --verbose)
                VERBOSE=1
                shift
                ;;
            --coverage)
                COVERAGE=1
                shift
                ;;
            --report)
                REPORT_FILE="$2"
                shift 2
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# Check if linking fix is available
check_linking_fix() {
    if [[ ! -f "$LINKING_FIX" ]]; then
        log_error "Linking fix script not found: $LINKING_FIX"
        log_info "Please ensure fix_linking.sh exists in the project root"
        exit 1
    fi

    if [[ ! -x "$LINKING_FIX" ]]; then
        log_warning "Making linking fix script executable..."
        chmod +x "$LINKING_FIX"
    fi
}

# Run a test with linking fix and capture output
run_test_with_linking_fix() {
    local test_name="$1"
    local test_args="${2:-}"
    local output_file=$(mktemp)
    local start_time=$(date +%s)
    
    log_info "Running $test_name..."
    
    if [[ $VERBOSE -eq 1 ]]; then
        if "$LINKING_FIX" cargo test --test "$test_name" $test_args 2>&1 | tee "$output_file"; then
            local exit_code=0
        else
            local exit_code=$?
        fi
    else
        if "$LINKING_FIX" cargo test --test "$test_name" $test_args > "$output_file" 2>&1; then
            local exit_code=0
        else
            local exit_code=$?
        fi
    fi
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    if [[ $exit_code -eq 0 ]]; then
        local passed=$(grep -c "test result: ok" "$output_file" || echo "0")
        log_success "$test_name completed successfully in ${duration}s ($passed tests passed)"
        
        # Extract performance metrics if available
        if grep -q "throughput\|latency\|speedup\|improvement" "$output_file"; then
            log_info "Performance metrics found:"
            grep -E "throughput|latency|speedup|improvement|efficiency" "$output_file" | head -5 | sed 's/^/  /'
        fi
    else
        log_error "$test_name failed in ${duration}s (exit code: $exit_code)"
        if [[ $VERBOSE -eq 0 ]]; then
            echo "Error output:"
            tail -20 "$output_file" | sed 's/^/  /'
        fi
    fi
    
    # Store results for reporting
    echo "$test_name,$exit_code,$duration,$(wc -l < "$output_file")" >> "${REPORT_FILE}.csv" 2>/dev/null || true
    
    rm -f "$output_file"
    return $exit_code
}

# Performance test suite runner
run_performance_tests() {
    local test_type="${1:-all}"
    local failed_tests=()
    local passed_tests=()
    local total_duration=0
    
    log_info "Starting performance optimization tests (type: $test_type)..."
    
    # Test configuration based on mode
    local test_args=""
    if [[ $IGNORED -eq 1 ]]; then
        test_args="-- --ignored"
        log_info "Including ignored stress and performance tests"
    elif [[ $QUICK -eq 1 ]]; then
        test_args="-- --quiet"
        log_info "Running quick performance validation tests"
    fi
    
    # Performance test cases
    case $test_type in
        "gc"|"enhanced_gc"|"all")
            log_info "=== Enhanced GC Performance Tests ==="
            if run_test_with_linking_fix "enhanced_gc_performance_test" "$test_args"; then
                passed_tests+=("enhanced_gc_performance_test")
            else
                failed_tests+=("enhanced_gc_performance_test")
            fi
            ;;
    esac
    
    case $test_type in
        "type_conversion"|"conversions"|"all")
            log_info "=== Type Conversion Performance Tests ==="
            if run_test_with_linking_fix "type_conversion_performance_test" "$test_args"; then
                passed_tests+=("type_conversion_performance_test")
            else
                failed_tests+=("type_conversion_performance_test")
            fi
            ;;
    esac
    
    case $test_type in
        "channels"|"messaging"|"all")
            log_info "=== Channels Performance Tests ==="
            if run_test_with_linking_fix "channels_performance_test" "$test_args"; then
                passed_tests+=("channels_performance_test")
            else
                failed_tests+=("channels_performance_test")
            fi
            ;;
    esac
    
    case $test_type in
        "comprehensive"|"optimization"|"all")
            log_info "=== Comprehensive Optimization Tests ==="
            if run_test_with_linking_fix "performance_optimization_comprehensive_test" "$test_args"; then
                passed_tests+=("performance_optimization_comprehensive_test")
            else
                failed_tests+=("performance_optimization_comprehensive_test")
            fi
            ;;
    esac
    
    # Additional performance tests if running full suite
    if [[ $test_type == "all" ]] && [[ $IGNORED -eq 1 ]]; then
        log_info "=== Additional Performance Tests ==="
        
        # Run existing performance tests
        local additional_tests=(
            "performance_benchmark_test"
            "performance_optimization_test"
            "optimization_performance_test"
            "comprehensive_performance_optimization_test"
        )
        
        for test in "${additional_tests[@]}"; do
            if [[ -f "$PROJECT_ROOT/tests/$test.rs" ]]; then
                if run_test_with_linking_fix "$test" "$test_args"; then
                    passed_tests+=("$test")
                else
                    failed_tests+=("$test")
                fi
            fi
        done
    fi
    
    # Performance summary
    log_info "=== Performance Test Summary ==="
    echo "Passed tests: ${#passed_tests[@]}"
    echo "Failed tests: ${#failed_tests[@]}"
    
    if [[ ${#passed_tests[@]} -gt 0 ]]; then
        log_success "Passed: ${passed_tests[*]}"
    fi
    
    if [[ ${#failed_tests[@]} -gt 0 ]]; then
        log_error "Failed: ${failed_tests[*]}"
        return 1
    fi
    
    log_success "All performance tests completed successfully!"
    return 0
}

# Generate test coverage report
generate_coverage_report() {
    if [[ $COVERAGE -eq 1 ]]; then
        log_info "Generating performance test coverage report..."
        
        if command -v cargo-tarpaulin >/dev/null 2>&1; then
            local coverage_args="--tests --out Xml --out Html --output-dir coverage/performance"
            
            # Add specific performance test patterns
            coverage_args="$coverage_args --include-tests *performance* *optimization*"
            
            if "$LINKING_FIX" cargo tarpaulin $coverage_args; then
                log_success "Coverage report generated in coverage/performance/"
                
                if [[ -f "coverage/performance/tarpaulin-report.html" ]]; then
                    log_info "HTML report: coverage/performance/tarpaulin-report.html"
                fi
            else
                log_warning "Coverage generation failed, continuing..."
            fi
        else
            log_warning "cargo-tarpaulin not found, skipping coverage report"
            log_info "Install with: cargo install cargo-tarpaulin"
        fi
    fi
}

# Generate detailed test report
generate_detailed_report() {
    if [[ -n "$REPORT_FILE" ]]; then
        log_info "Generating detailed test report: $REPORT_FILE"
        
        {
            echo "# CURSED Performance Optimization Test Report"
            echo
            echo "Generated on: $(date)"
            echo "Test mode: $([ $QUICK -eq 1 ] && echo "Quick" || ([ $IGNORED -eq 1 ] && echo "Full" || echo "Standard"))"
            echo
            
            echo "## Test Execution Summary"
            echo
            if [[ -f "${REPORT_FILE}.csv" ]]; then
                echo "| Test Name | Status | Duration (s) | Output Lines |"
                echo "|-----------|--------|--------------|--------------|"
                
                while IFS=',' read -r test_name exit_code duration lines; do
                    local status=$([ "$exit_code" -eq 0 ] && echo "✅ PASS" || echo "❌ FAIL")
                    echo "| $test_name | $status | $duration | $lines |"
                done < "${REPORT_FILE}.csv"
            fi
            
            echo
            echo "## Performance Benchmarks"
            echo
            echo "### GC Performance"
            echo "- Allocation throughput target: >1000 objects/sec"
            echo "- Collection pause time target: <100ms average"
            echo "- Memory efficiency target: >85%"
            echo
            
            echo "### Type Conversion Performance"
            echo "- Primitive conversion target: >10K conversions/sec"
            echo "- Complex conversion target: >1K conversions/sec"
            echo "- Error handling overhead: <3x"
            echo
            
            echo "### Channel Performance"
            echo "- Message passing target: >10K messages/sec"
            echo "- Multi-producer scaling: >60% efficiency"
            echo "- Backpressure handling: graceful degradation"
            echo
            
            echo "### LLVM Optimization Effectiveness"
            echo "- Execution improvement: >1.2x speedup"
            echo "- Binary size reduction: >10%"
            echo "- Compilation overhead: <3x"
            echo
            
            echo "## System Information"
            echo "- Platform: $(uname -s -m)"
            echo "- Rust version: $(rustc --version)"
            echo "- CPU cores: $(nproc 2>/dev/null || echo "unknown")"
            echo "- Memory: $(free -h 2>/dev/null | grep "Mem:" | awk '{print $2}' || echo "unknown")"
            
        } > "$REPORT_FILE"
        
        log_success "Report generated: $REPORT_FILE"
        
        # Cleanup temporary CSV file
        rm -f "${REPORT_FILE}.csv"
    fi
}

# Cleanup function
cleanup() {
    log_info "Cleaning up temporary files..."
    rm -f /tmp/cursed_perf_test_*
}

# Main execution
main() {
    # Set up cleanup on exit
    trap cleanup EXIT
    
    # Parse arguments
    parse_args "$@"
    
    # Check prerequisites
    check_linking_fix
    
    # Change to project root
    cd "$PROJECT_ROOT"
    
    # Initialize report file if specified
    if [[ -n "$REPORT_FILE" ]]; then
        echo "test_name,exit_code,duration,output_lines" > "${REPORT_FILE}.csv"
    fi
    
    log_info "CURSED Performance Optimization Test Suite"
    log_info "Project root: $PROJECT_ROOT"
    log_info "Linking fix: $LINKING_FIX"
    
    # Run the performance tests
    local test_type="${TEST_TYPE:-all}"
    local overall_success=0
    
    if run_performance_tests "$test_type"; then
        overall_success=0
    else
        overall_success=1
    fi
    
    # Generate coverage report if requested
    generate_coverage_report
    
    # Generate detailed report if requested
    generate_detailed_report
    
    # Final status
    if [[ $overall_success -eq 0 ]]; then
        log_success "Performance optimization test suite completed successfully!"
    else
        log_error "Performance optimization test suite failed!"
    fi
    
    exit $overall_success
}

# Execute main function with all arguments
main "$@"
