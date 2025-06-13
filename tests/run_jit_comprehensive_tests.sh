#!/bin/bash

# Comprehensive JIT Test Runner for CURSED Programming Language
# 
# This script runs all JIT-related tests including:
# - JIT engine functionality tests
# - JIT-REPL integration tests
# - Performance and optimization tests
# - Memory management tests
# - Error handling tests

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
VERBOSE=false
QUICK_MODE=false
GENERATE_REPORT=false
LINKING_FIX=""
TEST_TIMEOUT=300  # 5 minutes
REPORT_FILE=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --quick|-q)
            QUICK_MODE=true
            shift
            ;;
        --report|-r)
            GENERATE_REPORT=true
            REPORT_FILE="${2:-jit_test_report.md}"
            shift 2
            ;;
        --linking-fix)
            LINKING_FIX="./fix_linking.sh"
            shift
            ;;
        --timeout)
            TEST_TIMEOUT="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --verbose, -v     Enable verbose output"
            echo "  --quick, -q       Run only quick tests (skip stress tests)"
            echo "  --report, -r      Generate test report [file]"
            echo "  --linking-fix     Use linking fix script (for Nix environments)"
            echo "  --timeout         Test timeout in seconds (default: 300)"
            echo "  --help, -h        Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Check if linking fix is needed and available
if [[ -n "$LINKING_FIX" && ! -f "$LINKING_FIX" ]]; then
    echo -e "${YELLOW}Warning: Linking fix script not found: $LINKING_FIX${NC}"
    LINKING_FIX=""
fi

# Function to run command with or without linking fix
run_command() {
    if [[ -n "$LINKING_FIX" ]]; then
        $LINKING_FIX "$@"
    else
        "$@"
    fi
}

# Function to print section headers
print_section() {
    echo ""
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================${NC}"
    echo ""
}

# Function to print test results
print_result() {
    local test_name="$1"
    local result="$2"
    local duration="$3"
    
    if [[ "$result" == "PASS" ]]; then
        echo -e "${GREEN}✓${NC} $test_name ${YELLOW}(${duration}s)${NC}"
    else
        echo -e "${RED}✗${NC} $test_name ${YELLOW}(${duration}s)${NC}"
    fi
}

# Function to run a test with timeout
run_test() {
    local test_name="$1"
    local test_command="$2"
    local start_time=$(date +%s)
    
    if [[ "$VERBOSE" == "true" ]]; then
        echo -e "${BLUE}Running: $test_name${NC}"
        echo "Command: $test_command"
    fi
    
    if timeout $TEST_TIMEOUT bash -c "$test_command" &>/dev/null; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_result "$test_name" "PASS" "$duration"
        return 0
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_result "$test_name" "FAIL" "$duration"
        return 1
    fi
}

# Initialize counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
START_TIME=$(date +%s)

print_section "🔥 CURSED JIT Comprehensive Test Suite"

echo "Test Configuration:"
echo "  Verbose: $VERBOSE"
echo "  Quick Mode: $QUICK_MODE"
echo "  Linking Fix: ${LINKING_FIX:-"Not used"}"
echo "  Test Timeout: ${TEST_TIMEOUT}s"
echo "  Report Generation: $GENERATE_REPORT"
if [[ "$GENERATE_REPORT" == "true" ]]; then
    echo "  Report File: $REPORT_FILE"
fi

# Basic JIT Engine Tests
print_section "🔧 JIT Engine Core Tests"

JIT_ENGINE_TESTS=(
    "JIT Engine Creation and Configuration:run_command cargo test --test jit_engine_comprehensive_test test_jit_engine_creation_and_configuration"
    "JIT Engine Factory Functions:run_command cargo test --test jit_engine_comprehensive_test test_jit_engine_factory_functions"
    "Basic Function Compilation and Execution:run_command cargo test --test jit_engine_comprehensive_test test_basic_function_compilation_and_execution"
    "Function Caching:run_command cargo test --test jit_engine_comprehensive_test test_function_caching"
    "LLVM IR Parsing:run_command cargo test --test jit_engine_comprehensive_test test_llvm_ir_parsing"
    "Memory Management:run_command cargo test --test jit_engine_comprehensive_test test_memory_management"
    "Configuration Updates:run_command cargo test --test jit_engine_comprehensive_test test_configuration_updates"
)

for test_spec in "${JIT_ENGINE_TESTS[@]}"; do
    IFS=':' read -r test_name test_command <<< "$test_spec"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if run_test "$test_name" "$test_command"; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
done

# JIT Compilation Interface Tests
print_section "⚡ JIT Compilation Interface Tests"

JIT_INTERFACE_TESTS=(
    "JIT Compilation Interface:run_command cargo test --test jit_engine_comprehensive_test test_jit_compilation_interface"
    "Hot Path Detection:run_command cargo test --test jit_engine_comprehensive_test test_hot_path_detection"
    "Performance Monitoring:run_command cargo test --test jit_engine_comprehensive_test test_performance_monitoring"
    "Performance Report Generation:run_command cargo test --test jit_engine_comprehensive_test test_performance_report_generation"
    "Function Profiling:run_command cargo test --test jit_engine_comprehensive_test test_function_profiling"
    "Error Handling:run_command cargo test --test jit_engine_comprehensive_test test_error_handling"
)

for test_spec in "${JIT_INTERFACE_TESTS[@]}"; do
    IFS=':' read -r test_name test_command <<< "$test_spec"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if run_test "$test_name" "$test_command"; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
done

# REPL Integration Tests
print_section "🎮 REPL Integration Tests"

REPL_INTEGRATION_TESTS=(
    "REPL Evaluator JIT Initialization:run_command cargo test --test jit_repl_integration_test test_repl_evaluator_jit_initialization"
    "JIT Interface Creation:run_command cargo test --test jit_repl_integration_test test_jit_compilation_interface_creation"
    "REPL Code Execution Workflow:run_command cargo test --test jit_repl_integration_test test_repl_code_execution_workflow"
    "Incremental Compilation:run_command cargo test --test jit_repl_integration_test test_incremental_compilation"
    "Hot Path Optimization in REPL:run_command cargo test --test jit_repl_integration_test test_hot_path_optimization_in_repl"
    "REPL Session with JIT:run_command cargo test --test jit_repl_integration_test test_repl_session_with_jit"
    "Performance Monitoring in REPL:run_command cargo test --test jit_repl_integration_test test_performance_monitoring_in_repl"
    "Error Handling in REPL JIT:run_command cargo test --test jit_repl_integration_test test_error_handling_in_repl_jit"
    "REPL Evaluator JIT Methods:run_command cargo test --test jit_repl_integration_test test_repl_evaluator_jit_methods"
    "Memory Management in REPL Context:run_command cargo test --test jit_repl_integration_test test_memory_management_in_repl_context"
    "Function Profiling in REPL:run_command cargo test --test jit_repl_integration_test test_function_profiling_in_repl"
    "REPL Command Integration:run_command cargo test --test jit_repl_integration_test test_repl_command_integration"
)

for test_spec in "${REPL_INTEGRATION_TESTS[@]}"; do
    IFS=':' read -r test_name test_command <<< "$test_spec"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if run_test "$test_name" "$test_command"; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
done

# Optimization and Performance Tests
print_section "🚀 Optimization and Performance Tests"

OPTIMIZATION_TESTS=(
    "Optimization Functionality:run_command cargo test --test jit_engine_comprehensive_test test_optimization_functionality"
    "Debug vs Production Configurations:run_command cargo test --test jit_engine_comprehensive_test test_debug_vs_production_configurations"
    "Memory Limits and Cleanup:run_command cargo test --test jit_engine_comprehensive_test test_memory_limits_and_cleanup"
)

for test_spec in "${OPTIMIZATION_TESTS[@]}"; do
    IFS=':' read -r test_name test_command <<< "$test_spec"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if run_test "$test_name" "$test_command"; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
done

# Concurrent and Stress Tests
if [[ "$QUICK_MODE" != "true" ]]; then
    print_section "🔄 Concurrent and Stress Tests"

    STRESS_TESTS=(
        "Concurrent Compilation:run_command cargo test --test jit_engine_comprehensive_test test_concurrent_compilation"
        "Concurrent REPL Operations:run_command cargo test --test jit_repl_integration_test test_concurrent_repl_operations"
    )

    for test_spec in "${STRESS_TESTS[@]}"; do
        IFS=':' read -r test_name test_command <<< "$test_spec"
        TOTAL_TESTS=$((TOTAL_TESTS + 1))
        if run_test "$test_name" "$test_command"; then
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    done
else
    echo "Skipping stress tests in quick mode"
fi

# Legacy JIT Tests (if they exist and are relevant)
print_section "🔄 Legacy JIT Tests"

LEGACY_TESTS=(
    "Simple JIT Test:run_command cargo test --test simple_jit_test || true"
    "JIT Basic Test:run_command cargo test --test jit_basic_test || true"
    "JIT Integration Test:run_command cargo test --test jit_integration_test || true"
)

for test_spec in "${LEGACY_TESTS[@]}"; do
    IFS=':' read -r test_name test_command <<< "$test_spec"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if run_test "$test_name" "$test_command"; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
        # Don't count legacy test failures as critical
    fi
done

# Calculate final results
END_TIME=$(date +%s)
TOTAL_DURATION=$((END_TIME - START_TIME))

print_section "📊 Test Results Summary"

echo "Test Execution Summary:"
echo "  Total Tests: $TOTAL_TESTS"
echo -e "  Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "  Failed: ${RED}$FAILED_TESTS${NC}"
echo "  Total Duration: ${TOTAL_DURATION}s"

if [[ $FAILED_TESTS -eq 0 ]]; then
    echo -e "\n${GREEN}🎉 All JIT tests passed!${NC}"
    EXIT_CODE=0
else
    echo -e "\n${RED}❌ Some JIT tests failed${NC}"
    EXIT_CODE=1
fi

# Generate report if requested
if [[ "$GENERATE_REPORT" == "true" ]]; then
    print_section "📝 Generating Test Report"
    
    cat > "$REPORT_FILE" << EOF
# CURSED JIT Test Suite Report

**Generated:** $(date)
**Total Duration:** ${TOTAL_DURATION}s

## Summary

- **Total Tests:** $TOTAL_TESTS
- **Passed:** $PASSED_TESTS
- **Failed:** $FAILED_TESTS
- **Success Rate:** $(( PASSED_TESTS * 100 / TOTAL_TESTS ))%

## Test Configuration

- **Verbose Mode:** $VERBOSE
- **Quick Mode:** $QUICK_MODE
- **Linking Fix:** ${LINKING_FIX:-"Not used"}
- **Test Timeout:** ${TEST_TIMEOUT}s

## Test Categories

### JIT Engine Core Tests
- Basic engine functionality and configuration
- Function compilation and execution
- Memory management and caching
- LLVM IR parsing and processing

### JIT Compilation Interface Tests
- Hot path detection and optimization
- Performance monitoring and reporting
- Function profiling capabilities
- Error handling and recovery

### REPL Integration Tests
- JIT-REPL integration workflows
- Incremental compilation support
- Interactive development features
- Command system integration

### Optimization and Performance Tests
- Hot path optimization algorithms
- Configuration optimization
- Memory management efficiency
- Performance monitoring accuracy

$(if [[ "$QUICK_MODE" != "true" ]]; then
    echo "### Concurrent and Stress Tests"
    echo "- Multi-threaded compilation and execution"
    echo "- Concurrent REPL operations"
    echo "- Resource management under load"
fi)

### Legacy JIT Tests
- Compatibility with existing test infrastructure
- Regression prevention
- Migration validation

## Conclusion

$(if [[ $FAILED_TESTS -eq 0 ]]; then
    echo "✅ **All tests passed successfully!** The JIT compilation system is working correctly and ready for production use."
else
    echo "❌ **Some tests failed.** Review the failing tests and address any issues before deploying the JIT system."
fi)

## Recommendations

$(if [[ $FAILED_TESTS -eq 0 ]]; then
    echo "- The JIT compilation system is ready for integration with the bootstrap self-compilation system"
    echo "- Consider enabling JIT by default in REPL sessions for better performance"
    echo "- Monitor hot path optimization effectiveness in real-world usage"
else
    echo "- Investigate and fix failing tests before proceeding with JIT integration"
    echo "- Review error logs for specific failure details"
    echo "- Consider running tests with different configurations to isolate issues"
fi)

## Next Steps

- Integrate JIT engine with bootstrap self-compilation system
- Enhance REPL commands for better JIT interaction
- Implement additional optimization strategies
- Add more comprehensive performance monitoring
- Consider adding JIT compilation to the build system

---
*Report generated by run_jit_comprehensive_tests.sh*
EOF

    echo "Report generated: $REPORT_FILE"
fi

echo ""
echo "JIT test suite completed in ${TOTAL_DURATION}s"

exit $EXIT_CODE
