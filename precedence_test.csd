fr fr Test mathematical expression precedence
fr fr "2 + 3 * 4" should equal 14, not 20

vibez.spill("Testing operator precedence...")
vibez.spill("2 + 3 * 4 =", 2 + 3 * 4)
vibez.spill("Expected: 14")

fr fr More precedence tests
vibez.spill("10 - 3 * 2 =", 10 - 3 * 2)
vibez.spill("Expected: 4")

vibez.spill("6 / 2 + 1 =", 6 / 2 + 1)
vibez.spill("Expected: 4")

vibez.spill("2 * 3 + 4 * 5 =", 2 * 3 + 4 * 5)
vibez.spill("Expected: 26")

vibez.spill("15 + 5 * 2 - 3 =", 15 + 5 * 2 - 3)
vibez.spill("Expected: 22")
