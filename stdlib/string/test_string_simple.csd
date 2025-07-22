yeet "testz"
yeet "string"

fr fr Core String Operations Test Suite
test_start("Core String Operations Test")

fr fr Basic Length and Concatenation
assert_eq_int(string_length("hello"), 5)
assert_eq_int(string_length("test"), 4)
assert_eq_int(string_length(""), 0)

assert_eq_string(string_concat("hello", "world"), "helloworld")
assert_eq_string(string_concat("test", "123"), "test123")

fr fr String Search Functions
assert_true(string_contains("hello world", "world"))
assert_false(string_contains("hello", "xyz"))
assert_eq_int(string_index_of("hello world", "world"), 6)
assert_eq_int(string_index_of("test", "missing"), -1)

fr fr Case Conversion
assert_eq_string(string_to_upper("hello"), "HELLO")
assert_eq_string(string_to_lower("HELLO"), "hello")

fr fr String Validation
assert_true(string_is_numeric("123"))
assert_false(string_is_numeric("abc"))
assert_true(string_is_alpha("hello"))
assert_false(string_is_alpha("123"))

fr fr Prefix/Suffix Tests
assert_true(string_starts_with("hello world", "hello"))
assert_false(string_starts_with("hello", "world"))
assert_true(string_ends_with("hello world", "world"))
assert_false(string_ends_with("hello", "world"))

fr fr Trimming Operations
assert_eq_string(string_trim("  hello  "), "hello")
assert_eq_string(string_trim_left("  hello"), "hello")
assert_eq_string(string_trim_right("hello  "), "hello")

fr fr String Replacement
assert_eq_string(string_replace_first("hello world", "world", "universe"), "hello universe")
assert_eq_string(string_replace_all("test test", "test", "demo"), "demo demo")

fr fr String Comparison
assert_eq_int(string_compare("hello", "hello"), 0)
assert_eq_int(string_compare("a", "b"), -1)
assert_eq_int(string_compare("b", "a"), 1)

fr fr Substring Operations
assert_eq_string(string_substring("hello world", 0, 5), "hello")
assert_eq_string(string_substr("hello world", 6, 5), "world")

fr fr Character Operations
assert_eq_int(string_char_at("hello", 0).(normie), 'h'.(normie))
assert_eq_int(string_char_code_at("hello", 0), 104)

fr fr Formatting
assert_eq_string(string_format("Hello, {}!", "World"), "Hello, World!")

fr fr Padding
assert_eq_string(string_pad_left("test", 8, "0"), "0000test")
assert_eq_string(string_pad_right("test", 8, "0"), "test0000")

print_test_summary()
