fr fr CURSED Stable Compiler Test Suite

fr fr Variable declarations
sus x drip = 42
sus name tea = "CURSED"
sus flag lit = based
sus flag2 lit = cringe

fr fr Output tests
vibez.spill("Variables:", x, name, flag)

fr fr Arithmetic
sus result drip = x * 2 + 5
vibez.spill("42 * 2 + 5 =", result)

fr fr Comparisons  
ready (x > 40) { vibez.spill("x is greater than 40") }
ready (x == 42) { vibez.spill("x equals 42") }

fr fr Arrays
sus numbers []drip = [1, 2, 3, 4, 5]
vibez.spill("Numbers:", numbers)
vibez.spill("Array length:", len(numbers))

fr fr Booleans
ready (flag) { vibez.spill("Flag is based") }
ready (flag2) { vibez.spill("This should not print") }

vibez.spill("Test complete!")
