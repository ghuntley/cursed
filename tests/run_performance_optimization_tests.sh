#!/bin/bash

# Performance Optimization System Test Runner
# 
# Comprehensive test suite for the CURSED performance optimization system
# including advanced LLVM passes, PGO, build optimization, and benchmarking

set -euo pipefail

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
VERBOSE=false
QUICK=false
REPORT=false
COVERAGE=false
BENCHMARK=false
IGNORED=false
OUTPUT_DIR="$PROJECT_ROOT/test_results"
REPORT_FILE="$OUTPUT_DIR/performance_optimization_test_report.md"

# Test categories
TESTS_BASIC=(
    "comprehensive_performance_optimization_test"
)

TESTS_BENCHMARKS=(
    "performance_optimization_benchmarks"
)

TESTS_IGNORED=(
    "performance_optimization_benchmarks --ignored"
)

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[0;37m'
BOLD='\033[1m'
RESET='\033[0m'

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${RESET}"
}

log_success() {
    echo -e "${GREEN}✅ $1${RESET}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${RESET}"
}

log_error() {
    echo -e "${RED}❌ $1${RESET}"
}

log_section() {
    echo -e "\n${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"
    echo -e "${BOLD}${CYAN}$1${RESET}"
    echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}\n"
}

show_usage() {
    cat << EOF
Performance Optimization System Test Runner

USAGE:
    $0 [OPTIONS] [TEST_CATEGORY]

OPTIONS:
    -h, --help      Show this help message
    -v, --verbose   Enable verbose output
    -q, --quick     Run only quick tests (skip benchmarks)
    -r, --report    Generate detailed test report
    -c, --coverage  Generate code coverage report
    -b, --benchmark Run performance benchmarks
    -i, --ignored   Run ignored/slow tests
    --all           Run all tests including benchmarks and ignored tests

TEST_CATEGORIES:
    basic           Run basic performance optimization tests
    benchmarks      Run performance optimization benchmarks
    all             Run all test categories

EXAMPLES:
    $0                      # Run basic tests
    $0 --quick             # Run quick tests only
    $0 --benchmark         # Run performance benchmarks
    $0 --all --report      # Run all tests and generate report
    $0 benchmarks          # Run only benchmark tests
    $0 --coverage          # Run with coverage analysis

EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -q|--quick)
                QUICK=true
                shift
                ;;
            -r|--report)
                REPORT=true
                shift
                ;;
            -c|--coverage)
                COVERAGE=true
                shift
                ;;
            -b|--benchmark)
                BENCHMARK=true
                shift
                ;;
            -i|--ignored)
                IGNORED=true
                shift
                ;;
            --all)
                BENCHMARK=true
                IGNORED=true
                REPORT=true
                shift
                ;;
            basic|benchmarks|all)
                TEST_CATEGORY="$1"
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
}

# Setup test environment
setup_environment() {
    log_section "Setting Up Test Environment"
    
    # Create output directory
    mkdir -p "$OUTPUT_DIR"
    
    # Check if we're in a Nix environment and apply linking fix
    if [[ -n "${NIX_STORE:-}" ]]; then
        log_info "Detected Nix environment, applying linking fix..."
        if [[ -f "$PROJECT_ROOT/fix_linking.sh" ]]; then
            export CARGO_CMD="$PROJECT_ROOT/fix_linking.sh cargo"
        else
            log_warning "fix_linking.sh not found, may encounter linking issues"
            export CARGO_CMD="cargo"
        fi
    else
        export CARGO_CMD="cargo"
    fi
    
    # Set environment variables for testing
    export RUST_LOG="${RUST_LOG:-info}"
    export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"
    
    # Create test workspace directory
    export TEST_WORKSPACE="$OUTPUT_DIR/performance_optimization_workspace"
    mkdir -p "$TEST_WORKSPACE"
    
    log_success "Test environment set up successfully"
}

# Run a single test with proper error handling
run_test() {
    local test_name="$1"
    local description="$2"
    local start_time
    local end_time
    local duration
    
    log_info "Running: $description"
    
    start_time=$(date +%s.%N)
    
    local cmd_args=("test" "--test" "$test_name")
    
    if [[ "$VERBOSE" == "true" ]]; then
        cmd_args+=("--" "--nocapture")
    fi
    
    if [[ "$COVERAGE" == "true" ]]; then
        # Use cargo-tarpaulin for coverage if available
        if command -v cargo-tarpaulin >/dev/null 2>&1; then
            cmd_args[0]="tarpaulin"
            cmd_args+=("--out" "Html" "--output-dir" "$OUTPUT_DIR/coverage")
        fi
    fi
    
    if cd "$PROJECT_ROOT" && $CARGO_CMD "${cmd_args[@]}"; then
        end_time=$(date +%s.%N)
        duration=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "N/A")
        log_success "$description completed in ${duration}s"
        return 0
    else
        end_time=$(date +%s.%N)
        duration=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "N/A")
        log_error "$description failed after ${duration}s"
        return 1
    fi
}

# Run basic performance optimization tests
run_basic_tests() {
    log_section "Running Basic Performance Optimization Tests"
    
    local failed_tests=0
    local total_tests=${#TESTS_BASIC[@]}
    
    for test in "${TESTS_BASIC[@]}"; do
        if ! run_test "$test" "Performance Optimization Test: $test"; then
            ((failed_tests++))
        fi
    done
    
    if [[ $failed_tests -eq 0 ]]; then
        log_success "All basic tests passed ($total_tests/$total_tests)"
    else
        log_error "$failed_tests out of $total_tests basic tests failed"
    fi
    
    return $failed_tests
}

# Run performance benchmark tests
run_benchmark_tests() {
    log_section "Running Performance Optimization Benchmarks"
    
    local failed_tests=0
    local total_tests=${#TESTS_BENCHMARKS[@]}
    
    for test in "${TESTS_BENCHMARKS[@]}"; do
        if ! run_test "$test" "Performance Benchmark: $test"; then
            ((failed_tests++))
        fi
    done
    
    if [[ $failed_tests -eq 0 ]]; then
        log_success "All benchmark tests passed ($total_tests/$total_tests)"
    else
        log_error "$failed_tests out of $total_tests benchmark tests failed"
    fi
    
    return $failed_tests
}

# Run ignored/slow tests
run_ignored_tests() {
    log_section "Running Ignored/Slow Performance Tests"
    
    local failed_tests=0
    local total_tests=${#TESTS_IGNORED[@]}
    
    for test in "${TESTS_IGNORED[@]}"; do
        # Split test name and args
        read -ra test_parts <<< "$test"
        local test_name="${test_parts[0]}"
        local test_args="${test_parts[@]:1}"
        
        log_info "Running ignored test: $test_name with args: $test_args"
        
        local cmd_args=("test" "--test" "$test_name" "--" "$test_args")
        
        if [[ "$VERBOSE" == "true" ]]; then
            cmd_args+=("--nocapture")
        fi
        
        if cd "$PROJECT_ROOT" && $CARGO_CMD "${cmd_args[@]}"; then
            log_success "Ignored test completed: $test_name"
        else
            log_error "Ignored test failed: $test_name"
            ((failed_tests++))
        fi
    done
    
    if [[ $failed_tests -eq 0 ]]; then
        log_success "All ignored tests passed ($total_tests/$total_tests)"
    else
        log_error "$failed_tests out of $total_tests ignored tests failed"
    fi
    
    return $failed_tests
}

# Generate comprehensive test report
generate_report() {
    log_section "Generating Test Report"
    
    cat > "$REPORT_FILE" << EOF
# Performance Optimization System Test Report

Generated: $(date '+%Y-%m-%d %H:%M:%S')
Test Runner: $0
Project: CURSED Programming Language

## Test Environment

- Operating System: $(uname -s)
- Architecture: $(uname -m)
- Rust Version: $(rustc --version 2>/dev/null || echo "N/A")
- Cargo Version: $(cargo --version 2>/dev/null || echo "N/A")
- Nix Environment: ${NIX_STORE:+Yes}${NIX_STORE:-No}

## Test Configuration

- Verbose Output: $VERBOSE
- Quick Mode: $QUICK
- Benchmarks: $BENCHMARK
- Ignored Tests: $IGNORED
- Coverage Analysis: $COVERAGE

## Test Categories Executed

EOF
    
    if [[ "${TEST_CATEGORY:-all}" == "all" ]] || [[ "${TEST_CATEGORY:-}" == "basic" ]]; then
        echo "- ✅ Basic Performance Optimization Tests" >> "$REPORT_FILE"
    fi
    
    if [[ "$BENCHMARK" == "true" ]] || [[ "${TEST_CATEGORY:-}" == "benchmarks" ]]; then
        echo "- ✅ Performance Optimization Benchmarks" >> "$REPORT_FILE"
    fi
    
    if [[ "$IGNORED" == "true" ]]; then
        echo "- ✅ Ignored/Slow Performance Tests" >> "$REPORT_FILE"
    fi
    
    cat >> "$REPORT_FILE" << EOF

## Test Results Summary

$(cd "$PROJECT_ROOT" && $CARGO_CMD test --test comprehensive_performance_optimization_test -- --list 2>/dev/null | grep -c "test" || echo "N/A") comprehensive optimization tests
$(cd "$PROJECT_ROOT" && $CARGO_CMD test --test performance_optimization_benchmarks -- --list 2>/dev/null | grep -c "test" || echo "N/A") performance benchmark tests

## Performance Metrics

EOF
    
    # Add performance metrics if available
    if [[ -f "$OUTPUT_DIR/test_performance_export.json" ]]; then
        echo "Performance data exported to: test_performance_export.json" >> "$REPORT_FILE"
    fi
    
    cat >> "$REPORT_FILE" << EOF

## Recommendations

Based on the test results, consider the following optimizations:

1. **Compilation Performance**: Review build optimization settings
2. **Runtime Performance**: Analyze PGO effectiveness 
3. **Memory Usage**: Monitor memory consumption patterns
4. **Cache Effectiveness**: Optimize compilation caching strategies

## Next Steps

- Run regular performance benchmarks to track improvements
- Monitor for performance regressions in CI/CD
- Consider enabling PGO for production builds
- Analyze detailed performance metrics for optimization opportunities

---
Report generated by CURSED Performance Optimization Test Suite
EOF
    
    log_success "Test report generated: $REPORT_FILE"
}

# Generate coverage report
generate_coverage() {
    if command -v cargo-tarpaulin >/dev/null 2>&1; then
        log_section "Generating Code Coverage Report"
        
        local coverage_dir="$OUTPUT_DIR/coverage"
        mkdir -p "$coverage_dir"
        
        cd "$PROJECT_ROOT"
        $CARGO_CMD tarpaulin \
            --tests \
            --out Html \
            --output-dir "$coverage_dir" \
            --include-tests \
            2>/dev/null || log_warning "Coverage generation failed"
        
        if [[ -f "$coverage_dir/tarpaulin-report.html" ]]; then
            log_success "Coverage report generated: $coverage_dir/tarpaulin-report.html"
        fi
    else
        log_warning "cargo-tarpaulin not found, skipping coverage report"
        log_info "Install with: cargo install cargo-tarpaulin"
    fi
}

# Main execution function
main() {
    local start_time
    local end_time
    local total_duration
    local failed_tests=0
    
    start_time=$(date +%s)
    
    # Parse arguments
    parse_args "$@"
    
    # Setup environment
    setup_environment
    
    log_section "CURSED Performance Optimization System Test Suite"
    log_info "Starting comprehensive performance optimization tests..."
    
    # Run tests based on configuration
    case "${TEST_CATEGORY:-all}" in
        basic)
            if ! run_basic_tests; then
                ((failed_tests++))
            fi
            ;;
        benchmarks)
            if ! run_benchmark_tests; then
                ((failed_tests++))
            fi
            ;;
        all)
            # Run basic tests unless quick mode
            if [[ "$QUICK" != "true" ]]; then
                if ! run_basic_tests; then
                    ((failed_tests++))
                fi
                
                # Run benchmarks if enabled
                if [[ "$BENCHMARK" == "true" ]]; then
                    if ! run_benchmark_tests; then
                        ((failed_tests++))
                    fi
                fi
                
                # Run ignored tests if enabled
                if [[ "$IGNORED" == "true" ]]; then
                    if ! run_ignored_tests; then
                        ((failed_tests++))
                    fi
                fi
            else
                log_info "Quick mode: running basic tests only"
                if ! run_basic_tests; then
                    ((failed_tests++))
                fi
            fi
            ;;
        *)
            log_error "Unknown test category: ${TEST_CATEGORY:-}"
            exit 1
            ;;
    esac
    
    # Generate reports if requested
    if [[ "$REPORT" == "true" ]]; then
        generate_report
    fi
    
    if [[ "$COVERAGE" == "true" ]]; then
        generate_coverage
    fi
    
    # Calculate duration
    end_time=$(date +%s)
    total_duration=$((end_time - start_time))
    
    # Final summary
    log_section "Test Suite Summary"
    
    if [[ $failed_tests -eq 0 ]]; then
        log_success "All performance optimization tests passed! 🎉"
        log_info "Total execution time: ${total_duration}s"
        exit 0
    else
        log_error "$failed_tests test category(ies) failed"
        log_info "Total execution time: ${total_duration}s"
        log_info "Check the logs above for details on failed tests"
        exit 1
    fi
}

# Handle script interruption
trap 'log_warning "Test execution interrupted"; exit 130' INT TERM

# Run main function with all arguments
main "$@"
