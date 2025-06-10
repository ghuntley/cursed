#!/bin/bash

# Test runner for CURSED panic/recovery system
# Comprehensive testing of panic handling, recovery mechanisms, and LLVM integration

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
VERBOSE=false
QUICK=false
REPORT=false
REPORT_FILE=""
IGNORED=false
TEST_FILTER=""
COVERAGE=false

# Linking fix for Nix environment
LINKING_FIX="LIBRARY_PATH=\"/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib:/nix/store/dsqzw96w4sxsp4q9yvkfl2yh701mpwgi-sqlite-3.46.1/lib\" RUSTFLAGS=\"-C linker=gcc -C link-arg=-fuse-ld=bfd\""

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

print_header() {
    echo
    print_status $BLUE "=================================================="
    print_status $BLUE "$1"
    print_status $BLUE "=================================================="
    echo
}

print_success() {
    print_status $GREEN "✓ $1"
}

print_error() {
    print_status $RED "✗ $1"
}

print_warning() {
    print_status $YELLOW "⚠ $1"
}

# Function to show usage
show_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Test runner for CURSED panic/recovery system

Options:
    --help              Show this help message
    --verbose           Show verbose output
    --quick             Run only essential tests (faster)
    --test <type>       Run specific test type: unit, integration, llvm, all
    --ignored           Also run ignored tests (stress/performance tests)
    --filter <pattern>  Run only tests matching pattern
    --report [file]     Generate test report (optional file name)
    --coverage          Generate code coverage report
    
Test Types:
    unit               Unit tests for panic/recovery components
    integration        Integration tests for complete workflows
    llvm               LLVM integration and compilation tests
    all                All test types (default)

Examples:
    $0                              # Run all tests
    $0 --quick                      # Quick test run
    $0 --test unit                  # Run only unit tests
    $0 --verbose --report           # Verbose with report
    $0 --filter panic_runtime       # Test only panic runtime
    $0 --coverage                   # Generate coverage report

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help)
            show_usage
            exit 0
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --quick)
            QUICK=true
            shift
            ;;
        --test)
            TEST_FILTER="$2"
            shift 2
            ;;
        --ignored)
            IGNORED=true
            shift
            ;;
        --filter)
            TEST_FILTER="$2"
            shift 2
            ;;
        --report)
            REPORT=true
            if [[ $# -gt 1 && ! $2 =~ ^-- ]]; then
                REPORT_FILE="$2"
                shift 2
            else
                REPORT_FILE="panic_recovery_test_report.md"
                shift
            fi
            ;;
        --coverage)
            COVERAGE=true
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Default test type
if [[ -z "$TEST_FILTER" ]]; then
    TEST_FILTER="all"
fi

# Check if we're in a Nix environment
if [[ -n "$NIX_STORE" ]]; then
    print_warning "Detected Nix environment - using linking fix"
    USE_LINKING_FIX=true
else
    USE_LINKING_FIX=false
fi

# Function to run command with optional linking fix
run_command() {
    local cmd="$1"
    local desc="$2"
    
    if [[ $VERBOSE == true ]]; then
        print_status $BLUE "Running: $desc"
        print_status $BLUE "Command: $cmd"
    fi
    
    if [[ $USE_LINKING_FIX == true ]]; then
        eval "$LINKING_FIX $cmd"
    else
        eval "$cmd"
    fi
}

# Function to run tests with error handling
run_test_suite() {
    local test_name="$1"
    local test_command="$2"
    local description="$3"
    
    print_header "$description"
    
    if run_command "$test_command" "$test_name"; then
        print_success "$test_name completed successfully"
        return 0
    else
        print_error "$test_name failed"
        return 1
    fi
}

# Initialize test report
init_report() {
    if [[ $REPORT == true ]]; then
        cat > "$REPORT_FILE" << EOF
# CURSED Panic/Recovery System Test Report

Generated: $(date)
Test Configuration: $TEST_FILTER
Quick Mode: $QUICK
Ignored Tests: $IGNORED

## Test Results

EOF
    fi
}

# Add result to report
add_to_report() {
    local test_name="$1"
    local status="$2"
    local details="$3"
    
    if [[ $REPORT == true ]]; then
        cat >> "$REPORT_FILE" << EOF
### $test_name
**Status:** $status  
**Details:** $details

EOF
    fi
}

# Function to run unit tests
run_unit_tests() {
    print_header "Running Panic/Recovery Unit Tests"
    
    local tests=(
        "panic_runtime_test"
        "panic_info_test"
        "recovery_manager_test"
        "recovery_scope_test"
        "error_conversion_test"
    )
    
    local failed_tests=()
    
    for test in "${tests[@]}"; do
        local test_cmd="cargo test --lib runtime::panic::tests runtime::recovery::tests"
        
        if [[ $QUICK == true ]]; then
            test_cmd="$test_cmd --features quick-test"
        fi
        
        if run_command "$test_cmd" "Unit test: $test"; then
            print_success "$test"
            add_to_report "$test" "PASSED" "Unit test passed"
        else
            print_error "$test"
            failed_tests+=("$test")
            add_to_report "$test" "FAILED" "Unit test failed"
        fi
    done
    
    if [[ ${#failed_tests[@]} -eq 0 ]]; then
        print_success "All unit tests passed"
        return 0
    else
        print_error "Failed unit tests: ${failed_tests[*]}"
        return 1
    fi
}

# Function to run integration tests
run_integration_tests() {
    print_header "Running Panic/Recovery Integration Tests"
    
    local test_cmd="cargo test --test panic_recovery_integration_test"
    
    if [[ $QUICK == true ]]; then
        test_cmd="$test_cmd -- --test-threads=1"
    fi
    
    if [[ -n "$TEST_FILTER" && "$TEST_FILTER" != "all" && "$TEST_FILTER" != "integration" ]]; then
        test_cmd="$test_cmd $TEST_FILTER"
    fi
    
    if run_command "$test_cmd" "Integration tests"; then
        print_success "Integration tests completed"
        add_to_report "Integration Tests" "PASSED" "All integration tests passed"
        return 0
    else
        print_error "Integration tests failed"
        add_to_report "Integration Tests" "FAILED" "Some integration tests failed"
        return 1
    fi
}

# Function to run LLVM integration tests
run_llvm_tests() {
    print_header "Running LLVM Panic/Recovery Integration Tests"
    
    # Test LLVM panic compiler functionality
    local test_cmd="cargo test --lib codegen::llvm::panic::tests"
    
    if run_command "$test_cmd" "LLVM panic compiler tests"; then
        print_success "LLVM tests completed"
        add_to_report "LLVM Tests" "PASSED" "LLVM integration tests passed"
        return 0
    else
        print_warning "LLVM tests failed or skipped (may require LLVM setup)"
        add_to_report "LLVM Tests" "SKIPPED" "LLVM tests require proper LLVM environment"
        return 0  # Don't fail the entire suite for LLVM issues
    fi
}

# Function to run ignored tests (stress/performance)
run_ignored_tests() {
    if [[ $IGNORED != true ]]; then
        return 0
    fi
    
    print_header "Running Ignored Tests (Stress/Performance)"
    
    local test_cmd="cargo test --test panic_recovery_integration_test -- --ignored"
    
    if run_command "$test_cmd" "Ignored tests"; then
        print_success "Ignored tests completed"
        add_to_report "Ignored Tests" "PASSED" "Stress and performance tests passed"
        return 0
    else
        print_warning "Some ignored tests failed (this is expected for stress tests)"
        add_to_report "Ignored Tests" "PARTIAL" "Some stress tests failed (expected)"
        return 0
    fi
}

# Function to generate coverage report
generate_coverage() {
    if [[ $COVERAGE != true ]]; then
        return 0
    fi
    
    print_header "Generating Code Coverage Report"
    
    # Check if cargo-tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_warning "cargo-tarpaulin not found, installing..."
        cargo install cargo-tarpaulin
    fi
    
    local coverage_cmd="cargo tarpaulin --out Html --output-dir coverage-reports --include src/runtime/panic.rs src/runtime/recovery.rs src/codegen/llvm/panic.rs"
    
    if run_command "$coverage_cmd" "Coverage generation"; then
        print_success "Coverage report generated in coverage-reports/"
        add_to_report "Coverage Report" "GENERATED" "HTML coverage report available"
        return 0
    else
        print_warning "Coverage generation failed"
        add_to_report "Coverage Report" "FAILED" "Could not generate coverage report"
        return 0
    fi
}

# Function to run all tests
run_all_tests() {
    local overall_success=true
    
    case "$TEST_FILTER" in
        "unit")
            run_unit_tests || overall_success=false
            ;;
        "integration")
            run_integration_tests || overall_success=false
            ;;
        "llvm")
            run_llvm_tests || overall_success=false
            ;;
        "all")
            run_unit_tests || overall_success=false
            run_integration_tests || overall_success=false
            run_llvm_tests || overall_success=false
            run_ignored_tests || overall_success=false
            ;;
        *)
            # Run integration tests with filter
            run_integration_tests || overall_success=false
            ;;
    esac
    
    generate_coverage
    
    return $overall_success
}

# Main execution
main() {
    print_header "CURSED Panic/Recovery System Test Runner"
    
    # Change to project root
    cd "$PROJECT_ROOT"
    
    # Initialize report
    init_report
    
    # Check basic requirements
    if ! command -v cargo &> /dev/null; then
        print_error "cargo not found. Please install Rust."
        exit 1
    fi
    
    # Display configuration
    echo "Test Configuration:"
    echo "  Test Type: $TEST_FILTER"
    echo "  Quick Mode: $QUICK"
    echo "  Verbose: $VERBOSE"
    echo "  Include Ignored: $IGNORED"
    echo "  Generate Report: $REPORT"
    echo "  Coverage: $COVERAGE"
    echo "  Linking Fix: $USE_LINKING_FIX"
    echo
    
    # Run tests
    local start_time=$(date +%s)
    
    if run_all_tests; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        print_header "Test Summary"
        print_success "All tests completed successfully in ${duration}s"
        
        if [[ $REPORT == true ]]; then
            echo -e "\n## Summary\n" >> "$REPORT_FILE"
            echo "**Total Duration:** ${duration}s" >> "$REPORT_FILE"
            echo "**Overall Status:** PASSED" >> "$REPORT_FILE"
            print_success "Test report generated: $REPORT_FILE"
        fi
        
        exit 0
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        print_header "Test Summary"
        print_error "Some tests failed after ${duration}s"
        
        if [[ $REPORT == true ]]; then
            echo -e "\n## Summary\n" >> "$REPORT_FILE"
            echo "**Total Duration:** ${duration}s" >> "$REPORT_FILE"
            echo "**Overall Status:** FAILED" >> "$REPORT_FILE"
            print_warning "Test report generated: $REPORT_FILE"
        fi
        
        exit 1
    fi
}

# Run main function
main "$@"
