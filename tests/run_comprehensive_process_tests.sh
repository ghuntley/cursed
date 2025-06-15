#!/bin/bash

# Comprehensive Process Management Test Runner for CURSED
# 
# This script runs all process management tests including the new enhanced
# features for SlayCommand, pipelines, background tasks, shell execution,
# and IPC communication.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
VERBOSE=false
QUICK_MODE=false
COVERAGE=false
REPORT_FILE=""
FAILED_TESTS=()

# Test categories
BASIC_TESTS=(
    "process_management_comprehensive_integration_test"
    "process_basic_test"
    "process_integration_test"
    "process_enhanced_test"
)

PERFORMANCE_TESTS=(
    "process_management_comprehensive_integration_test::performance_tests"
)

IPC_TESTS=(
    "process_ipc_integration_test"
    "ipc_comprehensive_test"
)

# Function to print colored output
print_color() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to print usage
print_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Options:
    -h, --help              Show this help message
    -v, --verbose           Enable verbose output
    -q, --quick             Run only quick tests (skip performance tests)
    -c, --coverage          Generate code coverage report
    -r, --report FILE       Generate detailed test report to FILE
    --basic                 Run only basic process management tests
    --performance           Run only performance tests (with --ignored)
    --ipc                   Run only IPC tests
    --all                   Run all tests including ignored ones

Examples:
    $0                      # Run all basic tests
    $0 --quick              # Run quick tests only
    $0 --coverage           # Run tests with coverage
    $0 --verbose --report   # Verbose output with report generation

EOF
}

# Function to run a single test
run_test() {
    local test_name=$1
    local description=$2
    local cargo_args=("${@:3}")
    
    print_color "$BLUE" "Running: $description"
    
    if [[ "$VERBOSE" == "true" ]]; then
        echo "Command: cargo ${cargo_args[*]}"
    fi
    
    local start_time=$(date +%s)
    
    if cargo "${cargo_args[@]}" 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_color "$GREEN" "✓ PASSED: $description (${duration}s)"
        return 0
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_color "$RED" "✗ FAILED: $description (${duration}s)"
        FAILED_TESTS+=("$test_name")
        return 1
    fi
}

# Function to run basic tests
run_basic_tests() {
    print_color "$YELLOW" "=== Running Basic Process Management Tests ==="
    
    local failed=0
    
    # Core process management integration test
    run_test "comprehensive_integration" "Comprehensive Process Management Integration" \
        test --test process_management_comprehensive_integration_test || ((failed++))
    
    # Enhanced SlayCommand tests
    run_test "enhanced_slay_command" "Enhanced SlayCommand Tests" \
        test --lib cursed::stdlib::process::enhanced_exec_slay --verbose || ((failed++))
    
    # Pipeline tests
    run_test "pipeline_tests" "Pipeline Execution Tests" \
        test --lib cursed::stdlib::process::pipeline --verbose || ((failed++))
    
    # Background task tests
    run_test "background_tasks" "Background Task Management Tests" \
        test --lib cursed::stdlib::process::background_tasks --verbose || ((failed++))
    
    # Shell command tests
    run_test "shell_commands" "Shell Command Execution Tests" \
        test --lib cursed::stdlib::process::shell_commands --verbose || ((failed++))
    
    # Real IPC tests
    run_test "real_ipc" "Real IPC Communication Tests" \
        test --lib cursed::stdlib::process::real_ipc --verbose || ((failed++))
    
    return $failed
}

# Function to run performance tests
run_performance_tests() {
    print_color "$YELLOW" "=== Running Performance Tests ==="
    
    local failed=0
    
    # Performance tests (ignored by default)
    run_test "performance_many_tasks" "Many Background Tasks Performance" \
        test --test process_management_comprehensive_integration_test \
        performance_tests::test_performance_many_tasks -- --ignored || ((failed++))
    
    run_test "performance_pipeline" "Pipeline Performance Tests" \
        test --test process_management_comprehensive_integration_test \
        performance_tests::test_performance_pipeline_chaining -- --ignored || ((failed++))
    
    return $failed
}

# Function to run IPC tests
run_ipc_tests() {
    print_color "$YELLOW" "=== Running IPC Tests ==="
    
    local failed=0
    
    # Real IPC integration tests
    for test_case in "${IPC_TESTS[@]}"; do
        if [[ -f "$PROJECT_ROOT/tests/${test_case}.rs" ]]; then
            run_test "$test_case" "IPC Test: $test_case" \
                test --test "$test_case" || ((failed++))
        fi
    done
    
    return $failed
}

# Function to generate coverage report
generate_coverage() {
    print_color "$YELLOW" "=== Generating Coverage Report ==="
    
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_color "$RED" "cargo-tarpaulin not found. Installing..."
        cargo install cargo-tarpaulin || {
            print_color "$RED" "Failed to install cargo-tarpaulin"
            return 1
        }
    fi
    
    local coverage_args=(
        tarpaulin
        --verbose
        --timeout 120
        --out Html
        --output-dir "$PROJECT_ROOT/coverage"
    )
    
    # Add test filters for process management
    coverage_args+=(--test process_management_comprehensive_integration_test)
    coverage_args+=(--lib)
    
    if [[ "$VERBOSE" == "true" ]]; then
        echo "Coverage command: cargo ${coverage_args[*]}"
    fi
    
    cargo "${coverage_args[@]}" || {
        print_color "$RED" "Coverage generation failed"
        return 1
    }
    
    print_color "$GREEN" "Coverage report generated in $PROJECT_ROOT/coverage/"
}

# Function to generate detailed test report
generate_report() {
    local report_file=$1
    
    print_color "$YELLOW" "=== Generating Test Report ==="
    
    {
        echo "# CURSED Process Management Test Report"
        echo "Generated on: $(date)"
        echo "Platform: $(uname -a)"
        echo "Rust version: $(rustc --version)"
        echo ""
        
        echo "## Test Summary"
        echo "Total test categories: $(( ${#BASIC_TESTS[@]} + ${#PERFORMANCE_TESTS[@]} + ${#IPC_TESTS[@]} ))"
        echo "Failed tests: ${#FAILED_TESTS[@]}"
        echo ""
        
        if [[ ${#FAILED_TESTS[@]} -gt 0 ]]; then
            echo "## Failed Tests"
            for test in "${FAILED_TESTS[@]}"; do
                echo "- $test"
            done
            echo ""
        fi
        
        echo "## Test Categories"
        echo "### Basic Tests"
        for test in "${BASIC_TESTS[@]}"; do
            echo "- $test"
        done
        echo ""
        
        echo "### Performance Tests"
        for test in "${PERFORMANCE_TESTS[@]}"; do
            echo "- $test"
        done
        echo ""
        
        echo "### IPC Tests"
        for test in "${IPC_TESTS[@]}"; do
            echo "- $test"
        done
        echo ""
        
        echo "## Environment Information"
        echo "- Working directory: $PROJECT_ROOT"
        echo "- Test runner: $0"
        echo "- Options: verbose=$VERBOSE, quick=$QUICK_MODE, coverage=$COVERAGE"
        echo ""
        
        if [[ "$COVERAGE" == "true" ]]; then
            echo "## Coverage Information"
            echo "Coverage report available in: $PROJECT_ROOT/coverage/"
            echo ""
        fi
        
    } > "$report_file"
    
    print_color "$GREEN" "Test report generated: $report_file"
}

# Main execution function
main() {
    local mode="basic"
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                print_usage
                exit 0
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -q|--quick)
                QUICK_MODE=true
                shift
                ;;
            -c|--coverage)
                COVERAGE=true
                shift
                ;;
            -r|--report)
                REPORT_FILE="${2:-process_test_report.md}"
                shift 2
                ;;
            --basic)
                mode="basic"
                shift
                ;;
            --performance)
                mode="performance"
                shift
                ;;
            --ipc)
                mode="ipc"
                shift
                ;;
            --all)
                mode="all"
                shift
                ;;
            *)
                print_color "$RED" "Unknown option: $1"
                print_usage
                exit 1
                ;;
        esac
    done
    
    # Change to project root
    cd "$PROJECT_ROOT"
    
    # Apply linking fix if available
    if [[ -f "./fix_linking.sh" ]]; then
        print_color "$BLUE" "Applying linking fix..."
        source ./fix_linking.sh
    fi
    
    print_color "$BLUE" "Starting CURSED Process Management Test Suite"
    print_color "$BLUE" "Mode: $mode, Verbose: $VERBOSE, Quick: $QUICK_MODE, Coverage: $COVERAGE"
    echo ""
    
    local total_failed=0
    local start_time=$(date +%s)
    
    # Run tests based on mode
    case $mode in
        basic)
            run_basic_tests || total_failed=$?
            ;;
        performance)
            run_performance_tests || total_failed=$?
            ;;
        ipc)
            run_ipc_tests || total_failed=$?
            ;;
        all)
            run_basic_tests || ((total_failed += $?))
            if [[ "$QUICK_MODE" != "true" ]]; then
                run_performance_tests || ((total_failed += $?))
            fi
            run_ipc_tests || ((total_failed += $?))
            ;;
    esac
    
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    
    # Generate coverage if requested
    if [[ "$COVERAGE" == "true" ]]; then
        generate_coverage || ((total_failed++))
    fi
    
    # Generate report if requested
    if [[ -n "$REPORT_FILE" ]]; then
        generate_report "$REPORT_FILE"
    fi
    
    # Print summary
    echo ""
    print_color "$BLUE" "=== Test Summary ==="
    echo "Total duration: ${total_duration}s"
    echo "Failed test categories: $total_failed"
    echo "Failed individual tests: ${#FAILED_TESTS[@]}"
    
    if [[ $total_failed -eq 0 ]]; then
        print_color "$GREEN" "🎉 All process management tests passed!"
        exit 0
    else
        print_color "$RED" "❌ Some tests failed:"
        for test in "${FAILED_TESTS[@]}"; do
            echo "  - $test"
        done
        exit 1
    fi
}

# Handle script interruption
trap 'print_color "$RED" "Test run interrupted"; exit 130' INT TERM

# Run main function
main "$@"
