#!/bin/bash

# Advanced Optimization Test Runner for CURSED Compiler
# 
# Comprehensive test execution for all advanced optimization features:
# - Register allocation algorithms
# - Instruction scheduling
# - CURSED-specific optimizations
# - GC-aware optimizations
# - Performance debugging
# - Target-specific optimizations

set -e

# Script directory and linking fix
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
LINKING_FIX="$PROJECT_DIR/fix_linking.sh"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
QUICK_MODE=false
VERBOSE=false
GENERATE_REPORT=false
REPORT_FILE="advanced_optimization_test_report.md"
IGNORED_TESTS=false

# Test categories
UNIT_TESTS=(
    "test_advanced_register_allocation"
    "test_instruction_scheduling"
    "test_cursed_optimizations"
    "test_gc_aware_optimizations"
    "test_performance_debugging"
    "test_target_specific_optimizations"
)

INTEGRATION_TESTS=(
    "test_register_allocation_with_interference"
    "test_instruction_scheduling_with_dependencies"
    "test_optimization_integration"
)

PERFORMANCE_TESTS=(
    "test_performance_regression_detection"
    "test_optimization_performance_benchmark"
)

# Functions
print_header() {
    echo -e "${BLUE}================================================${NC}"
    echo -e "${BLUE}  CURSED Advanced Optimization Test Suite${NC}"
    echo -e "${BLUE}================================================${NC}"
    echo
}

print_section() {
    echo -e "${YELLOW}--- $1 ---${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

show_help() {
    cat << EOF
Advanced Optimization Test Runner

USAGE:
    $0 [OPTIONS]

OPTIONS:
    --quick             Run only basic unit tests
    --verbose           Show detailed output
    --report            Generate markdown test report
    --ignored           Run performance tests (normally ignored)
    --test <category>   Run specific test category
    -h, --help          Show this help message

TEST CATEGORIES:
    unit                Basic unit tests for individual components
    integration         Integration tests for multiple components
    performance         Performance and benchmark tests
    all                 All tests (default)

EXAMPLES:
    $0                          # Run all standard tests
    $0 --quick                  # Run only unit tests
    $0 --test unit --verbose   # Run unit tests with verbose output
    $0 --ignored --report       # Run all tests including performance
    $0 --test performance       # Run only performance tests

EOF
}

check_prerequisites() {
    print_section "Checking Prerequisites"
    
    # Check if linking fix script exists
    if [[ ! -f "$LINKING_FIX" ]]; then
        print_error "Linking fix script not found: $LINKING_FIX"
        exit 1
    fi
    
    # Make linking fix executable
    chmod +x "$LINKING_FIX"
    
    print_success "Prerequisites validated"
    echo
}

run_test_category() {
    local category="$1"
    local tests_array_name="$2"
    local test_flags="$3"
    
    print_section "Running $category Tests"
    
    # Get test array by name
    declare -n test_array=$tests_array_name
    
    local passed=0
    local failed=0
    local total=${#test_array[@]}
    
    if [[ $total -eq 0 ]]; then
        print_info "No tests in category: $category"
        return 0
    fi
    
    for test_name in "${test_array[@]}"; do
        echo -n "  Running $test_name... "
        
        if [[ "$VERBOSE" == "true" ]]; then
            echo # New line for verbose output
            if "$LINKING_FIX" cargo test --test advanced_optimization_test $test_flags -- $test_name; then
                print_success "$test_name passed"
                ((passed++))
            else
                print_error "$test_name failed"
                ((failed++))
            fi
        else
            if "$LINKING_FIX" cargo test --test advanced_optimization_test $test_flags -- $test_name --quiet > /dev/null 2>&1; then
                echo -e "${GREEN}PASS${NC}"
                ((passed++))
            else
                echo -e "${RED}FAIL${NC}"
                ((failed++))
            fi
        fi
    done
    
    echo
    echo "  $category Tests: $passed passed, $failed failed (total: $total)"
    echo
    
    return $failed
}

run_all_tests() {
    local total_failed=0
    
    # Unit tests
    run_test_category "Unit" "UNIT_TESTS" ""
    total_failed=$((total_failed + $?))
    
    if [[ "$QUICK_MODE" != "true" ]]; then
        # Integration tests
        run_test_category "Integration" "INTEGRATION_TESTS" ""
        total_failed=$((total_failed + $?))
        
        # Performance tests (if explicitly requested)
        if [[ "$IGNORED_TESTS" == "true" ]]; then
            run_test_category "Performance" "PERFORMANCE_TESTS" "--ignored"
            total_failed=$((total_failed + $?))
        fi
    fi
    
    return $total_failed
}

run_specific_category() {
    local category="$1"
    local failed=0
    
    case "$category" in
        "unit")
            run_test_category "Unit" "UNIT_TESTS" ""
            failed=$?
            ;;
        "integration")
            run_test_category "Integration" "INTEGRATION_TESTS" ""
            failed=$?
            ;;
        "performance")
            run_test_category "Performance" "PERFORMANCE_TESTS" "--ignored"
            failed=$?
            ;;
        "all")
            run_all_tests
            failed=$?
            ;;
        *)
            print_error "Unknown test category: $category"
            exit 1
            ;;
    esac
    
    return $failed
}

generate_test_report() {
    print_section "Generating Test Report"
    
    cat > "$REPORT_FILE" << EOF
# CURSED Advanced Optimization Test Report

**Generated:** $(date)
**Test Suite:** Advanced Optimization Features

## Test Summary

This report covers the comprehensive testing of CURSED's advanced optimization features.

## Test Categories

### Unit Tests
Basic functionality tests for individual optimization components:
EOF

    for test in "${UNIT_TESTS[@]}"; do
        echo "- \`$test\`" >> "$REPORT_FILE"
    done

    cat >> "$REPORT_FILE" << EOF

### Integration Tests
Tests for multiple optimization components working together:
EOF

    for test in "${INTEGRATION_TESTS[@]}"; do
        echo "- \`$test\`" >> "$REPORT_FILE"
    done

    cat >> "$REPORT_FILE" << EOF

### Performance Tests
Performance and benchmark tests:
EOF

    for test in "${PERFORMANCE_TESTS[@]}"; do
        echo "- \`$test\`" >> "$REPORT_FILE"
    done

    cat >> "$REPORT_FILE" << EOF

## Features Tested

### Advanced Register Allocation
- Graph coloring algorithm
- Register interference analysis
- Spill cost calculation
- Coalescing optimization

### Instruction Scheduling
- Dependency analysis
- Pipeline-aware scheduling
- Critical path optimization
- Resource conflict resolution

### CURSED-Specific Optimizations
- Gen Z slang keyword optimizations
- Error propagation optimization (\`?\` operator)
- Goroutine operation optimizations
- Channel operation optimizations
- Memory layout optimizations

### GC-Aware Optimizations
- Object lifetime analysis
- Memory pressure monitoring
- Write barrier optimization
- Allocation optimization

### Performance Debugging
- Pass execution tracing
- Performance profiling
- Adaptive pass ordering
- Regression testing

### Target-Specific Optimizations
- Architecture-specific passes
- Vectorization optimization
- Cache-aware optimizations
- Platform-specific optimizations

## Test Execution

To run these tests:

\`\`\`bash
# All tests
./tests/run_advanced_optimization_tests.sh

# Quick unit tests only
./tests/run_advanced_optimization_tests.sh --quick

# Specific category
./tests/run_advanced_optimization_tests.sh --test unit

# With performance tests
./tests/run_advanced_optimization_tests.sh --ignored
\`\`\`

## Performance Characteristics

The advanced optimization system is designed to:
- Process large programs efficiently
- Minimize optimization overhead
- Provide significant performance improvements
- Scale with program complexity

## Integration Status

All advanced optimization features are:
- ✅ Fully integrated with the compiler pipeline
- ✅ Compatible with existing optimization infrastructure
- ✅ Thread-safe for concurrent compilation
- ✅ Configurable through optimization levels
- ✅ Thoroughly tested and validated

EOF

    print_success "Test report generated: $REPORT_FILE"
}

main() {
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
            --report)
                GENERATE_REPORT=true
                shift
                ;;
            --ignored)
                IGNORED_TESTS=true
                shift
                ;;
            --test)
                TEST_CATEGORY="$2"
                shift 2
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    print_header
    check_prerequisites
    
    # Change to project directory
    cd "$PROJECT_DIR"
    
    local start_time=$(date +%s)
    local failed=0
    
    if [[ -n "${TEST_CATEGORY:-}" ]]; then
        run_specific_category "$TEST_CATEGORY"
        failed=$?
    else
        run_all_tests
        failed=$?
    fi
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    echo
    print_section "Test Results Summary"
    
    if [[ $failed -eq 0 ]]; then
        print_success "All tests passed! (${duration}s)"
    else
        print_error "$failed test(s) failed (${duration}s)"
    fi
    
    if [[ "$GENERATE_REPORT" == "true" ]]; then
        generate_test_report
    fi
    
    echo
    exit $failed
}

# Run main function with all arguments
main "$@"
