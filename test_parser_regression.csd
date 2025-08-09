# Test specific patterns that might cause parser regression

# Test 1: Multiple imports with commas (should fail if regression exists)
yeet "mathz", "stringz"

# Test 2: Import with space in module name (edge case)
yeet "test module"

# Test 3: Import with unusual characters
yeet "test-module_v2"

# Test 4: Empty import
yeet ""

# Test 5: Import without quotes (should error)
yeet mathz

vibez.spill("Parser regression test completed")
