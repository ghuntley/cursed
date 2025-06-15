#!/bin/bash

# Comprehensive test runner for Process Management and IPC System
# Tests exec_vibez, signal_boost, and IPC functionality

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LINKING_FIX_SCRIPT="$SCRIPT_DIR/fix_linking.sh"
TEST_TIMEOUT="300" # 5 minutes
VERBOSE=false
SPECIFIC_TEST=""
REPORT_FILE=""
IGNORED_TESTS=false

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to print usage
print_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -v, --verbose         Enable verbose output"
    echo "  -t, --test TEST_NAME  Run specific test"
    echo "  -r, --report FILE     Generate detailed report"
    echo "  -i, --ignored         Run ignored/stress tests"
    echo "  -h, --help           Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Run all basic tests"
    echo "  $0 --verbose                         # Run with verbose output"
    echo "  $0 --test test_basic_command         # Run specific test"
    echo "  $0 --report process_ipc_report.md    # Generate detailed report"
    echo "  $0 --ignored                         # Run stress/performance tests"
    echo ""
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -t|--test)
            SPECIFIC_TEST="$2"
            shift 2
            ;;
        -r|--report)
            REPORT_FILE="$2"
            shift 2
            ;;
        -i|--ignored)
            IGNORED_TESTS=true
            shift
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            print_usage
            exit 1
            ;;
    esac
done

# Function to check if linking fix is available
check_linking_fix() {
    if [[ -f "$LINKING_FIX_SCRIPT" ]]; then
        print_status "$BLUE" "✓ Linking fix script found: $LINKING_FIX_SCRIPT"
        return 0
    else
        print_status "$YELLOW" "⚠ Linking fix script not found, using direct cargo commands"
        return 1
    fi
}

# Function to run cargo command with linking fix if available
run_cargo_command() {
    local cmd="$1"
    local args="${2:-}"
    
    if check_linking_fix; then
        if [[ $VERBOSE == true ]]; then
            print_status "$BLUE" "Running: $LINKING_FIX_SCRIPT $cmd $args"
        fi
        "$LINKING_FIX_SCRIPT" "$cmd" $args
    else
        if [[ $VERBOSE == true ]]; then
            print_status "$BLUE" "Running: cargo $cmd $args"
        fi
        cargo "$cmd" $args
    fi
}

# Function to run a specific test
run_test() {
    local test_name="$1"
    local test_args="${2:-}"
    local description="${3:-$test_name}"
    
    print_status "$BLUE" "Running: $description"
    
    if [[ $VERBOSE == true ]]; then
        echo "Command: cargo test --test $test_name $test_args"
    fi
    
    local start_time=$(date +%s)
    local result=0
    
    if run_cargo_command "test" "--test $test_name $test_args" > /tmp/test_output_$test_name.log 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_status "$GREEN" "✓ $description completed in ${duration}s"
        
        if [[ $VERBOSE == true ]]; then
            echo "--- Test Output ---"
            cat /tmp/test_output_$test_name.log
            echo "--- End Output ---"
        fi
    else
        result=$?
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_status "$RED" "✗ $description failed after ${duration}s (exit code: $result)"
        
        echo "--- Error Output ---"
        cat /tmp/test_output_$test_name.log
        echo "--- End Error Output ---"
        
        return $result
    fi
    
    # Clean up log file
    rm -f /tmp/test_output_$test_name.log
    return 0
}

# Function to check if tests compile
check_test_compilation() {
    print_status "$BLUE" "Checking test compilation..."
    
    if run_cargo_command "check" "--tests"; then
        print_status "$GREEN" "✓ All tests compile successfully"
        return 0
    else
        print_status "$RED" "✗ Test compilation failed"
        return 1
    fi
}

# Function to run all basic tests
run_basic_tests() {
    print_status "$YELLOW" "=== Running Basic Process Management and IPC Tests ==="
    
    local tests_run=0
    local tests_passed=0
    local start_time=$(date +%s)
    
    # Integration test
    if [[ -z "$SPECIFIC_TEST" || "$SPECIFIC_TEST" == *"integration"* ]]; then
        if run_test "process_ipc_integration_test" "" "Process and IPC Integration Tests"; then
            ((tests_passed++))
        fi
        ((tests_run++))
    fi
    
    # Individual component tests
    local component_tests=(
        "exec_vibez_basic_test:Basic Command Execution Tests"
        "exec_vibez_advanced_test:Advanced Command Features Tests"
        "signal_boost_core_test:Signal Handling Core Tests"
        "signal_boost_graceful_test:Graceful Shutdown Tests"
        "ipc_basic_test:Basic IPC Functionality Tests"
        "ipc_advanced_test:Advanced IPC Features Tests"
    )
    
    for test_spec in "${component_tests[@]}"; do
        IFS=':' read -r test_name test_desc <<< "$test_spec"
        
        if [[ -z "$SPECIFIC_TEST" || "$SPECIFIC_TEST" == *"$test_name"* ]]; then
            # Check if test file exists before running
            if [[ -f "tests/${test_name}.rs" ]]; then
                if run_test "$test_name" "" "$test_desc"; then
                    ((tests_passed++))
                fi
            else
                print_status "$YELLOW" "⚠ Test file tests/${test_name}.rs not found, skipping"
            fi
            ((tests_run++))
        fi
    done
    
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    
    print_status "$YELLOW" "=== Basic Tests Summary ==="
    print_status "$BLUE" "Tests run: $tests_run"
    print_status "$GREEN" "Tests passed: $tests_passed"
    
    if [[ $tests_passed -eq $tests_run ]]; then
        print_status "$GREEN" "✓ All basic tests passed in ${total_duration}s"
        return 0
    else
        local failed=$((tests_run - tests_passed))
        print_status "$RED" "✗ $failed tests failed in ${total_duration}s"
        return 1
    fi
}

# Function to run stress/performance tests
run_stress_tests() {
    print_status "$YELLOW" "=== Running Stress and Performance Tests ==="
    
    local tests_run=0
    local tests_passed=0
    local start_time=$(date +%s)
    
    local stress_tests=(
        "process_stress_test:Process Management Stress Tests"
        "ipc_stress_test:IPC Performance and Stress Tests"
        "signal_stress_test:Signal Handling Stress Tests"
        "memory_safety_test:Memory Safety and Resource Tests"
    )
    
    for test_spec in "${stress_tests[@]}"; do
        IFS=':' read -r test_name test_desc <<< "$test_spec"
        
        if [[ -z "$SPECIFIC_TEST" || "$SPECIFIC_TEST" == *"$test_name"* ]]; then
            # Check if test file exists before running
            if [[ -f "tests/${test_name}.rs" ]]; then
                if run_test "$test_name" "-- --ignored" "$test_desc"; then
                    ((tests_passed++))
                fi
            else
                print_status "$YELLOW" "⚠ Test file tests/${test_name}.rs not found, skipping"
            fi
            ((tests_run++))
        fi
    done
    
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    
    print_status "$YELLOW" "=== Stress Tests Summary ==="
    print_status "$BLUE" "Tests run: $tests_run"
    print_status "$GREEN" "Tests passed: $tests_passed"
    
    if [[ $tests_passed -eq $tests_run ]]; then
        print_status "$GREEN" "✓ All stress tests passed in ${total_duration}s"
        return 0
    else
        local failed=$((tests_run - tests_passed))
        print_status "$RED" "✗ $failed stress tests failed in ${total_duration}s"
        return 1
    fi
}

# Function to generate detailed report
generate_report() {
    local report_file="$1"
    
    print_status "$BLUE" "Generating detailed report: $report_file"
    
    cat > "$report_file" << EOF
# Process Management and IPC System Test Report

Generated: $(date)
Platform: $(uname -a)

## Test Environment

- Rust version: $(rustc --version)
- Cargo version: $(cargo --version)
- Test runner: $0
- Linking fix: $(if [[ -f "$LINKING_FIX_SCRIPT" ]]; then echo "Available"; else echo "Not available"; fi)

## Test Categories

### 1. Basic Integration Tests
- **exec_vibez**: External command execution with enhanced features
- **signal_boost**: OS signal handling with graceful shutdown patterns
- **IPC**: Inter-process communication mechanisms

### 2. Component Tests
- **Command Execution**: Basic and advanced command execution features
- **Process Groups**: Multiple process coordination and management
- **Signal Handling**: Signal notification, multiplexing, and filtering
- **IPC Mechanisms**: Named pipes, message queues, shared memory, semaphores

### 3. Stress and Performance Tests
- **Resource Management**: Memory usage and cleanup validation
- **Concurrent Operations**: Multi-threaded and multi-process scenarios
- **Performance Benchmarks**: Throughput and latency measurements

## Test Results

EOF

    # Run tests and capture results for report
    local overall_result=0
    
    # Basic tests
    echo "### Basic Tests" >> "$report_file"
    echo "" >> "$report_file"
    
    if run_basic_tests >> "$report_file" 2>&1; then
        echo "✓ Basic tests: PASSED" >> "$report_file"
    else
        echo "✗ Basic tests: FAILED" >> "$report_file"
        overall_result=1
    fi
    echo "" >> "$report_file"
    
    # Stress tests if requested
    if [[ $IGNORED_TESTS == true ]]; then
        echo "### Stress Tests" >> "$report_file"
        echo "" >> "$report_file"
        
        if run_stress_tests >> "$report_file" 2>&1; then
            echo "✓ Stress tests: PASSED" >> "$report_file"
        else
            echo "✗ Stress tests: FAILED" >> "$report_file"
            overall_result=1
        fi
        echo "" >> "$report_file"
    fi
    
    # Summary
    cat >> "$report_file" << EOF

## Summary

- **Overall Result**: $(if [[ $overall_result -eq 0 ]]; then echo "PASSED"; else echo "FAILED"; fi)
- **Generated**: $(date)
- **Test Duration**: Test execution completed

## Recommendations

1. **If tests passed**: The Process Management and IPC system is ready for production use
2. **If tests failed**: Review the error output above and address any issues
3. **Performance**: Monitor resource usage in production environments
4. **Security**: Ensure proper permissions and validation in production deployments

## Next Steps

1. Integrate with CI/CD pipeline for continuous testing
2. Add platform-specific tests for Windows/macOS/Linux
3. Implement additional IPC mechanisms as needed
4. Performance optimization based on benchmark results

EOF

    print_status "$GREEN" "✓ Report generated: $report_file"
    return $overall_result
}

# Function to validate prerequisites
validate_prerequisites() {
    print_status "$BLUE" "Validating prerequisites..."
    
    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]]; then
        print_status "$RED" "✗ Cargo.toml not found. Please run from the project root directory."
        exit 1
    fi
    
    # Check if Rust is available
    if ! command -v rustc &> /dev/null; then
        print_status "$RED" "✗ Rust compiler not found. Please install Rust."
        exit 1
    fi
    
    # Check if Cargo is available
    if ! command -v cargo &> /dev/null; then
        print_status "$RED" "✗ Cargo not found. Please install Cargo."
        exit 1
    fi
    
    print_status "$GREEN" "✓ Prerequisites validated"
}

# Function to clean up temporary files
cleanup() {
    print_status "$BLUE" "Cleaning up temporary files..."
    rm -f /tmp/test_output_*.log
    print_status "$GREEN" "✓ Cleanup completed"
}

# Main execution function
main() {
    local overall_result=0
    local start_time=$(date +%s)
    
    print_status "$YELLOW" "🚀 Process Management and IPC System Test Runner"
    print_status "$BLUE" "Started at: $(date)"
    echo ""
    
    # Validate prerequisites
    validate_prerequisites
    
    # Check test compilation first
    if ! check_test_compilation; then
        print_status "$RED" "✗ Test compilation failed. Please fix compilation errors first."
        exit 1
    fi
    
    # Generate report if requested
    if [[ -n "$REPORT_FILE" ]]; then
        if ! generate_report "$REPORT_FILE"; then
            overall_result=1
        fi
    else
        # Run basic tests
        if ! run_basic_tests; then
            overall_result=1
        fi
        
        # Run stress tests if requested
        if [[ $IGNORED_TESTS == true ]]; then
            echo ""
            if ! run_stress_tests; then
                overall_result=1
            fi
        fi
    fi
    
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    
    echo ""
    print_status "$YELLOW" "=== Final Summary ==="
    print_status "$BLUE" "Total execution time: ${total_duration}s"
    
    if [[ $overall_result -eq 0 ]]; then
        print_status "$GREEN" "🎉 All tests completed successfully!"
        print_status "$GREEN" "The Process Management and IPC system is ready for production use."
    else
        print_status "$RED" "💥 Some tests failed!"
        print_status "$RED" "Please review the error output and fix any issues."
    fi
    
    # Cleanup
    cleanup
    
    exit $overall_result
}

# Set up signal handling for cleanup
trap cleanup EXIT

# Run main function
main "$@"
