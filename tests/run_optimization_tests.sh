#!/bin/bash

# Comprehensive Optimization Testing Runner
# This script provides a unified interface for running all optimization tests
# with comprehensive reporting and CI/CD integration.

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TEST_RESULTS_DIR="$PROJECT_ROOT/test_results/optimization_tests"
REPORT_FILE="$TEST_RESULTS_DIR/optimization_test_report.md"
LINKING_FIX_SCRIPT="$PROJECT_ROOT/fix_linking.sh"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test categories
declare -A TEST_CATEGORIES
TEST_CATEGORIES[unit]="optimization_unit_test"
TEST_CATEGORIES[integration]="optimization_comprehensive_test"
TEST_CATEGORIES[stress]="optimization_stress_test"
TEST_CATEGORIES[performance]="optimization_performance_test"
TEST_CATEGORIES[regression]="optimization_regression_test"

# Command line options
VERBOSE=false
QUICK=false
IGNORED=false
COVERAGE=false
REPORT=false
CATEGORY=""
HELP=false

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --verbose|-v)
                VERBOSE=true
                shift
                ;;
            --quick|-q)
                QUICK=true
                shift
                ;;
            --ignored|-i)
                IGNORED=true
                shift
                ;;
            --coverage|-c)
                COVERAGE=true
                shift
                ;;
            --report|-r)
                REPORT=true
                shift
                ;;
            --test|-t)
                CATEGORY="$2"
                shift 2
                ;;
            --help|-h)
                HELP=true
                shift
                ;;
            *)
                echo "Unknown option: $1"
                exit 1
                ;;
        esac
    done
}

# Display help message
show_help() {
    cat << EOF
Optimization Test Runner

USAGE:
    $0 [OPTIONS]

OPTIONS:
    --verbose, -v      Enable verbose output
    --quick, -q        Run only quick tests (skip stress tests)
    --ignored, -i      Run ignored tests (stress and performance)
    --coverage, -c     Generate code coverage report
    --report, -r       Generate detailed test report
    --test, -t TEST    Run specific test category
    --help, -h         Show this help message

TEST CATEGORIES:
    unit               Unit tests for optimization components
    integration        Integration tests for complete pipeline
    stress             Stress tests under extreme conditions
    performance        Performance benchmarking tests
    regression         Regression detection tests
    all                All test categories

EXAMPLES:
    $0                          # Run standard optimization tests
    $0 --quick                  # Quick validation tests only
    $0 --ignored                # Run stress and performance tests
    $0 --test integration       # Run integration tests only
    $0 --coverage --report      # Generate coverage and detailed report
    $0 --verbose --ignored      # Verbose stress testing

EOF
}

# Initialize environment and directories
setup_environment() {
    print_header "Setting up test environment"
    
    # Create test results directory
    mkdir -p "$TEST_RESULTS_DIR"
    
    # Check for linking fix script
    if [[ -f "$LINKING_FIX_SCRIPT" ]]; then
        echo "✓ Found linking fix script"
    else
        echo "⚠ Warning: Linking fix script not found at $LINKING_FIX_SCRIPT"
    fi
    
    # Initialize test report
    if [[ "$REPORT" == true ]]; then
        init_test_report
    fi
    
    echo "✓ Test environment ready"
}

# Initialize test report
init_test_report() {
    cat > "$REPORT_FILE" << EOF
# Optimization System Test Report

**Generated:** $(date)
**Test Configuration:** $(describe_test_config)

## Test Summary

EOF
}

# Print formatted header
print_header() {
    echo -e "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

# Print step information
print_step() {
    echo -e "${GREEN}▶${NC} $1"
}

# Print warning
print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Print error
print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Print success
print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

# Describe test configuration
describe_test_config() {
    local config="Standard"
    [[ "$QUICK" == true ]] && config="Quick"
    [[ "$IGNORED" == true ]] && config="Stress"
    [[ -n "$CATEGORY" ]] && config="Category: $CATEGORY"
    echo "$config"
}

# Build cargo command with linking fix
build_cargo_command() {
    local cmd="$1"
    shift
    
    if [[ -f "$LINKING_FIX_SCRIPT" ]]; then
        echo "$LINKING_FIX_SCRIPT cargo $cmd $*"
    else
        echo "cargo $cmd $*"
    fi
}

# Run a single test category
run_test_category() {
    local category="$1"
    local test_name="${TEST_CATEGORIES[$category]:-$category}"
    
    print_step "Running $category tests: $test_name"
    
    local cargo_args="test --test $test_name"
    
    # Add test-specific flags
    if [[ "$VERBOSE" == true ]]; then
        cargo_args="$cargo_args -- --nocapture"
    fi
    
    if [[ "$IGNORED" == true && ("$category" == "stress" || "$category" == "performance") ]]; then
        cargo_args="$cargo_args -- --ignored"
    fi
    
    # Build command with linking fix
    local cmd=$(build_cargo_command $cargo_args)
    
    # Run the test
    local start_time=$(date +%s)
    local exit_code=0
    
    if [[ "$VERBOSE" == true ]]; then
        echo "Running: $cmd"
    fi
    
    eval "$cmd" || exit_code=$?
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    # Report results
    if [[ $exit_code -eq 0 ]]; then
        print_success "$category tests passed (${duration}s)"
        record_test_result "$category" "PASSED" "$duration" ""
    else
        print_error "$category tests failed (${duration}s)"
        record_test_result "$category" "FAILED" "$duration" "Exit code: $exit_code"
        return $exit_code
    fi
    
    return 0
}

# Record test results for reporting
record_test_result() {
    local category="$1"
    local status="$2"
    local duration="$3"
    local notes="$4"
    
    if [[ "$REPORT" == true ]]; then
        cat >> "$REPORT_FILE" << EOF
### $category Tests

- **Status:** $status
- **Duration:** ${duration}s
- **Notes:** $notes

EOF
    fi
}

# Run unit tests
run_unit_tests() {
    print_header "Running Optimization Unit Tests"
    
    # Run standard unit tests from lib
    print_step "Running optimization module unit tests"
    local cmd=$(build_cargo_command "test" "--lib optimization::")
    
    if [[ "$VERBOSE" == true ]]; then
        echo "Running: $cmd"
    fi
    
    eval "$cmd" || return $?
    
    print_success "Unit tests completed"
    return 0
}

# Run integration tests  
run_integration_tests() {
    print_header "Running Optimization Integration Tests"
    
    run_test_category "integration" || return $?
    
    return 0
}

# Run stress tests
run_stress_tests() {
    print_header "Running Optimization Stress Tests"
    
    if [[ "$QUICK" == true ]]; then
        print_warning "Skipping stress tests in quick mode"
        return 0
    fi
    
    run_test_category "stress" || return $?
    
    return 0
}

# Run performance tests
run_performance_tests() {
    print_header "Running Optimization Performance Tests"
    
    if [[ "$QUICK" == true ]]; then
        print_warning "Skipping performance tests in quick mode"
        return 0
    fi
    
    # Create performance-specific test if it exists
    if [[ -f "$PROJECT_ROOT/tests/optimization_performance_test.rs" ]]; then
        run_test_category "performance" || return $?
    else
        print_warning "Performance test file not found, skipping"
    fi
    
    return 0
}

# Run regression tests
run_regression_tests() {
    print_header "Running Optimization Regression Tests"
    
    # Create regression-specific test if it exists
    if [[ -f "$PROJECT_ROOT/tests/optimization_regression_test.rs" ]]; then
        run_test_category "regression" || return $?
    else
        print_warning "Regression test file not found, skipping"
    fi
    
    return 0
}

# Generate code coverage report
generate_coverage() {
    print_header "Generating Code Coverage Report"
    
    # Check if cargo-tarpaulin is available
    if ! command -v cargo-tarpaulin >/dev/null 2>&1; then
        print_warning "cargo-tarpaulin not found, skipping coverage"
        return 0
    fi
    
    print_step "Running coverage analysis"
    
    local coverage_cmd=$(build_cargo_command "tarpaulin" "--out Html --output-dir $TEST_RESULTS_DIR/coverage --tests --timeout 300")
    
    if [[ "$VERBOSE" == true ]]; then
        echo "Running: $coverage_cmd"
    fi
    
    eval "$coverage_cmd" || {
        print_warning "Coverage generation failed"
        return 0
    }
    
    print_success "Coverage report generated: $TEST_RESULTS_DIR/coverage/tarpaulin-report.html"
    
    if [[ "$REPORT" == true ]]; then
        cat >> "$REPORT_FILE" << EOF
## Code Coverage

Coverage report available at: \`$TEST_RESULTS_DIR/coverage/tarpaulin-report.html\`

EOF
    fi
    
    return 0
}

# Generate final test report
finalize_report() {
    if [[ "$REPORT" != true ]]; then
        return 0
    fi
    
    print_header "Finalizing Test Report"
    
    cat >> "$REPORT_FILE" << EOF

## Test Configuration

- **Quick Mode:** $QUICK
- **Ignored Tests:** $IGNORED
- **Coverage:** $COVERAGE
- **Verbose:** $VERBOSE
- **Category Filter:** ${CATEGORY:-"None"}

## Environment

- **Project Root:** $PROJECT_ROOT
- **Test Results:** $TEST_RESULTS_DIR
- **Linking Fix:** $([ -f "$LINKING_FIX_SCRIPT" ] && echo "Available" || echo "Not Found")

---

**Report generated at:** $(date)

EOF
    
    print_success "Test report saved: $REPORT_FILE"
}

# Main test execution function
run_optimization_tests() {
    local exit_code=0
    
    # Determine which tests to run
    if [[ -n "$CATEGORY" ]]; then
        # Run specific category
        case "$CATEGORY" in
            unit)
                run_unit_tests || exit_code=$?
                ;;
            integration)
                run_integration_tests || exit_code=$?
                ;;
            stress)
                run_stress_tests || exit_code=$?
                ;;
            performance)
                run_performance_tests || exit_code=$?
                ;;
            regression)
                run_regression_tests || exit_code=$?
                ;;
            all)
                run_unit_tests || exit_code=$?
                run_integration_tests || exit_code=$?
                run_stress_tests || exit_code=$?
                run_performance_tests || exit_code=$?
                run_regression_tests || exit_code=$?
                ;;
            *)
                print_error "Unknown test category: $CATEGORY"
                exit_code=1
                ;;
        esac
    else
        # Run standard test suite
        run_unit_tests || exit_code=$?
        run_integration_tests || exit_code=$?
        
        if [[ "$IGNORED" == true ]]; then
            run_stress_tests || exit_code=$?
            run_performance_tests || exit_code=$?
        fi
        
        run_regression_tests || exit_code=$?
    fi
    
    return $exit_code
}

# Main function
main() {
    parse_args "$@"
    
    if [[ "$HELP" == true ]]; then
        show_help
        exit 0
    fi
    
    print_header "CURSED Optimization System Test Suite"
    echo "Test configuration: $(describe_test_config)"
    echo ""
    
    setup_environment
    
    local start_time=$(date +%s)
    local exit_code=0
    
    # Run the tests
    run_optimization_tests || exit_code=$?
    
    # Generate coverage if requested
    if [[ "$COVERAGE" == true ]]; then
        generate_coverage
    fi
    
    # Finalize report
    finalize_report
    
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    
    # Final summary
    print_header "Test Summary"
    
    if [[ $exit_code -eq 0 ]]; then
        print_success "All optimization tests passed! (${total_duration}s total)"
    else
        print_error "Some optimization tests failed (${total_duration}s total)"
    fi
    
    echo ""
    echo "Test results directory: $TEST_RESULTS_DIR"
    [[ "$REPORT" == true ]] && echo "Detailed report: $REPORT_FILE"
    [[ "$COVERAGE" == true ]] && echo "Coverage report: $TEST_RESULTS_DIR/coverage/"
    
    exit $exit_code
}

# Run main function
main "$@"
