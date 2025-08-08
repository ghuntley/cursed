// Test case 1: 2 + 3 * 4 should equal 14 (not 20)
sus result1 drip = 2 + 3 * 4
vibez.spill("✅ 2 + 3 * 4 =", result1, "(expected 14)")

// Test case 2: (2 + 3) * 4 should equal 20
sus result2 drip = (2 + 3) * 4
vibez.spill("✅ (2 + 3) * 4 =", result2, "(expected 20)")

// Test case 3: 10 - 6 / 2 should equal 7 (not 2)
sus result3 drip = 10 - 6 / 2
vibez.spill("✅ 10 - 6 / 2 =", result3, "(expected 7)")

// Test case 4: (5 + 3) * 2 should equal 16
sus result4 drip = (5 + 3) * 2
vibez.spill("✅ (5 + 3) * 2 =", result4, "(expected 16)")

// Additional test cases to verify correct precedence
sus result5 drip = 1 + 2 * 3 + 4
vibez.spill("✅ 1 + 2 * 3 + 4 =", result5, "(expected 11)")

sus result6 drip = 24 / 3 * 2
vibez.spill("✅ 24 / 3 * 2 =", result6, "(expected 16)")

sus result7 drip = 2 * 3 + 4 * 5
vibez.spill("✅ 2 * 3 + 4 * 5 =", result7, "(expected 26)")

vibez.spill("🎉 All arithmetic precedence tests completed successfully!")
