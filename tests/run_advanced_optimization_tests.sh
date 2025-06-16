#!/bin/bash

# Advanced LLVM Optimization System Test Runner
# Comprehensive testing for the CURSED advanced optimization system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
QUICK_MODE=false
VERBOSE=false
GENERATE_REPORT=false
REPORT_FILE=""
TEST_FILTER=""

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
show_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Test runner for the Advanced LLVM Optimization System

OPTIONS:
    --quick             Run quick tests only (skip stress tests)
    --verbose           Enable verbose output
    --report FILE       Generate test report to FILE
    --test FILTER       Run only tests matching FILTER
    --help              Show this help message

EXAMPLES:
    $0                           # Run all tests
    $0 --quick                   # Run quick tests only
    $0 --verbose --report report.md
    $0 --test loop_detection     # Run loop detection tests only

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)
            QUICK_MODE=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --report)
            GENERATE_REPORT=true
            REPORT_FILE="$2"
            shift 2
            ;;
        --test)
            TEST_FILTER="$2"
            shift 2
            ;;
        --help)
            show_usage
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Check if linking fix is available
if [[ -f "./fix_linking.sh" ]]; then
    print_status "Using linking fix for Nix environment"
    CARGO_CMD="./fix_linking.sh cargo"
else
    CARGO_CMD="cargo"
fi

# Set up environment
export RUST_LOG=${RUST_LOG:-"debug"}
export RUST_BACKTRACE=${RUST_BACKTRACE:-"1"}

if [[ "$VERBOSE" == "true" ]]; then
    export RUST_LOG="trace"
fi

# Initialize report if requested
if [[ "$GENERATE_REPORT" == "true" ]]; then
    if [[ -z "$REPORT_FILE" ]]; then
        REPORT_FILE="advanced_optimization_test_report_$(date +%Y%m%d_%H%M%S).md"
    fi
    
    cat > "$REPORT_FILE" << EOF
# Advanced LLVM Optimization System Test Report

**Generated:** $(date)
**System:** $(uname -a)
**Rust Version:** $(rustc --version)

## Test Configuration
- Quick Mode: $QUICK_MODE
- Verbose: $VERBOSE
- Test Filter: ${TEST_FILTER:-"(all tests)"}

## Test Results

EOF
    print_status "Test report will be written to: $REPORT_FILE"
fi

# Function to log test result
log_test_result() {
    local test_name="$1"
    local status="$2"
    local duration="$3"
    local output="$4"
    
    if [[ "$GENERATE_REPORT" == "true" ]]; then
        cat >> "$REPORT_FILE" << EOF
### $test_name
- **Status:** $status
- **Duration:** $duration
- **Output:**
\`\`\`
$output
\`\`\`

EOF
    fi
}

# Function to run a test with error handling
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    print_status "Running test: $test_name"
    
    local start_time=$(date +%s)
    
    if [[ "$VERBOSE" == "true" ]]; then
        local output
        if output=$(eval "$test_command" 2>&1); then
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            print_success "$test_name completed in ${duration}s"
            log_test_result "$test_name" "PASSED" "${duration}s" "$output"
            return 0
        else
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            print_error "$test_name failed after ${duration}s"
            log_test_result "$test_name" "FAILED" "${duration}s" "$output"
            return 1
        fi
    else
        if eval "$test_command" > /dev/null 2>&1; then
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            print_success "$test_name completed in ${duration}s"
            log_test_result "$test_name" "PASSED" "${duration}s" "(output suppressed)"
            return 0
        else
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            print_error "$test_name failed after ${duration}s"
            log_test_result "$test_name" "FAILED" "${duration}s" "(output suppressed)"
            return 1
        fi
    fi
}

# Function to check test dependencies
check_dependencies() {
    print_status "Checking test dependencies..."
    
    # Check if LLVM is available
    if ! command -v llvm-config &> /dev/null; then
        print_warning "LLVM not found in PATH, some tests may fail"
    fi
    
    # Check Rust toolchain
    if ! command -v rustc &> /dev/null; then
        print_error "Rust compiler not found"
        exit 1
    fi
    
    # Check if we can compile
    if ! $CARGO_CMD check --quiet 2>/dev/null; then
        print_error "Project does not compile, cannot run tests"
        exit 1
    fi
    
    print_success "All dependencies check passed"
}

# Main test execution
main() {
    local total_tests=0
    local passed_tests=0
    local failed_tests=0
    
    print_status "Starting Advanced LLVM Optimization System Tests"
    print_status "Quick mode: $QUICK_MODE, Verbose: $VERBOSE"
    
    # Check dependencies first
    check_dependencies
    
    # Define test categories
    declare -A test_categories
    
    # Core optimization tests
    test_categories["loop_detection"]="${CARGO_CMD} test --test advanced_optimization_integration_test test_loop_detection_and_analysis"
    test_categories["vectorization"]="${CARGO_CMD} test --test advanced_optimization_integration_test test_vectorization_analysis"
    test_categories["target_optimization"]="${CARGO_CMD} test --test advanced_optimization_integration_test test_target_specific_optimizations"
    test_categories["enhanced_optimizer"]="${CARGO_CMD} test --test advanced_optimization_integration_test test_enhanced_llvm_optimizer"
    
    # Performance and monitoring tests
    test_categories["performance_monitoring"]="${CARGO_CMD} test --test advanced_optimization_integration_test test_performance_monitoring"
    test_categories["optimization_effectiveness"]="${CARGO_CMD} test --test advanced_optimization_integration_test test_optimization_effectiveness"
    test_categories["adaptive_optimization"]="${CARGO_CMD} test --test advanced_optimization_integration_test test_adaptive_optimization"
    test_categories["memory_optimization"]="${CARGO_CMD} test --test advanced_optimization_integration_test test_memory_usage_optimization"
    
    # Advanced integration tests (may be skipped in quick mode)
    if [[ "$QUICK_MODE" != "true" ]]; then
        test_categories["stress_optimization"]="${CARGO_CMD} test --test advanced_optimization_integration_test --release -- --ignored"
        test_categories["performance_benchmark"]="${CARGO_CMD} test --bench optimization_benchmarks"
    fi
    
    # Unit tests for individual components
    test_categories["advanced_llvm_unit"]="${CARGO_CMD} test --lib optimization::advanced_llvm_integration"
    test_categories["target_optimization_unit"]="${CARGO_CMD} test --lib optimization::target_optimization"
    test_categories["enhanced_llvm_unit"]="${CARGO_CMD} test --lib optimization::enhanced_llvm_optimization"
    
    # Filter tests if requested
    if [[ -n "$TEST_FILTER" ]]; then
        declare -A filtered_tests
        for test_name in "${!test_categories[@]}"; do
            if [[ "$test_name" == *"$TEST_FILTER"* ]]; then
                filtered_tests["$test_name"]="${test_categories[$test_name]}"
            fi
        done
        test_categories=()
        for test_name in "${!filtered_tests[@]}"; do
            test_categories["$test_name"]="${filtered_tests[$test_name]}"
        done
        
        if [[ ${#test_categories[@]} -eq 0 ]]; then
            print_error "No tests match filter: $TEST_FILTER"
            exit 1
        fi
    fi
    
    # Run tests
    for test_name in "${!test_categories[@]}"; do
        total_tests=$((total_tests + 1))
        
        if run_test "$test_name" "${test_categories[$test_name]}"; then
            passed_tests=$((passed_tests + 1))
        else
            failed_tests=$((failed_tests + 1))
        fi
    done
    
    # Print summary
    echo
    print_status "Test Summary:"
    print_status "  Total tests: $total_tests"
    print_success "  Passed: $passed_tests"
    
    if [[ $failed_tests -gt 0 ]]; then
        print_error "  Failed: $failed_tests"
    else
        print_status "  Failed: $failed_tests"
    fi
    
    # Calculate success rate
    local success_rate=0
    if [[ $total_tests -gt 0 ]]; then
        success_rate=$((passed_tests * 100 / total_tests))
    fi
    
    print_status "  Success rate: ${success_rate}%"
    
    # Finalize report
    if [[ "$GENERATE_REPORT" == "true" ]]; then
        cat >> "$REPORT_FILE" << EOF

## Summary
- **Total Tests:** $total_tests
- **Passed:** $passed_tests
- **Failed:** $failed_tests
- **Success Rate:** ${success_rate}%

## Conclusion
EOF
        
        if [[ $failed_tests -eq 0 ]]; then
            echo "✅ All tests passed successfully!" >> "$REPORT_FILE"
        else
            echo "❌ Some tests failed. Review the failed tests above for details." >> "$REPORT_FILE"
        fi
        
        print_success "Test report written to: $REPORT_FILE"
    fi
    
    # Exit with appropriate code
    if [[ $failed_tests -gt 0 ]]; then
        print_error "Some tests failed!"
        exit 1
    else
        print_success "All tests passed!"
        exit 0
    fi
}

# Run the main function
main "$@"
