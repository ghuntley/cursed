#!/bin/bash

# Quick validation of core test functionality

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

# Build compiler
log_info "Building CURSED compiler..."
if zig build-exe src-zig/main_unified.zig -lc --name cursed-unified; then
    log_success "Compiler built successfully"
else
    log_error "Failed to build compiler"
    exit 1
fi

# Test basic functionality
log_info "Testing basic functionality..."

echo 'vibez.spill("Hello from test suite!")' > basic_test.csd

if ./cursed-unified basic_test.csd > /dev/null 2>&1; then
    log_success "Basic interpretation test"
else
    log_error "Basic interpretation test failed"
fi

if ./cursed-unified --compile basic_test.csd > /dev/null 2>&1; then
    if [ -f ./basic_test ]; then
        if ./basic_test > /dev/null 2>&1; then
            log_success "Basic compilation test"
        else
            log_error "Basic compilation execution failed"
        fi
        rm -f basic_test
    else
        log_error "Basic compilation failed to produce executable"
    fi
else
    log_error "Basic compilation test failed"
fi

# Cleanup
rm -f basic_test.csd

# Test core unit tests
log_info "Running core unit tests..."

TESTS_PASSED=0
TESTS_FAILED=0

run_test() {
    local test_script="$1"
    if [ -f "$test_script" ] && [ -x "$test_script" ]; then
        if "$test_script" > /dev/null 2>&1; then
            log_success "$(basename "$test_script")"
            ((TESTS_PASSED++))
        else
            log_error "$(basename "$test_script")"
            ((TESTS_FAILED++))
        fi
    else
        log_error "Test script not found or not executable: $test_script"
        ((TESTS_FAILED++))
    fi
}

# Run available unit tests
run_test "./test_suite/unit_tests/test_lexer_basic.sh"
run_test "./test_suite/unit_tests/test_lexer_keywords.sh"
run_test "./test_suite/unit_tests/test_lexer_strings.sh"
run_test "./test_suite/unit_tests/test_parser_variables.sh"
run_test "./test_suite/unit_tests/test_parser_functions.sh"

# Run integration tests
run_test "./test_suite/integration_tests/test_e2e_hello_world.sh"

# Run feature tests
run_test "./test_suite/feature_tests/test_variables.sh"

log_info "=== QUICK VALIDATION SUMMARY ==="
echo "Passed: $TESTS_PASSED"
echo "Failed: $TESTS_FAILED"

if [ $TESTS_FAILED -eq 0 ]; then
    log_success "Quick validation completed successfully!"
    exit 0
else
    log_error "Some tests failed during quick validation"
    exit 1
fi
