yeet "testz"

test_start("simple_test")
# Test basic functionality
sus x := 42
assert_eq_int(x, 42)

sus y := "hello"
assert_eq_string(y, "hello")

sus z := based
assert_eq_string(z, based)

print_test_summary()
