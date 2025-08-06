#!/bin/bash
# Simple test script to run basic tests only

echo "Running basic CURSED tests..."
echo "================================"

# Test a single file
echo -e "\n1. Testing single file:"
./zig-out/bin/cursed test tests/e2e/basic/01_variables.csd 2>/dev/null

echo -e "\n2. Testing basic directory (limited):"
# Run a few specific basic tests
for test_file in tests/e2e/basic/01_variables.csd tests/e2e/basic/02_functions.csd tests/e2e/basic/03_basic_io.csd; do
    if [ -f "$test_file" ]; then
        echo "Running: $test_file"
        ./zig-out/bin/cursed test "$test_file" 2>/dev/null
        echo ""
    fi
done

echo "Basic test run completed."
