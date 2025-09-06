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
    local test_name=$(basename "$file" .cursed)
    
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
echo "Testing pointer syntax files..."
echo "================================"

# Test basic pointer operations
echo "Memory Tests:"
for file in test_programs/memory/*.cursed; do
    if [ -f "$file" ]; then
        test_file "$file"
    fi
done

# Test function pointer usage
echo
echo "Function Tests:"
for file in test_programs/functions/*.cursed; do
    if [ -f "$file" ]; then
        test_file "$file"
    fi
done

# Test complex scenarios
echo
echo "Complex Tests:"
for file in test_programs/complex/*.cursed; do
    if [ -f "$file" ]; then
        test_file "$file"
    fi
done

# Test validation
echo
echo "Validation Tests:"
for file in test_programs/validation/*.cursed; do
    if [ -f "$file" ]; then
        test_file "$file"
    fi
done

# Test comprehensive scenarios
echo
echo "Comprehensive Tests:"
for file in test_programs/comprehensive/*.cursed; do
    if [ -f "$file" ]; then
        test_file "$file"
    fi
done

# Test edge cases
echo
echo "Edge Case Tests:"
for file in test_programs/edge_cases/*.cursed; do
    if [ -f "$file" ]; then
        test_file "$file"
    fi
done

# Test performance
echo
echo "Performance Tests:"
for file in test_programs/performance/*.cursed; do
    if [ -f "$file" ]; then
        test_file "$file"
    fi
done

# Test regression
echo  
echo "Regression Tests:"
for file in test_programs/regression/*.cursed; do
    if [ -f "$file" ]; then
        test_file "$file"
    fi
done

# Test old syntax rejection (expected to fail)
echo
echo "Error Tests (expected failures):"
echo -n "Testing old syntax rejection... "
if $COMPILER --interpret test_programs/errors/02_old_syntax_rejection.cursed 2>&1 | grep -q "UnexpectedCharacter"; then
    echo -e "${RED}UNEXPECTED PASS${RESET} (old syntax should fail!)"
    FAILED=$((FAILED + 1))
else
    echo -e "${GREEN}EXPECTED FAIL${RESET} (old syntax properly rejected)"
    PASSED=$((PASSED + 1))
fi
TOTAL=$((TOTAL + 1))

# Test new syntax works
echo -n "Testing new syntax acceptance... "
if $COMPILER --interpret test_programs/errors/01_pointer_syntax_errors.cursed > /dev/null 2>&1; then
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
    exit 0
else
    echo -e "  ${RED}❌ Some tests failed${RESET}"
    exit 1
fi
