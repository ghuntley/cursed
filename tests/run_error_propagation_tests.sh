#!/bin/bash

#
# Comprehensive Error Propagation Test Runner
#
# This script runs the complete test suite for the CURSED error propagation
# system, including integration tests, LLVM IR generation tests, and runtime
# execution tests. It provides detailed reporting and performance analysis.
#

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_RESULTS_DIR="$PROJECT_ROOT/test_results"
COVERAGE_DIR="$PROJECT_ROOT/coverage"
REPORT_FILE="$TEST_RESULTS_DIR/error_propagation_test_report.md"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test categories
INTEGRATION_TESTS="error_propagation_integration_test"
LLVM_TESTS="error_propagation_llvm_test" 
RUNTIME_TESTS="error_propagation_runtime_test"

# Performance thresholds
MAX_COMPILATION_TIME_MS=5000
MAX_EXECUTION_TIME_MS=2000

usage() {
    echo "Usage: $0 [OPTIONS]"
    echo
    echo "Options:"
    echo "  --quick                Run only basic integration tests"
    echo "  --test CATEGORY        Run specific test category:"
    echo "                           integration, llvm, runtime, all"
    echo "  --performance          Run performance benchmarks"
    echo "  --coverage             Generate coverage report"
    echo "  --report FILE          Generate detailed report to file"
    echo "  --verbose              Enable verbose output"
    echo "  --parallel             Run tests in parallel where possible"
    echo "  --help                 Show this help message"
    echo
    echo "Examples:"
    echo "  $0 --quick                    # Quick validation"
    echo "  $0 --test integration        # Integration tests only"
    echo "  $0 --performance --report     # Performance testing with report"
    echo "  $0 --coverage --verbose       # Full coverage analysis"
}

# Parse command line arguments
QUICK_MODE=false
TEST_CATEGORY="all"
RUN_PERFORMANCE=false
GENERATE_COVERAGE=false
GENERATE_REPORT=false
VERBOSE=false
PARALLEL=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)
            QUICK_MODE=true
            shift
            ;;
        --test)
            TEST_CATEGORY="$2"
            shift 2
            ;;
        --performance)
            RUN_PERFORMANCE=true
            shift
            ;;
        --coverage)
            GENERATE_COVERAGE=true
            shift
            ;;
        --report)
            GENERATE_REPORT=true
            if [[ $# -gt 1 && ! "$2" =~ ^-- ]]; then
                REPORT_FILE="$2"
                shift 2
            else
                shift
            fi
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --parallel)
            PARALLEL=true
            shift
            ;;
        --help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

log() {
    if [[ "$VERBOSE" == "true" ]]; then
        echo -e "${BLUE}[$(date '+%H:%M:%S')]${NC} $1"
    fi
}

info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Setup test environment
setup_test_environment() {
    log "Setting up test environment..."
    
    # Create necessary directories
    mkdir -p "$TEST_RESULTS_DIR"
    mkdir -p "$COVERAGE_DIR"
    
    # Change to project root
    cd "$PROJECT_ROOT"
    
    # Apply linking fix if needed (for Nix environment)
    if [[ -f "./fix_linking.sh" ]]; then
        log "Applying linking fix for Nix environment"
        export CARGO_WRAPPER="./fix_linking.sh"
    else
        export CARGO_WRAPPER=""
    fi
}

# Run a single test with timing and error handling
run_test_category() {
    local category="$1"
    local test_name="$2"
    local start_time=$(date +%s%N)
    
    info "Running $category tests: $test_name"
    
    local cmd="cargo test --test $test_name"
    if [[ "$VERBOSE" == "true" ]]; then
        cmd="$cmd -- --nocapture"
    fi
    
    if [[ -n "$CARGO_WRAPPER" ]]; then
        cmd="$CARGO_WRAPPER $cmd"
    fi
    
    log "Executing: $cmd"
    
    if eval "$cmd"; then
        local end_time=$(date +%s%N)
        local duration_ms=$(( (end_time - start_time) / 1000000 ))
        
        info "✓ $category tests passed (${duration_ms}ms)"
        echo "$category,$test_name,PASS,$duration_ms" >> "$TEST_RESULTS_DIR/test_times.csv"
        return 0
    else
        local end_time=$(date +%s%N)
        local duration_ms=$(( (end_time - start_time) / 1000000 ))
        
        error "✗ $category tests failed (${duration_ms}ms)"
        echo "$category,$test_name,FAIL,$duration_ms" >> "$TEST_RESULTS_DIR/test_times.csv"
        return 1
    fi
}

# Run integration tests
run_integration_tests() {
    if [[ "$TEST_CATEGORY" == "integration" || "$TEST_CATEGORY" == "all" ]]; then
        info "Running error propagation integration tests..."
        run_test_category "Integration" "$INTEGRATION_TESTS"
        return $?
    fi
    return 0
}

# Run LLVM IR generation tests
run_llvm_tests() {
    if [[ "$TEST_CATEGORY" == "llvm" || "$TEST_CATEGORY" == "all" ]]; then
        info "Running LLVM IR generation tests..."
        run_test_category "LLVM" "$LLVM_TESTS"
        return $?
    fi
    return 0
}

# Run runtime execution tests
run_runtime_tests() {
    if [[ "$TEST_CATEGORY" == "runtime" || "$TEST_CATEGORY" == "all" ]]; then
        info "Running runtime execution tests..."
        run_test_category "Runtime" "$RUNTIME_TESTS"
        return $?
    fi
    return 0
}

# Run performance benchmarks
run_performance_tests() {
    if [[ "$RUN_PERFORMANCE" == "true" ]]; then
        info "Running performance benchmarks..."
        
        # Run specific performance tests
        local performance_tests=(
            "test_error_propagation_performance"
            "test_error_propagation_compilation_benchmark"
            "test_runtime_performance_success_path"
            "test_runtime_performance_error_path"
        )
        
        for test in "${performance_tests[@]}"; do
            info "Running performance test: $test"
            local cmd="cargo test --test error_propagation_integration_test $test"
            if [[ -n "$CARGO_WRAPPER" ]]; then
                cmd="$CARGO_WRAPPER $cmd"
            fi
            
            if ! eval "$cmd"; then
                warn "Performance test $test failed"
            fi
        done
    fi
}

# Generate coverage report
generate_coverage_report() {
    if [[ "$GENERATE_COVERAGE" == "true" ]]; then
        info "Generating coverage report..."
        
        # Check if cargo-tarpaulin is available
        if ! command -v cargo-tarpaulin &> /dev/null; then
            warn "cargo-tarpaulin not found, skipping coverage report"
            return 0
        fi
        
        local coverage_cmd="cargo tarpaulin --out Html --output-dir $COVERAGE_DIR"
        coverage_cmd="$coverage_cmd --include-tests"
        coverage_cmd="$coverage_cmd --exclude-files 'target/*' 'tests/*'"
        
        # Focus on error propagation modules
        coverage_cmd="$coverage_cmd --packages cursed"
        
        if [[ -n "$CARGO_WRAPPER" ]]; then
            coverage_cmd="$CARGO_WRAPPER $coverage_cmd"
        fi
        
        log "Executing coverage: $coverage_cmd"
        
        if eval "$coverage_cmd"; then
            info "✓ Coverage report generated at $COVERAGE_DIR/tarpaulin-report.html"
        else
            warn "Coverage report generation failed"
        fi
    fi
}

# Generate detailed test report
generate_test_report() {
    if [[ "$GENERATE_REPORT" == "true" ]]; then
        info "Generating detailed test report..."
        
        cat > "$REPORT_FILE" << EOF
# Error Propagation Test Report

Generated: $(date)
Test Category: $TEST_CATEGORY
Quick Mode: $QUICK_MODE

## Test Summary

EOF
        
        if [[ -f "$TEST_RESULTS_DIR/test_times.csv" ]]; then
            echo "### Test Execution Times" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
            echo "| Category | Test | Status | Duration (ms) |" >> "$REPORT_FILE"
            echo "|----------|------|--------|---------------|" >> "$REPORT_FILE"
            
            while IFS=',' read -r category test status duration; do
                echo "| $category | $test | $status | $duration |" >> "$REPORT_FILE"
            done < "$TEST_RESULTS_DIR/test_times.csv"
            
            echo "" >> "$REPORT_FILE"
        fi
        
        echo "### Performance Analysis" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "- Compilation time threshold: ${MAX_COMPILATION_TIME_MS}ms" >> "$REPORT_FILE"
        echo "- Execution time threshold: ${MAX_EXECUTION_TIME_MS}ms" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        
        if [[ "$GENERATE_COVERAGE" == "true" && -f "$COVERAGE_DIR/tarpaulin-report.html" ]]; then
            echo "### Coverage Report" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
            echo "Coverage report available at: [$COVERAGE_DIR/tarpaulin-report.html](file://$COVERAGE_DIR/tarpaulin-report.html)" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
        fi
        
        echo "### Test Categories Executed" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "- **Integration Tests**: End-to-end error propagation functionality" >> "$REPORT_FILE"
        echo "- **LLVM Tests**: IR generation correctness and optimization" >> "$REPORT_FILE"
        echo "- **Runtime Tests**: Execution behavior and performance" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        
        echo "### Key Test Areas" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "1. **Basic Error Propagation**: \`?\` operator with Result<T,E> and Option<T>" >> "$REPORT_FILE"
        echo "2. **Chained Propagation**: Multiple \`?\` operators in sequence" >> "$REPORT_FILE"
        echo "3. **Type System Integration**: Generic types and custom error types" >> "$REPORT_FILE"
        echo "4. **Control Flow**: Error propagation in if/else, loops, and complex expressions" >> "$REPORT_FILE"
        echo "5. **Memory Safety**: Resource cleanup and memory management" >> "$REPORT_FILE"
        echo "6. **Performance**: Compilation and execution performance characteristics" >> "$REPORT_FILE"
        echo "7. **Concurrency**: Thread safety and concurrent execution" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        
        info "✓ Test report generated at $REPORT_FILE"
    fi
}

# Run quick validation tests
run_quick_tests() {
    info "Running quick error propagation validation..."
    
    local quick_tests=(
        "test_error_propagation_modules_enabled"
        "test_basic_error_propagation_compilation"
        "test_option_error_propagation"
        "test_integration_all_modules"
    )
    
    for test in "${quick_tests[@]}"; do
        local cmd="cargo test --test error_propagation_integration_test $test"
        if [[ -n "$CARGO_WRAPPER" ]]; then
            cmd="$CARGO_WRAPPER $cmd"
        fi
        
        if ! eval "$cmd -- --nocapture"; then
            error "Quick test $test failed"
            return 1
        fi
    done
    
    info "✓ Quick validation completed successfully"
    return 0
}

# Main execution function
main() {
    local start_time=$(date +%s)
    
    info "Starting error propagation test suite..."
    log "Configuration: category=$TEST_CATEGORY, quick=$QUICK_MODE, performance=$RUN_PERFORMANCE"
    
    # Setup environment
    setup_test_environment
    
    # Initialize test results tracking
    echo "Category,Test,Status,Duration" > "$TEST_RESULTS_DIR/test_times.csv"
    
    local exit_code=0
    
    # Run quick tests if requested
    if [[ "$QUICK_MODE" == "true" ]]; then
        if ! run_quick_tests; then
            exit_code=1
        fi
    else
        # Run full test suite
        if ! run_integration_tests; then
            exit_code=1
        fi
        
        if ! run_llvm_tests; then
            exit_code=1
        fi
        
        if ! run_runtime_tests; then
            exit_code=1
        fi
        
        # Run performance tests if requested
        run_performance_tests
    fi
    
    # Generate coverage report if requested
    generate_coverage_report
    
    # Generate detailed report if requested
    generate_test_report
    
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    
    if [[ $exit_code -eq 0 ]]; then
        info "✓ All error propagation tests completed successfully in ${total_duration}s"
    else
        error "✗ Some error propagation tests failed (total time: ${total_duration}s)"
    fi
    
    return $exit_code
}

# Execute main function
main "$@"
