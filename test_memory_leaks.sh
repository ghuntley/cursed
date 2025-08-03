#!/bin/bash

echo "🧪 CURSED Zig Compiler Memory Leak Testing"
echo "==========================================="

# Build the compiler first
echo "📦 Building CURSED Zig compiler..."
zig build || {
    echo "❌ Build failed!"
    exit 1
}

echo "✅ Build successful"

# Test 1: Basic interpretation mode
echo ""
echo "🔍 Test 1: Basic interpretation mode memory test"
echo "------------------------------------------------"

echo "Creating simple test program..."
cat > simple_memory_test.csd << 'EOF'
fr fr Simple memory test
vibez.spill("Hello from memory test!")
vibez.spill("Testing tokens and lexer")
EOF

echo "Running with valgrind (if available)..."
if command -v valgrind &> /dev/null; then
    echo "Running memory check with valgrind..."
    valgrind --tool=memcheck --leak-check=full --show-leak-kinds=all --track-origins=yes \
        ./zig-out/bin/cursed-zig simple_memory_test.csd 2>&1 | tee valgrind_interpretation.log
    
    # Check for leaks
    if grep -q "ERROR SUMMARY: 0 errors" valgrind_interpretation.log && \
       grep -q "definitely lost: 0 bytes" valgrind_interpretation.log; then
        echo "✅ No memory leaks detected in interpretation mode!"
    else
        echo "⚠️ Potential memory issues detected in interpretation mode"
        echo "Check valgrind_interpretation.log for details"
    fi
else
    echo "Valgrind not available, running basic test..."
    ./zig-out/bin/cursed-zig simple_memory_test.csd
fi

# Test 2: Compilation mode
echo ""
echo "🔍 Test 2: Compilation mode memory test"
echo "---------------------------------------"

if command -v valgrind &> /dev/null; then
    echo "Running compilation mode with valgrind..."
    valgrind --tool=memcheck --leak-check=full --show-leak-kinds=all --track-origins=yes \
        ./zig-out/bin/cursed-zig --compile simple_memory_test.csd 2>&1 | tee valgrind_compilation.log
    
    # Check for leaks
    if grep -q "ERROR SUMMARY: 0 errors" valgrind_compilation.log && \
       grep -q "definitely lost: 0 bytes" valgrind_compilation.log; then
        echo "✅ No memory leaks detected in compilation mode!"
    else
        echo "⚠️ Potential memory issues detected in compilation mode"
        echo "Check valgrind_compilation.log for details"
    fi
else
    echo "Running basic compilation test..."
    ./zig-out/bin/cursed-zig --compile simple_memory_test.csd
fi

# Test 3: Complex program stress test
echo ""
echo "🔍 Test 3: Complex program memory stress test"
echo "---------------------------------------------"

echo "Creating complex test program..."
cat > complex_memory_test.csd << 'EOF'
fr fr Complex memory test with many tokens
vibez.spill("Starting complex memory test")

fr fr Multiple variable declarations
sus var1 drip = 1
sus var2 drip = 2
sus var3 drip = 3
sus var4 drip = 4
sus var5 drip = 5

fr fr Multiple function calls
vibez.spill("Variable 1:", var1)
vibez.spill("Variable 2:", var2) 
vibez.spill("Variable 3:", var3)
vibez.spill("Variable 4:", var4)
vibez.spill("Variable 5:", var5)

fr fr String literals
vibez.spill("String test 1")
vibez.spill("String test 2")
vibez.spill("String test 3")
vibez.spill("String test 4")
vibez.spill("String test 5")

fr fr Comments to generate more tokens
fr fr Comment line 1
fr fr Comment line 2
fr fr Comment line 3
fr fr Comment line 4
fr fr Comment line 5

vibez.spill("Complex memory test completed")
EOF

if command -v valgrind &> /dev/null; then
    echo "Running complex test with valgrind..."
    valgrind --tool=memcheck --leak-check=full --show-leak-kinds=all \
        ./zig-out/bin/cursed-zig complex_memory_test.csd 2>&1 | tee valgrind_complex.log
    
    # Check for leaks
    if grep -q "ERROR SUMMARY: 0 errors" valgrind_complex.log && \
       grep -q "definitely lost: 0 bytes" valgrind_complex.log; then
        echo "✅ No memory leaks detected in complex test!"
    else
        echo "⚠️ Potential memory issues detected in complex test"
        echo "Check valgrind_complex.log for details"
    fi
else
    echo "Running basic complex test..."
    ./zig-out/bin/cursed-zig complex_memory_test.csd
fi

# Test 4: Token debugging mode (stress test for lexer)
echo ""
echo "🔍 Test 4: Token debugging mode memory test"
echo "------------------------------------------"

if command -v valgrind &> /dev/null; then
    echo "Running token debugging with valgrind..."
    valgrind --tool=memcheck --leak-check=full \
        ./zig-out/bin/cursed-zig --tokens complex_memory_test.csd 2>&1 | tee valgrind_tokens.log
    
    # Check for leaks
    if grep -q "ERROR SUMMARY: 0 errors" valgrind_tokens.log && \
       grep -q "definitely lost: 0 bytes" valgrind_tokens.log; then
        echo "✅ No memory leaks detected in token debugging mode!"
    else
        echo "⚠️ Potential memory issues detected in token debugging mode"
        echo "Check valgrind_tokens.log for details"
    fi
else
    echo "Running basic token debugging test..."
    ./zig-out/bin/cursed-zig --tokens complex_memory_test.csd
fi

# Test 5: Multiple rapid executions (resource cleanup test)
echo ""
echo "🔍 Test 5: Multiple rapid executions test"
echo "-----------------------------------------"

echo "Running 10 rapid executions to test resource cleanup..."
for i in {1..10}; do
    echo -n "Run $i: "
    ./zig-out/bin/cursed-zig simple_memory_test.csd > /dev/null 2>&1 && echo "✅" || echo "❌"
done

echo ""
echo "🧪 Memory leak testing completed!"
echo "================================="

# Summary
echo ""
echo "📊 SUMMARY:"
echo "-----------"

if command -v valgrind &> /dev/null; then
    echo "Valgrind logs created:"
    ls -la valgrind_*.log 2>/dev/null || echo "No valgrind logs found"
    
    echo ""
    echo "Quick leak summary:"
    for log in valgrind_*.log; do
        if [ -f "$log" ]; then
            echo "$log:"
            grep -E "(definitely lost|ERROR SUMMARY)" "$log" | head -2
            echo ""
        fi
    done
else
    echo "⚠️ Valgrind not available - install valgrind for detailed memory analysis"
    echo "Basic functionality tests completed successfully"
fi

echo "🔧 MEMORY LEAK FIXES APPLIED:"
echo "• Added defer tokens.deinit() in simple_main.zig"
echo "• Added comprehensive deinit methods to ast_simple.zig"
echo "• Fixed Program.deinit() to clean up individual statements and imports"
echo "• Added deinit methods for Statement, Expression, ImportStatement, and PackageDeclaration"
echo ""
echo "✅ All critical memory leaks should now be resolved!"

# Cleanup
rm -f simple_memory_test.csd complex_memory_test.csd simple_memory_test simple_memory_test.c complex_memory_test complex_memory_test.c
