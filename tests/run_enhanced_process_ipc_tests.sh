#!/bin/bash
# Comprehensive test runner for enhanced process management and IPC system

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
LINKING_FIX_SCRIPT="${PROJECT_ROOT}/fix_linking.sh"

# Test categories
declare -a BASIC_TESTS=(
    "enhanced_process_ipc_comprehensive_test"
)

declare -a INTEGRATION_TESTS=(
    "process_ipc_integration_test"
    "process_management_comprehensive_test"
    "ipc_comprehensive_test"
    "signal_boost_integration_test"
)

declare -a STRESS_TESTS=(
    "process_management_stress_test"
    "ipc_stress_test"
)

declare -a PERFORMANCE_TESTS=(
    "process_ipc_performance_test"
)

# Global variables
VERBOSE=false
QUICK=false
REPORT_FILE=""
COVERAGE=false
TEST_PATTERN=""
IGNORED_TESTS=false

# Function to print colored output
print_color() {
    local color=$1
    shift
    echo -e "${color}$@${NC}"
}

# Function to print section headers
print_section() {
    echo
    print_color $BLUE "============================================================"
    print_color $BLUE "$1"
    print_color $BLUE "============================================================"
    echo
}

# Function to run a test with linking fix
run_test_with_linking_fix() {
    local test_name=$1
    local extra_args="${2:-}"
    
    if [[ -f "$LINKING_FIX_SCRIPT" ]]; then
        print_color $YELLOW "Running test with linking fix: $test_name"
        if $VERBOSE; then
            "$LINKING_FIX_SCRIPT" cargo test --test "$test_name" $extra_args -- --nocapture
        else
            "$LINKING_FIX_SCRIPT" cargo test --test "$test_name" $extra_args
        fi
    else
        print_color $YELLOW "Running test without linking fix: $test_name"
        if $VERBOSE; then
            cargo test --test "$test_name" $extra_args -- --nocapture
        else
            cargo test --test "$test_name" $extra_args
        fi
    fi
}

# Function to run a specific test category
run_test_category() {
    local category=$1
    local test_array_name=$2
    local extra_args="${3:-}"
    
    print_section "$category Tests"
    
    # Get array reference
    local -n test_array=$test_array_name
    local passed=0
    local failed=0
    local test_results=()
    
    for test in "${test_array[@]}"; do
        echo
        print_color $YELLOW "Running $category test: $test"
        
        local start_time=$(date +%s)
        if run_test_with_linking_fix "$test" "$extra_args"; then
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            print_color $GREEN "✓ $test passed (${duration}s)"
            test_results+=("PASS: $test (${duration}s)")
            ((passed++))
        else
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            print_color $RED "✗ $test failed (${duration}s)"
            test_results+=("FAIL: $test (${duration}s)")
            ((failed++))
        fi
    done
    
    echo
    print_color $BLUE "$category Results:"
    for result in "${test_results[@]}"; do
        if [[ $result == PASS:* ]]; then
            print_color $GREEN "  $result"
        else
            print_color $RED "  $result"
        fi
    done
    
    echo
    print_color $BLUE "$category Summary: $passed passed, $failed failed"
    
    return $failed
}

# Function to generate test report
generate_report() {
    if [[ -n "$REPORT_FILE" ]]; then
        print_section "Generating Test Report"
        
        cat > "$REPORT_FILE" << EOF
# Enhanced Process Management and IPC Test Report

Generated on: $(date)
Platform: $(uname -a)
Rust Version: $(rustc --version)

## Test Categories

EOF
        
        # Add basic test results
        if [[ ${#BASIC_TESTS[@]} -gt 0 ]]; then
            echo "### Basic Tests" >> "$REPORT_FILE"
            echo >> "$REPORT_FILE"
            for test in "${BASIC_TESTS[@]}"; do
                echo "- $test" >> "$REPORT_FILE"
            done
            echo >> "$REPORT_FILE"
        fi
        
        # Add integration test results  
        if [[ ${#INTEGRATION_TESTS[@]} -gt 0 ]]; then
            echo "### Integration Tests" >> "$REPORT_FILE"
            echo >> "$REPORT_FILE"
            for test in "${INTEGRATION_TESTS[@]}"; do
                echo "- $test" >> "$REPORT_FILE"
            done
            echo >> "$REPORT_FILE"
        fi
        
        # Add performance information
        echo "### System Information" >> "$REPORT_FILE"
        echo >> "$REPORT_FILE"
        echo "- CPU: $(nproc) cores" >> "$REPORT_FILE"
        echo "- Memory: $(free -h | grep '^Mem:' | awk '{print $2}' || echo 'Unknown')" >> "$REPORT_FILE"
        echo "- Disk: $(df -h / | tail -1 | awk '{print $4}' || echo 'Unknown') available" >> "$REPORT_FILE"
        echo >> "$REPORT_FILE"
        
        print_color $GREEN "Report generated: $REPORT_FILE"
    fi
}

# Function to run coverage analysis
run_coverage() {
    if $COVERAGE; then
        print_section "Code Coverage Analysis"
        
        if command -v cargo-tarpaulin >/dev/null 2>&1; then
            print_color $YELLOW "Running code coverage analysis..."
            
            if [[ -f "$LINKING_FIX_SCRIPT" ]]; then
                "$LINKING_FIX_SCRIPT" cargo tarpaulin \
                    --tests \
                    --out Html \
                    --output-dir coverage \
                    --engine llvm \
                    --skip-clean \
                    --include-tests \
                    --exclude-files 'target/*' 'tests/*' \
                    --timeout 300
            else
                cargo tarpaulin \
                    --tests \
                    --out Html \
                    --output-dir coverage \
                    --engine llvm \
                    --skip-clean \
                    --include-tests \
                    --exclude-files 'target/*' 'tests/*' \
                    --timeout 300
            fi
            
            print_color $GREEN "Coverage report generated in coverage/"
        else
            print_color $RED "cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin"
        fi
    fi
}

# Function to show usage
show_usage() {
    cat << EOF
Usage: $0 [OPTIONS] [TEST_PATTERN]

Enhanced Process Management and IPC Test Runner

OPTIONS:
    --help, -h          Show this help message
    --verbose, -v       Enable verbose output
    --quick, -q         Run only quick tests (basic category)
    --report FILE       Generate test report to FILE
    --coverage          Run code coverage analysis
    --pattern PATTERN   Run tests matching pattern
    --ignored           Run ignored tests
    --test CATEGORY     Run specific test category

TEST CATEGORIES:
    basic              Enhanced process and IPC comprehensive tests
    integration        Integration tests across modules
    stress             Stress tests for performance validation
    performance        Performance benchmarks
    all                All test categories (default)

EXAMPLES:
    $0                          # Run all tests
    $0 --quick                  # Run only basic tests
    $0 --verbose --report report.md  # Verbose with report
    $0 --test basic             # Run only basic tests
    $0 --pattern "process"      # Run tests matching "process"
    $0 --coverage               # Run with coverage analysis

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
        --report)
            REPORT_FILE="$2"
            shift 2
            ;;
        --coverage)
            COVERAGE=true
            shift
            ;;
        --pattern)
            TEST_PATTERN="$2"
            shift 2
            ;;
        --ignored)
            IGNORED_TESTS=true
            shift
            ;;
        --test)
            TEST_CATEGORY="$2"
            shift 2
            ;;
        *)
            if [[ -z "$TEST_PATTERN" ]]; then
                TEST_PATTERN="$1"
            fi
            shift
            ;;
    esac
done

# Main execution
main() {
    cd "$PROJECT_ROOT"
    
    print_section "Enhanced Process Management and IPC Test Suite"
    
    echo "Project Root: $PROJECT_ROOT"
    echo "Linking Fix: $([ -f "$LINKING_FIX_SCRIPT" ] && echo "Available" || echo "Not Available")"
    echo "Verbose: $VERBOSE"
    echo "Quick Mode: $QUICK"
    echo "Coverage: $COVERAGE"
    echo "Test Pattern: ${TEST_PATTERN:-"(all)"}"
    echo
    
    local total_failed=0
    local categories_run=0
    
    # Determine which test categories to run
    local run_basic=false
    local run_integration=false
    local run_stress=false
    local run_performance=false
    
    if [[ -n "$TEST_CATEGORY" ]]; then
        case "$TEST_CATEGORY" in
            basic) run_basic=true ;;
            integration) run_integration=true ;;
            stress) run_stress=true ;;
            performance) run_performance=true ;;
            all) run_basic=true; run_integration=true; run_stress=true; run_performance=true ;;
            *) 
                print_color $RED "Unknown test category: $TEST_CATEGORY"
                exit 1
                ;;
        esac
    elif $QUICK; then
        run_basic=true
    else
        run_basic=true
        run_integration=true
        run_stress=true
        run_performance=true
    fi
    
    # Build extra arguments
    local extra_args=""
    if [[ -n "$TEST_PATTERN" ]]; then
        extra_args="$TEST_PATTERN"
    fi
    if $IGNORED_TESTS; then
        extra_args="$extra_args -- --ignored"
    fi
    
    # Run test categories
    if $run_basic; then
        run_test_category "Basic" BASIC_TESTS "$extra_args"
        total_failed=$((total_failed + $?))
        ((categories_run++))
    fi
    
    if $run_integration; then
        run_test_category "Integration" INTEGRATION_TESTS "$extra_args"
        total_failed=$((total_failed + $?))
        ((categories_run++))
    fi
    
    if $run_stress; then
        run_test_category "Stress" STRESS_TESTS "$extra_args"
        total_failed=$((total_failed + $?))
        ((categories_run++))
    fi
    
    if $run_performance; then
        run_test_category "Performance" PERFORMANCE_TESTS "$extra_args"
        total_failed=$((total_failed + $?))
        ((categories_run++))
    fi
    
    # Run coverage analysis
    run_coverage
    
    # Generate report
    generate_report
    
    # Final summary
    print_section "Final Results"
    
    if [[ $total_failed -eq 0 ]]; then
        print_color $GREEN "🎉 All tests passed! ($categories_run categories)"
        echo
        print_color $GREEN "Enhanced Process Management and IPC system is production-ready!"
    else
        print_color $RED "❌ $total_failed test failures across $categories_run categories"
        echo
        print_color $RED "Please review and fix failing tests before production use."
    fi
    
    # Exit with appropriate code
    exit $total_failed
}

# Check if running in CI environment
if [[ "${CI:-false}" == "true" ]]; then
    print_color $BLUE "Running in CI environment"
    VERBOSE=true
fi

# Run main function
main "$@"
