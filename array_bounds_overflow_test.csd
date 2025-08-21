# Array Bounds Overflow Test
# Tests positive index overflow bounds checking

yeet "vibez"

vibez.spill("Array Bounds Overflow Test")

# Create small array
sus small_array []drip = [100, 200, 300]
vibez.spill("Array length:", len(small_array))
vibez.spill("Valid indices: [0, 1, 2]")

# Test valid access first
vibez.spill("Valid access: small_array[0] =", small_array[0])
vibez.spill("Valid access: small_array[1] =", small_array[1])
vibez.spill("Valid access: small_array[2] =", small_array[2])

vibez.spill("\nAttempting out-of-bounds access...")
vibez.spill("Trying to access small_array[5] (index 5 >= length 3)")

# This should trigger bounds error
sus overflow_value drip = small_array[5]  # Should trigger bounds error

vibez.spill("❌ Should never reach this line - bounds check failed")
