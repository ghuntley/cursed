yeet "testz"
yeet "regex"

test_start("Basic Regex Functionality")

# Test basic pattern matching
assert_true(regex.match_pattern("hello", "hello"))
assert_false(regex.match_pattern("hello", "world"))

# Test wildcard matching
assert_true(regex.match_wildcard("hello", "*"))
assert_true(regex.match_wildcard("hello", "h*"))

# Test string utility functions
assert_eq_int(regex.string_length("hello"), 5)
assert_eq_int(regex.string_length(""), 0)

# Test character validation
assert_true(regex.is_word_character("a"))
assert_true(regex.is_digit_character("5"))
assert_false(regex.is_word_character("@"))

# Test unicode classification
assert_true(regex.is_unicode_letter(65))  # 'A'
assert_true(regex.is_unicode_number(48))  # '0'

print_test_summary()
