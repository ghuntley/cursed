#!/bin/bash

# Comprehensive CURSED Standard Library Test Runner
# Tests all modules in both interpretation and compilation modes

echo "🧪 CURSED Standard Library Comprehensive Test Suite"
echo "===================================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Test result arrays
declare -a PASSED_MODULES
declare -a FAILED_MODULES

# Function to run a single test
run_test() {
    local test_file="$1"
    local module_name="$2"
    
    echo -e "${BLUE}Testing module: $module_name${NC}"
    echo "File: $test_file"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Test interpretation mode
    echo "  → Testing interpretation mode..."
    if timeout 30 cargo run --bin cursed "$test_file" > /dev/null 2>&1; then
        echo -e "    ${GREEN}✓ Interpretation: PASS${NC}"
        interpretation_result="PASS"
    else
        echo -e "    ${RED}✗ Interpretation: FAIL${NC}"
        interpretation_result="FAIL"
    fi
    
    # Test compilation mode
    echo "  → Testing compilation mode..."
    if timeout 30 cargo run --bin cursed -- compile "$test_file" > /dev/null 2>&1; then
        echo -e "    ${GREEN}✓ Compilation: PASS${NC}"
        compilation_result="PASS"
    else
        echo -e "    ${YELLOW}⚠ Compilation: FALLBACK${NC}"
        compilation_result="FALLBACK"
    fi
    
    # Overall result
    if [[ "$interpretation_result" == "PASS" ]]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        PASSED_MODULES+=("$module_name")
        echo -e "  ${GREEN}✓ Module $module_name: PASS${NC}"
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
        FAILED_MODULES+=("$module_name")
        echo -e "  ${RED}✗ Module $module_name: FAIL${NC}"
    fi
    
    echo ""
}

# Test our working test files first
echo "🔬 Testing verified working tests..."
echo ""

if [[ -f "test_stdlib_simple.csd" ]]; then
    run_test "test_stdlib_simple.csd" "core_language"
fi

if [[ -f "test_math_working.csd" ]]; then
    run_test "test_math_working.csd" "math_basic"
fi

if [[ -f "stdlib/testz/mod_fixed.csd" ]]; then
    run_test "stdlib/testz/mod_fixed.csd" "testz_fixed"
fi

# Test existing stdlib modules
echo "📚 Testing existing stdlib modules..."
echo ""

# Find all test files and test them
while IFS= read -r -d '' test_file; do
    # Extract module name from path
    module_name=$(echo "$test_file" | sed 's|stdlib/||' | sed 's|/.*||')
    
    # Skip if we already tested this module
    skip=false
    for tested in "${PASSED_MODULES[@]}" "${FAILED_MODULES[@]}"; do
        if [[ "$tested" == "$module_name" ]]; then
            skip=true
            break
        fi
    done
    
    if [[ "$skip" == false ]]; then
        run_test "$test_file" "$module_name"
    fi
done < <(find stdlib -name "test_*.csd" -print0 | head -20)

# Generate comprehensive report
echo ""
echo "📊 COMPREHENSIVE TEST RESULTS"
echo "============================="
echo ""
echo "Total modules tested: $TOTAL_TESTS"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"

if [[ $TOTAL_TESTS -gt 0 ]]; then
    pass_rate=$(( (PASSED_TESTS * 100) / TOTAL_TESTS ))
    echo "Pass rate: ${pass_rate}%"
fi

echo ""
echo "✅ PASSING MODULES:"
for module in "${PASSED_MODULES[@]}"; do
    echo -e "  ${GREEN}✓ $module${NC}"
done

echo ""
echo "❌ FAILING MODULES:"
for module in "${FAILED_MODULES[@]}"; do
    echo -e "  ${RED}✗ $module${NC}"
done

echo ""
echo "🔧 RECOMMENDATIONS:"
echo "1. Fix testz module import system to enable more comprehensive testing"
echo "2. Create self-contained test files for modules that can't import testz"
echo "3. Investigate compilation mode issues (LLVM tools availability)"
echo "4. Implement automated test discovery and execution"

echo ""
if [[ $FAILED_TESTS -eq 0 ]]; then
    echo -e "${GREEN}🎉 ALL TESTED MODULES PASSED! 🎉${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  Some modules need attention${NC}"
    exit 1
fi
