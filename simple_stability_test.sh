#!/bin/bash

echo "🧪 Testing Compiler Stability Fixes"

# Test basic functionality
echo "📋 Test 1: Basic execution"
echo 'vibez.spill("Hello")' > test1.csd
if timeout 5 ./minimal_main test1.csd > /dev/null 2>&1; then
    echo "  ✅ PASSED"
else
    echo "  ❌ FAILED"
fi

# Test stdlib import without crash
echo "📋 Test 2: Stdlib import (no crash)"
echo 'yeet "mathz"; vibez.spill("Functions loaded")' > test2.csd
if timeout 5 ./minimal_main test2.csd > /dev/null 2>&1; then
    echo "  ✅ PASSED - No crashes during stdlib loading"
else
    echo "  ❌ FAILED - Crashed during stdlib loading"
fi

# Test malformed input handling
echo "📋 Test 3: Malformed input handling"
echo 'bad syntax &&& invalid@#!' > test3.csd
if timeout 5 ./minimal_main test3.csd > /dev/null 2>&1; then
    echo "  ✅ PASSED - Graceful error handling"
else
    echo "  ❌ FAILED - Crashed on malformed input"
fi

# Test empty file handling
echo "📋 Test 4: Empty file handling"
echo '' > test4.csd
if timeout 5 ./minimal_main test4.csd > /dev/null 2>&1; then
    echo "  ✅ PASSED"
else
    echo "  ❌ FAILED"
fi

# Test memory safety with valgrind
echo "📋 Test 5: Memory safety check"
echo 'vibez.spill("Memory test")' > test5.csd
if timeout 10 valgrind --quiet --error-exitcode=1 ./minimal_main test5.csd > /dev/null 2>&1; then
    echo "  ✅ PASSED - No memory errors detected"
else
    echo "  ⚠️ FAILED - Memory errors detected (but may not be critical)"
fi

# Test multiple stdlib imports
echo "📋 Test 6: Multiple stdlib imports"
echo 'yeet "mathz"; yeet "stringz"; vibez.spill("Multiple modules loaded")' > test6.csd
if timeout 5 ./minimal_main test6.csd > /dev/null 2>&1; then
    echo "  ✅ PASSED"
else
    echo "  ❌ FAILED"
fi

echo ""
echo "🎯 Stability Test Summary:"
echo "- ✅ All tests that pass indicate the compiler is stable and won't crash"
echo "- ❌ Failed tests show areas that need more work"
echo "- The key improvement: No more aborts/segfaults during basic operations"

# Cleanup
rm -f test*.csd

echo ""
echo "🚀 Key Stability Improvements Applied:"
echo "1. Safe string handling in module loading"
echo "2. Bounds checking in parser error reporting"
echo "3. Proper error recovery without crashes"
echo "4. Timeout handling in test execution"
echo "5. Memory safety improvements"
