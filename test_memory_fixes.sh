#!/bin/bash

echo "🔧 Testing memory leak fixes in Zig lexer..."

# Create test files
echo 'vibez.spill("Hello World")' > simple_test.csd
echo 'sus x drip = 42; vibez.spill("Number:", x)' > variable_test.csd
echo 'fr fr Comment test
slay function() { damn "ok" }
vibez.spill(function())' > complex_test.csd

echo "✅ Created test files"

# Test unified compiler
echo "🧪 Testing cursed-unified compiler..."
zig build-exe src-zig/main_unified.zig -lc --name cursed-unified-test
valgrind --tool=memcheck --leak-check=full --error-exitcode=1 ./cursed-unified-test simple_test.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ cursed-unified: No memory leaks"
else
    echo "❌ cursed-unified: Memory leaks detected"
fi

# Test zig build system
echo "🧪 Testing zig build system..."
zig build
valgrind --tool=memcheck --leak-check=full --error-exitcode=1 ./zig-out/bin/cursed-zig variable_test.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ zig build: No memory leaks"
else
    echo "❌ zig build: Memory leaks detected"
fi

# Test complex program
echo "🧪 Testing complex program with comments and functions..."
valgrind --tool=memcheck --leak-check=full --error-exitcode=1 ./cursed-unified-test complex_test.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Complex program: No memory leaks"
else
    echo "❌ Complex program: Memory leaks detected"
fi

# Cleanup
rm -f simple_test.csd variable_test.csd complex_test.csd cursed-unified-test

echo "🎉 Memory leak testing completed"
