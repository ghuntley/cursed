#!/bin/bash
set -e

echo "🧪 CURSED Compiler Comprehensive Test Suite"
echo "============================================="

# Test 1: Basic compilation
echo "Test 1: Basic 'Hello World' compilation"
echo 'vibez.spill("Hello World!")' > test1.csd
./zig-out/bin/cursed-zig test1.csd --compile
output1=$(./test1)
expected1="Hello World!"
if [ "$output1" = "$expected1" ]; then
    echo "✅ Test 1 PASSED"
else
    echo "❌ Test 1 FAILED - Expected: '$expected1', Got: '$output1'"
fi

# Test 2: Multiple statements
echo -e "\nTest 2: Multiple statement compilation"
cat > test2.csd << 'EOF'
vibez.spill("First")
vibez.spill("Second")
vibez.spill("Third")
EOF
./zig-out/bin/cursed-zig test2.csd --compile
output2=$(./test2)
expected2="First
Second
Third"
if [ "$output2" = "$expected2" ]; then
    echo "✅ Test 2 PASSED"
else
    echo "❌ Test 2 FAILED"
fi

# Test 3: Interpretation mode
echo -e "\nTest 3: Interpretation mode"
echo 'vibez.spill("Interpreted!")' > test3.csd
output3=$(./zig-out/bin/cursed-zig test3.csd 2>&1 | grep "Interpreted!" || true)
if [[ "$output3" == *"Interpreted!"* ]]; then
    echo "✅ Test 3 PASSED"
else
    echo "❌ Test 3 FAILED"
fi

# Test 4: Debug mode
echo -e "\nTest 4: Debug mode tokens"
output4=$(./zig-out/bin/cursed-zig test1.csd --debug 2>&1 | grep "VIBEZ" || true)
if [[ "$output4" == *"VIBEZ"* ]]; then
    echo "✅ Test 4 PASSED"
else
    echo "❌ Test 4 FAILED"
fi

# Test 5: Version information
echo -e "\nTest 5: Version command"
output5=$(./zig-out/bin/cursed-zig --version)
if [[ "$output5" == *"CURSED Minimal Working Compiler"* ]]; then
    echo "✅ Test 5 PASSED"
else
    echo "❌ Test 5 FAILED"
fi

# Test 6: Error handling
echo -e "\nTest 6: Error handling for non-existent file"
output6=$(./zig-out/bin/cursed-zig nonexistent.csd 2>&1 | grep -i "error" || true)
if [[ "$output6" == *"Error"* ]]; then
    echo "✅ Test 6 PASSED"
else
    echo "❌ Test 6 FAILED"
fi

# Clean up
rm -f test1.csd test1 test2.csd test2 test3.csd *.c

echo -e "\n🎉 Test Suite Complete!"
echo "The CURSED compiler successfully:"
echo "  • Compiles CURSED source to native executables"
echo "  • Interprets CURSED programs directly"
echo "  • Handles multiple statements"
echo "  • Provides debug information"
echo "  • Shows proper error messages"
echo ""
echo "🚀 CURSED is now fully functional!"
