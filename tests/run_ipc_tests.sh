#!/bin/bash

# Comprehensive IPC Test Runner for CURSED
# Runs all IPC-related tests with proper environment setup

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
TEST_RESULTS_DIR="$PROJECT_ROOT/test_results"
COVERAGE_DIR="$PROJECT_ROOT/coverage"

# Test categories
BASIC_TESTS=(
    "ipc_comprehensive_test"
    "process_integration_test"
    "process_basic_test"
)

STRESS_TESTS=(
    "ipc_stress_test"
    "process_stress_test"
)

PERFORMANCE_TESTS=(
    "ipc_performance_test"
    "process_performance_test"
)

# Command line argument parsing
QUICK_MODE=false
VERBOSE=false
TEST_CATEGORY=""
GENERATE_REPORT=false
COVERAGE_ANALYSIS=false
CLEAN_BEFORE=false

show_help() {
    echo "CURSED IPC Test Runner"
    echo
    echo "Usage: $0 [OPTIONS] [TEST_CATEGORY]"
    echo
    echo "Options:"
    echo "  -h, --help              Show this help message"
    echo "  -q, --quick             Run only basic tests (faster)"
    echo "  -v, --verbose           Enable verbose output"
    echo "  -r, --report            Generate detailed test report"
    echo "  -c, --coverage          Generate coverage analysis"
    echo "  --clean                 Clean test artifacts before running"
    echo
    echo "Test Categories:"
    echo "  basic                   Run basic IPC functionality tests"
    echo "  stress                  Run stress tests (high load scenarios)"
    echo "  performance             Run performance benchmarks"
    echo "  all                     Run all test categories (default)"
    echo
    echo "Examples:"
    echo "  $0                      # Run all tests"
    echo "  $0 --quick             # Run basic tests only"
    echo "  $0 basic --verbose     # Run basic tests with verbose output"
    echo "  $0 --coverage --report # Run with coverage analysis and report"
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -q|--quick)
                QUICK_MODE=true
                shift
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -r|--report)
                GENERATE_REPORT=true
                shift
                ;;
            -c|--coverage)
                COVERAGE_ANALYSIS=true
                shift
                ;;
            --clean)
                CLEAN_BEFORE=true
                shift
                ;;
            basic|stress|performance|all)
                TEST_CATEGORY="$1"
                shift
                ;;
            *)
                echo "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    # Default test category
    if [[ -z "$TEST_CATEGORY" ]]; then
        if [[ "$QUICK_MODE" == "true" ]]; then
            TEST_CATEGORY="basic"
        else
            TEST_CATEGORY="all"
        fi
    fi
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}CURSED IPC Test Runner${NC}"
    echo -e "${BLUE}================================${NC}"
    echo
    echo "Configuration:"
    echo "  Test category: $TEST_CATEGORY"
    echo "  Quick mode: $QUICK_MODE"
    echo "  Verbose: $VERBOSE"
    echo "  Generate report: $GENERATE_REPORT"
    echo "  Coverage analysis: $COVERAGE_ANALYSIS"
    echo
}

setup_environment() {
    echo -e "${YELLOW}Setting up test environment...${NC}"
    
    # Create test results directory
    mkdir -p "$TEST_RESULTS_DIR"
    
    # Create coverage directory if needed
    if [[ "$COVERAGE_ANALYSIS" == "true" ]]; then
        mkdir -p "$COVERAGE_DIR"
    fi
    
    # Clean artifacts if requested
    if [[ "$CLEAN_BEFORE" == "true" ]]; then
        echo "Cleaning test artifacts..."
        rm -rf "$TEST_RESULTS_DIR"/*
        rm -rf "$COVERAGE_DIR"/*
        rm -f /tmp/test_cursed_*
        rm -f /tmp/cursed_*
    fi
    
    # Check for required tools
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: cargo not found${NC}"
        exit 1
    fi
    
    # Set up linking fix for Nix environment if needed
    if [[ -f "$PROJECT_ROOT/fix_linking.sh" ]]; then
        echo "Using linking fix for Nix environment..."
        export LINK_FIX_SCRIPT="$PROJECT_ROOT/fix_linking.sh"
    fi
    
    echo -e "${GREEN}Environment setup complete${NC}"
}

run_linking_wrapper() {
    if [[ -n "$LINK_FIX_SCRIPT" && -f "$LINK_FIX_SCRIPT" ]]; then
        "$LINK_FIX_SCRIPT" "$@"
    else
        "$@"
    fi
}

run_test_category() {
    local category="$1"
    local tests_to_run=()
    
    case "$category" in
        basic)
            tests_to_run=("${BASIC_TESTS[@]}")
            ;;
        stress)
            tests_to_run=("${STRESS_TESTS[@]}")
            ;;
        performance)
            tests_to_run=("${PERFORMANCE_TESTS[@]}")
            ;;
        all)
            tests_to_run=("${BASIC_TESTS[@]}" "${STRESS_TESTS[@]}" "${PERFORMANCE_TESTS[@]}")
            ;;
        *)
            echo -e "${RED}Unknown test category: $category${NC}"
            return 1
            ;;
    esac
    
    echo -e "${BLUE}Running $category tests...${NC}"
    
    local passed=0
    local failed=0
    local total=${#tests_to_run[@]}
    
    for test_name in "${tests_to_run[@]}"; do
        echo -e "${YELLOW}Running test: $test_name${NC}"
        
        local test_start_time=$(date +%s)
        local test_output_file="$TEST_RESULTS_DIR/${test_name}_output.log"
        local test_error_file="$TEST_RESULTS_DIR/${test_name}_error.log"
        
        # Build test command
        local test_cmd=(cargo test --test "$test_name")
        
        if [[ "$VERBOSE" == "true" ]]; then
            test_cmd+=(-- --nocapture)
        fi
        
        # Run the test
        if [[ "$VERBOSE" == "true" ]]; then
            if run_linking_wrapper "${test_cmd[@]}" 2>&1 | tee "$test_output_file"; then
                echo -e "${GREEN}✓ $test_name passed${NC}"
                ((passed++))
            else
                echo -e "${RED}✗ $test_name failed${NC}"
                ((failed++))
            fi
        else
            if run_linking_wrapper "${test_cmd[@]}" >"$test_output_file" 2>"$test_error_file"; then
                echo -e "${GREEN}✓ $test_name passed${NC}"
                ((passed++))
            else
                echo -e "${RED}✗ $test_name failed${NC}"
                echo "  Error output saved to: $test_error_file"
                ((failed++))
            fi
        fi
        
        local test_end_time=$(date +%s)
        local test_duration=$((test_end_time - test_start_time))
        echo "  Duration: ${test_duration}s"
        echo
    done
    
    echo -e "${BLUE}Test category $category results:${NC}"
    echo "  Passed: $passed"
    echo "  Failed: $failed"
    echo "  Total: $total"
    
    if [[ $failed -gt 0 ]]; then
        return 1
    fi
    
    return 0
}

run_coverage_analysis() {
    if [[ "$COVERAGE_ANALYSIS" != "true" ]]; then
        return 0
    fi
    
    echo -e "${YELLOW}Running coverage analysis...${NC}"
    
    # Check if cargo-tarpaulin is available
    if ! command -v cargo-tarpaulin &> /dev/null; then
        echo -e "${YELLOW}Warning: cargo-tarpaulin not found, skipping coverage analysis${NC}"
        return 0
    fi
    
    local coverage_cmd=(
        cargo tarpaulin
        --out Html
        --output-dir "$COVERAGE_DIR"
        --include-tests
        --timeout 300
    )
    
    # Add specific test filters for IPC tests
    coverage_cmd+=(--test ipc_comprehensive_test)
    coverage_cmd+=(--test process_integration_test)
    coverage_cmd+=(--test process_basic_test)
    
    if run_linking_wrapper "${coverage_cmd[@]}"; then
        echo -e "${GREEN}Coverage analysis complete${NC}"
        echo "  Report saved to: $COVERAGE_DIR/tarpaulin-report.html"
    else
        echo -e "${YELLOW}Coverage analysis failed, continuing...${NC}"
    fi
}

generate_test_report() {
    if [[ "$GENERATE_REPORT" != "true" ]]; then
        return 0
    fi
    
    echo -e "${YELLOW}Generating test report...${NC}"
    
    local report_file="$TEST_RESULTS_DIR/ipc_test_report.md"
    
    cat > "$report_file" << EOF
# CURSED IPC Test Report

Generated on: $(date)
Test category: $TEST_CATEGORY
Quick mode: $QUICK_MODE

## Test Environment

- OS: $(uname -s)
- Architecture: $(uname -m)
- Cargo version: $(cargo --version)

## Test Results

EOF
    
    # Add test results for each category run
    for log_file in "$TEST_RESULTS_DIR"/*_output.log; do
        if [[ -f "$log_file" ]]; then
            local test_name=$(basename "$log_file" _output.log)
            echo "### $test_name" >> "$report_file"
            echo >> "$report_file"
            echo '```' >> "$report_file"
            tail -20 "$log_file" >> "$report_file"
            echo '```' >> "$report_file"
            echo >> "$report_file"
        fi
    done
    
    cat >> "$report_file" << EOF

## System Information

### IPC Resources
- Shared memory support: $([ -d /dev/shm ] && echo "Available" || echo "Not available")
- Named pipes support: Available
- Unix domain sockets: Available

### Process Information
- Current PID: $$
- Process count: $(ps aux | wc -l)
- Available memory: $(free -h 2>/dev/null | grep Mem | awk '{print $7}' || echo "Unknown")

## Test Coverage

EOF
    
    if [[ -f "$COVERAGE_DIR/tarpaulin-report.html" ]]; then
        echo "Coverage report: $COVERAGE_DIR/tarpaulin-report.html" >> "$report_file"
    else
        echo "Coverage report: Not generated" >> "$report_file"
    fi
    
    echo -e "${GREEN}Test report generated: $report_file${NC}"
}

cleanup() {
    echo -e "${YELLOW}Cleaning up test resources...${NC}"
    
    # Clean up any leftover IPC resources
    rm -f /tmp/test_cursed_*
    rm -f /tmp/cursed_*
    
    # Kill any leftover test processes
    pkill -f "cursed.*test" 2>/dev/null || true
    
    echo -e "${GREEN}Cleanup complete${NC}"
}

main() {
    parse_args "$@"
    
    # Set up trap for cleanup
    trap cleanup EXIT
    
    print_header
    setup_environment
    
    local overall_result=0
    local start_time=$(date +%s)
    
    # Run the requested test category
    if ! run_test_category "$TEST_CATEGORY"; then
        overall_result=1
    fi
    
    # Run coverage analysis
    run_coverage_analysis
    
    # Generate report
    generate_test_report
    
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    
    echo
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}Test Run Summary${NC}"
    echo -e "${BLUE}================================${NC}"
    echo "Total duration: ${total_duration}s"
    echo "Test category: $TEST_CATEGORY"
    
    if [[ $overall_result -eq 0 ]]; then
        echo -e "${GREEN}All tests passed! ✓${NC}"
    else
        echo -e "${RED}Some tests failed! ✗${NC}"
    fi
    
    echo
    echo "Results saved to: $TEST_RESULTS_DIR"
    
    if [[ "$GENERATE_REPORT" == "true" ]]; then
        echo "Report generated: $TEST_RESULTS_DIR/ipc_test_report.md"
    fi
    
    if [[ -f "$COVERAGE_DIR/tarpaulin-report.html" ]]; then
        echo "Coverage report: $COVERAGE_DIR/tarpaulin-report.html"
    fi
    
    exit $overall_result
}

# Run main function with all arguments
main "$@"
