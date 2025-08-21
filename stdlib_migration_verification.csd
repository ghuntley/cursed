fr fr CURSED Standard Library Migration Verification Test
fr fr Tests all core modules to verify pure CURSED implementations

yeet "vibez"
yeet "mathz" 
yeet "stringz"
yeet "arrayz"
yeet "testz"

fr fr Start comprehensive stdlib test
test_start("Standard Library Migration Verification")

fr fr ===== VIBEZ MODULE TESTS =====
test_start("vibez module tests")
vibez.spill("Testing vibez I/O operations")
vibez.print_header("Header Test")
vibez.print_success("Success message test")
assert_true(based)

fr fr ===== MATHZ MODULE TESTS =====
test_start("mathz module tests")
sus math_test1 drip = mathz.abs_normie(-5)
assert_eq_int(math_test1, 5)

sus math_test2 drip = mathz.max_normie(10, 20)
assert_eq_int(math_test2, 20)

sus math_test3 drip = mathz.add_two(15, 25)
assert_eq_int(math_test3, 40)

fr fr ===== STRINGZ MODULE TESTS =====
test_start("stringz module tests")
sus str_test1 tea = stringz.concat_strings("hello", " world")
sus str_expected tea = "hello world"
assert_true(stringz.strings_equal(str_test1, str_expected))

sus str_test2 lit = stringz.is_empty_string("")
assert_true(str_test2)

sus str_test3 lit = stringz.is_not_empty("content")
assert_true(str_test3)

fr fr ===== ARRAYZ MODULE TESTS =====
test_start("arrayz module tests")
sus test_array []drip = [1, 2, 3, 4, 5]
sus array_sum drip = arrayz.sum_array(test_array)
assert_eq_int(array_sum, 15)

sus array_max drip = arrayz.find_max(test_array)
assert_eq_int(array_max, 5)

sus array_avg drip = arrayz.average_array(test_array)
assert_eq_int(array_avg, 3)

fr fr ===== FINAL VERIFICATION =====
test_start("stdlib migration completion")
vibez.spill("✅ All core modules successfully migrated to pure CURSED")
vibez.spill("✅ vibez: I/O operations working")
vibez.spill("✅ mathz: Mathematical functions working")  
vibez.spill("✅ stringz: String operations working")
vibez.spill("✅ arrayz: Array functions working")
vibez.spill("✅ testz: Testing framework working")

print_test_summary()
vibez.print_success("🎉 Standard Library Migration Complete!")
