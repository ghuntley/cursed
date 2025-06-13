#!/bin/bash

# fr fr Comprehensive crypto test runner for CURSED - maximum testing periodt
# 
# This script runs all crypto tests with proper environment setup and reporting.
# Handles linking issues, generates coverage reports, and provides detailed output.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
REPORT_FILE="${PROJECT_ROOT}/crypto_test_report.md"

# Test categories
QUICK_TESTS=(
    "crypto_integration_test"
)

STANDARD_TESTS=(
    "crypto_integration_test"
    "crypto_security_test"
    "crypto_interop_test"
)

STRESS_TESTS=(
    "crypto_stress_test"
)

ALL_TESTS=("${STANDARD_TESTS[@]}" "${STRESS_TESTS[@]}")

# Command line options
QUICK_MODE=false
STRESS_MODE=false
VERBOSE=false
COVERAGE=false
REPORT=false
HELP=false

print_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Options:
    --quick         Run only quick crypto tests
    --stress        Run stress tests (use --ignored flag)
    --all           Run all tests including stress tests
    --verbose       Enable verbose output
    --coverage      Generate code coverage report
    --report        Generate detailed markdown report
    --help          Show this help message

Examples:
    $0                          # Run standard crypto tests
    $0 --quick                  # Run quick validation tests
    $0 --stress --ignored       # Run stress tests
    $0 --all --verbose --report # Full test suite with reporting
    $0 --coverage               # Run with coverage analysis

Environment:
    This script automatically handles linking fixes for Nix environments.
    Set CARGO_TEST_THREADS to control parallel test execution.

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)
            QUICK_MODE=true
            shift
            ;;
        --stress)
            STRESS_MODE=true
            shift
            ;;
        --all)
            QUICK_MODE=false
            STRESS_MODE=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --coverage)
            COVERAGE=true
            shift
            ;;
        --report)
            REPORT=true
            shift
            ;;
        --ignored)
            # Pass through to cargo test
            CARGO_EXTRA_ARGS="$CARGO_EXTRA_ARGS --ignored"
            shift
            ;;
        --help)
            HELP=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            print_usage
            exit 1
            ;;
    esac
done

if [[ "$HELP" == "true" ]]; then
    print_usage
    exit 0
fi

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

print_header() {
    echo
    print_status "$BLUE" "================================================"
    print_status "$BLUE" "$1"
    print_status "$BLUE" "================================================"
    echo
}

print_success() {
    print_status "$GREEN" "✅ $1"
}

print_warning() {
    print_status "$YELLOW" "⚠️  $1"
}

print_error() {
    print_status "$RED" "❌ $1"
}

# Function to check if linking fix is needed
setup_linking_fix() {
    print_header "Setting up linking environment"
    
    # Check if we're in a Nix environment and need linking fixes
    if [[ -n "$NIX_STORE" ]] || [[ -f "$PROJECT_ROOT/fix_linking.sh" ]]; then
        print_status "$YELLOW" "Detected Nix environment - applying linking fixes"
        
        if [[ -f "$PROJECT_ROOT/fix_linking.sh" ]]; then
            export LIBRARY_PATH="/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib:/nix/store/dsqzw96w4sxsp4q9yvkfl2yh701mpwgi-sqlite-3.46.1/lib"
            export RUSTFLAGS="-C linker=gcc -C link-arg=-fuse-ld=bfd"
            print_success "Linking environment configured"
        else
            print_warning "Nix environment detected but fix_linking.sh not found"
        fi
    else
        print_success "Standard linking environment"
    fi
}

# Function to run a single test
run_test() {
    local test_name=$1
    local extra_args=$2
    
    print_status "$BLUE" "Running test: $test_name"
    
    local cmd="cargo test --test $test_name $extra_args $CARGO_EXTRA_ARGS"
    if [[ "$VERBOSE" == "true" ]]; then
        cmd="$cmd -- --nocapture"
    fi
    
    echo "Command: $cmd"
    
    if eval "$cmd"; then
        print_success "Test passed: $test_name"
        return 0
    else
        print_error "Test failed: $test_name"
        return 1
    fi
}

# Function to run coverage analysis
run_coverage() {
    print_header "Running code coverage analysis"
    
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_warning "cargo-tarpaulin not found - attempting to install"
        cargo install cargo-tarpaulin || {
            print_error "Failed to install cargo-tarpaulin"
            return 1
        }
    fi
    
    local coverage_cmd="cargo tarpaulin --out Html --output-dir coverage/"
    if [[ "$STRESS_MODE" == "true" ]]; then
        coverage_cmd="$coverage_cmd --ignored"
    fi
    
    # Add test filters for crypto tests
    coverage_cmd="$coverage_cmd --bin cursed --tests crypto_integration_test,crypto_security_test,crypto_interop_test"
    
    if [[ "$STRESS_MODE" == "true" ]]; then
        coverage_cmd="$coverage_cmd,crypto_stress_test"
    fi
    
    echo "Coverage command: $coverage_cmd"
    
    if eval "$coverage_cmd"; then
        print_success "Coverage analysis completed"
        print_status "$GREEN" "Coverage report generated in: coverage/tarpaulin-report.html"
        return 0
    else
        print_error "Coverage analysis failed"
        return 1
    fi
}

# Function to generate detailed report
generate_report() {
    print_header "Generating detailed test report"
    
    cat > "$REPORT_FILE" << EOF
# CURSED Crypto Test Suite Report

Generated on: $(date)
Environment: $(uname -a)
Rust version: $(rustc --version)

## Test Configuration

EOF
    
    if [[ "$QUICK_MODE" == "true" ]]; then
        echo "- **Mode**: Quick tests only" >> "$REPORT_FILE"
    elif [[ "$STRESS_MODE" == "true" ]]; then
        echo "- **Mode**: All tests including stress tests" >> "$REPORT_FILE"
    else
        echo "- **Mode**: Standard test suite" >> "$REPORT_FILE"
    fi
    
    echo "- **Verbose**: $VERBOSE" >> "$REPORT_FILE"
    echo "- **Coverage**: $COVERAGE" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    
    # Test results will be appended during execution
    echo "## Test Results" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    
    print_success "Report initialized: $REPORT_FILE"
}

# Function to append test result to report
append_test_result() {
    local test_name=$1
    local result=$2
    local duration=$3
    
    if [[ "$REPORT" == "true" ]]; then
        local status_icon="✅"
        if [[ "$result" != "0" ]]; then
            status_icon="❌"
        fi
        
        echo "- $status_icon **$test_name**: $(printf "%.2f" "$duration")s" >> "$REPORT_FILE"
    fi
}

# Main execution
main() {
    cd "$PROJECT_ROOT"
    
    print_header "CURSED Crypto Test Suite Runner"
    print_status "$BLUE" "Project root: $PROJECT_ROOT"
    
    # Setup environment
    setup_linking_fix
    
    # Initialize report if requested
    if [[ "$REPORT" == "true" ]]; then
        generate_report
    fi
    
    # Determine which tests to run
    local tests_to_run=()
    if [[ "$QUICK_MODE" == "true" ]]; then
        tests_to_run=("${QUICK_TESTS[@]}")
        print_status "$YELLOW" "Running quick tests only"
    elif [[ "$STRESS_MODE" == "true" ]]; then
        tests_to_run=("${ALL_TESTS[@]}")
        print_status "$YELLOW" "Running all tests including stress tests"
    else
        tests_to_run=("${STANDARD_TESTS[@]}")
        print_status "$YELLOW" "Running standard test suite"
    fi
    
    # Run tests
    local total_tests=${#tests_to_run[@]}
    local passed_tests=0
    local failed_tests=0
    
    print_header "Executing crypto tests ($total_tests tests)"
    
    for test in "${tests_to_run[@]}"; do
        local start_time=$(date +%s.%N)
        
        if run_test "$test"; then
            ((passed_tests++))
            local result=0
        else
            ((failed_tests++))
            local result=1
        fi
        
        local end_time=$(date +%s.%N)
        local duration=$(echo "$end_time - $start_time" | bc -l)
        
        append_test_result "$test" "$result" "$duration"
        echo
    done
    
    # Run coverage if requested
    if [[ "$COVERAGE" == "true" ]]; then
        if run_coverage; then
            print_success "Coverage analysis completed"
        else
            print_warning "Coverage analysis failed"
        fi
    fi
    
    # Final report
    print_header "Test Suite Summary"
    print_status "$BLUE" "Total tests: $total_tests"
    print_status "$GREEN" "Passed: $passed_tests"
    
    if [[ $failed_tests -gt 0 ]]; then
        print_status "$RED" "Failed: $failed_tests"
    else
        print_status "$GREEN" "Failed: $failed_tests"
    fi
    
    local success_rate=$(( passed_tests * 100 / total_tests ))
    print_status "$BLUE" "Success rate: ${success_rate}%"
    
    # Finalize report
    if [[ "$REPORT" == "true" ]]; then
        cat >> "$REPORT_FILE" << EOF

## Summary

- **Total tests**: $total_tests
- **Passed**: $passed_tests
- **Failed**: $failed_tests
- **Success rate**: ${success_rate}%

## Environment Details

\`\`\`
$(env | grep -E "(RUST|CARGO|NIX)" | sort)
\`\`\`

## Notes

This test suite validates the CURSED crypto infrastructure including:
- Integration testing across all crypto packages
- Security property validation
- Interoperability with standard implementations
- Stress testing under extreme conditions

For detailed logs, run with --verbose flag.
For coverage analysis, run with --coverage flag.
EOF
        
        print_success "Detailed report generated: $REPORT_FILE"
    fi
    
    echo
    if [[ $failed_tests -eq 0 ]]; then
        print_success "All crypto tests passed! 🎉"
        exit 0
    else
        print_error "Some crypto tests failed! 😞"
        exit 1
    fi
}

# Run main function
main "$@"
