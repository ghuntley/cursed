#!/bin/bash

# PGO Test Runner
# Comprehensive test runner for Profile-Guided Optimization system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LINKING_FIX="$PROJECT_ROOT/fix_linking.sh"

# Test categories
UNIT_TESTS="pgo_integration_test"
PERFORMANCE_TESTS="pgo_performance_test"
ALL_TESTS="$UNIT_TESTS $PERFORMANCE_TESTS"

# Default settings
VERBOSE=false
RUN_PERFORMANCE=false
RUN_IGNORED=false
GENERATE_REPORT=false
REPORT_FILE=""
TEST_FILTER=""

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

print_header() {
    print_status "$BLUE" "=================================="
    print_status "$BLUE" "$1"
    print_status "$BLUE" "=================================="
}

print_success() {
    print_status "$GREEN" "✓ $1"
}

print_warning() {
    print_status "$YELLOW" "⚠ $1"
}

print_error() {
    print_status "$RED" "✗ $1"
}

# Function to show usage
show_usage() {
    cat << EOF
CURSED PGO Test Runner

USAGE:
    $0 [OPTIONS] [TEST_CATEGORY]

OPTIONS:
    -h, --help              Show this help message
    -v, --verbose           Enable verbose output
    -p, --performance       Run performance tests (may take longer)
    -i, --ignored           Run ignored/slow tests
    -r, --report FILE       Generate test report to file
    -f, --filter PATTERN    Run only tests matching pattern
    
    --quick                 Run only quick validation tests
    --all                   Run all tests including performance
    --integration           Run integration tests only
    --unit                  Run unit tests only

TEST CATEGORIES:
    integration             PGO integration tests
    performance             PGO performance and benchmark tests
    all                     All PGO tests (default)

EXAMPLES:
    $0                                    # Run basic integration tests
    $0 --all --verbose                   # Run all tests with verbose output
    $0 --performance --report pgo.md     # Run performance tests with report
    $0 --filter "test_optimization"      # Run specific test pattern
    $0 --quick                          # Quick validation only

EOF
}

# Function to check if linking fix is needed and available
check_linking() {
    if [[ -f "$LINKING_FIX" ]]; then
        print_status "$YELLOW" "Using linking fix script: $LINKING_FIX"
        return 0
    else
        print_warning "Linking fix script not found, tests may fail in Nix environment"
        return 1
    fi
}

# Function to run tests with appropriate environment
run_test_command() {
    local test_name=$1
    local extra_args=$2
    
    local cmd="cargo test --test $test_name"
    
    if [[ -n "$extra_args" ]]; then
        cmd="$cmd $extra_args"
    fi
    
    if [[ -n "$TEST_FILTER" ]]; then
        cmd="$cmd $TEST_FILTER"
    fi
    
    if [[ "$VERBOSE" == "true" ]]; then
        cmd="$cmd -- --nocapture"
    fi
    
    print_status "$BLUE" "Running: $cmd"
    
    if [[ -f "$LINKING_FIX" ]]; then
        "$LINKING_FIX" $cmd
    else
        eval $cmd
    fi
}

# Function to run integration tests
run_integration_tests() {
    print_header "PGO Integration Tests"
    
    local tests_passed=0
    local tests_failed=0
    
    for test in $UNIT_TESTS; do
        print_status "$BLUE" "Running $test..."
        
        if run_test_command "$test" ""; then
            print_success "$test completed successfully"
            ((tests_passed++))
        else
            print_error "$test failed"
            ((tests_failed++))
        fi
    done
    
    print_status "$BLUE" "Integration Tests Summary: $tests_passed passed, $tests_failed failed"
    return $tests_failed
}

# Function to run performance tests
run_performance_tests() {
    print_header "PGO Performance Tests"
    
    if [[ "$RUN_PERFORMANCE" != "true" ]]; then
        print_warning "Performance tests skipped (use --performance to enable)"
        return 0
    fi
    
    local tests_passed=0
    local tests_failed=0
    
    for test in $PERFORMANCE_TESTS; do
        print_status "$BLUE" "Running $test..."
        
        local extra_args=""
        if [[ "$RUN_IGNORED" == "true" ]]; then
            extra_args="-- --ignored"
        fi
        
        if run_test_command "$test" "$extra_args"; then
            print_success "$test completed successfully"
            ((tests_passed++))
        else
            print_error "$test failed"
            ((tests_failed++))
        fi
    done
    
    print_status "$BLUE" "Performance Tests Summary: $tests_passed passed, $tests_failed failed"
    return $tests_failed
}

# Function to run quick validation tests
run_quick_tests() {
    print_header "PGO Quick Validation Tests"
    
    # Run only essential tests for quick validation
    local quick_tests="test_pgo_manager_creation test_pgo_config_serialization test_profile_data_validation"
    
    for test_pattern in $quick_tests; do
        print_status "$BLUE" "Running quick test: $test_pattern"
        
        if run_test_command "pgo_integration_test" "-- $test_pattern"; then
            print_success "$test_pattern passed"
        else
            print_error "$test_pattern failed"
            return 1
        fi
    done
    
    print_success "All quick validation tests passed"
    return 0
}

# Function to generate test report
generate_report() {
    local report_file=$1
    
    print_header "Generating PGO Test Report"
    
    cat > "$report_file" << EOF
# CURSED PGO Test Report

Generated on: $(date)
Test Runner: $0
Environment: $(uname -a)

## Test Configuration

- Verbose Output: $VERBOSE
- Performance Tests: $RUN_PERFORMANCE
- Ignored Tests: $RUN_IGNORED
- Test Filter: ${TEST_FILTER:-"(none)"}

## Test Categories

### Integration Tests
$(for test in $UNIT_TESTS; do echo "- $test"; done)

### Performance Tests
$(for test in $PERFORMANCE_TESTS; do echo "- $test"; done)

## Test Results

EOF

    # Run tests and capture results
    print_status "$BLUE" "Running tests for report generation..."
    
    local temp_output=$(mktemp)
    
    # Integration tests
    echo "### Integration Test Results" >> "$report_file"
    echo "" >> "$report_file"
    
    for test in $UNIT_TESTS; do
        if run_test_command "$test" "" > "$temp_output" 2>&1; then
            echo "- ✅ $test: PASSED" >> "$report_file"
        else
            echo "- ❌ $test: FAILED" >> "$report_file"
        fi
    done
    
    echo "" >> "$report_file"
    
    # Performance tests (if enabled)
    if [[ "$RUN_PERFORMANCE" == "true" ]]; then
        echo "### Performance Test Results" >> "$report_file"
        echo "" >> "$report_file"
        
        for test in $PERFORMANCE_TESTS; do
            if run_test_command "$test" "" > "$temp_output" 2>&1; then
                echo "- ✅ $test: PASSED" >> "$report_file"
            else
                echo "- ❌ $test: FAILED" >> "$report_file"
            fi
        done
        
        echo "" >> "$report_file"
    fi
    
    # Add system information
    echo "## System Information" >> "$report_file"
    echo "" >> "$report_file"
    echo "- OS: $(uname -s)" >> "$report_file"
    echo "- Architecture: $(uname -m)" >> "$report_file"
    echo "- Rust Version: $(rustc --version 2>/dev/null || echo 'Not available')" >> "$report_file"
    echo "- Cargo Version: $(cargo --version 2>/dev/null || echo 'Not available')" >> "$report_file"
    
    rm -f "$temp_output"
    
    print_success "Test report generated: $report_file"
}

# Function to check test environment
check_environment() {
    print_header "Checking Test Environment"
    
    # Check if we're in the right directory
    if [[ ! -f "$PROJECT_ROOT/Cargo.toml" ]]; then
        print_error "Not in CURSED project root directory"
        exit 1
    fi
    
    # Check if Rust is available
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found. Please install Rust."
        exit 1
    fi
    
    # Check linking setup
    check_linking
    
    print_success "Test environment ready"
}

# Main execution function
main() {
    local test_category=""
    local exit_code=0
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -p|--performance)
                RUN_PERFORMANCE=true
                shift
                ;;
            -i|--ignored)
                RUN_IGNORED=true
                shift
                ;;
            -r|--report)
                GENERATE_REPORT=true
                REPORT_FILE="$2"
                shift 2
                ;;
            -f|--filter)
                TEST_FILTER="$2"
                shift 2
                ;;
            --quick)
                test_category="quick"
                shift
                ;;
            --all)
                test_category="all"
                RUN_PERFORMANCE=true
                shift
                ;;
            --integration)
                test_category="integration"
                shift
                ;;
            --unit)
                test_category="integration"  # Unit tests are part of integration
                shift
                ;;
            --performance)
                test_category="performance"
                RUN_PERFORMANCE=true
                shift
                ;;
            integration|performance|all|quick)
                test_category="$1"
                if [[ "$1" == "performance" || "$1" == "all" ]]; then
                    RUN_PERFORMANCE=true
                fi
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
    
    # Default to integration tests if no category specified
    if [[ -z "$test_category" ]]; then
        test_category="integration"
    fi
    
    # Check environment
    check_environment
    
    # Generate report if requested
    if [[ "$GENERATE_REPORT" == "true" ]]; then
        if [[ -z "$REPORT_FILE" ]]; then
            REPORT_FILE="pgo_test_report_$(date +%Y%m%d_%H%M%S).md"
        fi
        generate_report "$REPORT_FILE"
        return 0
    fi
    
    print_header "CURSED PGO Test Suite"
    print_status "$BLUE" "Test Category: $test_category"
    print_status "$BLUE" "Verbose: $VERBOSE"
    print_status "$BLUE" "Performance Tests: $RUN_PERFORMANCE"
    print_status "$BLUE" "Test Filter: ${TEST_FILTER:-"(none)"}"
    echo
    
    # Run tests based on category
    case $test_category in
        "quick")
            run_quick_tests
            exit_code=$?
            ;;
        "integration")
            run_integration_tests
            exit_code=$?
            ;;
        "performance")
            run_performance_tests
            exit_code=$?
            ;;
        "all")
            run_integration_tests
            local integration_exit=$?
            
            run_performance_tests
            local performance_exit=$?
            
            exit_code=$((integration_exit + performance_exit))
            ;;
        *)
            print_error "Unknown test category: $test_category"
            exit 1
            ;;
    esac
    
    echo
    if [[ $exit_code -eq 0 ]]; then
        print_success "All PGO tests completed successfully!"
    else
        print_error "Some PGO tests failed (exit code: $exit_code)"
    fi
    
    return $exit_code
}

# Make script executable and run main function
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
    exit $?
fi
