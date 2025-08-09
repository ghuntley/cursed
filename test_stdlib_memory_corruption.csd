# Comprehensive test for stdlib memory corruption issues
# This test loads multiple modules in complex patterns to trigger memory issues

# Test 1: Basic single module import
yeet "mathz"
sus result1 drip = abs_normie(-10)
vibez.spill("Test 1 - Math result:", result1)

# Test 2: Multiple module imports in sequence
yeet "stringz"
yeet "arrayz" 
yeet "cryptz"

# Test 3: Module usage after import
sus arr []drip = [1, 2, 3, 4, 5]
sus array_len drip = len(arr)
vibez.spill("Test 3 - Array length:", array_len)

# Test 4: String operations
sus text tea = "Hello World"
sus text_length drip = len(text)
vibez.spill("Test 4 - String length:", text_length)

# Test 5: Complex nested operations
sus nested_result drip = add_two(multiply_two(5, 3), abs_normie(-7))
vibez.spill("Test 5 - Nested result:", nested_result)

# Test 6: Multiple function calls
sus math1 drip = max_normie(10, 20)
sus math2 drip = min_normie(30, 15)
sus math3 drip = power_int(2, 3)
vibez.spill("Test 6 - Math operations:", math1, math2, math3)

vibez.spill("All tests completed successfully!")
