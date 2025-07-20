yeet "testz"
yeet "string"

# Comprehensive Unicode String Operations Test Suite
test_start("Unicode String Operations Integration Test")

# Basic String Length Tests
assert_eq_int(string_length(""), 0)
assert_eq_int(string_length("hello"), 5)
assert_eq_int(string_length("hello world"), 11)
assert_eq_int(string_length("programming"), 11)

# String Concatenation Tests
assert_eq_string(string_concat("hello", " world"), "hello world")
assert_eq_string(string_concat("", "test"), "test")
assert_eq_string(string_concat("test", ""), "test")
assert_eq_string(string_concat("", ""), "")

# String Reversal Tests  
assert_eq_string(string_reverse("abc"), "cba")
assert_eq_string(string_reverse("hello"), "olleh")
assert_eq_string(string_reverse("test"), "tset")

# Case Conversion Tests
assert_eq_string(string_to_upper("hello"), "HELLO")
assert_eq_string(string_to_upper("test"), "TEST")
assert_eq_string(string_to_upper("world"), "WORLD")
assert_eq_string(string_to_lower("HELLO"), "hello")
assert_eq_string(string_to_lower("TEST"), "test")
assert_eq_string(string_to_lower("Hello"), "hello")

# String Search and Contains Tests
assert_true(string_contains("hello world", "world"))
assert_true(string_contains("hello world", "hello"))
assert_true(string_contains("programming", "gram"))
assert_false(string_contains("hello", "xyz"))
assert_true(string_contains("hello123", "123"))

# String Index Operations
assert_eq_int(string_index_of("hello world", "world"), 6)
assert_eq_int(string_index_of("programming", "gram"), 3)
assert_eq_int(string_index_of("hello world", "hello"), 0)
assert_eq_int(string_index_of("test", "xyz"), -1)

# String Prefix and Suffix Tests
assert_true(string_starts_with("hello world", "hello"))
assert_true(string_starts_with("programming", "prog"))
assert_false(string_starts_with("hello", "world"))
assert_true(string_ends_with("hello world", "world"))
assert_true(string_ends_with("programming", "ming"))
assert_false(string_ends_with("hello", "world"))

# String Validation Tests
assert_true(string_is_numeric("123"))
assert_true(string_is_numeric("42"))
assert_true(string_is_numeric("0"))
assert_false(string_is_numeric("abc"))
assert_false(string_is_numeric("12a3"))
assert_false(string_is_numeric("hello123"))

assert_true(string_is_alpha("hello"))
assert_true(string_is_alpha("ABC"))
assert_true(string_is_alpha("test"))
assert_false(string_is_alpha("123"))
assert_false(string_is_alpha("hello123"))

assert_true(string_is_alphanumeric("hello123"))
assert_true(string_is_alphanumeric("test456"))
assert_true(string_is_alphanumeric("hello"))
assert_true(string_is_alphanumeric("123"))
assert_false(string_is_alphanumeric("hello!"))

# String Trimming Tests
assert_eq_string(string_trim("  hello  "), "hello")
assert_eq_string(string_trim(" world "), "world")
assert_eq_string(string_trim("hello"), "hello")
assert_eq_string(string_trim_left("  hello"), "hello")
assert_eq_string(string_trim_left(" world"), "world")
assert_eq_string(string_trim_right("hello  "), "hello")
assert_eq_string(string_trim_right("world "), "world")

# String Replacement Tests
assert_eq_string(string_replace_first("hello world", "world", "universe"), "hello universe")
assert_eq_string(string_replace_first("test string test", "test", "demo"), "demo string test")
assert_eq_string(string_replace_all("hello world hello", "hello", "hi"), "hi world hi")
assert_eq_string(string_replace_all("test string test", "test", "demo"), "demo string demo")

# String Comparison Tests
assert_eq_int(string_compare("hello", "hello"), 0)
assert_eq_int(string_compare("a", "b"), -1)
assert_eq_int(string_compare("b", "a"), 1)
assert_eq_int(string_compare_ignore_case("Hello", "hello"), 0)
assert_eq_int(string_compare_ignore_case("A", "b"), -1)

# String Substring Operations
assert_eq_string(string_substring("hello world", 0, 5), "hello")
assert_eq_string(string_substring("hello world", 6, 11), "world")
assert_eq_string(string_substr("hello world", 0, 5), "hello")
assert_eq_string(string_substr("hello world", 6, 5), "world")

# String Formatting Tests
assert_eq_string(string_format("Hello, {}!", "World"), "Hello, World!")
assert_eq_string(string_format_three("{} + {} = {}", "2", "3", "5"), "2 + 3 = 5")

# String Padding Tests
assert_eq_string(string_pad_left("test", 8, "0"), "0000test")
assert_eq_string(string_pad_left("hello", 10, " "), "     hello")
assert_eq_string(string_pad_right("test", 8, "0"), "test0000")
assert_eq_string(string_pad_right("hello", 10, " "), "hello     ")

# Unicode Character Tests
assert_eq_int(string_char_at("hello", 0).(normie), 'h'.(normie))
assert_eq_int(string_char_at("hello", 1).(normie), 'e'.(normie))
assert_eq_int(string_char_at("test", 0).(normie), 't'.(normie))
assert_eq_int(string_char_code_at("hello", 0), 104)
assert_eq_int(string_char_code_at("hello", 1), 101)

# Edge Cases and Error Conditions
assert_eq_int(string_length(""), 0)
assert_eq_string(string_concat("", ""), "")
assert_false(string_contains("", "test"))
assert_eq_int(string_index_of("", "test"), -1)
assert_false(string_starts_with("", "test"))
assert_false(string_ends_with("", "test"))

# Complex Integration Tests
sus complex_string tea = string_concat("Hello", " ")
complex_string = string_concat(complex_string, "World")
assert_eq_string(complex_string, "Hello World")

sus processed_string tea = string_to_upper(complex_string)
assert_eq_string(processed_string, "HELLO WORLD")

processed_string = string_replace_all(processed_string, "HELLO", "HI")
assert_eq_string(processed_string, "HI WORLD")

assert_true(string_contains(processed_string, "WORLD"))
assert_eq_int(string_index_of(processed_string, "WORLD"), 3)

print_test_summary()
