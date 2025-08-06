#!/bin/bash

echo "🧪 Testing CURSED Import Resolution System"
echo "==========================================="

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASSED=0
FAILED=0

# Function to run a test
run_test() {
    local test_name="$1"
    local command="$2"
    local expect_success="$3"
    
    echo -e "\n${YELLOW}Testing: $test_name${NC}"
    echo "Command: $command"
    
    if eval "$command" >/dev/null 2>&1; then
        if [ "$expect_success" = "true" ]; then
            echo -e "${GREEN}✅ PASS${NC}"
            ((PASSED++))
        else
            echo -e "${RED}❌ FAIL (expected failure but got success)${NC}"
            ((FAILED++))
        fi
    else
        if [ "$expect_success" = "false" ]; then
            echo -e "${GREEN}✅ PASS (expected failure)${NC}"
            ((PASSED++))
        else
            echo -e "${RED}❌ FAIL${NC}"
            ((FAILED++))
        fi
    fi
}

# Test 1: Basic import resolution from project root
run_test "Import from project root" \
    "./zig-out/bin/cursed-zig tests/e2e/basic/01_variables.csd" \
    "true"

# Test 2: Import resolution from subdirectory
run_test "Import from subdirectory" \
    "cd tests/e2e && ../../zig-out/bin/cursed-zig basic/01_variables.csd" \
    "true"

# Test 3: Import resolution with custom stdlib path
run_test "Import with custom stdlib path" \
    "cd tests/e2e && ../../zig-out/bin/cursed-syscall --stdlib-path=../../stdlib basic/01_variables.csd" \
    "true"

# Test 4: Import resolution with wrong stdlib path (should fail)
run_test "Import with wrong stdlib path" \
    "cd tests/e2e && ../../zig-out/bin/cursed-syscall --stdlib-path=/wrong/path basic/01_variables.csd" \
    "false"

# Test 5: Multiple test files from subdirectory
run_test "Multiple tests from subdirectory" \
    "cd tests/e2e && ../../zig-out/bin/cursed-zig stdlib/01_testz_framework.csd" \
    "true"

# Test 6: Test that uses different stdlib modules
run_test "Test with vibez module" \
    "cd tests/e2e && ../../zig-out/bin/cursed-zig stdlib/02_vibez_io.csd" \
    "true"

# Summary
echo -e "\n🎯 Import Resolution Test Summary"
echo "================================="
echo -e "Total tests: $((PASSED + FAILED))"
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"

if [ $FAILED -eq 0 ]; then
    echo -e "\n${GREEN}🎉 All import resolution tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}💥 Some tests failed!${NC}"
    exit 1
fi
