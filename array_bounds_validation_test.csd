# Array Bounds Checking Validation Test
# Tests Oracle's Week 2: Memory & Performance array bounds checking implementation

yeet "vibez"
yeet "arrayz"

# Test 1: Valid array access
vibez.spill("Test 1: Valid array bounds access")
sus numbers []drip = [10, 20, 30, 40, 50]
vibez.spill("Array length:", len(numbers))

# Access valid indices
vibez.spill("numbers[0] =", numbers[0])  # Should work: 10
vibez.spill("numbers[2] =", numbers[2])  # Should work: 30  
vibez.spill("numbers[4] =", numbers[4])  # Should work: 50 (last valid index)

vibez.spill("✅ Valid bounds access test completed")

# Test 2: Negative index bounds violation
vibez.spill("\nTest 2: Negative index bounds violation")
vibez.spill("Attempting to access numbers[-1]...")
vibez.spill("This should trigger bounds error:")

# This should trigger immediate bounds error
sus invalid_value drip = numbers[-1]  # Should trigger bounds error and trap

vibez.spill("❌ Should never reach this line - bounds check failed")
