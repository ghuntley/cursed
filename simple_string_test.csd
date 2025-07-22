fr fr Simple test for string_simple module improvements
yeet "testz"
yeet "string_simple"

test_start("string concatenation test")
result := string_concat("hello", " world")
assert_eq_string(result, "hello world")
print_test_summary()

test_start("string equality test")
assert_true(string_equal("test", "test"))
assert_false(string_equal("test", "different"))
print_test_summary()

test_start("string formatting test")
formatted := string_format_bool(based)
assert_eq_string(formatted, "true")
print_test_summary()

vibez.spill("✅ String_simple module enhanced successfully!")
