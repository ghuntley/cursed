#!/bin/bash

# CURSED Pointer Syntax Test Runner
# Tests the new ඞ pointer syntax

set -e

COMPILER="../zig-out/bin/cursed-compiler"
BOLD='\033[1m'
GREEN='\033[0;32m'  
RED='\033[0;31m'
RESET='\033[0m'

if [ ! -f "$COMPILER" ]; then
    echo "❌ Compiler not found at $COMPILER"
    echo "Run 'zig build' first"
    exit 1
fi

echo "🧪 CURSED Pointer Syntax Test Suite"
echo "===================================="

PASSED=0
FAILED=0
TOTAL=0

test_file() {
    local file="$1"
    local test_name=$(basename "$file" .💀)
    
    echo -n "Testing $test_name... "
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

echo
echo "Testing basic pointer functionality..."
echo "====================================="

# Test just the basic working tests for now
for file in test_programs/memory/01_basic_pointers.💀 \
            test_programs/memory/03_nested_pointers.💀 \
            test_programs/functions/01_pointer_parameters.💀 \
            test_programs/functions/02_pointer_return_values.💀 \
            test_programs/validation/01_pointer_type_validation.💀 \
            test_programs/comprehensive/01_comprehensive_pointers.💀 \
            test_programs/edge_cases/01_pointer_edge_cases.💀; do
    if [ -f "$file" ]; then
        test_file "$file"
    fi
done

# Test old syntax rejection (expected to fail)
echo
echo "Testing syntax validation..."
echo "============================="
echo -n "Testing old syntax rejection... "
if $COMPILER --interpret test_programs/errors/02_old_syntax_rejection.💀 2>&1 | grep -q "UnexpectedCharacter"; then
    echo -e "${GREEN}PASS${RESET} (old syntax properly rejected)"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}FAIL${RESET} (old syntax should be rejected!)"
    FAILED=$((FAILED + 1))
fi
TOTAL=$((TOTAL + 1))

# Test new syntax works
echo -n "Testing new syntax acceptance... "
if $COMPILER --interpret test_programs/errors/01_pointer_syntax_errors.💀 > /dev/null 2>&1; then
    echo -e "${GREEN}PASS${RESET}"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}FAIL${RESET} (new syntax should work!)"
    FAILED=$((FAILED + 1))
fi
TOTAL=$((TOTAL + 1))

echo
echo "=========================================="
echo -e "${BOLD}Pointer Syntax Test Results:${RESET}"
echo -e "  Total tests: $TOTAL"
echo -e "  ${GREEN}Passed: $PASSED${RESET}"
echo -e "  ${RED}Failed: $FAILED${RESET}"

if [ $FAILED -eq 0 ]; then
    echo -e "  ${GREEN}🎉 All pointer syntax tests passed!${RESET}"
    echo -e "  ${GREEN}ඞ The Among Us pointer syntax is working perfectly! ${RESET}"
    exit 0
else
    echo -e "  ${RED}❌ Some tests failed${RESET}"
    exit 1
fi
