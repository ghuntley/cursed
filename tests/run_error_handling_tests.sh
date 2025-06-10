#!/bin/bash
#
# Comprehensive Error Handling Test Runner for CURSED
#
# This script provides automated testing for the complete error handling system
# including integration tests, stress tests, and edge case validation.
#
# Features:
# - Multiple test execution modes (quick, comprehensive, stress)
# - Nix environment linking fix integration
# - Coverage analysis with cargo-tarpaulin
# - Performance benchmarking and reporting
# - CI/CD ready with proper exit codes
# - Detailed logging and progress tracking

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
REPORT_DIR="$PROJECT_ROOT/target/error_handling_reports"
COVERAGE_FILE="$REPORT_DIR/error_handling_coverage.html"
PERFORMANCE_LOG="$REPORT_DIR/error_handling_performance.log"

# Test categories
INTEGRATION_TESTS="error_handling_integration_test"
STRESS_TESTS="error_handling_stress_test"
EDGE_CASE_TESTS="error_handling_edge_cases_test"

# Default configuration
RUN_INTEGRATION=true
RUN_STRESS=false
RUN_EDGE_CASES=true
RUN_COVERAGE=false
VERBOSE=false
QUICK_MODE=false
GENERATE_REPORT=false
JOBS=1
TIMEOUT=300

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
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

# Show usage information
show_help() {
    cat << EOF
Error Handling Test Runner for CURSED

USAGE:
    $(basename "$0") [OPTIONS]

OPTIONS:
    -h, --help              Show this help message
    -q, --quick             Run only quick tests (skip stress tests)
    -a, --all               Run all tests including stress tests
    -i, --integration       Run only integration tests
    -s, --stress           Run only stress tests
    -e, --edge-cases       Run only edge case tests
    -c, --coverage         Generate coverage report
    -v, --verbose          Enable verbose output
    -r, --report FILE      Generate detailed report to file
    -j, --jobs N           Number of parallel test jobs
    -t, --timeout SECS     Test timeout in seconds (default: 300)
    --ignored              Run ignored tests (stress tests)

EXAMPLES:
    $(basename "$0")                    # Run standard tests
    $(basename "$0") --quick            # Quick validation
    $(basename "$0") --all --coverage   # Full test suite with coverage
    $(basename "$0") --stress --verbose # Stress tests with detailed output
    $(basename "$0") --report results.md # Generate detailed report

EXIT CODES:
    0    All tests passed
    1    Test failures or errors
    2    Setup/configuration errors
    3    Coverage generation failed
EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -q|--quick)
                QUICK_MODE=true
                RUN_STRESS=false
                shift
                ;;
            -a|--all)
                RUN_INTEGRATION=true
                RUN_STRESS=true
                RUN_EDGE_CASES=true
                shift
                ;;
            -i|--integration)
                RUN_INTEGRATION=true
                RUN_STRESS=false
                RUN_EDGE_CASES=false
                shift
                ;;
            -s|--stress)
                RUN_INTEGRATION=false
                RUN_STRESS=true
                RUN_EDGE_CASES=false
                shift
                ;;
            -e|--edge-cases)
                RUN_INTEGRATION=false
                RUN_STRESS=false
                RUN_EDGE_CASES=true
                shift
                ;;
            -c|--coverage)
                RUN_COVERAGE=true
                shift
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -r|--report)
                GENERATE_REPORT=true
                REPORT_FILE="$2"
                shift 2
                ;;
            -j|--jobs)
                JOBS="$2"
                shift 2
                ;;
            -t|--timeout)
                TIMEOUT="$2"
                shift 2
                ;;
            --ignored)
                RUN_STRESS=true
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                show_help
                exit 2
                ;;
        esac
    done
}

# Setup test environment
setup_environment() {
    print_status "Setting up error handling test environment..."
    
    # Create report directory
    mkdir -p "$REPORT_DIR"
    
    # Check for linking fix script
    if [[ ! -f "$PROJECT_ROOT/fix_linking.sh" ]]; then
        print_error "Linking fix script not found at $PROJECT_ROOT/fix_linking.sh"
        print_error "Please ensure the linking fix is available for Nix environment"
        exit 2
    fi
    
    # Make linking fix executable
    chmod +x "$PROJECT_ROOT/fix_linking.sh"
    
    # Clear previous logs
    > "$PERFORMANCE_LOG"
    
    print_success "Environment setup complete"
}

# Run a test category with timing
run_test_category() {
    local test_name="$1"
    local test_args="$2"
    local start_time
    local end_time
    local duration
    
    print_status "Running $test_name tests..."
    start_time=$(date +%s)
    
    local cmd="$PROJECT_ROOT/fix_linking.sh cargo test --test $test_name"
    
    if [[ "$test_args" != "" ]]; then
        cmd="$cmd $test_args"
    fi
    
    if [[ "$VERBOSE" == "true" ]]; then
        cmd="$cmd -- --nocapture"
    fi
    
    if [[ "$JOBS" -gt 1 ]]; then
        cmd="$cmd --jobs $JOBS"
    fi
    
    # Add timeout
    timeout "$TIMEOUT" bash -c "$cmd" || {
        local exit_code=$?
        if [[ $exit_code -eq 124 ]]; then
            print_error "$test_name tests timed out after ${TIMEOUT}s"
        else
            print_error "$test_name tests failed with exit code $exit_code"
        fi
        return $exit_code
    }
    
    end_time=$(date +%s)
    duration=$((end_time - start_time))
    
    echo "$(date): $test_name completed in ${duration}s" >> "$PERFORMANCE_LOG"
    print_success "$test_name tests completed in ${duration}s"
}

# Generate coverage report
generate_coverage() {
    print_status "Generating coverage report for error handling tests..."
    
    # Check if cargo-tarpaulin is available
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_warning "cargo-tarpaulin not found, skipping coverage analysis"
        print_warning "Install with: cargo install cargo-tarpaulin"
        return 0
    fi
    
    local coverage_cmd="$PROJECT_ROOT/fix_linking.sh cargo tarpaulin"
    coverage_cmd="$coverage_cmd --tests"
    coverage_cmd="$coverage_cmd --exclude-files 'target/*'"
    coverage_cmd="$coverage_cmd --exclude-files 'tests/*'"
    coverage_cmd="$coverage_cmd --include-tests"
    coverage_cmd="$coverage_cmd --out Html"
    coverage_cmd="$coverage_cmd --output-dir $REPORT_DIR"
    coverage_cmd="$coverage_cmd --timeout 600"
    
    # Include specific error handling modules
    coverage_cmd="$coverage_cmd --packages cursed"
    
    if timeout 600 bash -c "$coverage_cmd"; then
        print_success "Coverage report generated: $COVERAGE_FILE"
    else
        print_error "Coverage generation failed"
        return 3
    fi
}

# Generate performance report
generate_performance_report() {
    local report_file="$1"
    
    print_status "Generating performance report..."
    
    cat > "$report_file" << EOF
# Error Handling Test Performance Report

Generated: $(date)

## Test Execution Times

EOF
    
    if [[ -f "$PERFORMANCE_LOG" ]]; then
        echo '```' >> "$report_file"
        cat "$PERFORMANCE_LOG" >> "$report_file"
        echo '```' >> "$report_file"
    else
        echo "No performance data available" >> "$report_file"
    fi
    
    cat >> "$report_file" << EOF

## Test Categories Executed

EOF
    
    if [[ "$RUN_INTEGRATION" == "true" ]]; then
        echo "- ✅ Integration Tests" >> "$report_file"
    else
        echo "- ⏭️ Integration Tests (Skipped)" >> "$report_file"
    fi
    
    if [[ "$RUN_STRESS" == "true" ]]; then
        echo "- ✅ Stress Tests" >> "$report_file"
    else
        echo "- ⏭️ Stress Tests (Skipped)" >> "$report_file"
    fi
    
    if [[ "$RUN_EDGE_CASES" == "true" ]]; then
        echo "- ✅ Edge Case Tests" >> "$report_file"
    else
        echo "- ⏭️ Edge Case Tests (Skipped)" >> "$report_file"
    fi
    
    cat >> "$report_file" << EOF

## Configuration

- Parallel Jobs: $JOBS
- Timeout: ${TIMEOUT}s
- Verbose Mode: $VERBOSE
- Coverage Generation: $RUN_COVERAGE

## Test Environment

- Project Root: $PROJECT_ROOT
- Report Directory: $REPORT_DIR
- Linking Fix: Available

EOF
    
    print_success "Performance report generated: $report_file"
}

# Main test execution
main() {
    local total_start_time
    local total_end_time
    local total_duration
    local failed_tests=()
    
    print_status "Starting CURSED Error Handling Test Suite"
    print_status "Configuration: Integration=$RUN_INTEGRATION, Stress=$RUN_STRESS, EdgeCases=$RUN_EDGE_CASES"
    
    total_start_time=$(date +%s)
    
    # Setup environment
    setup_environment || exit 2
    
    # Change to project directory
    cd "$PROJECT_ROOT"
    
    # Run integration tests
    if [[ "$RUN_INTEGRATION" == "true" ]]; then
        if ! run_test_category "$INTEGRATION_TESTS" ""; then
            failed_tests+=("Integration Tests")
        fi
    fi
    
    # Run edge case tests
    if [[ "$RUN_EDGE_CASES" == "true" ]]; then
        if ! run_test_category "$EDGE_CASE_TESTS" ""; then
            failed_tests+=("Edge Case Tests")
        fi
    fi
    
    # Run stress tests (usually ignored)
    if [[ "$RUN_STRESS" == "true" ]]; then
        print_warning "Running stress tests - this may take a while..."
        if ! run_test_category "$STRESS_TESTS" "-- --ignored"; then
            failed_tests+=("Stress Tests")
        fi
    fi
    
    # Generate coverage if requested
    if [[ "$RUN_COVERAGE" == "true" ]]; then
        if ! generate_coverage; then
            print_warning "Coverage generation failed but tests continue"
        fi
    fi
    
    # Calculate total time
    total_end_time=$(date +%s)
    total_duration=$((total_end_time - total_start_time))
    
    # Generate report if requested
    if [[ "$GENERATE_REPORT" == "true" ]]; then
        generate_performance_report "$REPORT_FILE"
    fi
    
    # Print summary
    echo
    print_status "Error Handling Test Suite Summary"
    print_status "Total execution time: ${total_duration}s"
    
    if [[ ${#failed_tests[@]} -eq 0 ]]; then
        print_success "All error handling tests passed! ✅"
        echo
        print_status "Test artifacts:"
        if [[ -f "$PERFORMANCE_LOG" ]]; then
            print_status "- Performance log: $PERFORMANCE_LOG"
        fi
        if [[ -f "$COVERAGE_FILE" ]]; then
            print_status "- Coverage report: $COVERAGE_FILE"
        fi
        if [[ "$GENERATE_REPORT" == "true" ]] && [[ -f "$REPORT_FILE" ]]; then
            print_status "- Test report: $REPORT_FILE"
        fi
        exit 0
    else
        print_error "Some error handling tests failed:"
        for test in "${failed_tests[@]}"; do
            print_error "  - $test"
        done
        exit 1
    fi
}

# Parse arguments and run main function
parse_args "$@"
main
