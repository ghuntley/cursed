#!/bin/bash

# Comprehensive CURSED Compiler Test Suite
# Validates all compiler functionality and catches regressions

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((PASSED_TESTS++))
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1"
    ((FAILED_TESTS++))
}

log_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

run_test() {
    local test_name="$1"
    local test_command="$2"
    ((TOTAL_TESTS++))
    
    log_info "Running test: $test_name"
    
    if eval "$test_command" > /dev/null 2>&1; then
        log_success "$test_name"
        return 0
    else
        log_error "$test_name"
        return 1
    fi
}

# Build compiler before testing
log_info "Building CURSED compiler..."
if ! zig build-exe src-zig/main_unified.zig -lc --name cursed-unified; then
    log_error "Failed to build compiler"
    exit 1
fi

log_success "Compiler built successfully"

# 1. Unit Tests for Core Components
log_info "=== UNIT TESTS ==="

# Lexer tests
run_test "Lexer Basic Tokens" "./test_suite/unit_tests/test_lexer_basic.sh"
run_test "Lexer Keywords" "./test_suite/unit_tests/test_lexer_keywords.sh"
run_test "Lexer Operators" "./test_suite/unit_tests/test_lexer_operators.sh"
run_test "Lexer String Literals" "./test_suite/unit_tests/test_lexer_strings.sh"

# Parser tests
run_test "Parser Variable Declarations" "./test_suite/unit_tests/test_parser_variables.sh"
run_test "Parser Function Definitions" "./test_suite/unit_tests/test_parser_functions.sh"
run_test "Parser Struct Definitions" "./test_suite/unit_tests/test_parser_structs.sh"
run_test "Parser Interface Definitions" "./test_suite/unit_tests/test_parser_interfaces.sh"

# AST tests
run_test "AST Node Creation" "./test_suite/unit_tests/test_ast_nodes.sh"
run_test "AST Type Checking" "./test_suite/unit_tests/test_ast_types.sh"

# Codegen tests
run_test "Codegen Basic Operations" "./test_suite/unit_tests/test_codegen_basic.sh"
run_test "Codegen Function Calls" "./test_suite/unit_tests/test_codegen_functions.sh"

# 2. Integration Tests
log_info "=== INTEGRATION TESTS ==="

run_test "End-to-End Hello World" "./test_suite/integration_tests/test_e2e_hello_world.sh"
run_test "End-to-End Complex Program" "./test_suite/integration_tests/test_e2e_complex.sh"
run_test "Compilation Pipeline" "./test_suite/integration_tests/test_compilation_pipeline.sh"
run_test "Interpretation Pipeline" "./test_suite/integration_tests/test_interpretation_pipeline.sh"

# 3. Language Feature Tests
log_info "=== LANGUAGE FEATURE TESTS ==="

run_test "Variable Declarations" "./test_suite/feature_tests/test_variables.sh"
run_test "Function Definitions" "./test_suite/feature_tests/test_functions.sh"
run_test "Struct Definitions" "./test_suite/feature_tests/test_structs.sh"
run_test "Interface Implementations" "./test_suite/feature_tests/test_interfaces.sh"
run_test "Import System" "./test_suite/feature_tests/test_imports.sh"
run_test "Error Handling" "./test_suite/feature_tests/test_error_handling.sh"
run_test "Pattern Matching" "./test_suite/feature_tests/test_pattern_matching.sh"
run_test "Concurrency Features" "./test_suite/feature_tests/test_concurrency.sh"
run_test "Generic Types" "./test_suite/feature_tests/test_generics.sh"

# 4. Performance Tests
log_info "=== PERFORMANCE TESTS ==="

run_test "Memory Leak Detection" "./test_suite/performance_tests/test_memory_leaks.sh"
run_test "Garbage Collector Performance" "./test_suite/performance_tests/test_gc_performance.sh"
run_test "Compilation Speed" "./test_suite/performance_tests/test_compilation_speed.sh"
run_test "Runtime Performance" "./test_suite/performance_tests/test_runtime_performance.sh"

# 5. Cross-Platform Tests
log_info "=== CROSS-PLATFORM TESTS ==="

run_test "Linux x86_64 Build" "./test_suite/cross_platform_tests/test_linux_x64.sh"
run_test "Platform Abstraction Layer" "./test_suite/cross_platform_tests/test_pal.sh"

# 6. Regression Tests
log_info "=== REGRESSION TESTS ==="

run_test "Previous Bug Fixes" "./test_suite/regression_tests/test_bug_fixes.sh"
run_test "API Compatibility" "./test_suite/regression_tests/test_api_compatibility.sh"

# 7. Negative Tests
log_info "=== NEGATIVE TESTS ==="

run_test "Syntax Error Handling" "./test_suite/negative_tests/test_syntax_errors.sh"
run_test "Type Error Detection" "./test_suite/negative_tests/test_type_errors.sh"
run_test "Runtime Error Handling" "./test_suite/negative_tests/test_runtime_errors.sh"

# Summary
log_info "=== TEST SUMMARY ==="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"

if [ $FAILED_TESTS -eq 0 ]; then
    log_success "All tests passed!"
    exit 0
else
    log_error "$FAILED_TESTS test(s) failed"
    exit 1
fi
