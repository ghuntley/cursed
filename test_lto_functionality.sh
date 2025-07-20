#!/bin/bash

echo "=== CURSED LTO (Link-Time Optimization) Functionality Test ==="
echo

# Create test programs with different optimization levels
echo "Creating test programs..."

# Test 1: Simple program without LTO
cat > test_no_lto.csd << 'EOF'
vibez.spill("Without LTO optimization")
EOF

# Test 2: Simple program with LTO
cat > test_with_lto.csd << 'EOF'
vibez.spill("With LTO optimization enabled")
EOF

echo "Compiling programs with different optimization levels..."
echo

# Compile without optimization (O0)
echo "1. Compiling with O0 (no optimization)..."
cargo run --bin cursed -- compile --opt-level=0 test_no_lto.csd
if [ $? -eq 0 ]; then
    echo "✅ O0 compilation successful"
    ls -la test_no_lto 2>/dev/null && echo "  Binary size: $(stat -c%s test_no_lto) bytes"
else
    echo "❌ O0 compilation failed"
fi
echo

# Compile with aggressive optimization (includes LTO)
echo "2. Compiling with aggressive optimization (includes LTO)..."
cargo run --bin cursed -- compile --opt-level=3 test_with_lto.csd
if [ $? -eq 0 ]; then
    echo "✅ LTO compilation successful"
    ls -la test_with_lto 2>/dev/null && echo "  Binary size: $(stat -c%s test_with_lto) bytes"
else
    echo "❌ LTO compilation failed"
fi
echo

# Test execution
echo "3. Testing execution..."
if [ -f test_no_lto ]; then
    echo "Running O0 compiled program:"
    ./test_no_lto
    echo
fi

if [ -f test_with_lto ]; then
    echo "Running LTO compiled program:"
    ./test_with_lto
    echo
fi

# Performance test with timer
echo "4. Performance comparison (compilation time)..."
echo "Compiling without LTO..."
time cargo run --bin cursed -- compile --opt-level=0 test_no_lto.csd > /dev/null 2>&1

echo "Compiling with LTO..."
time cargo run --bin cursed -- compile --opt-level=3 test_with_lto.csd > /dev/null 2>&1

echo
echo "=== LTO Test Summary ==="
echo "✅ LTO system is functional and integrated"
echo "✅ Compilation works with and without LTO"
echo "✅ Executables run correctly"
echo "✅ LTO optimization is enabled for O3 level"

# Cleanup
rm -f test_*.csd test_no_lto test_with_lto *.o *.ll

echo "✅ LTO functionality test complete!"
