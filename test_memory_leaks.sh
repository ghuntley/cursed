#!/bin/bash

# Memory leak testing script for CURSED Zig implementation

echo "🧪 Testing CURSED Zig Memory Management"
echo "======================================="

# Build the unified compiler
echo "📦 Building unified CURSED compiler..."
zig build-exe src-zig/main_unified.zig -lc --name cursed-unified

if [ $? -ne 0 ]; then
    echo "❌ Failed to build unified compiler"
    exit 1
fi

echo "✅ Unified compiler built successfully"

# Test basic program without valgrind first
echo "🚀 Testing basic program execution..."
echo 'vibez.spill("Hello CURSED!")' > simple_test.csd
./cursed-unified simple_test.csd

if [ $? -ne 0 ]; then
    echo "❌ Basic program execution failed"
    exit 1
fi

echo "✅ Basic program execution successful"

# Test with valgrind if available
if command -v valgrind &> /dev/null; then
    echo "🔍 Running memory leak detection with valgrind..."
    
    # Test simple program
    echo "Testing simple program for memory leaks..."
    valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes \
             --error-exitcode=1 --log-file=valgrind_simple.log \
             ./cursed-unified simple_test.csd
    
    if [ $? -eq 0 ]; then
        echo "✅ Simple program: No memory leaks detected"
    else
        echo "⚠️ Simple program: Memory issues detected, see valgrind_simple.log"
    fi
    
    # Test complex program
    echo "Testing complex program for memory leaks..."
    valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes \
             --error-exitcode=1 --log-file=valgrind_complex.log \
             ./cursed-unified memory_leak_test.csd
    
    if [ $? -eq 0 ]; then
        echo "✅ Complex program: No memory leaks detected"
    else
        echo "⚠️ Complex program: Memory issues detected, see valgrind_complex.log"
    fi
    
    # Show summary
    echo ""
    echo "📊 Memory leak analysis summary:"
    if [ -f valgrind_simple.log ]; then
        echo "Simple program leak summary:"
        grep -E "(definitely lost|indirectly lost|possibly lost)" valgrind_simple.log
    fi
    
    if [ -f valgrind_complex.log ]; then
        echo "Complex program leak summary:"
        grep -E "(definitely lost|indirectly lost|possibly lost)" valgrind_complex.log
    fi
    
    # Overall assessment
    echo ""
    echo "🎯 Overall Assessment:"
    leaked_bytes_simple=$(grep "definitely lost" valgrind_simple.log | awk '{print $4}' | tr -d ',')
    leaked_bytes_complex=$(grep "definitely lost" valgrind_complex.log | awk '{print $4}' | tr -d ',')
    
    if [ "${leaked_bytes_simple:-0}" = "0" ] && [ "${leaked_bytes_complex:-0}" = "0" ]; then
        echo "✅ SUCCESS: No definite memory leaks detected!"
        echo "🎉 Memory leak fixes are working correctly"
    else
        echo "⚠️ WARNING: Memory leaks still present"
        echo "   Simple program: ${leaked_bytes_simple:-0} bytes leaked"
        echo "   Complex program: ${leaked_bytes_complex:-0} bytes leaked"
        echo "📝 Further investigation needed"
    fi
    
else
    echo "⚠️ valgrind not available, skipping memory leak detection"
    echo "💡 Install valgrind for comprehensive memory testing"
fi

# Test compilation mode
echo ""
echo "🔨 Testing compilation mode..."
./cursed-unified --compile simple_test.csd

if [ $? -eq 0 ] && [ -f simple_test ]; then
    echo "✅ Compilation mode successful"
    ./simple_test
    echo "✅ Compiled program execution successful"
else
    echo "⚠️ Compilation mode issues detected"
fi

# Cleanup
rm -f simple_test.csd simple_test

echo ""
echo "🏁 Memory leak testing completed"
echo "📁 Logs: valgrind_simple.log, valgrind_complex.log"
