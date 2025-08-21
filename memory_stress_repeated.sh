#!/bin/bash

# Repeated Memory Stress Testing for CURSED v1.0
echo "🔄 Running Repeated Memory Stress Tests..."

ITERATIONS=10
PASSED=0
FAILED=0

for i in $(seq 1 $ITERATIONS); do
    echo -n "Iteration $i/$ITERATIONS: "
    
    if timeout 30s valgrind --leak-check=full --error-exitcode=1 \
       ./zig-out/bin/cursed-stable memory_audit_simple.csd >/dev/null 2>&1; then
        echo "✅ PASS"
        PASSED=$((PASSED + 1))
    else
        echo "❌ FAIL"
        FAILED=$((FAILED + 1))
    fi
done

echo
echo "Results: $PASSED passed, $FAILED failed out of $ITERATIONS iterations"

if [[ $FAILED -eq 0 ]]; then
    echo "🎉 All repeated tests passed - consistent memory safety!"
    exit 0
else
    echo "❌ Some iterations failed - potential intermittent memory issues"
    exit 1
fi
