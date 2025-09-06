#!/bin/bash

# CURSED Comprehensive Pointer Test Suite
# World's first Among Us character (ඞ) pointer syntax test runner

set -e

COMPILER="../zig-out/bin/cursed-compiler"
BOLD='\033[1m'
GREEN='\033[0;32m'  
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RESET='\033[0m'

if [ ! -f "$COMPILER" ]; then
    echo -e "${RED}❌ Compiler not found at $COMPILER${RESET}"
    echo "Run 'zig build' first"
    exit 1
fi

echo -e "${BOLD}🚀 CURSED Comprehensive Pointer Test Suite${RESET}"
echo -e "${BLUE}World's First Among Us Character (ඞ) Pointer Syntax${RESET}"
echo "=================================================="

PASSED=0
FAILED=0
TOTAL=0
CATEGORIES_TESTED=0

test_file() {
    local file="$1"
    local category="$2"
    local test_name=$(basename "$file" .💀)
    
    echo -n "  Testing $test_name... "
    TOTAL=$((TOTAL + 1))
    
    # Test interpreter mode
    if $COMPILER --interpret "$file" > /tmp/cursed_interp.out 2>&1; then
        # Test compiled mode  
        if $COMPILER --compile "$file" -o /tmp/cursed_test 2>/dev/null && /tmp/cursed_test > /tmp/cursed_comp.out 2>&1; then
            # Compare outputs
            if diff /tmp/cursed_interp.out /tmp/cursed_comp.out > /dev/null; then
                echo -e "${GREEN}PASS${RESET}"
                PASSED=$((PASSED + 1))
            else
                echo -e "${RED}FAIL${RESET} (output mismatch)"
                FAILED=$((FAILED + 1))
            fi
        else
            echo -e "${RED}FAIL${RESET} (compilation failed)"
            FAILED=$((FAILED + 1))
        fi
    else
        echo -e "${RED}FAIL${RESET} (interpretation failed)"
        FAILED=$((FAILED + 1))
    fi
    
    # Cleanup
    rm -f /tmp/cursed_test /tmp/cursed_interp.out /tmp/cursed_comp.out
}

test_category() {
    local category="$1"
    local description="$2"
    shift 2
    
    echo
    echo -e "${YELLOW}$category: $description${RESET}"
    echo "$(printf '=%.0s' $(seq 1 ${#category}))"
    
    CATEGORIES_TESTED=$((CATEGORIES_TESTED + 1))
    
    for file in "$@"; do
        if [ -f "$file" ]; then
            test_file "$file" "$category"
        else
            echo -e "  ${YELLOW}SKIP${RESET} $file (not found)"
        fi
    done
}

# Test Categories

test_category "Basic Pointer Operations" "Fundamental ඞ pointer functionality" \
    "test_programs/memory/01_basic_pointers.💀" \
    "test_programs/memory/02_pointer_arithmetic.💀" \
    "test_programs/memory/03_nested_pointers.💀"

test_category "Function Integration" "Pointers in function parameters and returns" \
    "test_programs/functions/01_pointer_parameters.💀" \
    "test_programs/functions/02_pointer_return_values.💀"

test_category "Type Safety Validation" "Type system integration with ඞ syntax" \
    "test_programs/validation/01_pointer_type_validation.💀"

test_category "Edge Cases & Boundary Conditions" "Extreme scenarios and corner cases" \
    "test_programs/edge_cases/01_pointer_edge_cases.💀"

test_category "Comprehensive Integration" "Complex real-world scenarios" \
    "test_programs/comprehensive/01_comprehensive_pointers.💀" \
    "test_programs/comprehensive/02_pointer_stress_test.💀" \
    "test_programs/comprehensive/03_pointer_performance_test.💀"

test_category "Syntax Validation" "New ඞ syntax vs legacy @ syntax" \
    "test_programs/syntax_validation/01_amongus_vs_legacy_syntax.💀"

test_category "Performance Testing" "Performance characteristics" \
    "test_programs/performance/01_pointer_intensive.💀"

test_category "Regression Testing" "Prevention of regressions" \
    "test_programs/regression/01_pointer_regression.💀"

# Test syntax rejection (expected to fail)
echo
echo -e "${YELLOW}Syntax Rejection Tests${RESET}"
echo "======================"
echo -n "  Testing legacy syntax rejection... "
if $COMPILER --interpret test_programs/syntax_validation/02_legacy_syntax_rejection.💀 2>&1 | grep -q -i "error\|unexpected\|fail"; then
    echo -e "${GREEN}PASS${RESET} (legacy syntax properly rejected)"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}FAIL${RESET} (legacy syntax should be rejected!)"
    FAILED=$((FAILED + 1))
fi
TOTAL=$((TOTAL + 1))

# Calculate statistics
PASS_RATE=$(( (PASSED * 100) / TOTAL ))

echo
echo "=================================================="
echo -e "${BOLD}📊 COMPREHENSIVE TEST RESULTS${RESET}"
echo "=================================================="
echo -e "  ${BLUE}Test Categories:${RESET} $CATEGORIES_TESTED"
echo -e "  ${BLUE}Total Tests:${RESET} $TOTAL"
echo -e "  ${GREEN}Passed:${RESET} $PASSED"
echo -e "  ${RED}Failed:${RESET} $FAILED"
echo -e "  ${YELLOW}Pass Rate:${RESET} ${PASS_RATE}%"

echo
echo -e "${BOLD}🌟 CULTURAL & TECHNICAL SIGNIFICANCE${RESET}"
echo "=================================================="
echo -e "  ${BLUE}🥇 World's First:${RESET} Among Us character (ඞ) pointer syntax"
echo -e "  ${BLUE}🎮 Cultural Impact:${RESET} Internet culture meets programming languages"
echo -e "  ${BLUE}🔧 Technical Achievement:${RESET} Full UTF-8 Unicode operator support"
echo -e "  ${BLUE}💪 Robustness:${RESET} ${PASS_RATE}% pass rate demonstrates mature implementation"

if [ $FAILED -eq 0 ]; then
    echo
    echo -e "${GREEN}🎉 ALL TESTS PASSED!${RESET}"
    echo -e "${GREEN}ඞ The Among Us pointer syntax is working perfectly! ඞ${RESET}"
    echo -e "${GREEN}Sus but it works - this is a historic achievement! 🚀${RESET}"
    exit 0
else
    echo
    echo -e "${YELLOW}⚠️  Some tests failed, but ${PASS_RATE}% pass rate shows strong progress${RESET}"
    echo -e "${BLUE}ඞ The Among Us pointer revolution continues! ඞ${RESET}"
    exit 1
fi
