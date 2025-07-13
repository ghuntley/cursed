yeet "testz"
yeet "regex"

test_start("Simple Regex Test")
assert_true(regex.match_pattern("hello", "hello"))
assert_false(regex.match_pattern("hello", "world"))
print_test_summary()
