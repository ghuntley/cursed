yeet "testz"

test_start("Arithmetic Expression Precedence Tests")

// Test case 1: 2 + 3 * 4 should equal 14 (not 20)
sus result1 drip = 2 + 3 * 4
assert_eq_int(result1, 14)
vibez.spill("✅ 2 + 3 * 4 =", result1, "(expected 14)")

// Test case 2: (2 + 3) * 4 should equal 20
sus result2 drip = (2 + 3) * 4
assert_eq_int(result2, 20)
vibez.spill("✅ (2 + 3) * 4 =", result2, "(expected 20)")

// Test case 3: 10 - 6 / 2 should equal 7 (not 2)
sus result3 drip = 10 - 6 / 2
assert_eq_int(result3, 7)
vibez.spill("✅ 10 - 6 / 2 =", result3, "(expected 7)")

// Test case 4: (5 + 3) * 2 should equal 16
sus result4 drip = (5 + 3) * 2
assert_eq_int(result4, 16)
vibez.spill("✅ (5 + 3) * 2 =", result4, "(expected 16)")

// Additional test cases to verify correct precedence
sus result5 drip = 1 + 2 * 3 + 4
assert_eq_int(result5, 11)
vibez.spill("✅ 1 + 2 * 3 + 4 =", result5, "(expected 11)")

sus result6 drip = 24 / 3 * 2
assert_eq_int(result6, 16)
vibez.spill("✅ 24 / 3 * 2 =", result6, "(expected 16)")

sus result7 drip = 2 * 3 + 4 * 5
assert_eq_int(result7, 26)
vibez.spill("✅ 2 * 3 + 4 * 5 =", result7, "(expected 26)")

print_test_summary()
