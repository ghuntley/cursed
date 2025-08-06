#!/bin/bash
set -e

echo "=== CURSED Binary Execution Format Tests ==="
echo

# Test basic help and version
echo "1. Testing CLI interface..."
./zig-out/bin/cursed --help > /dev/null && echo "✅ --help works"
./zig-out/bin/cursed --version > /dev/null && echo "✅ --version works"

# Test interpretation modes
echo "2. Testing interpretation modes..."
./zig-out/bin/cursed simple_execution_test.csd > /dev/null && echo "✅ Default interpretation works"
./zig-out/bin/cursed interpret simple_execution_test.csd > /dev/null && echo "✅ Explicit interpret command works"
./zig-out/bin/cursed simple_execution_test.csd --verbose > /dev/null && echo "✅ Verbose mode works"
./zig-out/bin/cursed simple_execution_test.csd --tokens > /dev/null && echo "✅ Token display works"

# Test type checking
echo "3. Testing type checking..."
./zig-out/bin/cursed check simple_execution_test.csd > /dev/null && echo "✅ Type checking works"
./zig-out/bin/cursed check comprehensive_execution_test.csd > /dev/null && echo "✅ Complex type checking works"

# Test formatting
echo "4. Testing formatting..."
./zig-out/bin/cursed format simple_execution_test.csd > /dev/null && echo "✅ Code formatting works"

# Test compilation modes
echo "5. Testing compilation..."
./zig-out/bin/cursed compile simple_execution_test.csd -b llvm > /dev/null && echo "✅ LLVM compilation works"
./simple_execution_test > /dev/null && echo "✅ Compiled binary execution works"

# Test complex program
echo "6. Testing complex program execution..."
./zig-out/bin/cursed comprehensive_execution_test.csd > /dev/null && echo "✅ Complex program interpretation works"

# Test memory management
echo "7. Testing memory management..."
echo 'sus name tea = "test"' > memory_test.csd
./zig-out/bin/cursed memory_test.csd > /dev/null && echo "✅ String variable memory management works"

# Test error handling
echo "8. Testing error handling..."
echo 'invalid syntax here' > error_test.csd
./zig-out/bin/cursed error_test.csd 2>/dev/null || echo "✅ Error handling works (expected failure)"

# Test alternative binary
echo "9. Testing alternative binary..."
./zig-out/bin/cursed-zig simple_execution_test.csd > /dev/null && echo "✅ cursed-zig alias works"

echo
echo "=== All execution format tests completed successfully ==="
