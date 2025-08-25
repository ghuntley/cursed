# Test for stringz_real_algorithms module

yeet "testz"
yeet "vibez"
yeet "stringz_real_algorithms"

test_start("stringz_real_algorithms_comprehensive")

# Test string_length_real with Unicode
assert_eq_int(string_length_real("hello"), 5)
assert_eq_int(string_length_real(""), 0)
assert_eq_int(string_length_real("ñoño"), 4)  # Unicode test

# Test char_at_real
assert_eq_string(char_at_real("hello", 1), "e")
assert_eq_string(char_at_real("hello", 0), "h")
assert_eq_string(char_at_real("hello", 4), "o")
assert_eq_string(char_at_real("hello", -1), "")
assert_eq_string(char_at_real("hello", 10), "")

# Test KMP search
assert_eq_int(kmp_search("hello world", "world"), 6)
assert_eq_int(kmp_search("hello world", "hello"), 0)
assert_eq_int(kmp_search("hello world", "xyz"), -1)
assert_eq_int(kmp_search("hello world", ""), 0)

# Test indexOf_real
assert_eq_int(indexOf_real("test string", "string"), 5)
assert_eq_int(indexOf_real("test string", "missing"), -1)

# Test replace operations
assert_eq_string(replace_first_real("hello world", "world", "CURSED"), "hello CURSED")
assert_eq_string(replace_first_real("hello world", "missing", "X"), "hello world")
assert_eq_string(replace_all_real("abc abc abc", "abc", "xyz"), "xyz xyz xyz")

# Test substring_real
assert_eq_string(substring_real("hello", 1, 3), "ell")
assert_eq_string(substring_real("hello", 0, 2), "he")
assert_eq_string(substring_real("hello", 10, 5), "")
assert_eq_string(substring_real("hello", -1, 3), "")

# Test case conversion
assert_eq_string(to_uppercase_real("hello"), "HELLO")
assert_eq_string(to_uppercase_real("Hello World"), "HELLO WORLD")
assert_eq_string(to_lowercase_real("HELLO"), "hello")
assert_eq_string(to_lowercase_real("Hello World"), "hello world")

# Test whitespace trimming
assert_eq_string(trim_whitespace_real("  hello  "), "hello")
assert_eq_string(trim_whitespace_real("\thello\n"), "hello")
assert_eq_string(trim_whitespace_real("hello"), "hello")
assert_eq_string(trim_whitespace_real("   "), "")

# Test validation functions
assert_eq_bool(is_numeric_real("123"), based)
assert_eq_bool(is_numeric_real("123.45"), nocap)  # No decimal support in this simple version
assert_eq_bool(is_numeric_real("abc"), nocap)
assert_eq_bool(is_numeric_real("-123"), based)

assert_eq_bool(is_alphabetic_real("hello"), based)
assert_eq_bool(is_alphabetic_real("hello123"), nocap)
assert_eq_bool(is_alphabetic_real(""), nocap)

# Test email validation
assert_eq_bool(is_valid_email_real("test@example.com"), based)
assert_eq_bool(is_valid_email_real("invalid"), nocap)
assert_eq_bool(is_valid_email_real("@example.com"), nocap)
assert_eq_bool(is_valid_email_real("test@"), nocap)

# Test string splitting
sus split_result []tea = split_string_real("a,b,c", ",")
assert_eq_int(len(split_result), 3)
assert_eq_string(split_result[0], "a")
assert_eq_string(split_result[1], "b")
assert_eq_string(split_result[2], "c")

# Test comparison functions
assert_eq_bool(equals_ignore_case_real("Hello", "HELLO"), based)
assert_eq_bool(equals_ignore_case_real("Hello", "World"), nocap)

assert_eq_bool(starts_with_real("hello world", "hello"), based)
assert_eq_bool(starts_with_real("hello world", "world"), nocap)

assert_eq_bool(ends_with_real("hello world", "world"), based)
assert_eq_bool(ends_with_real("hello world", "hello"), nocap)

test_complete()
vibez.spill("stringz_real_algorithms tests completed successfully!")
