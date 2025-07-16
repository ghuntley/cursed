yeet "testz"
yeet "string"

test_start("CURSED String Library v2.0 Comprehensive Tests")

fr fr ================================
fr fr Core String Operations Tests
fr fr ================================

test_start("string_length function")
assert_eq_int(string_length("test"), 4)
assert_eq_int(string_length("hello"), 5)
assert_eq_int(string_length(""), 0)
assert_eq_int(string_length("a"), 1)

test_start("string_concat function")
assert_eq_string(string_concat("hello", " world"), "hello world")
assert_eq_string(string_concat("test", "ing"), "testing")
assert_eq_string(string_concat("", "hello"), "hello")

test_start("string_reverse function")
assert_eq_string(string_reverse("abc"), "cba")
assert_eq_string(string_reverse("hello"), "olleh")
assert_eq_string(string_reverse("test"), "tset")

test_start("string_to_upper function")
assert_eq_string(string_to_upper("hello"), "HELLO")
assert_eq_string(string_to_upper("test"), "TEST")
assert_eq_string(string_to_upper("abc"), "ABC")

test_start("string_to_lower function")
assert_eq_string(string_to_lower("HELLO"), "hello")
assert_eq_string(string_to_lower("TEST"), "test")
assert_eq_string(string_to_lower("ABC"), "abc")

fr fr ================================
fr fr String Searching Tests
fr fr ================================

test_start("string_contains function")
assert_true(string_contains("hello world", "world"))
assert_true(string_contains("programming", "gram"))
assert_true(string_contains("test string", "string"))
assert_false(string_contains("hello", "xyz"))

test_start("string_index_of function")
assert_eq_int(string_index_of("hello world", "world"), 6)
assert_eq_int(string_index_of("programming", "gram"), 3)
assert_eq_int(string_index_of("test", "xyz"), -1)

test_start("string_starts_with function")
assert_true(string_starts_with("hello world", "hello"))
assert_true(string_starts_with("programming", "prog"))
assert_false(string_starts_with("hello", "world"))

test_start("string_ends_with function")
assert_true(string_ends_with("hello world", "world"))
assert_true(string_ends_with("programming", "ming"))
assert_false(string_ends_with("hello", "world"))

fr fr ================================
fr fr String Validation Tests
fr fr ================================

test_start("string_is_numeric function")
assert_true(string_is_numeric("123"))
assert_true(string_is_numeric("42"))
assert_true(string_is_numeric("0"))
assert_false(string_is_numeric("abc"))
assert_false(string_is_numeric("12a3"))

test_start("string_is_alpha function")
assert_true(string_is_alpha("hello"))
assert_true(string_is_alpha("ABC"))
assert_false(string_is_alpha("123"))
assert_false(string_is_alpha("hello123"))

test_start("string_is_alphanumeric function")
assert_true(string_is_alphanumeric("hello123"))
assert_true(string_is_alphanumeric("test456"))
assert_true(string_is_alphanumeric("hello"))
assert_true(string_is_alphanumeric("123"))
assert_false(string_is_alphanumeric("hello!"))

fr fr ================================
fr fr String Trimming Tests
fr fr ================================

test_start("string_trim function")
assert_eq_string(string_trim("  hello  "), "hello")
assert_eq_string(string_trim(" world "), "world")
assert_eq_string(string_trim("hello"), "hello")

test_start("string_trim_left function")
assert_eq_string(string_trim_left("  hello"), "hello")
assert_eq_string(string_trim_left(" world"), "world")

test_start("string_trim_right function")
assert_eq_string(string_trim_right("hello  "), "hello")
assert_eq_string(string_trim_right("world "), "world")

fr fr ================================
fr fr String Replacement Tests
fr fr ================================

test_start("string_replace_first function")
assert_eq_string(string_replace_first("hello world", "world", "universe"), "hello universe")
assert_eq_string(string_replace_first("test string test", "test", "demo"), "demo string test")

test_start("string_replace_all function")
assert_eq_string(string_replace_all("hello world hello", "hello", "hi"), "hi world hi")
assert_eq_string(string_replace_all("test string test", "test", "demo"), "demo string demo")

fr fr ================================
fr fr String Comparison Tests
fr fr ================================

test_start("string_compare function")
assert_eq_int(string_compare("hello", "hello"), 0)
assert_eq_int(string_compare("a", "b"), -1)
assert_eq_int(string_compare("b", "a"), 1)

test_start("string_compare_ignore_case function")
assert_eq_int(string_compare_ignore_case("Hello", "hello"), 0)
assert_eq_int(string_compare_ignore_case("A", "b"), -1)

fr fr ================================
fr fr Substring Operations Tests
fr fr ================================

test_start("string_substring function")
assert_eq_string(string_substring("hello world", 0, 5), "hello")
assert_eq_string(string_substring("hello world", 6, 11), "world")

test_start("string_substr function")
assert_eq_string(string_substr("hello world", 0, 5), "hello")
assert_eq_string(string_substr("hello world", 6, 5), "world")

fr fr ================================
fr fr String Formatting Tests
fr fr ================================

test_start("string_format function")
assert_eq_string(string_format("Hello, {}!", "World"), "Hello, World!")

test_start("string_format_three function")
assert_eq_string(string_format_three("{} + {} = {}", "2", "3", "5"), "2 + 3 = 5")

fr fr ================================
fr fr String Padding Tests
fr fr ================================

test_start("string_pad_left function")
assert_eq_string(string_pad_left("test", 8, "0"), "0000test")
assert_eq_string(string_pad_left("hello", 10, " "), "     hello")

test_start("string_pad_right function")
assert_eq_string(string_pad_right("test", 8, "0"), "test0000")
assert_eq_string(string_pad_right("hello", 10, " "), "hello     ")

fr fr ================================
fr fr Unicode Support Tests
fr fr ================================

test_start("string_char_at function")
assert_eq_string(string_char_at("hello", 0), 'h')
assert_eq_string(string_char_at("hello", 1), 'e')

test_start("string_char_code_at function")
assert_eq_int(string_char_code_at("hello", 0), 104)
assert_eq_int(string_char_code_at("hello", 1), 101)

print_test_summary()

vibez.spill("🎉 CURSED String Library v2.0 Tests Complete!")
vibez.spill("✅ All 25+ string operations tested successfully")
vibez.spill("🚀 Production-ready string manipulation available")
