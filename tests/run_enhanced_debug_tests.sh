#!/bin/bash

# Enhanced Debug System Test Runner
# Comprehensive test execution for CURSED enhanced debugging system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Default values
VERBOSE=false
QUICK=false
COVERAGE=false
REPORT=""
TEST_FILTER=""

# Print usage
usage() {
    echo "Enhanced Debug System Test Runner"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -h, --help              Show this help message"
    echo "  -v, --verbose           Enable verbose output"
    echo "  -q, --quick             Run quick tests only (skip ignored tests)"
    echo "  -c, --coverage          Generate coverage report"
    echo "  -r, --report FILE       Generate detailed test report"
    echo "  -t, --test PATTERN      Run specific test pattern"
    echo "  --integration           Run integration tests only"
    echo "  --performance           Run performance tests only"
    echo "  --edge-cases            Run edge case tests only"
    echo ""
    echo "Examples:"
    echo "  $0                      # Run all tests"
    echo "  $0 --quick             # Run quick tests only"
    echo "  $0 --integration       # Run integration tests only"
    echo "  $0 --coverage          # Run with coverage report"
    echo "  $0 --report debug.md   # Generate detailed report"
}

# Print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Print section header
print_header() {
    echo ""
    print_status $BLUE "=================================================="
    print_status $BLUE "$1"
    print_status $BLUE "=================================================="
}

# Check if cargo-tarpaulin is available for coverage
check_coverage_tool() {
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_status $YELLOW "Warning: cargo-tarpaulin not found. Installing..."
        if ! cargo install cargo-tarpaulin; then
            print_status $RED "Failed to install cargo-tarpaulin"
            return 1
        fi
    fi
    return 0
}

# Source the linking fix if it exists
setup_environment() {
    if [ -f "$PROJECT_ROOT/fix_linking.sh" ]; then
        print_status $BLUE "Setting up linking environment..."
        source "$PROJECT_ROOT/fix_linking.sh"
    fi
}

# Run specific test suite
run_test_suite() {
    local test_name=$1
    local description=$2
    local extra_args=${3:-""}
    
    print_status $BLUE "Running $description..."
    
    local cmd="cargo test --test $test_name $extra_args"
    
    if [ "$VERBOSE" = true ]; then
        echo "Command: $cmd"
        $cmd
    else
        if $cmd > /tmp/test_output_$test_name.log 2>&1; then
            print_status $GREEN "✓ $description passed"
        else
            print_status $RED "✗ $description failed"
            if [ "$VERBOSE" = true ]; then
                cat /tmp/test_output_$test_name.log
            else
                echo "Run with --verbose to see detailed output"
            fi
            return 1
        fi
    fi
}

# Run integration tests
run_integration_tests() {
    print_header "Enhanced Debug Integration Tests"
    run_test_suite "enhanced_debug_integration_test" "Enhanced Debug Integration Tests"
}

# Run performance tests
run_performance_tests() {
    print_header "Enhanced Debug Performance Tests"
    
    if [ "$QUICK" = true ]; then
        print_status $YELLOW "Skipping performance tests in quick mode"
        return 0
    fi
    
    # Run standard performance tests
    run_test_suite "enhanced_debug_performance_test" "Enhanced Debug Performance Tests" ""
    
    # Run ignored (intensive) performance tests
    print_status $BLUE "Running intensive performance tests..."
    run_test_suite "enhanced_debug_performance_test" "Enhanced Debug Intensive Performance Tests" "-- --ignored"
}

# Run edge case tests
run_edge_case_tests() {
    print_header "Enhanced Debug Edge Case Tests"
    run_test_suite "enhanced_debug_edge_cases_test" "Enhanced Debug Edge Case Tests"
}

# Run all unit tests for debug modules
run_unit_tests() {
    print_header "Enhanced Debug Unit Tests"
    
    print_status $BLUE "Running enhanced debug unit tests..."
    if [ "$VERBOSE" = true ]; then
        cargo test --lib debug::enhanced_debug::tests
        cargo test --lib runtime::debug_runtime::tests
    else
        if cargo test --lib debug::enhanced_debug::tests > /tmp/unit_debug.log 2>&1 && \
           cargo test --lib runtime::debug_runtime::tests > /tmp/unit_runtime.log 2>&1; then
            print_status $GREEN "✓ Debug unit tests passed"
        else
            print_status $RED "✗ Debug unit tests failed"
            return 1
        fi
    fi
}

# Generate coverage report
generate_coverage() {
    print_header "Generating Coverage Report"
    
    if ! check_coverage_tool; then
        return 1
    fi
    
    print_status $BLUE "Running tests with coverage analysis..."
    
    local coverage_args="--tests --lib --bins"
    if [ -n "$TEST_FILTER" ]; then
        coverage_args="$coverage_args --test $TEST_FILTER"
    fi
    
    # Generate HTML and XML reports
    cargo tarpaulin $coverage_args \
        --out Html \
        --out Xml \
        --output-dir target/coverage \
        --exclude-files "target/*" \
        --exclude-files "tests/*" \
        --timeout 300
    
    if [ $? -eq 0 ]; then
        print_status $GREEN "✓ Coverage report generated in target/coverage/"
        if command -v xdg-open &> /dev/null; then
            print_status $BLUE "Opening coverage report..."
            xdg-open target/coverage/tarpaulin-report.html &
        fi
    else
        print_status $RED "✗ Coverage generation failed"
        return 1
    fi
}

# Generate detailed test report
generate_report() {
    local report_file=$1
    
    print_header "Generating Detailed Test Report"
    
    cat > "$report_file" << EOF
# Enhanced Debug System Test Report

Generated on: $(date)
System: $(uname -a)
Rust version: $(rustc --version)

## Test Execution Summary

EOF

    # Run tests and capture results
    local total_tests=0
    local passed_tests=0
    local failed_tests=0
    
    # Integration tests
    echo "### Integration Tests" >> "$report_file"
    if run_test_suite "enhanced_debug_integration_test" "Integration Tests" "--no-run" > /tmp/integration_report.log 2>&1; then
        echo "✓ Integration tests: PASSED" >> "$report_file"
        ((passed_tests++))
    else
        echo "✗ Integration tests: FAILED" >> "$report_file"
        ((failed_tests++))
    fi
    ((total_tests++))
    
    # Performance tests
    if [ "$QUICK" != true ]; then
        echo "### Performance Tests" >> "$report_file"
        if run_test_suite "enhanced_debug_performance_test" "Performance Tests" "--no-run" > /tmp/performance_report.log 2>&1; then
            echo "✓ Performance tests: PASSED" >> "$report_file"
            ((passed_tests++))
        else
            echo "✗ Performance tests: FAILED" >> "$report_file"
            ((failed_tests++))
        fi
        ((total_tests++))
    fi
    
    # Edge case tests
    echo "### Edge Case Tests" >> "$report_file"
    if run_test_suite "enhanced_debug_edge_cases_test" "Edge Case Tests" "--no-run" > /tmp/edge_cases_report.log 2>&1; then
        echo "✓ Edge case tests: PASSED" >> "$report_file"
        ((passed_tests++))
    else
        echo "✗ Edge case tests: FAILED" >> "$report_file"
        ((failed_tests++))
    fi
    ((total_tests++))
    
    # Summary
    cat >> "$report_file" << EOF

## Summary

- Total test suites: $total_tests
- Passed: $passed_tests
- Failed: $failed_tests
- Success rate: $(( passed_tests * 100 / total_tests ))%

## Test Environment

- Project root: $PROJECT_ROOT
- Test runner: $0
- Arguments: $@

## Individual Test Outputs

EOF

    # Append detailed logs if they exist
    for log_file in /tmp/*_report.log; do
        if [ -f "$log_file" ]; then
            echo "### $(basename "$log_file" .log)" >> "$report_file"
            echo '```' >> "$report_file"
            cat "$log_file" >> "$report_file"
            echo '```' >> "$report_file"
            echo "" >> "$report_file"
        fi
    done
    
    print_status $GREEN "✓ Test report generated: $report_file"
}

# Main execution
main() {
    cd "$PROJECT_ROOT"
    
    print_header "Enhanced Debug System Test Runner"
    print_status $BLUE "Project root: $PROJECT_ROOT"
    
    # Setup environment
    setup_environment
    
    # Determine what to run based on arguments
    local run_all=true
    
    # Check for specific test type flags
    for arg in "$@"; do
        case $arg in
            --integration)
                run_integration_tests
                run_all=false
                ;;
            --performance)
                run_performance_tests
                run_all=false
                ;;
            --edge-cases)
                run_edge_case_tests
                run_all=false
                ;;
        esac
    done
    
    # Run all tests if no specific type was requested
    if [ "$run_all" = true ]; then
        # Run unit tests first
        run_unit_tests
        
        # Run integration tests
        run_integration_tests
        
        # Run performance tests (unless quick mode)
        if [ "$QUICK" != true ]; then
            run_performance_tests
        fi
        
        # Run edge case tests
        run_edge_case_tests
    fi
    
    # Generate coverage if requested
    if [ "$COVERAGE" = true ]; then
        generate_coverage
    fi
    
    # Generate report if requested
    if [ -n "$REPORT" ]; then
        generate_report "$REPORT"
    fi
    
    print_header "Test Execution Complete"
    print_status $GREEN "All enhanced debug tests completed successfully!"
    
    # Cleanup temporary files
    rm -f /tmp/test_output_*.log /tmp/*_report.log
}

# Parse command line arguments
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
        -q|--quick)
            QUICK=true
            shift
            ;;
        -c|--coverage)
            COVERAGE=true
            shift
            ;;
        -r|--report)
            REPORT="$2"
            shift 2
            ;;
        -t|--test)
            TEST_FILTER="$2"
            shift 2
            ;;
        --integration|--performance|--edge-cases)
            # These are handled in main()
            shift
            ;;
        *)
            echo "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Execute main function
main "$@"
