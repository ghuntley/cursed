#!/bin/bash

# Script to run all JIT tests for the CURSED language

# Set the terminal colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Move to project root directory
cd "$(dirname "$0")/.." || exit

# Build the compiler
echo -e "${YELLOW}Building the CURSED compiler...${NC}"
cargo build || { echo -e "${RED}Failed to build compiler!${NC}"; exit 1; }

# Initialize counters
TOTAL=0
PASSED=0
FAILED=0

# Function to run a test
run_test() {
    local test_file="$1"
    local test_name="${test_file##*/}"
    
    TOTAL=$((TOTAL + 1))
    
    echo -e "\n${YELLOW}Running test: ${test_name}${NC}"
    echo -e "${YELLOW}------------------------------------------${NC}"
    
    # Run the test
    ./target/debug/cursed "$test_file"
    local exit_code=$?
    
    # Check the result
    if [ $exit_code -eq 0 ]; then
        echo -e "${GREEN}✓ Test passed: ${test_name}${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗ Test failed: ${test_name} (Exit code: ${exit_code})${NC}"
        FAILED=$((FAILED + 1))
    fi
}

# Run all tests in the jit directory
echo -e "\n${YELLOW}==== Running JIT Tests ====${NC}"
for test_file in tests/jit/*.csd; do
    run_test "$test_file"
done

# Print summary
echo -e "\n${YELLOW}==== Test Summary ====${NC}"
echo -e "Total tests: ${TOTAL}"
echo -e "${GREEN}Passed: ${PASSED}${NC}"
echo -e "${RED}Failed: ${FAILED}${NC}"

# Return exit code based on test results
if [ $FAILED -eq 0 ]; then
    echo -e "\n${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}Some tests failed.${NC}"
    exit 1
fi

