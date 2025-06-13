#!/bin/bash

# Comprehensive test runner for CURSED Process Management and IPC modules
# 
# This script runs all process management and IPC tests with proper environment
# setup, error handling, and reporting capabilities.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
REPORT_FILE="${PROJECT_ROOT}/test_results/process_ipc_test_report.md"
COVERAGE_FILE="${PROJECT_ROOT}/test_results/process_ipc_coverage.html"

# Test categories
declare -a TEST_CATEGORIES=(
    "process_management_comprehensive_test"
    "ipc_comprehensive_test"
)

# Test execution modes
RUN_IGNORED=false
VERBOSE=false
GENERATE_REPORT=false
GENERATE_COVERAGE=false
TEST_FILTER=""
QUICK_MODE=false

# Function to print colored output
print_colored() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

print_status() {
    print_colored "$BLUE" "ℹ️  $1"
}

print_success() {
    print_colored "$GREEN" "✅ $1"
}

print_warning() {
    print_colored "$YELLOW" "⚠️  $1"
}

print_error() {
    print_colored "$RED" "❌ $1"
}

# Function to show usage
show_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Run comprehensive tests for CURSED Process Management and IPC modules.

OPTIONS:
    --help              Show this help message
    --verbose           Enable verbose output
    --ignored           Run ignored tests (long-running, stress tests)
    --quick             Run only quick tests (skip ignored and performance tests)
    --report            Generate detailed test report
    --coverage          Generate code coverage report
    --test PATTERN      Run only tests matching pattern
    --filter FILTER     Apply test filter

EXAMPLES:
    $0                          # Run all standard tests
    $0 --quick                  # Run only quick tests
    $0 --ignored                # Run all tests including ignored ones
    $0 --verbose --report       # Run with verbose output and generate report
    $0 --test process           # Run only process-related tests
    $0 --filter lifecycle       # Run tests with 'lifecycle' in name

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
        --ignored)
            RUN_IGNORED=true
            shift
            ;;
        --quick|-q)
            QUICK_MODE=true
            shift
            ;;
        --report|-r)
            GENERATE_REPORT=true
            shift
            ;;
        --coverage|-c)
            GENERATE_COVERAGE=true
            shift
            ;;
        --test|-t)
            TEST_FILTER="$2"
            shift 2
            ;;
        --filter|-f)
            TEST_FILTER="$2"
            shift 2
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Setup test environment
setup_test_environment() {
    print_status "Setting up test environment..."
    
    # Create test results directory
    mkdir -p "${PROJECT_ROOT}/test_results"
    
    # Check if linking fix is available
    if [[ -f "${PROJECT_ROOT}/fix_linking.sh" ]]; then
        print_status "Using linking fix for Nix environment"
        export LINKING_FIX="${PROJECT_ROOT}/fix_linking.sh"
    else
        export LINKING_FIX=""
    fi
    
    # Set test-specific environment variables
    export RUST_BACKTRACE=1
    export RUST_LOG=${RUST_LOG:-"cursed=debug"}
    export CURSED_TEST_MODE=1
    
    print_success "Test environment ready"
}

# Function to run cargo with linking fix if available
run_cargo() {
    if [[ -n "$LINKING_FIX" ]]; then
        "$LINKING_FIX" cargo "$@"
    else
        cargo "$@"
    fi
}

# Function to run a single test category
run_test_category() {
    local test_name="$1"
    local start_time=$(date +%s)
    
    print_status "Running $test_name tests..."
    
    # Build test command
    local cmd_args=("test" "--test" "$test_name")
    
    if [[ "$VERBOSE" == true ]]; then
        cmd_args+=("--" "--nocapture")
    fi
    
    if [[ "$RUN_IGNORED" == true ]]; then
        cmd_args+=("--" "--ignored")
    elif [[ "$QUICK_MODE" == true ]]; then
        cmd_args+=("--" "--skip" "stress" "--skip" "performance")
    fi
    
    if [[ -n "$TEST_FILTER" ]]; then
        cmd_args+=("$TEST_FILTER")
    fi
    
    # Run the test
    local test_output
    local test_result=0
    
    if test_output=$(run_cargo "${cmd_args[@]}" 2>&1); then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_success "$test_name completed in ${duration}s"
        
        # Extract test statistics
        local passed=$(echo "$test_output" | grep -o '[0-9]\+ passed' | cut -d' ' -f1 || echo "0")
        local failed=$(echo "$test_output" | grep -o '[0-9]\+ failed' | cut -d' ' -f1 || echo "0")
        local ignored=$(echo "$test_output" | grep -o '[0-9]\+ ignored' | cut -d' ' -f1 || echo "0")
        
        echo "  - Passed: $passed"
        echo "  - Failed: $failed"
        echo "  - Ignored: $ignored"
        
        return 0
    else
        test_result=$?
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_error "$test_name failed after ${duration}s"
        
        if [[ "$VERBOSE" == true ]]; then
            echo "$test_output"
        fi
        
        return $test_result
    fi
}

# Function to generate test report
generate_test_report() {
    if [[ "$GENERATE_REPORT" != true ]]; then
        return 0
    fi
    
    print_status "Generating test report..."
    
    cat > "$REPORT_FILE" << EOF
# CURSED Process Management and IPC Test Report

Generated on: $(date)
Test run configuration:
- Verbose: $VERBOSE
- Include ignored tests: $RUN_IGNORED
- Quick mode: $QUICK_MODE
- Test filter: ${TEST_FILTER:-"none"}

## Test Results Summary

EOF
    
    # Run tests again to collect detailed results for report
    for test_category in "${TEST_CATEGORIES[@]}"; do
        echo "### $test_category" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        
        local test_output
        if test_output=$(run_cargo test --test "$test_category" -- --format json 2>/dev/null || true); then
            echo '```json' >> "$REPORT_FILE"
            echo "$test_output" >> "$REPORT_FILE"
            echo '```' >> "$REPORT_FILE"
        else
            echo "Test execution failed or not supported." >> "$REPORT_FILE"
        fi
        
        echo "" >> "$REPORT_FILE"
    done
    
    print_success "Test report generated: $REPORT_FILE"
}

# Function to generate coverage report
generate_coverage_report() {
    if [[ "$GENERATE_COVERAGE" != true ]]; then
        return 0
    fi
    
    print_status "Generating coverage report..."
    
    # Check if cargo-tarpaulin is available
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_warning "cargo-tarpaulin not found, skipping coverage report"
        return 0
    fi
    
    # Generate coverage for process and IPC modules
    local coverage_cmd=(
        "tarpaulin"
        "--verbose"
        "--out" "Html"
        "--output-dir" "$(dirname "$COVERAGE_FILE")"
        "--include-tests"
        "--exclude-files" "tests/*"
        "--exclude-files" "benches/*"
        "--exclude-files" "examples/*"
    )
    
    # Add test filters
    for test_category in "${TEST_CATEGORIES[@]}"; do
        coverage_cmd+=("--test" "$test_category")
    done
    
    if run_cargo "${coverage_cmd[@]}"; then
        print_success "Coverage report generated: $COVERAGE_FILE"
    else
        print_warning "Coverage report generation failed"
    fi
}

# Function to run all tests
run_all_tests() {
    local overall_result=0
    local failed_categories=()
    
    print_status "Starting CURSED Process Management and IPC test suite"
    echo "Configuration:"
    echo "  - Verbose: $VERBOSE"
    echo "  - Include ignored: $RUN_IGNORED"
    echo "  - Quick mode: $QUICK_MODE"
    echo "  - Test filter: ${TEST_FILTER:-"none"}"
    echo ""
    
    # Filter test categories if needed
    local categories_to_run=()
    if [[ -n "$TEST_FILTER" ]]; then
        for category in "${TEST_CATEGORIES[@]}"; do
            if [[ "$category" == *"$TEST_FILTER"* ]]; then
                categories_to_run+=("$category")
            fi
        done
    else
        categories_to_run=("${TEST_CATEGORIES[@]}")
    fi
    
    if [[ ${#categories_to_run[@]} -eq 0 ]]; then
        print_error "No test categories match filter: $TEST_FILTER"
        return 1
    fi
    
    # Run each test category
    for test_category in "${categories_to_run[@]}"; do
        if ! run_test_category "$test_category"; then
            overall_result=1
            failed_categories+=("$test_category")
        fi
        echo ""
    done
    
    # Print summary
    echo "=========================================="
    print_status "Test Summary"
    echo "  Total categories: ${#categories_to_run[@]}"
    echo "  Passed: $((${#categories_to_run[@]} - ${#failed_categories[@]}))"
    echo "  Failed: ${#failed_categories[@]}"
    
    if [[ ${#failed_categories[@]} -gt 0 ]]; then
        echo "  Failed categories:"
        for category in "${failed_categories[@]}"; do
            echo "    - $category"
        done
    fi
    
    return $overall_result
}

# Function to validate test environment
validate_environment() {
    print_status "Validating test environment..."
    
    # Check if we're in the right directory
    if [[ ! -f "$PROJECT_ROOT/Cargo.toml" ]]; then
        print_error "Not in CURSED project root directory"
        return 1
    fi
    
    # Check if test files exist
    for test_category in "${TEST_CATEGORIES[@]}"; do
        local test_file="$PROJECT_ROOT/tests/${test_category}.rs"
        if [[ ! -f "$test_file" ]]; then
            print_error "Test file not found: $test_file"
            return 1
        fi
    done
    
    # Check if linking fix is executable
    if [[ -f "$PROJECT_ROOT/fix_linking.sh" && ! -x "$PROJECT_ROOT/fix_linking.sh" ]]; then
        print_warning "Making fix_linking.sh executable"
        chmod +x "$PROJECT_ROOT/fix_linking.sh"
    fi
    
    print_success "Environment validation passed"
    return 0
}

# Main execution
main() {
    local start_time=$(date +%s)
    
    # Change to project root
    cd "$PROJECT_ROOT"
    
    # Validate environment
    if ! validate_environment; then
        exit 1
    fi
    
    # Setup test environment
    setup_test_environment
    
    # Run tests
    local test_result=0
    if ! run_all_tests; then
        test_result=1
    fi
    
    # Generate reports
    generate_test_report
    generate_coverage_report
    
    # Print final summary
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    
    echo "=========================================="
    print_status "Test run completed in ${total_duration}s"
    
    if [[ $test_result -eq 0 ]]; then
        print_success "All tests passed!"
    else
        print_error "Some tests failed"
    fi
    
    exit $test_result
}

# Run main function
main "$@"
