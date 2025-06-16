#!/bin/bash

# Comprehensive test runner for PackRat archive/compression package

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Test configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TEST_OUTPUT_DIR="$PROJECT_ROOT/test_results/packrat"
COVERAGE_DIR="$PROJECT_ROOT/coverage/packrat"

# Default values
VERBOSE=false
QUICK=false
SPECIFIC_TEST=""
GENERATE_REPORT=false
REPORT_FILE=""
RUN_EXAMPLES=false

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to print section headers
print_section() {
    local message=$1
    echo
    print_status $BLUE "=================================================="
    print_status $BLUE "$message"
    print_status $BLUE "=================================================="
    echo
}

# Function to show usage
show_usage() {
    cat << EOF
PackRat Test Suite Runner

USAGE:
    $0 [OPTIONS]

OPTIONS:
    --help, -h              Show this help message
    --verbose, -v           Enable verbose output
    --quick, -q             Run quick tests only (no stress tests)
    --test <name>          Run specific test suite
    --report <file>        Generate test report to file
    --examples             Run example programs
    --all                  Run all tests including stress tests

TEST SUITES:
    unit                   Unit tests for individual components
    integration            Integration tests for full workflows
    stress                 Stress tests with large files
    examples               Example program execution

EXAMPLES:
    $0                     Run standard test suite
    $0 --quick             Run quick tests only
    $0 --test integration  Run integration tests only
    $0 --verbose --report packrat_report.md
    $0 --examples          Run example programs
    $0 --all               Run comprehensive test suite

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help|-h)
            show_usage
            exit 0
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --quick|-q)
            QUICK=true
            shift
            ;;
        --test)
            SPECIFIC_TEST="$2"
            shift 2
            ;;
        --report)
            GENERATE_REPORT=true
            REPORT_FILE="$2"
            shift 2
            ;;
        --examples)
            RUN_EXAMPLES=true
            shift
            ;;
        --all)
            QUICK=false
            RUN_EXAMPLES=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Create output directories
mkdir -p "$TEST_OUTPUT_DIR"
mkdir -p "$COVERAGE_DIR"

# Change to project root
cd "$PROJECT_ROOT"

# Initialize report file if requested
if [ "$GENERATE_REPORT" = true ]; then
    if [ -z "$REPORT_FILE" ]; then
        REPORT_FILE="$TEST_OUTPUT_DIR/packrat_test_report.md"
    fi
    
    cat > "$REPORT_FILE" << EOF
# PackRat Test Suite Report

**Date:** $(date)
**Test Configuration:** $([ "$QUICK" = true ] && echo "Quick" || echo "Full")

## Test Results

EOF
fi

# Function to log test results
log_result() {
    local test_name=$1
    local status=$2
    local details=$3
    
    if [ "$GENERATE_REPORT" = true ]; then
        echo "### $test_name" >> "$REPORT_FILE"
        echo "**Status:** $status" >> "$REPORT_FILE"
        if [ -n "$details" ]; then
            echo "**Details:** $details" >> "$REPORT_FILE"
        fi
        echo "" >> "$REPORT_FILE"
    fi
}

# Function to run tests with linking fix
run_test_with_linking_fix() {
    local test_command=$1
    local test_name=$2
    
    print_status $CYAN "Running: $test_name"
    
    if [ "$VERBOSE" = true ]; then
        if ./fix_linking.sh $test_command 2>&1 | tee "$TEST_OUTPUT_DIR/${test_name}.log"; then
            print_status $GREEN "✓ $test_name passed"
            log_result "$test_name" "PASSED" ""
            return 0
        else
            print_status $RED "✗ $test_name failed"
            log_result "$test_name" "FAILED" "See ${test_name}.log for details"
            return 1
        fi
    else
        if ./fix_linking.sh $test_command > "$TEST_OUTPUT_DIR/${test_name}.log" 2>&1; then
            print_status $GREEN "✓ $test_name passed"
            log_result "$test_name" "PASSED" ""
            return 0
        else
            print_status $RED "✗ $test_name failed"
            log_result "$test_name" "FAILED" "See ${test_name}.log for details"
            return 1
        fi
    fi
}

# Function to run unit tests
run_unit_tests() {
    print_section "PackRat Unit Tests"
    
    local test_files=(
        "tests/packrat_integration_test.rs"
    )
    
    local passed=0
    local total=0
    
    for test_file in "${test_files[@]}"; do
        if [ -f "$test_file" ]; then
            total=$((total + 1))
            local test_name="unit_$(basename "$test_file" .rs)"
            
            if run_test_with_linking_fix "cargo test --test $(basename "$test_file" .rs)" "$test_name"; then
                passed=$((passed + 1))
            fi
        fi
    done
    
    print_status $YELLOW "Unit Tests: $passed/$total passed"
    return $((total - passed))
}

# Function to run integration tests
run_integration_tests() {
    print_section "PackRat Integration Tests"
    
    local passed=0
    local total=0
    
    # Test TAR operations
    total=$((total + 1))
    if run_test_with_linking_fix "cargo test packrat_integration_test::test_rat" "tar_operations"; then
        passed=$((passed + 1))
    fi
    
    # Test ZIP operations
    total=$((total + 1))
    if run_test_with_linking_fix "cargo test packrat_integration_test::test_hoard" "zip_operations"; then
        passed=$((passed + 1))
    fi
    
    # Test format detection
    total=$((total + 1))
    if run_test_with_linking_fix "cargo test packrat_integration_test::test_format_detection" "format_detection"; then
        passed=$((passed + 1))
    fi
    
    # Test compression utilities
    total=$((total + 1))
    if run_test_with_linking_fix "cargo test packrat_integration_test::test_archive_round_trip" "compression_utilities"; then
        passed=$((passed + 1))
    fi
    
    print_status $YELLOW "Integration Tests: $passed/$total passed"
    return $((total - passed))
}

# Function to run stress tests
run_stress_tests() {
    if [ "$QUICK" = true ]; then
        print_status $YELLOW "Skipping stress tests (quick mode)"
        return 0
    fi
    
    print_section "PackRat Stress Tests"
    
    local passed=0
    local total=0
    
    # Test large file handling
    total=$((total + 1))
    if run_test_with_linking_fix "cargo test packrat_integration_test::test_large_file_handling" "large_files"; then
        passed=$((passed + 1))
    fi
    
    # Test concurrent operations
    total=$((total + 1))
    if run_test_with_linking_fix "cargo test packrat_integration_test::test_concurrent_archive_operations" "concurrent_operations"; then
        passed=$((passed + 1))
    fi
    
    # Test metadata preservation
    total=$((total + 1))
    if run_test_with_linking_fix "cargo test packrat_integration_test::test_archive_metadata_preservation" "metadata_preservation"; then
        passed=$((passed + 1))
    fi
    
    print_status $YELLOW "Stress Tests: $passed/$total passed"
    return $((total - passed))
}

# Function to run example programs
run_examples() {
    if [ "$RUN_EXAMPLES" != true ]; then
        return 0
    fi
    
    print_section "PackRat Example Programs"
    
    local passed=0
    local total=0
    
    # Build examples
    total=$((total + 1))
    if run_test_with_linking_fix "cargo build --example packrat_usage_examples" "build_examples"; then
        passed=$((passed + 1))
        
        # Run examples
        total=$((total + 1))
        if run_test_with_linking_fix "./target/debug/examples/packrat_usage_examples" "run_examples"; then
            passed=$((passed + 1))
        fi
    fi
    
    print_status $YELLOW "Example Programs: $passed/$total passed"
    return $((total - passed))
}

# Function to generate coverage report
generate_coverage() {
    print_section "Generating Coverage Report"
    
    if command -v cargo-tarpaulin >/dev/null 2>&1; then
        print_status $CYAN "Running coverage analysis..."
        
        if ./fix_linking.sh cargo tarpaulin \
            --out Html \
            --output-dir "$COVERAGE_DIR" \
            --include-tests \
            --exclude-files "tests/*" \
            --timeout 300 \
            2>&1 | tee "$TEST_OUTPUT_DIR/coverage.log"; then
            
            print_status $GREEN "✓ Coverage report generated: $COVERAGE_DIR/tarpaulin-report.html"
            
            if [ "$GENERATE_REPORT" = true ]; then
                echo "## Coverage Report" >> "$REPORT_FILE"
                echo "Coverage report available at: $COVERAGE_DIR/tarpaulin-report.html" >> "$REPORT_FILE"
                echo "" >> "$REPORT_FILE"
            fi
        else
            print_status $RED "✗ Coverage generation failed"
        fi
    else
        print_status $YELLOW "⚠ cargo-tarpaulin not installed, skipping coverage"
    fi
}

# Main execution
main() {
    print_section "PackRat Test Suite"
    
    print_status $CYAN "Test configuration:"
    print_status $CYAN "  Verbose: $VERBOSE"
    print_status $CYAN "  Quick mode: $QUICK"
    print_status $CYAN "  Specific test: ${SPECIFIC_TEST:-'All'}"
    print_status $CYAN "  Examples: $RUN_EXAMPLES"
    print_status $CYAN "  Report: ${REPORT_FILE:-'None'}"
    echo
    
    local total_failures=0
    
    # Run specific test if requested
    if [ -n "$SPECIFIC_TEST" ]; then
        case "$SPECIFIC_TEST" in
            unit)
                run_unit_tests
                total_failures=$?
                ;;
            integration)
                run_integration_tests
                total_failures=$?
                ;;
            stress)
                run_stress_tests
                total_failures=$?
                ;;
            examples)
                run_examples
                total_failures=$?
                ;;
            *)
                print_status $RED "Unknown test suite: $SPECIFIC_TEST"
                exit 1
                ;;
        esac
    else
        # Run all tests
        run_unit_tests
        total_failures=$((total_failures + $?))
        
        run_integration_tests
        total_failures=$((total_failures + $?))
        
        run_stress_tests
        total_failures=$((total_failures + $?))
        
        run_examples
        total_failures=$((total_failures + $?))
    fi
    
    # Generate coverage if not in quick mode
    if [ "$QUICK" != true ] && [ -z "$SPECIFIC_TEST" ]; then
        generate_coverage
    fi
    
    # Final summary
    print_section "Test Suite Summary"
    
    if [ $total_failures -eq 0 ]; then
        print_status $GREEN "🎉 All PackRat tests passed successfully!"
        
        if [ "$GENERATE_REPORT" = true ]; then
            echo "## Summary" >> "$REPORT_FILE"
            echo "**Result:** All tests passed ✓" >> "$REPORT_FILE"
            echo "**Total Failures:** 0" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
        fi
        
        exit 0
    else
        print_status $RED "❌ $total_failures test(s) failed"
        
        if [ "$GENERATE_REPORT" = true ]; then
            echo "## Summary" >> "$REPORT_FILE"
            echo "**Result:** Some tests failed ✗" >> "$REPORT_FILE"
            echo "**Total Failures:** $total_failures" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
        fi
        
        exit 1
    fi
}

# Show startup banner
echo
print_status $PURPLE "🐀 PackRat Archive Package Test Suite 🐀"
print_status $PURPLE "=========================================="
echo

# Run main function
main
