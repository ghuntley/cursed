fr fr Simple generic function test
yeet "testz"

fr fr Generic identity function
slay identity<T>(value T) -> T {
    damn value
}

fr fr Test the identity function with different types
test_start("Generic identity function")
sus int_result = identity(42)
assert_eq_int(int_result, 42)

sus string_result = identity("hello")
assert_eq_string(string_result, "hello")

sus bool_result = identity(based)
assert_true(bool_result)

print_test_summary()
