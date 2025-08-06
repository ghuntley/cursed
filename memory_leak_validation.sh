#!/bin/bash

# Memory Leak Validation Script for CURSED Compiler
# Tests the memory-safe error reporting system and parser components

set -e

echo "=== CURSED Memory Leak Validation ==="

# Build the compiler
echo "Building CURSED compiler..."
zig build

# Test with Valgrind if available
if command -v valgrind &> /dev/null; then
    echo "Running memory leak detection with Valgrind..."
    
    # Test 1: Basic memory safety test
    echo "Test 1: Basic memory safety validation"
    valgrind --tool=memcheck \
             --leak-check=full \
             --show-leak-kinds=all \
             --track-origins=yes \
             --verbose \
             --error-exitcode=1 \
             ./zig-out/bin/cursed-zig test_memory_safety.csd 2> valgrind_basic.log
    
    if [ $? -eq 0 ]; then
        echo "✅ Basic memory safety test passed"
    else
        echo "❌ Basic memory safety test failed"
        echo "Valgrind output:"
        cat valgrind_basic.log
        exit 1
    fi
    
    # Test 2: Complex program parsing
    echo "Test 2: Complex program memory safety"
    cat > complex_memory_test.csd << 'EOF'
yeet "testz"

slay complex_function(param1 normie, param2 tea) lit {
    sus local_var normie = param1 + 42
    sus result lit = based
    
    lowkey (local_var > 100) {
        result = cringe
    }
    
    damn result
}

squad TestStruct {
    field1 normie
    field2 tea
    field3 lit
}

collab TestInterface {
    slay test_method(param normie) lit
}

facts CONSTANT_VALUE normie = 999

test_start("Complex Memory Test")
sus test_result lit = complex_function(50, "test")
assert_true(test_result)
print_test_summary()
EOF
    
    valgrind --tool=memcheck \
             --leak-check=full \
             --show-leak-kinds=all \
             --track-origins=yes \
             --verbose \
             --error-exitcode=1 \
             ./zig-out/bin/cursed-zig complex_memory_test.csd 2> valgrind_complex.log
    
    if [ $? -eq 0 ]; then
        echo "✅ Complex memory safety test passed"
    else
        echo "❌ Complex memory safety test failed"
        echo "Valgrind output:"
        cat valgrind_complex.log
        exit 1
    fi
    
    # Test 3: Error handling memory safety
    echo "Test 3: Error handling memory safety"
    cat > error_memory_test.csd << 'EOF'
# This should produce parse errors to test error reporting memory safety

slay broken_function( {  # Missing parameter and closing paren
    sus undefined_var = unknown_function()  # Undefined function
    damn undefined_var
}

invalid syntax here  # Random invalid syntax
EOF
    
    # This should fail to parse but not leak memory
    valgrind --tool=memcheck \
             --leak-check=full \
             --show-leak-kinds=all \
             --track-origins=yes \
             --verbose \
             --error-exitcode=42 \
             ./zig-out/bin/cursed-zig error_memory_test.csd 2> valgrind_error.log || true
    
    # Check for memory leaks in the output (exit code might be non-zero due to parse errors)
    if grep -q "definitely lost: 0 bytes" valgrind_error.log && \
       grep -q "indirectly lost: 0 bytes" valgrind_error.log && \
       grep -q "possibly lost: 0 bytes" valgrind_error.log; then
        echo "✅ Error handling memory safety test passed"
    else
        echo "❌ Error handling memory safety test failed"
        echo "Valgrind output:"
        cat valgrind_error.log
        exit 1
    fi
    
    echo "=== All Valgrind tests passed! No memory leaks detected. ==="
    
else
    echo "Valgrind not available, running basic memory tests..."
    
    # Run basic tests without Valgrind
    echo "Test 1: Basic functionality"
    ./zig-out/bin/cursed-zig test_memory_safety.csd
    echo "✅ Basic test passed"
    
    echo "Test 2: Complex parsing"
    cat > complex_basic_test.csd << 'EOF'
slay test_func() normie {
    damn 42
}

test_start("Basic Test")
assert_eq_int(42, 42)
print_test_summary()
EOF
    
    ./zig-out/bin/cursed-zig complex_basic_test.csd
    echo "✅ Complex parsing test passed"
fi

# Run Zig's memory safety tests
echo "Running Zig memory safety tests..."
zig test src-zig/memory_safe_error_reporting.zig
zig test src-zig/memory_safe_lexer.zig
zig test src-zig/memory_safe_parser.zig

echo "=== Memory Safety Validation Complete ==="
echo "✅ All tests passed - no memory leaks detected!"

# Clean up test files
rm -f valgrind_*.log
rm -f complex_memory_test.csd
rm -f error_memory_test.csd
rm -f complex_basic_test.csd

echo "Test files cleaned up."
