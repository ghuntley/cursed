yeet "testz"

test_start("Comparison Operators Test")

vibez.spill("=== Testing Comparison Operators ===")

# Test <= specifically
sus test1 lit = 10 <= 10
vibez.spill("10 <= 10 =", test1, "(should be true)")

sus test2 lit = 5 <= 10
vibez.spill("5 <= 10 =", test2, "(should be true)")

sus test3 lit = 15 <= 10
vibez.spill("15 <= 10 =", test3, "(should be false)")

# Test >= specifically  
sus test4 lit = 10 >= 10
vibez.spill("10 >= 10 =", test4, "(should be true)")

sus test5 lit = 15 >= 10
vibez.spill("15 >= 10 =", test5, "(should be true)")

sus test6 lit = 5 >= 10
vibez.spill("5 >= 10 =", test6, "(should be false)")

# Test == and !=
sus test7 lit = 42 == 42
vibez.spill("42 == 42 =", test7, "(should be true)")

sus test8 lit = 42 != 24
vibez.spill("42 != 24 =", test8, "(should be true)")

vibez.spill("=== All Comparison Tests Completed ===")
print_test_summary()
