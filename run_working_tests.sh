#!/bin/bash
# Run all currently working tests in the CURSED project

echo "Running CURSED Test Suite - Working Tests Only"
echo "=============================================="

WORKING_TESTS=(
    "very_simple_test"
    "simple_core_test"
    "simple_lexer_test"
    "simple_llvm_test"
    "simple_jit_test"
    "minimal_interface_test"
)

PASSED=0
FAILED=0

for test in "${WORKING_TESTS[@]}"; do
    echo "Running $test..."
    if ./fix_linking.sh cargo test --test "$test" >/dev/null 2>&1; then
        echo "  ✅ $test - PASS"
        ((PASSED++))
    else
        echo "  ❌ $test - FAIL"
        ((FAILED++))
    fi
done

echo ""
echo "=============================================="
echo "TEST SUMMARY:"
echo "  Passed: $PASSED"
echo "  Failed: $FAILED" 
echo "  Total:  $((PASSED + FAILED))"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "🎉 All working tests passed!"
    exit 0
else
    echo "⚠️  Some tests failed"
    exit 1
fi
