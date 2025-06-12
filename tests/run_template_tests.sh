#!/bin/bash

# Template System Test Runner for CURSED Programming Language
# Comprehensive test execution and validation

set -euo pipefail

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

# Test configuration
VERBOSE=false
QUICK=false
SPECIFIC_TEST=""
GENERATE_REPORT=false
REPORT_FILE=""

# Usage information
usage() {
    cat << EOF
Template System Test Runner

USAGE:
    $0 [OPTIONS]

OPTIONS:
    -h, --help              Show this help message
    -v, --verbose           Enable verbose output
    -q, --quick             Run only quick validation tests
    -t, --test TEST         Run specific test suite
    -r, --report [FILE]     Generate test report (optional filename)
    
AVAILABLE TEST SUITES:
    comprehensive           Complete template system tests
    integration            Template integration tests
    unit                   Unit tests for template components
    web                    Web framework integration tests
    formats                Template format rendering tests
    performance            Performance and caching tests
    security               Security and escaping tests
    
EXAMPLES:
    $0                          # Run all template tests
    $0 --quick                  # Quick validation only
    $0 --test comprehensive     # Run comprehensive tests
    $0 --verbose --report       # Verbose with report generation
EOF
}

# Parse command line arguments
parse_args() {
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
            -t|--test)
                SPECIFIC_TEST="$2"
                shift 2
                ;;
            -r|--report)
                GENERATE_REPORT=true
                if [[ $# -gt 1 && ! "$2" =~ ^- ]]; then
                    REPORT_FILE="$2"
                    shift 2
                else
                    REPORT_FILE="template_test_report_$(date +%Y%m%d_%H%M%S).md"
                    shift
                fi
                ;;
            *)
                echo "Unknown option: $1"
                usage
                exit 1
                ;;
        esac
    done
}

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if linking fix is available and working
check_linking_fix() {
    if [[ -f "$LINKING_FIX" ]]; then
        log_info "Using linking fix for Nix environment compatibility"
        return 0
    else
        log_warning "Linking fix script not found, using direct cargo"
        return 1
    fi
}

# Execute cargo command with linking fix if available
run_cargo() {
    if check_linking_fix; then
        "$LINKING_FIX" "$@"
    else
        cargo "$@"
    fi
}

# Run specific test suite
run_test_suite() {
    local test_name="$1"
    local test_file="$2"
    local description="$3"
    
    if [[ $VERBOSE == true ]]; then
        log_info "Running $description..."
    fi
    
    local start_time=$(date +%s)
    local test_result=0
    
    if [[ $VERBOSE == true ]]; then
        run_cargo test --test "$test_file" -- --nocapture
        test_result=$?
    else
        run_cargo test --test "$test_file" > /dev/null 2>&1
        test_result=$?
    fi
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    if [[ $test_result -eq 0 ]]; then
        log_success "$description completed successfully (${duration}s)"
        return 0
    else
        log_error "$description failed"
        return 1
    fi
}

# Generate test report
generate_report() {
    local report_file="$1"
    local total_tests="$2"
    local passed_tests="$3"
    local failed_tests="$4"
    
    cat > "$report_file" << EOF
# Template System Test Report

**Generated:** $(date)
**Project:** CURSED Programming Language Template System

## Test Summary

- **Total Test Suites:** $total_tests
- **Passed:** $passed_tests
- **Failed:** $failed_tests
- **Success Rate:** $(( passed_tests * 100 / total_tests ))%

## Test Results

EOF

    log_info "Test report generated: $report_file"
}

# Main test execution
main() {
    parse_args "$@"
    
    log_info "Starting CURSED Template System Tests"
    log_info "Project Root: $PROJECT_ROOT"
    
    cd "$PROJECT_ROOT"
    
    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]]; then
        log_error "Not in a Rust project directory"
        exit 1
    fi
    
    # Initialize counters
    local total_tests=0
    local passed_tests=0
    local failed_tests=0
    
    # Define test suites
    declare -A test_suites=(
        ["comprehensive"]="template_comprehensive_test:Complete template system functionality"
        ["integration"]="template_integration_test:End-to-end template workflows"
    )
    
    # Determine which tests to run
    local tests_to_run=()
    
    if [[ -n "$SPECIFIC_TEST" ]]; then
        if [[ -v test_suites["$SPECIFIC_TEST"] ]]; then
            tests_to_run=("$SPECIFIC_TEST")
        else
            log_error "Unknown test suite: $SPECIFIC_TEST"
            log_info "Available test suites: ${!test_suites[*]}"
            exit 1
        fi
    elif [[ $QUICK == true ]]; then
        # Quick validation tests
        tests_to_run=("integration")
    else
        # All tests
        tests_to_run=("${!test_suites[@]}")
    fi
    
    # Run selected tests
    log_info "Running ${#tests_to_run[@]} test suite(s)..."
    echo ""
    
    for test_key in "${tests_to_run[@]}"; do
        IFS=':' read -r test_file description <<< "${test_suites[$test_key]}"
        
        total_tests=$((total_tests + 1))
        
        if run_test_suite "$test_key" "$test_file" "$description"; then
            passed_tests=$((passed_tests + 1))
        else
            failed_tests=$((failed_tests + 1))
        fi
        
        echo ""
    done
    
    # Generate report if requested
    if [[ $GENERATE_REPORT == true ]]; then
        generate_report "$REPORT_FILE" "$total_tests" "$passed_tests" "$failed_tests"
    fi
    
    # Summary
    echo "======================================"
    log_info "Template System Test Summary"
    echo "======================================"
    log_info "Total Test Suites: $total_tests"
    log_success "Passed: $passed_tests"
    
    if [[ $failed_tests -gt 0 ]]; then
        log_error "Failed: $failed_tests"
        echo ""
        log_error "Some template system tests failed!"
        exit 1
    else
        echo ""
        log_success "All template system tests passed!"
        exit 0
    fi
}

# Run main function with all arguments
main "$@"
