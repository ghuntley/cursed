yeet "testz"
yeet "mathz"

fr fr Test basic mathz functionality after parsing fixes
test_start("Basic Math Operations - Parsing Fixed")

fr fr Test constants access (should work)
assert_true(mathz.PI > 3.14 && mathz.PI < 3.15)
assert_true(mathz.E > 2.71 && mathz.E < 2.72)

fr fr Test function calls with placeholder values
sus add_result meal = mathz.math_add(2.0, 3.0)
assert_true(add_result == 0.0) fr fr Placeholder value

sus abs_result normie = mathz.abs_normie(-42)
assert_eq_int(abs_result, 0) fr fr Placeholder value

sus fact_result normie = mathz.factorial(5)
assert_eq_int(fact_result, 0) fr fr Placeholder value

fr fr Test that the parsing no longer crashes
test_start("Parsing Validation")
sus sqrt_val meal = mathz.sqrt_meal(25.0)
sus sin_val meal = mathz.sin_meal(0.0) 
sus max_val normie = mathz.max_normie(10, 5)

fr fr All should be placeholders but parsed successfully
assert_true(sqrt_val == 0.0)
assert_true(sin_val == 0.0) 
assert_eq_int(max_val, 0)

print_test_summary()
