#!/bin/bash

# Comprehensive Process Management and IPC Test Runner
# 
# This script runs all process management and IPC tests with proper
# environment setup, linking fixes, and comprehensive reporting.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
RESET='\033[0m'

# Test configuration
TEST_TIMEOUT=600  # 10 minutes
VERBOSE=false
QUICK_MODE=false
GENERATE_REPORT=false
COVERAGE_MODE=false
STRESS_TESTS=false
REPORT_FILE=""
SPECIFIC_TEST=""

# Function to print colored output
print_color() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${RESET}"
}

# Function to print help
print_help() {
    echo "Process Management and IPC Comprehensive Test Runner"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --quick              Run only quick validation tests"
    echo "  --verbose            Enable verbose output"
    echo "  --stress             Include stress tests (may take longer)"
    echo "  --coverage           Generate code coverage report"
    echo "  --report FILE        Generate detailed test report"
    echo "  --test NAME          Run specific test suite"
    echo "  --timeout SECONDS    Set test timeout (default: 600)"
    echo "  --help               Show this help message"
    echo ""
    echo "Test suites:"
    echo "  unit                 Process management unit tests"
    echo "  integration          Process management integration tests"
    echo "  ipc-unit             IPC unit tests"
    echo "  ipc-integration      IPC integration tests"
    echo "  stress               Stress tests"
    echo "  ffi                  FFI integration tests"
    echo "  all                  All tests (default)"
    echo ""
    echo "Examples:"
    echo "  $0 --quick                    # Quick validation"
    echo "  $0 --test unit --verbose      # Unit tests with verbose output"
    echo "  $0 --stress --report results.md  # Full test with report"
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
        --stress)
            STRESS_TESTS=true
            shift
            ;;
        --coverage)
            COVERAGE_MODE=true
            shift
            ;;
        --report)
            GENERATE_REPORT=true
            REPORT_FILE="$2"
            shift 2
            ;;
        --test)
            SPECIFIC_TEST="$2"
            shift 2
            ;;
        --timeout)
            TEST_TIMEOUT="$2"
            shift 2
            ;;
        --help)
            print_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            print_help
            exit 1
            ;;
    esac
done

# Set default report file if not specified
if [[ "$GENERATE_REPORT" == "true" && -z "$REPORT_FILE" ]]; then
    REPORT_FILE="process_ipc_test_report_$(date +%Y%m%d_%H%M%S).md"
fi

# Verbose flag for cargo
CARGO_VERBOSE=""
if [[ "$VERBOSE" == "true" ]]; then
    CARGO_VERBOSE="--verbose"
fi

print_color $BLUE "🔧 Process Management and IPC Comprehensive Test Suite"
print_color $BLUE "=================================================="

# Check if we're in a Nix environment and set up linking
if [[ -n "$NIX_STORE" ]]; then
    print_color $YELLOW "📦 Detected Nix environment, setting up linking..."
    
    # Check if linking fix script exists
    if [[ -f "./fix_linking.sh" ]]; then
        CARGO_PREFIX="./fix_linking.sh"
        print_color $GREEN "✅ Using linking fix script"
    else
        print_color $YELLOW "⚠️ Linking fix script not found, proceeding without it"
        CARGO_PREFIX=""
    fi
else
    CARGO_PREFIX=""
fi

# Function to run a test with proper error handling
run_test() {
    local test_name="$1"
    local test_file="$2"
    local description="$3"
    local is_stress="${4:-false}"
    
    # Skip stress tests if not requested
    if [[ "$is_stress" == "true" && "$STRESS_TESTS" != "true" && "$QUICK_MODE" == "true" ]]; then
        print_color $YELLOW "⏭️ Skipping stress test: $test_name (use --stress to include)"
        return 0
    fi
    
    print_color $CYAN "🧪 Running $description..."
    
    local start_time=$(date +%s)
    local cmd="$CARGO_PREFIX cargo test $CARGO_VERBOSE --test $test_file"
    
    if [[ "$COVERAGE_MODE" == "true" ]]; then
        cmd="$CARGO_PREFIX cargo tarpaulin --test $test_file --out Xml --output-dir coverage/"
    fi
    
    if timeout $TEST_TIMEOUT $cmd; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_color $GREEN "✅ $test_name completed successfully (${duration}s)"
        
        # Store result for report
        if [[ "$GENERATE_REPORT" == "true" ]]; then
            echo "- ✅ **$test_name**: $description - Passed (${duration}s)" >> "$REPORT_FILE.tmp"
        fi
        
        return 0
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_color $RED "❌ $test_name failed or timed out (${duration}s)"
        
        # Store result for report
        if [[ "$GENERATE_REPORT" == "true" ]]; then
            echo "- ❌ **$test_name**: $description - Failed (${duration}s)" >> "$REPORT_FILE.tmp"
        fi
        
        return 1
    fi
}

# Create temporary report file if needed
if [[ "$GENERATE_REPORT" == "true" ]]; then
    echo "# Process Management and IPC Test Report" > "$REPORT_FILE.tmp"
    echo "" >> "$REPORT_FILE.tmp"
    echo "Generated on: $(date)" >> "$REPORT_FILE.tmp"
    echo "Test mode: ${SPECIFIC_TEST:-all}" >> "$REPORT_FILE.tmp"
    if [[ "$QUICK_MODE" == "true" ]]; then
        echo "Quick mode: enabled" >> "$REPORT_FILE.tmp"
    fi
    if [[ "$STRESS_TESTS" == "true" ]]; then
        echo "Stress tests: enabled" >> "$REPORT_FILE.tmp"
    fi
    echo "" >> "$REPORT_FILE.tmp"
    echo "## Test Results" >> "$REPORT_FILE.tmp"
    echo "" >> "$REPORT_FILE.tmp"
fi

# Track test results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
START_TIME=$(date +%s)

# Function to update test counters
update_counters() {
    local result=$1
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if [[ $result -eq 0 ]]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

# Define test suites
declare -A TEST_SUITES
TEST_SUITES[unit]="process_management_unit_test,Process Management Unit Tests,false"
TEST_SUITES[integration]="process_management_integration_test,Process Management Integration Tests,false"
TEST_SUITES[ipc-unit]="ipc_comprehensive_unit_test,IPC Comprehensive Unit Tests,false"
TEST_SUITES[ipc-integration]="ipc_advanced_integration_test,IPC Advanced Integration Tests,false"
TEST_SUITES[stress]="process_ipc_stress_test,Process and IPC Stress Tests,true"
TEST_SUITES[ffi]="ffi_process_integration_test,FFI Process Integration Tests,false"

# Run specific test or all tests
if [[ -n "$SPECIFIC_TEST" ]]; then
    if [[ -n "${TEST_SUITES[$SPECIFIC_TEST]}" ]]; then
        IFS=',' read -r test_file description is_stress <<< "${TEST_SUITES[$SPECIFIC_TEST]}"
        run_test "$SPECIFIC_TEST" "$test_file" "$description" "$is_stress"
        update_counters $?
    else
        print_color $RED "❌ Unknown test suite: $SPECIFIC_TEST"
        print_color $YELLOW "Available test suites: ${!TEST_SUITES[@]}"
        exit 1
    fi
else
    # Run all tests based on mode
    if [[ "$QUICK_MODE" == "true" ]]; then
        print_color $YELLOW "🚀 Running quick validation tests..."
        
        # Run only essential tests in quick mode
        for test_suite in unit ipc-unit; do
            IFS=',' read -r test_file description is_stress <<< "${TEST_SUITES[$test_suite]}"
            run_test "$test_suite" "$test_file" "$description" "$is_stress"
            update_counters $?
        done
    else
        print_color $YELLOW "🔄 Running comprehensive test suite..."
        
        # Run all tests in order of complexity
        test_order=("unit" "ipc-unit" "integration" "ipc-integration" "ffi")
        
        # Add stress tests if requested
        if [[ "$STRESS_TESTS" == "true" ]]; then
            test_order+=("stress")
        fi
        
        for test_suite in "${test_order[@]}"; do
            IFS=',' read -r test_file description is_stress <<< "${TEST_SUITES[$test_suite]}"
            run_test "$test_suite" "$test_file" "$description" "$is_stress"
            update_counters $?
        done
    fi
fi

# Calculate total time
END_TIME=$(date +%s)
TOTAL_DURATION=$((END_TIME - START_TIME))

# Print summary
print_color $BLUE ""
print_color $BLUE "📊 Test Summary"
print_color $BLUE "==============="
print_color $GREEN "✅ Passed: $PASSED_TESTS"
print_color $RED "❌ Failed: $FAILED_TESTS"
print_color $BLUE "📝 Total:  $TOTAL_TESTS"
print_color $BLUE "⏱️ Duration: ${TOTAL_DURATION}s"

# Calculate success rate
if [[ $TOTAL_TESTS -gt 0 ]]; then
    SUCCESS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    print_color $BLUE "📈 Success Rate: ${SUCCESS_RATE}%"
fi

# Generate final report
if [[ "$GENERATE_REPORT" == "true" ]]; then
    {
        echo ""
        echo "## Summary"
        echo ""
        echo "- **Total Tests**: $TOTAL_TESTS"
        echo "- **Passed**: $PASSED_TESTS"
        echo "- **Failed**: $FAILED_TESTS"
        echo "- **Success Rate**: ${SUCCESS_RATE}%"
        echo "- **Total Duration**: ${TOTAL_DURATION}s"
        echo ""
        
        if [[ $FAILED_TESTS -gt 0 ]]; then
            echo "## Recommendations"
            echo ""
            echo "Some tests failed. This could be due to:"
            echo "- Restricted test environment (CI/containers may limit process spawning)"
            echo "- Missing system dependencies"
            echo "- Resource constraints under load"
            echo "- Platform-specific limitations"
            echo ""
            echo "For production use, ensure all tests pass in the target environment."
            echo ""
        fi
        
        echo "## Test Environment"
        echo ""
        echo "- Platform: $(uname -a)"
        echo "- Rust version: $(rustc --version 2>/dev/null || echo 'Not available')"
        echo "- Cargo version: $(cargo --version 2>/dev/null || echo 'Not available')"
        if [[ -n "$NIX_STORE" ]]; then
            echo "- Nix environment: Yes"
        else
            echo "- Nix environment: No"
        fi
        echo ""
        
    } >> "$REPORT_FILE.tmp"
    
    mv "$REPORT_FILE.tmp" "$REPORT_FILE"
    print_color $GREEN "📄 Test report generated: $REPORT_FILE"
fi

# Generate coverage report if requested
if [[ "$COVERAGE_MODE" == "true" ]]; then
    if [[ -d "coverage" ]]; then
        print_color $GREEN "📊 Coverage report generated in coverage/ directory"
    else
        print_color $YELLOW "⚠️ Coverage directory not found"
    fi
fi

# Exit with appropriate code
if [[ $FAILED_TESTS -eq 0 ]]; then
    print_color $GREEN "🎉 All tests passed!"
    exit 0
else
    print_color $RED "💥 Some tests failed"
    exit 1
fi
