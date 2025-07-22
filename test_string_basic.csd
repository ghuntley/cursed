yeet "testz"
yeet "string"

test_start("Basic string operations")

fr fr Test string length
sus len1 normie = string_length("test")
assert_eq_int(len1, 4)

sus len2 normie = string_length("hello")
assert_eq_int(len2, 5)

sus len3 normie = string_length("")
assert_eq_int(len3, 0)

fr fr Test string concatenation
sus concat1 tea = string_concat("hello", " world")
assert_eq_string(concat1, "hello world")

fr fr Test string reversal  
sus rev1 tea = string_reverse("abc")
assert_eq_string(rev1, "cba")

fr fr Test case conversion
sus upper1 tea = string_to_upper("hello")
assert_eq_string(upper1, "HELLO")

sus lower1 tea = string_to_lower("HELLO")
assert_eq_string(lower1, "hello")

print_test_summary()

vibez.spill("Basic string tests complete!")
