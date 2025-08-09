# Final comprehensive import validation test

# Test all major stdlib modules with comma-separated imports
yeet "mathz", "stringz", "arrayz", "testz", "vibez"

# Validate all modules are working
test_start("import_validation")

# Math module test
sus math_result drip = abs_normie(-100)
assert_eq_int(math_result, 100)
vibez.spill("✅ Math module working:", math_result)

# String module test  
sus str_len drip = string_length("CURSED")
assert_eq_int(str_len, 6)
vibez.spill("✅ String module working:", str_len)

# Array module test
sus test_arr []drip = [10, 20, 30]
sus arr_sum drip = sum_array(test_arr)
assert_eq_int(arr_sum, 60)
vibez.spill("✅ Array module working:", arr_sum)

# Testing framework validation
assert_true(based)
assert_false(cringe)

# Final test summary
print_test_summary()
vibez.spill("🎉 All stdlib import functionality validated!")
