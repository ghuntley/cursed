#!/bin/bash
# Comprehensive Type Conversion Test Runner
# This script runs all type conversion tests and provides detailed reporting

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
VERBOSE=false
RUN_PERFORMANCE=false
RUN_INTEGRATION=true
RUN_ERROR_TESTS=true
GENERATE_REPORT=false
TEST_FILTER=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --performance|-p)
            RUN_PERFORMANCE=true
            shift
            ;;
        --no-integration)
            RUN_INTEGRATION=false
            shift
            ;;
        --no-error-tests)
            RUN_ERROR_TESTS=false
            shift
            ;;
        --report|-r)
            GENERATE_REPORT=true
            shift
            ;;
        --filter|-f)
            TEST_FILTER="$2"
            shift 2
            ;;
        --help|-h)
            echo "Type Conversion Test Runner"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --verbose, -v           Enable verbose output"
            echo "  --performance, -p       Run performance tests"
            echo "  --no-integration        Skip integration tests"
            echo "  --no-error-tests        Skip error handling tests"
            echo "  --report, -r           Generate test report"
            echo "  --filter, -f PATTERN   Filter tests by pattern"
            echo "  --help, -h             Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

print_header() {
    echo ""
    print_status $BLUE "=================================================================================="
    print_status $BLUE "$1"
    print_status $BLUE "=================================================================================="
    echo ""
}

# Function to run a single test suite
run_test_suite() {
    local test_name=$1
    local description=$2
    
    print_status $YELLOW "Running $description..."
    
    if [[ $VERBOSE == true ]]; then
        RUST_LOG=debug cargo test --test "$test_name" -- --nocapture
    else
        cargo test --test "$test_name" 2>&1 | grep -E "(test result:|passed|failed|ERROR|WARN)"
    fi
    
    local exit_code=${PIPESTATUS[0]}
    
    if [[ $exit_code -eq 0 ]]; then
        print_status $GREEN "✓ $description completed successfully"
    else
        print_status $RED "✗ $description failed with exit code $exit_code"
        return $exit_code
    fi
}

# Function to compile the type conversion system
compile_system() {
    print_header "Compiling Type Conversion System"
    
    print_status $YELLOW "Building type conversion system..."
    
    if [[ $VERBOSE == true ]]; then
        cargo build --lib
    else
        cargo build --lib 2>&1 | grep -E "(error|warning|Compiling|Finished)"
    fi
    
    local exit_code=${PIPESTATUS[0]}
    
    if [[ $exit_code -eq 0 ]]; then
        print_status $GREEN "✓ Type conversion system compiled successfully"
    else
        print_status $RED "✗ Compilation failed with exit code $exit_code"
        return $exit_code
    fi
}

# Function to check test environment
check_environment() {
    print_header "Checking Test Environment"
    
    # Check if required tools are available
    if ! command -v cargo &> /dev/null; then
        print_status $RED "Error: cargo not found"
        exit 1
    fi
    
    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]]; then
        print_status $RED "Error: Not in a Rust project directory"
        exit 1
    fi
    
    # Check if test files exist
    local test_files=("type_conversion_integration_test.rs" "type_conversion_performance_test.rs" "type_conversion_error_test.rs")
    for test_file in "${test_files[@]}"; do
        if [[ ! -f "tests/$test_file" ]]; then
            print_status $RED "Error: Test file tests/$test_file not found"
            exit 1
        fi
    done
    
    print_status $GREEN "✓ Environment check passed"
}

# Function to generate test report
generate_test_report() {
    print_header "Generating Test Report"
    
    local report_file="type_conversion_test_report.md"
    
    cat > "$report_file" << EOF
# Type Conversion System Test Report

Generated on: $(date)

## Test Summary

EOF

    # Run tests with JSON output for report generation
    if [[ $RUN_INTEGRATION == true ]]; then
        echo "### Integration Tests" >> "$report_file"
        cargo test --test type_conversion_integration_test -- --format=json 2>/dev/null | \
            jq -r 'select(.type == "test") | "- \(.name): \(.event)"' >> "$report_file" || \
            echo "- Integration tests: See console output" >> "$report_file"
        echo "" >> "$report_file"
    fi
    
    if [[ $RUN_ERROR_TESTS == true ]]; then
        echo "### Error Handling Tests" >> "$report_file"
        cargo test --test type_conversion_error_test -- --format=json 2>/dev/null | \
            jq -r 'select(.type == "test") | "- \(.name): \(.event)"' >> "$report_file" || \
            echo "- Error handling tests: See console output" >> "$report_file"
        echo "" >> "$report_file"
    fi
    
    if [[ $RUN_PERFORMANCE == true ]]; then
        echo "### Performance Tests" >> "$report_file"
        cargo test --test type_conversion_performance_test -- --format=json 2>/dev/null | \
            jq -r 'select(.type == "test") | "- \(.name): \(.event)"' >> "$report_file" || \
            echo "- Performance tests: See console output" >> "$report_file"
        echo "" >> "$report_file"
    fi
    
    echo "## Environment Information" >> "$report_file"
    echo "- Rust version: $(rustc --version)" >> "$report_file"
    echo "- Cargo version: $(cargo --version)" >> "$report_file"
    echo "- Platform: $(uname -a)" >> "$report_file"
    
    print_status $GREEN "✓ Test report generated: $report_file"
}

# Main execution
main() {
    local start_time=$(date +%s)
    local failed_tests=0
    
    print_header "Type Conversion System Test Suite"
    
    # Check environment
    check_environment
    
    # Compile the system
    compile_system || ((failed_tests++))
    
    # Run integration tests
    if [[ $RUN_INTEGRATION == true ]]; then
        print_header "Integration Tests"
        
        if [[ -n "$TEST_FILTER" ]]; then
            print_status $YELLOW "Running integration tests with filter: $TEST_FILTER"
            if [[ $VERBOSE == true ]]; then
                RUST_LOG=debug cargo test --test type_conversion_integration_test "$TEST_FILTER" -- --nocapture
            else
                cargo test --test type_conversion_integration_test "$TEST_FILTER"
            fi
        else
            run_test_suite "type_conversion_integration_test" "Integration Tests"
        fi
        
        if [[ $? -ne 0 ]]; then
            ((failed_tests++))
        fi
    fi
    
    # Run error handling tests
    if [[ $RUN_ERROR_TESTS == true ]]; then
        print_header "Error Handling Tests"
        
        if [[ -n "$TEST_FILTER" ]]; then
            print_status $YELLOW "Running error tests with filter: $TEST_FILTER"
            if [[ $VERBOSE == true ]]; then
                RUST_LOG=debug cargo test --test type_conversion_error_test "$TEST_FILTER" -- --nocapture
            else
                cargo test --test type_conversion_error_test "$TEST_FILTER"
            fi
        else
            run_test_suite "type_conversion_error_test" "Error Handling Tests"
        fi
        
        if [[ $? -ne 0 ]]; then
            ((failed_tests++))
        fi
    fi
    
    # Run performance tests
    if [[ $RUN_PERFORMANCE == true ]]; then
        print_header "Performance Tests"
        
        if [[ -n "$TEST_FILTER" ]]; then
            print_status $YELLOW "Running performance tests with filter: $TEST_FILTER"
            if [[ $VERBOSE == true ]]; then
                RUST_LOG=info cargo test --test type_conversion_performance_test "$TEST_FILTER" -- --nocapture
            else
                cargo test --test type_conversion_performance_test "$TEST_FILTER"
            fi
        else
            run_test_suite "type_conversion_performance_test" "Performance Tests"
        fi
        
        if [[ $? -ne 0 ]]; then
            ((failed_tests++))
        fi
    fi
    
    # Generate report if requested
    if [[ $GENERATE_REPORT == true ]]; then
        generate_test_report
    fi
    
    # Final summary
    print_header "Test Suite Summary"
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    if [[ $failed_tests -eq 0 ]]; then
        print_status $GREEN "✓ All test suites passed successfully!"
    else
        print_status $RED "✗ $failed_tests test suite(s) failed"
    fi
    
    print_status $BLUE "Total execution time: ${duration} seconds"
    
    if [[ $failed_tests -gt 0 ]]; then
        print_status $YELLOW "To debug failures, run with --verbose flag"
        exit 1
    fi
}

# Execute main function
main "$@"
