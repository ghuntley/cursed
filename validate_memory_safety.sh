#!/bin/bash

# CURSED Compiler Memory Safety Validation Script
# Tests for memory leaks in LLVM backend compilation pipeline

set -e

echo "🧪 CURSED Compiler Memory Safety Validation"
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Function to run valgrind test
run_valgrind_test() {
    local test_name="$1"
    local test_file="$2"
    local extra_args="$3"
    
    echo -n "Testing ${test_name}... "
    
    # Run valgrind with full leak check
    if valgrind --leak-check=full --error-exitcode=1 ./zig-out/bin/cursed-zig "$test_file" $extra_args &>/dev/null; then
        echo -e "${GREEN}✅ PASS${NC}"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC}"
        echo "   Memory leaks detected in $test_name"
        ((TESTS_FAILED++))
    fi
}

# Check if the compiler binary exists
echo "Checking CURSED compiler binary..."
if [ ! -f "./zig-out/bin/cursed-zig" ]; then
    echo "Building CURSED compiler..."
    if ! zig build &>/dev/null; then
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    fi
fi
echo -e "${GREEN}✅ Compiler ready${NC}"
echo

# Test 1: Basic variable assignment
echo "# Test 1: Basic Variable Assignment"
cat > memory_test_1.csd << 'EOF'
sus x drip = 42
vibez.spill("Value:", x)
EOF
run_valgrind_test "Basic variables" "memory_test_1.csd" ""
run_valgrind_test "Basic variables (LLVM)" "memory_test_1.csd" "--compile"
echo

# Test 2: Standard library imports
echo "# Test 2: Standard Library Imports"
cat > memory_test_2.csd << 'EOF'
yeet "mathz"
yeet "stringz"
yeet "arrayz"
vibez.spill("Modules loaded")
EOF
run_valgrind_test "Stdlib imports" "memory_test_2.csd" ""
run_valgrind_test "Stdlib imports (LLVM)" "memory_test_2.csd" "--compile"
echo

# Test 3: Array operations
echo "# Test 3: Array Operations"
cat > memory_test_3.csd << 'EOF'
yeet "arrayz"
sus nums []drip = [1, 2, 3, 4, 5]
vibez.spill("Array length:", len(nums))
EOF
run_valgrind_test "Array operations" "memory_test_3.csd" ""
run_valgrind_test "Array operations (LLVM)" "memory_test_3.csd" "--compile"
echo

# Test 4: Functions and expressions  
echo "# Test 4: Functions and Expressions"
cat > memory_test_4.csd << 'EOF'
slay add(a drip, b drip) drip {
    damn a + b
}
sus result drip = add(10, 20)
vibez.spill("Result:", result)
EOF
run_valgrind_test "Functions" "memory_test_4.csd" ""
run_valgrind_test "Functions (LLVM)" "memory_test_4.csd" "--compile"
echo

# Test 5: Loops and control flow
echo "# Test 5: Loops and Control Flow"
cat > memory_test_5.csd << 'EOF'
sus i drip = 0
bestie (i < 3) {
    vibez.spill("Count:", i)
    i = i + 1
}
ready (i > 2) {
    vibez.spill("Loop completed")
}
EOF
run_valgrind_test "Control flow" "memory_test_5.csd" ""
run_valgrind_test "Control flow (LLVM)" "memory_test_5.csd" "--compile"
echo

# Test 6: Pattern matching
echo "# Test 6: Pattern Matching"
cat > memory_test_6.csd << 'EOF'
sus x drip = 5
ready (x) {
    1 => vibez.spill("one")
    5 => vibez.spill("five") 
    _ => vibez.spill("other")
}
EOF
run_valgrind_test "Pattern matching" "memory_test_6.csd" ""
run_valgrind_test "Pattern matching (LLVM)" "memory_test_6.csd" "--compile"
echo

# Test 7: Complex stdlib usage
echo "# Test 7: Complex Standard Library Usage"
cat > memory_test_7.csd << 'EOF'
yeet "mathz"
yeet "stringz"
yeet "arrayz"
yeet "cryptz"

sus numbers []drip = [1, 2, 3, 4, 5]
sus total drip = 0
sus idx drip = 0

bestie (idx < len(numbers)) {
    total = total + numbers[idx]
    idx = idx + 1
}

vibez.spill("Total:", total)
vibez.spill("Math abs:", abs_normie(-42))
EOF
run_valgrind_test "Complex stdlib" "memory_test_7.csd" ""
run_valgrind_test "Complex stdlib (LLVM)" "memory_test_7.csd" "--compile"
echo

# Test 8: Memory stress test (multiple files)
echo "# Test 8: Memory Stress Test"
for i in {1..5}; do
    cat > "stress_test_$i.csd" << EOF
yeet "mathz"
sus x drip = $i * 10
sus y drip = abs_normie(-$i)
vibez.spill("Stress test $i:", x, y)
EOF
    run_valgrind_test "Stress test $i" "stress_test_$i.csd" ""
    run_valgrind_test "Stress test $i (LLVM)" "stress_test_$i.csd" "--compile"
done
echo

# Summary
echo "=============================================="
echo "Memory Safety Validation Results:"
echo -e "  ${GREEN}Tests Passed: $TESTS_PASSED${NC}"
echo -e "  ${RED}Tests Failed: $TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}🎉 ALL MEMORY SAFETY TESTS PASSED!${NC}"
    echo "✅ Zero memory leaks detected in LLVM compilation pipeline"
    echo "✅ All LLVM resources properly disposed"
    echo "✅ Memory management fixes are working correctly"
else
    echo -e "\n${RED}❌ MEMORY LEAKS DETECTED${NC}"
    echo "Some tests failed - memory leaks still present"
    exit 1
fi

# Cleanup test files
rm -f memory_test_*.csd stress_test_*.csd

echo -e "\n${GREEN}✅ Memory safety validation complete!${NC}"
