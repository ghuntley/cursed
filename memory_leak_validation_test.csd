yeet "vibez"

// Test 1: Simple arrays and cleanup
sus arr1 []drip = [1, 2, 3, 4, 5]
sus arr2 []drip = [10, 20, 30]
sus arr3 []drip = [100, 200, 300, 400]

vibez.spill("Array test completed")

// Test 2: Function calls with arguments (tests ArrayList cleanup)
slay test_func(a drip, b drip) drip {
    damn a + b
}

sus result1 drip = test_func(5, 10)
sus result2 drip = test_func(arr1[0], arr2[1])
vibez.spill("Function call results:", result1, result2)

// Test 3: Nested expressions (tests error cleanup paths)
sus nested_result drip = test_func(test_func(1, 2), test_func(3, 4))
vibez.spill("Nested result:", nested_result)

// Test 4: String interpolation (tests string ArrayList cleanup)
sus name tea = "CURSED"
sus version drip = 1
vibez.spill("Language", name, "version", version)

vibez.spill("Memory leak test completed successfully!")
