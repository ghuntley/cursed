// Test file for import resolution system
yeet "testz"
yeet "mathz"
yeet "stringz"
yeet "arrayz"

// Test that module imports work
test_start("import resolution test")

sus result drip = abs_normie(-5)
assert_eq_int(result, 5)

sus text tea = "hello"
sus length drip = len_str(text)
assert_eq_int(length, 5)

sus arr []drip = [1, 2, 3]
sus arr_len drip = len(arr)
assert_eq_int(arr_len, 3)

print_test_summary()
