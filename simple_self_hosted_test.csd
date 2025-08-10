fr fr Simple test of self-hosted built-ins without module imports

fr fr Test basic string operations that should work with pure CURSED
vibez.spill("🧪 Testing Basic Pure CURSED Operations...")

fr fr Test string concatenation (basic operator)
sus concat_result tea = "Hello" + " World"
vibez.spill("String concat result:", concat_result)

fr fr Test array length (built-in len function)
sus test_array []drip = [1, 2, 3, 4, 5]
sus array_length drip = len(test_array)
vibez.spill("Array length:", array_length)

fr fr Test array access
sus element drip = test_array[2]
vibez.spill("Array element at index 2:", element)

fr fr Test basic math operations
sus add_result drip = 15 + 27
sus mult_result drip = 6 * 7
sus sub_result drip = 100 - 58
sus div_result drip = 84 / 2

vibez.spill("Math operations:")
vibez.spill("  15 + 27 =", add_result)
vibez.spill("  6 * 7 =", mult_result)
vibez.spill("  100 - 58 =", sub_result)
vibez.spill("  84 / 2 =", div_result)

fr fr Test string array operations
sus string_array []tea = ["hello", "world", "cursed"]
sus string_array_length drip = len(string_array)
vibez.spill("String array length:", string_array_length)
vibez.spill("First string:", string_array[0])
vibez.spill("Second string:", string_array[1])

fr fr Test boolean operations
sus bool_test1 lit = based
sus bool_test2 lit = cringe
vibez.spill("Boolean test1:", bool_test1)
vibez.spill("Boolean test2:", bool_test2)

fr fr Test conditional logic
ready (bool_test1) {
    vibez.spill("✅ Conditional logic working")
} otherwise {
    vibez.spill("❌ Conditional logic failed")
}

fr fr Test loops
vibez.spill("Testing loop iteration:")
sus i drip = 0
bestie (i < 3) {
    vibez.spill("  Loop iteration:", i)
    i = i + 1
}

fr fr Test string comparison
sus str1 tea = "test"
sus str2 tea = "test"
sus str3 tea = "different"

ready (str1 == str2) {
    vibez.spill("✅ String equality working")
} otherwise {
    vibez.spill("❌ String equality failed")
}

ready (str1 != str3) {
    vibez.spill("✅ String inequality working")
} otherwise {
    vibez.spill("❌ String inequality failed")
}

vibez.spill("")
vibez.spill("🎉 Basic CURSED operations are working!")
vibez.spill("✅ Core language features ready for self-hosting")
