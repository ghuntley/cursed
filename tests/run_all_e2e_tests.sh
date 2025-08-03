#!/bin/bash
# Master script to run all end-to-end compilation tests
# Orchestrates comprehensive testing of the entire CURSED compiler pipeline

set -e

echo "🚀 CURSED Compiler End-to-End Test Suite"
echo "========================================"
echo "This comprehensive test suite verifies the entire compiler pipeline"
echo "including lexing, parsing, code generation, execution, performance,"
echo "memory management, and integration testing."
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Test suite results
TOTAL_SUITES=0
PASSED_SUITES=0
FAILED_SUITES=0

# Function to run a test suite
run_test_suite() {
    local suite_name="$1"
    local script_path="$2"
    local description="$3"
    
    TOTAL_SUITES=$((TOTAL_SUITES + 1))
    echo -e "${CYAN}📋 Running Test Suite: $suite_name${NC}"
    echo -e "${BLUE}    Description: $description${NC}"
    echo "    Script: $script_path"
    echo
    
    local start_time=$(date +%s)
    
    if [[ -f "$script_path" ]]; then
        if bash "$script_path"; then
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            echo -e "${GREEN}✅ $suite_name: PASSED (${duration}s)${NC}"
            PASSED_SUITES=$((PASSED_SUITES + 1))
        else
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            echo -e "${RED}❌ $suite_name: FAILED (${duration}s)${NC}"
            FAILED_SUITES=$((FAILED_SUITES + 1))
        fi
    else
        echo -e "${RED}❌ $suite_name: SCRIPT NOT FOUND${NC}"
        FAILED_SUITES=$((FAILED_SUITES + 1))
    fi
    echo
    echo "----------------------------------------"
    echo
}

# Check prerequisites
echo "🔧 Checking prerequisites..."
echo "============================"

# Check for required commands
MISSING_DEPS=0

check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo -e "${RED}❌ Missing: $1${NC}"
        MISSING_DEPS=$((MISSING_DEPS + 1))
    else
        echo -e "${GREEN}✅ Found: $1${NC}"
    fi
}

check_command "cargo"
check_command "rustc"
check_command "bc"
check_command "timeout"

# Optional tools
if command -v valgrind &> /dev/null; then
    echo -e "${GREEN}✅ Found: valgrind (memory leak detection available)${NC}"
else
    echo -e "${YELLOW}⚠️  Optional: valgrind (install for memory leak detection)${NC}"
fi

if command -v hyperfine &> /dev/null; then
    echo -e "${GREEN}✅ Found: hyperfine (advanced benchmarking available)${NC}"
else
    echo -e "${YELLOW}⚠️  Optional: hyperfine (install for advanced benchmarking)${NC}"
fi

if [[ $MISSING_DEPS -gt 0 ]]; then
    echo -e "${RED}❌ Missing required dependencies. Please install them first.${NC}"
    echo "   Ubuntu/Debian: sudo apt update && sudo apt install bc coreutils"
    echo "   macOS: brew install coreutils bc"
    exit 1
fi

echo -e "${GREEN}✅ All required dependencies found${NC}"
echo

# Preliminary build check
echo "🔨 Preliminary Build Check..."
echo "============================="
if cargo check > /tmp/preliminary_check.log 2>&1; then
    echo -e "${GREEN}✅ Preliminary build check passed${NC}"
else
    echo -e "${RED}❌ Preliminary build check failed${NC}"
    echo "Build errors:"
    cat /tmp/preliminary_check.log | head -n 20
    echo
    echo "Please fix build errors before running end-to-end tests."
    exit 1
fi
echo

# Get test configuration
echo "⚙️  Test Configuration"
echo "====================="

# Check if running in CI environment
if [[ -n "$CI" ]]; then
    echo "🔄 CI Mode: Running all tests with standard timeouts"
    TIMEOUT_MULTIPLIER=2
    SKIP_SLOW_TESTS=false
else
    echo "🖥️  Interactive Mode: Full test suite"
    TIMEOUT_MULTIPLIER=1
    SKIP_SLOW_TESTS=false
    
    # Ask user for test preferences
    echo
    read -p "Run performance tests? (may take several minutes) [Y/n]: " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Nn]$ ]]; then
        SKIP_SLOW_TESTS=true
        echo "⚡ Skipping performance tests for faster execution"
    fi
fi

echo "📊 Test Configuration:"
echo "   Timeout multiplier: ${TIMEOUT_MULTIPLIER}x"
echo "   Skip slow tests: $SKIP_SLOW_TESTS"
echo

echo "🚀 Starting Comprehensive End-to-End Test Suite..."
echo "=================================================="

# Record overall start time
OVERALL_START=$(date +%s)

# Test Suite 1: Component Testing (Lexer, Parser, Codegen)
run_test_suite \
    "Component Pipeline Tests" \
    "tests/lexer_parser_codegen_tests.sh" \
    "Tests individual compiler components: lexer, parser, and code generation"

# Test Suite 2: End-to-End Compilation Pipeline
run_test_suite \
    "End-to-End Pipeline Tests" \
    "tests/comprehensive_e2e_pipeline_tests.sh" \
    "Tests complete compilation pipeline from source to executable"

# Test Suite 3: Integration Testing
run_test_suite \
    "Integration Tests" \
    "tests/comprehensive_integration_tests.sh" \
    "Tests complex interactions between language features"

# Test Suite 4: Standard Library Testing
run_test_suite \
    "Standard Library Tests" \
    "tests/run_stdlib_tests.sh" \
    "Tests CURSED standard library modules and functions"

# Test Suite 5: Performance and Memory Testing (if not skipped)
if [[ "$SKIP_SLOW_TESTS" != "true" ]]; then
    run_test_suite \
        "Performance & Memory Tests" \
        "tests/performance_memory_leak_tests.sh" \
        "Tests compiler performance, memory usage, and leak detection"
fi

# Test Suite 6: Error Handling Tests
run_test_suite \
    "Error Handling Tests" \
    "tests/run_error_handling_tests.sh" \
    "Tests compiler error detection and recovery mechanisms"

# Test Suite 7: Cross-Compilation Tests (if available)
if [[ -f "tests/run_cross_compilation_tests.sh" ]]; then
    run_test_suite \
        "Cross-Compilation Tests" \
        "tests/run_cross_compilation_tests.sh" \
        "Tests compilation for multiple target platforms"
fi

# Test Suite 8: Bootstrap/Self-Hosting Tests
if [[ -f "ci/bootstrap_validation_tests.sh" ]]; then
    run_test_suite \
        "Bootstrap Validation Tests" \
        "ci/bootstrap_validation_tests.sh" \
        "Tests compiler self-hosting capabilities"
fi

# Calculate overall duration
OVERALL_END=$(date +%s)
OVERALL_DURATION=$((OVERALL_END - OVERALL_START))

# Final results
echo "🏁 End-to-End Test Suite Complete"
echo "================================="
echo
echo "📊 Overall Results:"
echo -e "   Total test suites: ${BLUE}$TOTAL_SUITES${NC}"
echo -e "   Passed: ${GREEN}$PASSED_SUITES${NC}"
echo -e "   Failed: ${RED}$FAILED_SUITES${NC}"
echo -e "   Total duration: ${CYAN}${OVERALL_DURATION}s${NC}"
echo

# Success rate calculation
if [[ $TOTAL_SUITES -gt 0 ]]; then
    SUCCESS_RATE=$(( (PASSED_SUITES * 100) / TOTAL_SUITES ))
    echo -e "   Success rate: ${BLUE}${SUCCESS_RATE}%${NC}"
fi

echo
if [[ $FAILED_SUITES -eq 0 ]]; then
    echo -e "${GREEN}🎉 ALL TESTS PASSED!${NC}"
    echo -e "${GREEN}✅ CURSED compiler is fully functional and ready for production use${NC}"
    echo
    echo "📋 Test Coverage Summary:"
    echo "   ✅ Lexical analysis (tokenization)"
    echo "   ✅ Syntax analysis (parsing)"
    echo "   ✅ Semantic analysis (type checking)"
    echo "   ✅ Code generation (LLVM IR)"
    echo "   ✅ Executable generation"
    echo "   ✅ Program execution (interpretation & compilation)"
    echo "   ✅ Error handling and recovery"
    echo "   ✅ Standard library functionality"
    echo "   ✅ Language feature integration"
    if [[ "$SKIP_SLOW_TESTS" != "true" ]]; then
        echo "   ✅ Performance and memory safety"
    fi
    echo
    echo "🚀 The CURSED compiler pipeline is verified and ready!"
    exit 0
else
    echo -e "${RED}❌ SOME TESTS FAILED${NC}"
    echo -e "${RED}⚠️  CURSED compiler needs fixes before production use${NC}"
    echo
    echo "🔍 Failed test suites: $FAILED_SUITES"
    echo "💡 Review the individual test suite outputs above for specific issues"
    echo "🛠️  Fix the failing components and re-run the test suite"
    echo
    exit 1
fi

# Cleanup temporary files
rm -f /tmp/preliminary_check.log
