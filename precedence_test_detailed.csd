fr fr More detailed precedence testing
fr fr Test that "2 + 3 * 4" = 14 (not 20)

sus result1 drip = 2 + 3 * 4
vibez.spill("2 + 3 * 4 =", result1)

sus result2 drip = 10 - 3 * 2
vibez.spill("10 - 3 * 2 =", result2)

sus result3 drip = 6 / 2 + 1
vibez.spill("6 / 2 + 1 =", result3)

sus result4 drip = 2 * 3 + 4 * 5
vibez.spill("2 * 3 + 4 * 5 =", result4)

fr fr Test complex precedence
sus result5 drip = 15 + 5 * 2 - 3
vibez.spill("15 + 5 * 2 - 3 =", result5)

fr fr Expected results:
fr fr result1 should be 14 (multiplication first: 2 + (3 * 4) = 2 + 12 = 14)
fr fr result2 should be 4 (multiplication first: 10 - (3 * 2) = 10 - 6 = 4)
fr fr result3 should be 4 (division first: (6 / 2) + 1 = 3 + 1 = 4)
fr fr result4 should be 26 (both multiplications first: (2 * 3) + (4 * 5) = 6 + 20 = 26)
fr fr result5 should be 22 (multiplication first: 15 + (5 * 2) - 3 = 15 + 10 - 3 = 22)
