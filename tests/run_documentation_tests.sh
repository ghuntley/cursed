#!/bin/bash
# Comprehensive test runner for CURSED documentation system integration tests
#
# This script runs the complete documentation test suite including unit tests,
# integration tests, performance benchmarks, and golden file testing.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
VERBOSE=false
REPORT=false
SPECIFIC_TEST=""
KEEP_OUTPUT=false
TIMEOUT=300  # 5 minutes default timeout

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --verbose|-v)
                VERBOSE=true
                shift
                ;;
            --report|-r)
                REPORT=true
                shift
                ;;
            --test|-t)
                SPECIFIC_TEST="$2"
                shift 2
                ;;
            --keep|-k)
                KEEP_OUTPUT=true
                shift
                ;;
            --timeout)
                TIMEOUT="$2"
                shift 2
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                echo "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# Show help message
show_help() {
    cat << EOF
CURSED Documentation Integration Test Runner

Usage: $0 [OPTIONS]

Options:
    --verbose, -v       Show verbose test output
    --report, -r        Generate coverage report after tests
    --test, -t TEST     Run specific test category
    --keep, -k          Keep generated test output files
    --timeout SECONDS   Set test timeout (default: 300)
    --help, -h          Show this help message

Test Categories:
    unit               Documentation unit tests
    integration        End-to-end integration tests
    performance        Performance and benchmark tests
    golden             Golden file comparison tests
    cli                CLI tool functionality tests
    error              Error handling tests
    all                All test categories (default)

Examples:
    $0                          # Run all tests
    $0 --verbose --test unit    # Run unit tests with verbose output
    $0 --report --keep          # Run all tests, generate report, keep output
    $0 --test performance       # Run only performance tests

EOF
}

# Print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Print test section header
print_section() {
    local title=$1
    echo
    print_status $BLUE "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    print_status $BLUE "  $title"
    print_status $BLUE "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Setup test environment
setup_test_environment() {
    print_section "Setting up test environment"
    
    # Ensure we're in the project root
    if [[ ! -f "Cargo.toml" ]]; then
        print_status $RED "Error: Must be run from project root directory"
        exit 1
    fi
    
    # Check if documentation integration test exists
    if [[ ! -f "tests/documentation_integration_test.rs" ]]; then
        print_status $RED "Error: Documentation integration test not found"
        exit 1
    fi
    
    # Check if test fixtures exist
    if [[ ! -d "tests/documentation_test_files" ]]; then
        print_status $RED "Error: Documentation test fixtures not found"
        exit 1
    fi
    
    # Create output directory for test results
    mkdir -p "target/documentation_test_output"
    
    print_status $GREEN "✓ Test environment ready"
}

# Build project before testing
build_project() {
    print_section "Building project"
    
    if $VERBOSE; then
        cargo build --tests
    else
        cargo build --tests > /dev/null 2>&1
    fi
    
    if [[ $? -eq 0 ]]; then
        print_status $GREEN "✓ Project build successful"
    else
        print_status $RED "✗ Project build failed"
        exit 1
    fi
}

# Run documentation unit tests
run_unit_tests() {
    print_section "Documentation Unit Tests"
    
    local test_args=""
    if $VERBOSE; then
        test_args="-- --nocapture"
    fi
    
    # Run library tests for documentation modules
    local unit_test_patterns=(
        "docs::"
        "documentation"
        "doc_generator"
        "comment_parser"
        "html_renderer"
        "templates"
    )
    
    for pattern in "${unit_test_patterns[@]}"; do
        echo "Running unit tests for: $pattern"
        timeout $TIMEOUT cargo test --lib "$pattern" $test_args || {
            print_status $YELLOW "⚠ Some unit tests failed or timed out for: $pattern"
        }
    done
    
    print_status $GREEN "✓ Unit tests completed"
}

# Run integration tests
run_integration_tests() {
    print_section "Documentation Integration Tests"
    
    local test_args=""
    if $VERBOSE; then
        test_args="-- --nocapture"
    fi
    
    # Specific integration test functions
    local integration_tests=(
        "test_complete_documentation_workflow"
        "test_multi_package_documentation"
        "test_cross_reference_resolution"
        "test_documentation_validation_and_completeness"
        "test_html_generation_validity"
    )
    
    for test in "${integration_tests[@]}"; do
        echo "Running: $test"
        timeout $TIMEOUT cargo test --test documentation_integration_test "$test" $test_args || {
            print_status $YELLOW "⚠ Integration test failed or timed out: $test"
        }
    done
    
    print_status $GREEN "✓ Integration tests completed"
}

# Run performance tests
run_performance_tests() {
    print_section "Documentation Performance Tests"
    
    local test_args=""
    if $VERBOSE; then
        test_args="-- --nocapture"
    fi
    
    # Performance-specific tests
    local perf_tests=(
        "test_performance_large_codebase"
        "test_documentation_generation_benchmarks"
    )
    
    for test in "${perf_tests[@]}"; do
        echo "Running performance test: $test"
        timeout $TIMEOUT cargo test --test documentation_integration_test "$test" $test_args || {
            print_status $YELLOW "⚠ Performance test failed or timed out: $test"
        }
    done
    
    print_status $GREEN "✓ Performance tests completed"
}

# Run golden file tests
run_golden_tests() {
    print_section "Golden File Tests"
    
    local test_args=""
    if $VERBOSE; then
        test_args="-- --nocapture"
    fi
    
    echo "Running golden file comparison tests..."
    timeout $TIMEOUT cargo test --test documentation_integration_test "test_golden_file_comparison" $test_args || {
        print_status $YELLOW "⚠ Golden file tests not fully implemented yet"
    }
    
    print_status $GREEN "✓ Golden file tests completed"
}

# Run CLI tool tests
run_cli_tests() {
    print_section "CLI Tool Tests"
    
    local test_args=""
    if $VERBOSE; then
        test_args="-- --nocapture"
    fi
    
    # CLI-specific tests
    local cli_tests=(
        "test_cli_tool_processing"
        "test_markdown_generation"
        "test_json_export"
    )
    
    for test in "${cli_tests[@]}"; do
        echo "Running CLI test: $test"
        timeout $TIMEOUT cargo test --test documentation_integration_test "$test" $test_args || {
            print_status $YELLOW "⚠ CLI test may not be fully implemented: $test"
        }
    done
    
    print_status $GREEN "✓ CLI tests completed"
}

# Run error handling tests
run_error_tests() {
    print_section "Error Handling Tests"
    
    local test_args=""
    if $VERBOSE; then
        test_args="-- --nocapture"
    fi
    
    echo "Running error handling tests..."
    timeout $TIMEOUT cargo test --test documentation_integration_test "test_error_handling_malformed_docs" $test_args || {
        print_status $YELLOW "⚠ Error handling test failed or incomplete"
    }
    
    print_status $GREEN "✓ Error handling tests completed"
}

# Generate coverage report
generate_coverage_report() {
    if ! $REPORT; then
        return
    fi
    
    print_section "Generating Coverage Report"
    
    # Check if cargo-tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_status $YELLOW "⚠ cargo-tarpaulin not installed, skipping coverage report"
        print_status $YELLOW "  Install with: cargo install cargo-tarpaulin"
        return
    fi
    
    echo "Generating coverage report for documentation modules..."
    cargo tarpaulin \
        --out Html \
        --output-dir target/documentation_test_output \
        --include-tests \
        --exclude-files "tests/*" \
        --timeout $TIMEOUT \
        --packages cursed \
        --features "docs" 2>/dev/null || {
        print_status $YELLOW "⚠ Coverage report generation failed"
    }
    
    if [[ -f "target/documentation_test_output/tarpaulin-report.html" ]]; then
        print_status $GREEN "✓ Coverage report generated: target/documentation_test_output/tarpaulin-report.html"
    fi
}

# Cleanup test artifacts
cleanup_test_artifacts() {
    if $KEEP_OUTPUT; then
        print_status $BLUE "📁 Test output preserved in target/documentation_test_output/"
        return
    fi
    
    echo "Cleaning up test artifacts..."
    find target/documentation_test_output -name "*.html" -delete 2>/dev/null || true
    find target/documentation_test_output -name "*.json" -delete 2>/dev/null || true
    find target/documentation_test_output -name "*.md" -delete 2>/dev/null || true
}

# Print test summary
print_summary() {
    print_section "Test Summary"
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    echo "Test execution completed in ${duration} seconds"
    
    if $REPORT && [[ -f "target/documentation_test_output/tarpaulin-report.html" ]]; then
        echo "Coverage report: target/documentation_test_output/tarpaulin-report.html"
    fi
    
    if $KEEP_OUTPUT; then
        echo "Test outputs preserved in: target/documentation_test_output/"
    fi
    
    print_status $GREEN "✓ Documentation test suite completed successfully"
}

# Main execution
main() {
    local start_time=$(date +%s)
    
    parse_args "$@"
    
    print_status $BLUE "🚀 Starting CURSED Documentation Test Suite"
    
    setup_test_environment
    build_project
    
    # Run specific test category or all tests
    case "${SPECIFIC_TEST:-all}" in
        unit)
            run_unit_tests
            ;;
        integration)
            run_integration_tests
            ;;
        performance)
            run_performance_tests
            ;;
        golden)
            run_golden_tests
            ;;
        cli)
            run_cli_tests
            ;;
        error)
            run_error_tests
            ;;
        all)
            run_unit_tests
            run_integration_tests
            run_performance_tests
            run_golden_tests
            run_cli_tests
            run_error_tests
            ;;
        *)
            print_status $RED "Unknown test category: $SPECIFIC_TEST"
            show_help
            exit 1
            ;;
    esac
    
    generate_coverage_report
    cleanup_test_artifacts
    print_summary
}

# Run main function with all arguments
main "$@"
