#!/bin/bash

# CURSED Rule 30 Test Runner
# Quick test runner for individual test components

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}=== CURSED Rule 30 Test Runner ===${NC}"
echo

# Function to run a single test
run_single_test() {
    local test_file="$1"
    local test_name="$2"
    
    echo -e "${BLUE}Running $test_name...${NC}"
    
    if [ ! -f "$test_file" ]; then
        echo -e "${RED}Error: Test file $test_file not found${NC}"
        return 1
    fi
    
    if timeout 30 cursed "$test_file"; then
        echo -e "${GREEN}✓ $test_name completed successfully${NC}"
        return 0
    else
        echo -e "${RED}✗ $test_name failed${NC}"
        return 1
    fi
}

# Run specific test based on argument
if [ $# -eq 1 ]; then
    case "$1" in
        "rule30"|"algorithm")
            run_single_test "test_rule30.csd" "Rule 30 Algorithm Tests"
            ;;
        "conversion"|"convert")
            run_single_test "test_conversion.csd" "Conversion Tests"
            ;;
        "integration"|"pipeline")
            run_single_test "test_integration.csd" "Integration Tests"
            ;;
        "all")
            echo "Running all tests..."
            run_single_test "test_rule30.csd" "Rule 30 Algorithm Tests"
            echo
            run_single_test "test_conversion.csd" "Conversion Tests"
            echo
            run_single_test "test_integration.csd" "Integration Tests"
            ;;
        "validate")
            echo "Running comprehensive validation..."
            ./validate.sh
            ;;
        *)
            echo "Unknown test: $1"
            echo "Available tests: rule30, conversion, integration, all, validate"
            exit 1
            ;;
    esac
else
    # Run all tests by default
    echo "Running all tests (use './run_tests.sh [test_name]' for specific tests)..."
    echo
    
    run_single_test "test_rule30.csd" "Rule 30 Algorithm Tests"
    echo
    run_single_test "test_conversion.csd" "Conversion Tests"  
    echo
    run_single_test "test_integration.csd" "Integration Tests"
    echo
    
    echo -e "${GREEN}All tests completed!${NC}"
    echo
    echo "For comprehensive validation with expected results comparison, run:"
    echo "./validate.sh"
fi
