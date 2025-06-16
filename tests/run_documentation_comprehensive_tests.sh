#!/bin/bash

# Comprehensive Documentation Test Runner for CURSED Language
# 
# This script runs the complete documentation generation test suite
# including unit tests, integration tests, performance tests, and validation.
# 
# Usage: ./run_documentation_comprehensive_tests.sh [options]
# Options:
#   --quick           Run only basic tests
#   --coverage        Generate test coverage report
#   --verbose         Show detailed output
#   --report FILE     Generate detailed test report
#   --help           Show this help message

set -euo pipefail

# Color codes for output formatting
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[0;37m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TEST_OUTPUT_DIR="$PROJECT_ROOT/target/documentation_test_output"
COVERAGE_DIR="$PROJECT_ROOT/coverage"
LINKING_FIX="$PROJECT_ROOT/fix_linking.sh"

# Default options
QUICK_MODE=false
GENERATE_COVERAGE=false
VERBOSE=false
REPORT_FILE=""
TIMEOUT=300

# Print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

print_section() {
    local message=$1
    echo
    echo -e "${BLUE}============================================${NC}"
    echo -e "${BLUE} $message${NC}"
    echo -e "${BLUE}============================================${NC}"
    echo
}

print_subsection() {
    local message=$1
    echo -e "${CYAN}--- $message ---${NC}"
}

# Show help message
show_help() {
    cat << 'EOF'
CURSED Documentation Comprehensive Test Runner

This script validates the entire documentation generation system including:
- Comment parsing with JSDoc-style tags
- AST extraction and analysis
- HTML output generation with styling and search
- Markdown output with GitHub compatibility
- Configuration parsing and validation
- Cross-reference system functionality
- Search index generation
- End-to-end integration testing

Usage: ./run_documentation_comprehensive_tests.sh [options]

Options:
  --quick           Run only essential tests (faster execution)
  --coverage        Generate test coverage report with cargo-tarpaulin
  --verbose         Show detailed test output and debug information
  --report FILE     Generate detailed test report in markdown format
  --timeout SEC     Set timeout for individual tests (default: 300)
  --help           Show this help message

Test Categories:
  unit              Documentation parsing and generation unit tests
  integration       End-to-end documentation generation tests
  html              HTML output format and quality tests
  markdown          Markdown output format and validation tests
  config            Configuration parsing and validation tests
  performance       Performance and scalability tests

Examples:
  ./run_documentation_comprehensive_tests.sh
  ./run_documentation_comprehensive_tests.sh --quick --verbose
  ./run_documentation_comprehensive_tests.sh --coverage --report coverage_report.md
  ./run_documentation_comprehensive_tests.sh --timeout 600

Exit Codes:
  0    All tests passed successfully
  1    Some tests failed
  2    Test setup or configuration error
  3    Timeout or resource error
EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)
            QUICK_MODE=true
            shift
            ;;
        --coverage)
            GENERATE_COVERAGE=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --report)
            REPORT_FILE="$2"
            shift 2
            ;;
        --timeout)
            TIMEOUT="$2"
            shift 2
            ;;
        --help)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 2
            ;;
    esac
done

# Setup function
setup_test_environment() {
    print_section "Setting Up Test Environment"
    
    # Create output directories
    mkdir -p "$TEST_OUTPUT_DIR"
    mkdir -p "$COVERAGE_DIR"
    
    # Check if we're in a Nix environment and linking fix is needed
    if [[ -f "$LINKING_FIX" ]]; then
        print_status $YELLOW "Using linking fix for Nix environment"
        export CARGO_WRAPPER="$LINKING_FIX"
    else
        export CARGO_WRAPPER=""
    fi
    
    # Set up tracing for tests
    export RUST_LOG="debug"
    export RUST_BACKTRACE="1"
    
    # Verify test fixtures exist
    if [[ ! -d "$SCRIPT_DIR/documentation_test_files" ]]; then
        print_status $RED "Error: Documentation test fixtures not found"
        exit 2
    fi
    
    print_status $GREEN "Test environment setup complete"
}

# Run a specific test with timeout and error handling
run_test() {
    local test_name=$1
    local test_args=${2:-""}
    local description=${3:-"$test_name"}
    
    print_subsection "Running $description"
    
    local start_time=$(date +%s)
    local exit_code=0
    
    if [[ $VERBOSE == true ]]; then
        if [[ -n "$CARGO_WRAPPER" ]]; then
            timeout $TIMEOUT $CARGO_WRAPPER cargo test --test "$test_name" $test_args || exit_code=$?
        else
            timeout $TIMEOUT cargo test --test "$test_name" $test_args || exit_code=$?
        fi
    else
        if [[ -n "$CARGO_WRAPPER" ]]; then
            timeout $TIMEOUT $CARGO_WRAPPER cargo test --test "$test_name" $test_args >/dev/null 2>&1 || exit_code=$?
        else
            timeout $TIMEOUT cargo test --test "$test_name" $test_args >/dev/null 2>&1 || exit_code=$?
        fi
    fi
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    if [[ $exit_code -eq 0 ]]; then
        print_status $GREEN "✓ $description (${duration}s)"
        return 0
    elif [[ $exit_code -eq 124 ]]; then
        print_status $RED "✗ $description - TIMEOUT after ${TIMEOUT}s"
        return 1
    else
        print_status $RED "✗ $description - FAILED (exit code: $exit_code)"
        return 1
    fi
}

# Documentation parser tests
run_parser_tests() {
    print_section "Documentation Parser Tests"
    
    local failed=0
    
    run_test "documentation_parser_basic_test" "" "Comment Parser Basic Functionality" || ((failed++))
    run_test "documentation_parser_test" "" "Advanced Comment Parser Features" || ((failed++))
    
    if [[ $failed -eq 0 ]]; then
        print_status $GREEN "All parser tests passed"
    else
        print_status $RED "$failed parser tests failed"
    fi
    
    return $failed
}

# Documentation extraction tests
run_extraction_tests() {
    print_section "Documentation Extraction Tests"
    
    local failed=0
    
    run_test "documentation_extraction_test" "" "AST Documentation Extraction" || ((failed++))
    run_test "documentation_ast_test" "" "AST Analysis and Processing" || ((failed++))
    run_test "documentation_ast_simple_test" "" "Simple AST Extraction" || ((failed++))
    
    if [[ $failed -eq 0 ]]; then
        print_status $GREEN "All extraction tests passed"
    else
        print_status $RED "$failed extraction tests failed"
    fi
    
    return $failed
}

# Output format tests
run_output_format_tests() {
    print_section "Output Format Generation Tests"
    
    local failed=0
    
    run_test "documentation_html_test" "" "HTML Output Generation" || ((failed++))
    run_test "documentation_html_basic_test" "" "Basic HTML Generation" || ((failed++))
    run_test "documentation_markdown_test" "" "Markdown Output Generation" || ((failed++))
    
    if [[ $failed -eq 0 ]]; then
        print_status $GREEN "All output format tests passed"
    else
        print_status $RED "$failed output format tests failed"
    fi
    
    return $failed
}

# Configuration and validation tests
run_config_tests() {
    print_section "Configuration and Validation Tests"
    
    local failed=0
    
    run_test "documentation_cli_test" "" "CLI Configuration and Options" || ((failed++))
    run_test "documentation_test" "" "General Documentation System" || ((failed++))
    
    if [[ $failed -eq 0 ]]; then
        print_status $GREEN "All configuration tests passed"
    else
        print_status $RED "$failed configuration tests failed"
    fi
    
    return $failed
}

# Integration tests
run_integration_tests() {
    print_section "Integration Tests"
    
    local failed=0
    
    run_test "documentation_integration_test" "" "End-to-End Documentation Generation" || ((failed++))
    run_test "documentation_integration_simple_test" "" "Simple Integration Tests" || ((failed++))
    run_test "documentation_generation_test" "" "Documentation Generation Workflows" || ((failed++))
    
    if [[ $failed -eq 0 ]]; then
        print_status $GREEN "All integration tests passed"
    else
        print_status $RED "$failed integration tests failed"
    fi
    
    return $failed
}

# Performance and golden tests
run_performance_tests() {
    if [[ $QUICK_MODE == true ]]; then
        print_status $YELLOW "Skipping performance tests in quick mode"
        return 0
    fi
    
    print_section "Performance and Golden Tests"
    
    local failed=0
    
    run_test "documentation_performance_test" "" "Performance and Scalability" || ((failed++))
    run_test "documentation_golden_test" "" "Golden File Regression Tests" || ((failed++))
    
    if [[ $failed -eq 0 ]]; then
        print_status $GREEN "All performance tests passed"
    else
        print_status $RED "$failed performance tests failed"
    fi
    
    return $failed
}

# Generate coverage report
generate_coverage_report() {
    if [[ $GENERATE_COVERAGE != true ]]; then
        return 0
    fi
    
    print_section "Generating Test Coverage Report"
    
    # Check if cargo-tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_status $YELLOW "cargo-tarpaulin not found, installing..."
        if [[ -n "$CARGO_WRAPPER" ]]; then
            $CARGO_WRAPPER cargo install cargo-tarpaulin
        else
            cargo install cargo-tarpaulin
        fi
    fi
    
    local coverage_args="--out html --output-dir $COVERAGE_DIR"
    
    # Add documentation test patterns
    coverage_args="$coverage_args --tests documentation_*"
    
    if [[ $VERBOSE == true ]]; then
        coverage_args="$coverage_args --verbose"
    fi
    
    print_status $CYAN "Running coverage analysis..."
    
    if [[ -n "$CARGO_WRAPPER" ]]; then
        $CARGO_WRAPPER cargo tarpaulin $coverage_args
    else
        cargo tarpaulin $coverage_args
    fi
    
    if [[ $? -eq 0 ]]; then
        print_status $GREEN "Coverage report generated in $COVERAGE_DIR"
        if [[ -f "$COVERAGE_DIR/tarpaulin-report.html" ]]; then
            print_status $CYAN "Open $COVERAGE_DIR/tarpaulin-report.html in your browser to view coverage"
        fi
    else
        print_status $RED "Coverage generation failed"
        return 1
    fi
}

# Generate detailed test report
generate_test_report() {
    if [[ -z "$REPORT_FILE" ]]; then
        return 0
    fi
    
    print_section "Generating Test Report"
    
    cat > "$REPORT_FILE" << EOF
# CURSED Documentation Test Report

Generated on: $(date)
Test environment: $(uname -a)
Rust version: $(rustc --version)

## Test Summary

This report covers comprehensive testing of the CURSED documentation generation system.

### Test Categories Covered

1. **Comment Parser Tests**: JSDoc-style tag parsing, malformed input handling
2. **AST Extraction Tests**: Documentation extraction from CURSED AST structures
3. **HTML Output Tests**: HTML generation, styling, cross-references, search functionality
4. **Markdown Output Tests**: Markdown generation, GitHub compatibility, table formatting
5. **Configuration Tests**: TOML/JSON parsing, validation, environment variable handling
6. **Integration Tests**: End-to-end workflows, file processing, error recovery
7. **Performance Tests**: Large project handling, memory usage, processing speed

### Why Comprehensive Testing is Critical

Documentation generation testing is essential because:

- **Accuracy**: Ensures generated docs accurately reflect source code structure and comments
- **Regression Prevention**: Catches breaking changes in parsing or generation logic
- **Format Compliance**: Validates HTML, Markdown, and other output formats are well-formed
- **Cross-Reference Integrity**: Verifies that links between documentation elements work correctly
- **Search Functionality**: Ensures search indexes are complete and functional
- **Configuration Validation**: Prevents invalid configurations from causing runtime failures
- **Performance**: Detects performance regressions in large codebase processing
- **Error Handling**: Validates graceful handling of malformed input and edge cases

### Test Execution Details
EOF
    
    print_status $GREEN "Test report started in $REPORT_FILE"
}

# Main test execution
main() {
    local start_time=$(date +%s)
    local total_failed=0
    
    print_section "CURSED Documentation Comprehensive Test Suite"
    print_status $CYAN "Testing documentation generation system functionality"
    
    if [[ $QUICK_MODE == true ]]; then
        print_status $YELLOW "Running in quick mode - skipping performance tests"
    fi
    
    if [[ $VERBOSE == true ]]; then
        print_status $YELLOW "Verbose mode enabled - showing detailed output"
    fi
    
    # Setup
    setup_test_environment || exit 2
    
    # Generate report header
    generate_test_report
    
    # Run test categories
    run_parser_tests || ((total_failed++))
    run_extraction_tests || ((total_failed++))
    run_output_format_tests || ((total_failed++))
    run_config_tests || ((total_failed++))
    run_integration_tests || ((total_failed++))
    run_performance_tests || ((total_failed++))
    
    # Generate coverage if requested
    generate_coverage_report || ((total_failed++))
    
    # Final summary
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    
    print_section "Test Results Summary"
    
    if [[ $total_failed -eq 0 ]]; then
        print_status $GREEN "🎉 All documentation tests passed! (${total_duration}s total)"
        print_status $GREEN "Documentation generation system is working correctly"
        
        if [[ -n "$REPORT_FILE" ]]; then
            echo -e "\n## Final Result\n✅ **ALL TESTS PASSED** (${total_duration}s total)" >> "$REPORT_FILE"
        fi
        
        exit 0
    else
        print_status $RED "❌ $total_failed test categories failed (${total_duration}s total)"
        print_status $RED "Documentation system has issues that need to be addressed"
        
        if [[ -n "$REPORT_FILE" ]]; then
            echo -e "\n## Final Result\n❌ **$total_failed TEST CATEGORIES FAILED** (${total_duration}s total)" >> "$REPORT_FILE"
        fi
        
        exit 1
    fi
}

# Run main function
main "$@"
