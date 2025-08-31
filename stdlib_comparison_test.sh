#!/bin/bash

# CURSED Stdlib Comparison Test Suite
# Tests stdlib functions in both interpreter and compiled modes

echo "=== CURSED Standard Library Comparison Test Suite ==="
echo "Testing stdlib function calls in interpreter vs compiled modes"
echo "================================================================"

# Create test programs
cat > test_mathz_simple.csd << 'EOF'
yeet "mathz"
sus result drip = mathz.add_two(5, 3)
vibez.spill("Result: ")
vibez.spill(result)
EOF

cat > test_simple_output.csd << 'EOF'
vibez.spill("Hello from CURSED!")
vibez.spill(42)
vibez.spill(true)
EOF

echo "1. Testing basic vibez output..."

echo "--- Interpreter Mode ---"
./zig-out/bin/cursed-compiler test_simple_output.csd --mode interpret 2>/dev/null | grep -v "DEBUG\|ERROR\|INFO\|==="

echo "--- Compile Mode ---"
./zig-out/bin/cursed-compiler test_simple_output.csd --mode compile -o test_simple_compiled 2>/dev/null
if [ -f "test_simple_compiled" ]; then
    echo "Binary created successfully"
    ./test_simple_compiled 2>/dev/null || echo "Failed to run binary"
else
    echo "Binary not created"
fi

echo ""
echo "2. Testing mathz module..."

echo "--- Interpreter Mode ---"
./zig-out/bin/cursed-compiler test_mathz_simple.csd --mode interpret 2>/dev/null | grep -v "DEBUG\|ERROR\|INFO\|==="

echo "--- Compile Mode ---" 
./zig-out/bin/cursed-compiler test_mathz_simple.csd --mode compile -o test_mathz_compiled 2>/dev/null
if [ -f "test_mathz_compiled" ]; then
    echo "Binary created successfully"
    ./test_mathz_compiled 2>/dev/null || echo "Failed to run binary"
else
    echo "Binary not created"
fi

# Cleanup
rm -f test_mathz_simple.csd test_simple_output.csd test_simple_compiled test_mathz_compiled

echo ""
echo "=== Test Suite Complete ==="
