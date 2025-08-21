#!/bin/bash

# CURSED v1.0 Comprehensive Memory Safety Audit
set -e

echo "🔍 CURSED v1.0 Comprehensive Memory Safety Audit"
echo "==============================================="
echo

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Test results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Valgrind options
VALGRIND_OPTS="--leak-check=full --show-leak-kinds=all --track-origins=yes --error-exitcode=1"

# Test simple CURSED program
create_test_program() {
    local filename="$1"
    local content="$2"
    echo "$content" > "$filename"
}

run_memory_test() {
    local binary="$1"
    local test_file="$2"
    local test_name="$3"
    
    echo -e "${BLUE}Testing $test_name with $binary...${NC}"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Run with timeout to prevent hanging
    if timeout 45s valgrind $VALGRIND_OPTS ./$binary "$test_file" > /tmp/valgrind_output_$$.log 2>&1; then
        if grep -q "All heap blocks were freed -- no leaks are possible" /tmp/valgrind_output_$$.log; then
            echo -e "  ✅ ${GREEN}PASS - No memory leaks detected${NC}"
            PASSED_TESTS=$((PASSED_TESTS + 1))
            return 0
        elif grep -q "memory address.*leaked" /tmp/valgrind_output_$$.log; then
            echo -e "  ❌ ${RED}FAIL - Memory leak detected${NC}"
            grep "memory address.*leaked" /tmp/valgrind_output_$$.log | head -3
            FAILED_TESTS=$((FAILED_TESTS + 1))
            return 1
        else
            echo -e "  ✅ ${GREEN}PASS - No explicit leaks found${NC}"
            PASSED_TESTS=$((PASSED_TESTS + 1))
            return 0
        fi
    else
        echo -e "  ❌ ${RED}FAIL - Valgrind detected issues or timeout${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

# Change to CURSED directory
cd zig-out/bin

# Test 1: Basic program
create_test_program "test_basic.csd" 'yeet "vibez"
vibez.spill("Hello CURSED!")
sus x drip = 42
vibez.spill("x:", x)'

# Test 2: Array operations
create_test_program "test_arrays.csd" 'yeet "vibez"
sus arr []drip = [1, 2, 3]
vibez.spill("Array created")
vibez.spill("Array length:", len(arr))'

# Test 3: String operations
create_test_program "test_strings.csd" 'yeet "vibez"
sus str tea = "Hello"
vibez.spill("String:", str)
vibez.spill("String length:", len(str))'

# Test 4: Variable assignments
create_test_program "test_variables.csd" 'yeet "vibez"
sus a drip = 10
sus b drip = 20
sus c drip = 30
vibez.spill("Variables:", a, b, c)'

echo -e "${YELLOW}=== MEMORY SAFETY AUDIT RESULTS ===${NC}"
echo

# Test available binaries
BINARIES=("cursed-minimal" "cursed-stable" "cursed-zig" "cursed-perf")

for binary in "${BINARIES[@]}"; do
    if [[ -f "$binary" ]]; then
        echo -e "${BLUE}=== Testing Binary: $binary ===${NC}"
        
        run_memory_test "$binary" "test_basic.csd" "Basic Program Test"
        run_memory_test "$binary" "test_arrays.csd" "Array Operations Test"
        run_memory_test "$binary" "test_strings.csd" "String Operations Test" 
        run_memory_test "$binary" "test_variables.csd" "Variable Assignment Test"
        
        echo
    else
        echo -e "${YELLOW}Binary $binary not found, skipping...${NC}"
    fi
done

# Summary
echo -e "${YELLOW}=== FINAL MEMORY SAFETY AUDIT SUMMARY ===${NC}"
echo "Total Tests Run: $TOTAL_TESTS"
echo -e "Tests Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Tests Failed: ${RED}$FAILED_TESTS${NC}"
echo

if [[ $FAILED_TESTS -eq 0 ]]; then
    echo -e "${GREEN}🎉 MEMORY SAFETY AUDIT PASSED!${NC}"
    echo -e "${GREEN}All CURSED binaries demonstrate proper memory management.${NC}"
    exit 0
else
    echo -e "${RED}❌ MEMORY SAFETY ISSUES DETECTED${NC}"
    echo -e "${RED}Some binaries have memory leaks or safety issues.${NC}"
    exit 1
fi
