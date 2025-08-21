# Array Bounds Edge Cases Test
# Tests various edge cases for array bounds checking

yeet "vibez"

vibez.spill("Array Bounds Edge Cases Test")

# Test 1: Empty array bounds
vibez.spill("\nTest 1: Empty array access")
sus empty_array []drip = []
vibez.spill("Empty array length:", len(empty_array))

# Any access to empty array should fail
vibez.spill("Attempting access to empty_array[0]...")
sus empty_value drip = empty_array[0]  # Should trigger bounds error

vibez.spill("❌ Should never reach this line - empty array bounds check failed")
