yeet "testz"
yeet "stringz_enhanced"

fr fr Test the enhanced string module

test_start("Basic String Operations")
assert_eq_string(concat_strings("hello", "world"), "helloworld")
assert_eq_string(concat_three("a", "b", "c"), "abc")
assert_eq_string(repeat_string("x", 3), "xxx")

test_start("String Validation")
assert_true(is_empty_string(""))
assert_false(is_empty_string("hello"))
assert_true(strings_equal("test", "test"))
assert_false(strings_equal("test", "demo"))

test_start("Character Type Validation")
assert_true(is_digit_char("5"))
assert_false(is_digit_char("a"))
assert_true(is_alpha_char("z"))
assert_true(is_alpha_char("Z"))
assert_false(is_alpha_char("5"))
assert_true(is_alphanumeric_char("a"))
assert_true(is_alphanumeric_char("5"))
assert_true(is_whitespace_char(" "))

test_start("Case Conversion")
assert_eq_string(char_to_upper("a"), "A")
assert_eq_string(char_to_lower("Z"), "z")
assert_eq_string(simple_to_upper("hello"), "HELLO")
assert_eq_string(simple_to_lower("WORLD"), "world")

test_start("String Joining")
assert_eq_string(join_two_with_separator("a", "b", "-"), "a-b")
assert_eq_string(join_with_comma("first", "second"), "first, second")
assert_eq_string(join_with_space("hello", "world"), "hello world")

test_start("String Padding")
assert_eq_string(pad_left("hi", 5, " "), "   hi")
assert_eq_string(pad_right("hi", 5, " "), "hi   ")

test_start("String Formatting")
assert_eq_string(format_as_title("Test"), "=== Test ===")
assert_eq_string(format_as_bullet("item"), "• item")
assert_eq_string(format_key_value("name", "value"), "name: value")
assert_eq_string(surround_with_quotes("text"), "\"text\"")
assert_eq_string(surround_with_parens("content"), "(content)")

test_start("String Replacement")
assert_eq_string(simple_replace("hello", "hello", "hi"), "hi")
assert_eq_string(simple_replace("hello world", "hello", "hi"), "hi world")

test_start("String Utilities")
assert_eq_string(reverse_simple("abc"), "cba")
assert_true(is_palindrome_simple("aa"))
assert_eq_string(trim_whitespace_simple(" hello "), "hello")

print_test_summary()
