yeet "testz"
yeet "pure_string"

test_start("Pure CURSED String Module Tests")

// Test basic string operations
assert_eq_int(string_length("hello"), 5)
assert_eq_string(string_concat("hello", " world"), "hello world")

sus slice_result tea = string_slice("hello world", 6, 11);
assert_eq_string(slice_result, "world")

assert_eq_char(string_char_at("hello", 1), 'e')

// Test string search
assert_true(string_contains("hello world", "world"))
assert_false(string_contains("hello world", "xyz"))

assert_eq_int(string_find("hello world", "world"), 6)
assert_eq_int(string_find("hello world", "xyz"), -1)

// Test string replacement
sus replace_result tea = string_replace("hello world", "world", "universe");
assert_eq_string(replace_result, "hello universe")

sus replace_all_result tea = string_replace_all("hello hello hello", "hello", "hi");
assert_eq_string(replace_all_result, "hi hi hi")

// Test string transformations
assert_eq_string(string_to_upper("hello"), "HELLO")
assert_eq_string(string_to_lower("WORLD"), "world")
assert_eq_string(string_capitalize("hello"), "Hello")

sus trim_result tea = string_trim("  hello world  ");
assert_eq_string(trim_result, "hello world")

sus trim_left_result tea = string_trim_left("  hello world  ");
assert_eq_string(trim_left_result, "hello world  ")

sus trim_right_result tea = string_trim_right("  hello world  ");
assert_eq_string(trim_right_result, "  hello world")

assert_eq_string(string_reverse("hello"), "olleh")

// Test string splitting and joining
sus split_result [tea] = string_split("apple,banana,cherry", ",");
assert_eq_int(split_result.length, 3)
assert_eq_string(split_result[0], "apple")
assert_eq_string(split_result[1], "banana")
assert_eq_string(split_result[2], "cherry")

sus fruits [tea] = ["apple", "banana", "cherry"];
sus join_result tea = string_join(fruits, ", ");
assert_eq_string(join_result, "apple, banana, cherry")

// Test string validation
assert_true(string_is_empty(""))
assert_false(string_is_empty("hello"))

assert_true(string_is_whitespace("   "))
assert_false(string_is_whitespace("hello"))

assert_true(string_is_numeric("12345"))
assert_false(string_is_numeric("123a5"))

assert_true(string_is_alpha("hello"))
assert_false(string_is_alpha("hello123"))

assert_true(string_is_alphanumeric("hello123"))
assert_false(string_is_alphanumeric("hello@123"))

assert_true(string_starts_with("hello world", "hello"))
assert_false(string_starts_with("hello world", "world"))

assert_true(string_ends_with("hello world", "world"))
assert_false(string_ends_with("hello world", "hello"))

// Test string formatting
sus pad_left_result tea = string_pad_left("hello", 10, ' ');
assert_eq_int(string_length(pad_left_result), 10)

sus pad_right_result tea = string_pad_right("hello", 10, ' ');
assert_eq_int(string_length(pad_right_result), 10)

sus pad_center_result tea = string_pad_center("hello", 11, ' ');
assert_eq_int(string_length(pad_center_result), 11)

// Test character utilities
assert_true(is_whitespace(' '))
assert_false(is_whitespace('a'))

assert_true(is_digit('5'))
assert_false(is_digit('a'))

assert_true(is_alpha('a'))
assert_false(is_alpha('5'))

assert_true(is_alphanumeric('a'))
assert_true(is_alphanumeric('5'))
assert_false(is_alphanumeric('@'))

assert_true(is_upper('A'))
assert_false(is_upper('a'))

assert_true(is_lower('a'))
assert_false(is_lower('A'))

assert_eq_char(to_upper_char('a'), 'A')
assert_eq_char(to_lower_char('A'), 'a')

// Test string comparison
assert_eq_int(string_compare("apple", "banana"), -1)
assert_eq_int(string_compare("banana", "apple"), 1)
assert_eq_int(string_compare("apple", "apple"), 0)

assert_true(string_equals("hello", "hello"))
assert_false(string_equals("hello", "world"))

assert_true(string_equals_ignore_case("Hello", "HELLO"))
assert_false(string_equals_ignore_case("hello", "world"))

// Test string conversion
assert_eq_string(int_to_string(123), "123")
assert_eq_string(int_to_string(-45), "-45")

sus float_str tea = float_to_string(3.14159);
assert_true(string_contains(float_str, "3"))
assert_true(string_contains(float_str, "."))

assert_eq_int(string_to_int("123"), 123)
assert_eq_int(string_to_int("-45"), -45)

sus float_val meal = string_to_float("3.14");
assert_true(float_val > 3.13 && float_val < 3.15)

// Test string encoding
sus bytes [byte] = string_to_bytes("hello");
assert_eq_int(bytes.length, 5)

sus str_from_bytes tea = bytes_to_string(bytes);
assert_eq_string(str_from_bytes, "hello")

sus escaped tea = string_escape("hello\nworld");
assert_true(string_contains(escaped, "\\n"))

sus unescaped tea = string_unescape("hello\\nworld");
assert_true(string_contains(unescaped, "\n"))

print_test_summary()
