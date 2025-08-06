#!/bin/bash
set -e

echo "=== CURSED Compilation Test Matrix ==="
echo

# Test 1: Simple compilation
echo "1. Testing simple program compilation..."
./zig-out/bin/cursed compile simple_execution_test.csd -b llvm > /dev/null && echo "✅ Simple LLVM compilation works"
./simple_execution_test > /dev/null && echo "✅ Simple compiled binary works"

# Test 2: Different backends
echo "2. Testing compilation backends..."
./zig-out/bin/cursed compile simple_execution_test.csd -b c > /dev/null && echo "✅ C backend compilation works"
./zig-out/bin/cursed compile simple_execution_test.csd -b script > /dev/null && echo "✅ Script backend compilation works"

# Test 3: Optimization levels
echo "3. Testing optimization levels..."
./zig-out/bin/cursed compile simple_execution_test.csd -b llvm -O 0 > /dev/null && echo "✅ O0 compilation works"
./zig-out/bin/cursed compile simple_execution_test.csd -b llvm -O 1 > /dev/null && echo "✅ O1 compilation works"
./zig-out/bin/cursed compile simple_execution_test.csd -b llvm -O 3 > /dev/null && echo "✅ O3 compilation works"

# Test 4: Custom output
echo "4. Testing custom output..."
./zig-out/bin/cursed compile simple_execution_test.csd -b llvm -o custom_output > /dev/null && echo "✅ Custom output compilation works"
./custom_output > /dev/null && echo "✅ Custom output binary works"

# Test 5: Different input complexities
echo "5. Testing compilation complexity levels..."
echo 'vibez.spill("minimal")' > minimal.csd
./zig-out/bin/cursed compile minimal.csd -b llvm > /dev/null && echo "✅ Minimal program compilation works"

echo 'sus x drip = 42; vibez.spill(x)' > variable.csd
./zig-out/bin/cursed compile variable.csd -b llvm > /dev/null && echo "✅ Variable program compilation works"

echo 'slay add(a drip, b drip) drip { damn a + b } vibez.spill(add(1,2))' > function.csd
./zig-out/bin/cursed compile function.csd -b llvm > /dev/null && echo "✅ Function program compilation works"

echo
echo "=== Compilation test matrix completed ==="
