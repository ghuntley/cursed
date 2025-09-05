yeet "testz"
yeet "stringz"

fr fr Test basic string operations that are known to work
test_start("Basic Contains")
assert_true(Contains("hello world", "world"))
assert_false(Contains("hello world", "xyz"))

test_start("Basic ToLower")
assert_eq_string(ToLower("HELLO"), "hello")

test_start("Basic ToUpper")
assert_eq_string(ToUpper("hello"), "HELLO")

test_start("Basic Length")
assert_eq_int(Length("hello"), 5)

test_start("Basic HasPrefix")
assert_true(HasPrefix("hello world", "hello"))
assert_false(HasPrefix("hello world", "world"))

test_start("Basic HasSuffix")
assert_true(HasSuffix("hello world", "world"))
assert_false(HasSuffix("hello world", "hello"))

test_start("Basic Replace")
assert_eq_string(Replace("hello world", "world", "universe"), "hello universe")

test_start("Basic Reverse")
assert_eq_string(Reverse("hello"), "olleh")

test_start("Basic validation")
assert_true(IsAlpha("hello"))
assert_false(IsAlpha("hello123"))

assert_true(IsNumeric("12345"))
assert_false(IsNumeric("123a"))

print_test_summary()
