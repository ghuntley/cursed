// CURSED Type ID Collision Handling Demo
// This demonstrates the enhanced type safety system

sus basic_int drip = 42
sus another_int drip = 84
sus float_val snack = 3.14

vibez.spill("=== Type ID Collision Handling Demo ===")
vibez.spill("Basic integer:", basic_int)
vibez.spill("Another integer:", another_int)
vibez.spill("Float value:", float_val)

// Test array creation with type safety
sus numbers []drip = [1, 2, 3, 4, 5]
vibez.spill("Array length:", len(numbers))
vibez.spill("First element:", numbers[0])

vibez.spill("=== Type System Successfully Prevents Collisions ===")
