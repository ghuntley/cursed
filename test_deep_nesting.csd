// Test deep nesting for parser memory safety
sus deep_expr drip = ((((((((((((((((((((1 + 2) * 3) - 4) / 2) + 5) * 6) - 7) / 2) + 8) * 9) - 10) / 2) + 11) * 12) - 13) / 2) + 14) * 15) - 16) / 2)

vibez.spill("Deep expression result:", deep_expr)

// Nested function calls
slay nest1(x drip) drip { damn x + 1 }
slay nest2(x drip) drip { damn nest1(x) * 2 }
slay nest3(x drip) drip { damn nest2(x) + 3 }
slay nest4(x drip) drip { damn nest3(x) * 4 }
slay nest5(x drip) drip { damn nest4(x) + 5 }

sus nested_result drip = nest5(nest4(nest3(nest2(nest1(1)))))
vibez.spill("Nested calls result:", nested_result)

// Large array
sus large_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
sus sum drip = 0
sus i drip = 0
bestie (i < len(large_array)) {
    sum = sum + large_array[i]
    i = i + 1
}
vibez.spill("Array sum:", sum)
