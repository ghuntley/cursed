#!/bin/bash

echo "🧪 Memory Safety Validation Tests"
echo "================================="

# Test 1: Basic variable operations
echo "Test 1: Basic variable operations"
echo 'sus x drip = 42; vibez.spill(x)' > test1.csd
valgrind --tool=memcheck --leak-check=full --error-exitcode=1 ./zig-out/bin/cursed test1.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ PASS: No memory errors or leaks"
else
    echo "❌ FAIL: Memory errors detected"
fi

# Test 2: Complex arithmetic expressions 
echo "Test 2: Complex arithmetic expressions"
echo 'sus result drip = (5 + 3) * 2 - 4 / 2; vibez.spill(result)' > test2.csd
valgrind --tool=memcheck --leak-check=full --error-exitcode=1 ./zig-out/bin/cursed test2.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ PASS: No memory errors or leaks"
else
    echo "❌ FAIL: Memory errors detected"
fi

# Test 3: String handling
echo "Test 3: String handling"
echo 'sus text tea = "Hello World"; vibez.spill(text)' > test3.csd
valgrind --tool=memcheck --leak-check=full --error-exitcode=1 ./zig-out/bin/cursed test3.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ PASS: No memory errors or leaks"
else
    echo "❌ FAIL: Memory errors detected"
fi

# Test 4: Function calls with parameters
echo "Test 4: Function calls with parameters"
echo 'slay add(a drip, b drip) drip { damn a + b }
sus result drip = add(10, 20); vibez.spill(result)' > test4.csd
valgrind --tool=memcheck --leak-check=full --error-exitcode=1 ./zig-out/bin/cursed test4.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ PASS: No memory errors or leaks"
else
    echo "❌ FAIL: Memory errors detected"
fi

# Test 5: Multiple variable assignments
echo "Test 5: Multiple variable assignments"
echo 'sus a drip = 1; sus b drip = 2; sus c drip = a + b; vibez.spill(c)' > test5.csd
valgrind --tool=memcheck --leak-check=full --error-exitcode=1 ./zig-out/bin/cursed test5.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ PASS: No memory errors or leaks"
else
    echo "❌ FAIL: Memory errors detected"
fi

# Test 6: Standard library usage
echo "Test 6: Standard library usage"
echo 'yeet "testz"; test_start("memory test"); assert_true(based); print_test_summary()' > test6.csd
valgrind --tool=memcheck --leak-check=full --error-exitcode=1 ./zig-out/bin/cursed test6.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ PASS: No memory errors or leaks"
else
    echo "❌ FAIL: Memory errors detected"
fi

# Cleanup
rm -f test*.csd

echo ""
echo "🎉 Memory validation complete!"
echo "All tests checked for:"
echo "  - Memory leaks"
echo "  - Unaddressable byte access"
echo "  - Use of uninitialized values"
echo "  - Invalid memory operations"
