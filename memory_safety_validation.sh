#!/bin/bash

# CURSED v1.0 Memory Safety Validation Script
# Comprehensive Valgrind-based memory leak detection

set -e

echo "🔍 CURSED v1.0 Memory Safety Audit"
echo "=================================="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Valgrind options for comprehensive leak detection
VALGRIND_OPTS="--leak-check=full --show-leak-kinds=all --track-origins=yes --verbose --error-exitcode=1"

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run a single test
run_memory_test() {
    local binary="$1"
    local test_file="$2"
    local test_name="$3"
    local iterations="$4"
    
    echo -e "${BLUE}Testing $test_name with $binary (${iterations} iterations)...${NC}"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Run multiple iterations to catch intermittent leaks
    for i in $(seq 1 $iterations); do
        echo -n "  Iteration $i/$iterations... "
        
        # Run with valgrind
        if valgrind $VALGRIND_OPTS ./$binary $test_file > /dev/null 2>&1; then
            echo -e "${GREEN}PASS${NC}"
        else
            echo -e "${RED}FAIL (Memory leak detected!)${NC}"
            FAILED_TESTS=$((FAILED_TESTS + 1))
            echo "  ❌ Memory safety violation in $test_name (iteration $i)"
            return 1
        fi
    done
    
    PASSED_TESTS=$((PASSED_TESTS + 1))
    echo -e "  ✅ ${GREEN}$test_name passed all iterations${NC}"
    return 0
}

# Function to create a simple baseline test
create_baseline_test() {
    cat > memory_audit_baseline.csd << 'EOF'
# Minimal baseline test for memory safety
yeet "vibez"

vibez.spill("Hello, memory safe CURSED!")

sus x drip = 42
sus y tea = "Memory test"
sus arr []drip = [1, 2, 3, 4, 5]

vibez.spill("x:", x, "y:", y, "array length:", len(arr))
vibez.spill("Baseline memory test completed")
EOF
}

# Check if we're in the right directory
if [[ ! -f "build.zig" ]]; then
    echo -e "${RED}Error: Please run this script from the CURSED project root${NC}"
    exit 1
fi

# Build the project first
echo -e "${BLUE}Building CURSED binaries...${NC}"
if ! zig build; then
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi

# Check available binaries
cd zig-out/bin
BINARIES=()
for binary in cursed-zig cursed-stable cursed-minimal cursed-perf; do
    if [[ -f "$binary" ]]; then
        BINARIES+=("$binary")
        echo -e "${GREEN}Found binary: $binary${NC}"
    fi
done

if [[ ${#BINARIES[@]} -eq 0 ]]; then
    echo -e "${RED}No CURSED binaries found!${NC}"
    exit 1
fi

cd ../..

# Create test files if they don't exist
create_baseline_test

echo
echo -e "${YELLOW}Starting comprehensive memory safety audit...${NC}"
echo

# Test each binary with each test case
for binary in "${BINARIES[@]}"; do
    echo -e "${BLUE}=== Testing Binary: $binary ===${NC}"
    
    # Baseline test (10 iterations)
    run_memory_test "zig-out/bin/$binary" "memory_audit_baseline.csd" "Baseline Test" 10
    
    # Stress test (5 iterations)
    run_memory_test "zig-out/bin/$binary" "memory_audit_stress_test.csd" "Stress Test" 5
    
    # Edge cases test (5 iterations)
    run_memory_test "zig-out/bin/$binary" "memory_audit_edge_cases.csd" "Edge Cases Test" 5
    
    # Concurrent test (3 iterations)
    run_memory_test "zig-out/bin/$binary" "memory_audit_concurrent.csd" "Concurrent Test" 3
    
    echo
done

# Summary
echo -e "${YELLOW}=== MEMORY SAFETY AUDIT SUMMARY ===${NC}"
echo "Total Tests: $TOTAL_TESTS"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"
echo

if [[ $FAILED_TESTS -eq 0 ]]; then
    echo -e "${GREEN}🎉 ALL MEMORY SAFETY TESTS PASSED!${NC}"
    echo -e "${GREEN}CURSED v1.0 demonstrates zero memory leaks across all test scenarios.${NC}"
    exit 0
else
    echo -e "${RED}❌ MEMORY SAFETY ISSUES DETECTED!${NC}"
    echo -e "${RED}$FAILED_TESTS out of $TOTAL_TESTS tests failed.${NC}"
    exit 1
fi
