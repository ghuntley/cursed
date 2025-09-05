yeet "testz"
yeet "stringz"

test_start("stringz String Processing Comprehensive Tests")

fr fr ===== BASIC STRING OPERATIONS TESTS =====

test_group("Basic String Operations")

fr fr Test string concatenation
sus concat_result tea = concat_strings("Hello", " World")
assert_string_equals(concat_result, "Hello World", "Basic string concatenation")

concat_result = concat_three("A", "B", "C")
assert_string_equals(concat_result, "ABC", "Three string concatenation")

fr fr Test string repetition
sus repeat_result tea = repeat_string("Hi", 3)
assert_string_equals(repeat_result, "HiHiHi", "String repetition")

repeat_result = repeat_string("X", 0)
assert_string_equals(repeat_result, "", "String repetition zero times")

repeat_result = repeat_string("Test", 1)
assert_string_equals(repeat_result, "Test", "String repetition once")

fr fr ===== STRING VALIDATION TESTS =====

test_group("String Validation")

fr fr Test empty string validation
sus is_empty_result lit = is_empty_string("")
assert_bool(is_empty_result, "Empty string detection")

is_empty_result = is_empty_string("not empty")
assert_bool(!is_empty_result, "Non-empty string detection")

sus is_not_empty_result lit = is_not_empty("")
assert_bool(!is_not_empty_result, "Empty string is not not-empty")

is_not_empty_result = is_not_empty("has content")
assert_bool(is_not_empty_result, "Non-empty string is not-empty")

fr fr Test string equality
sus equals_result lit = strings_equal("same", "same")
assert_bool(equals_result, "Equal strings comparison")

equals_result = strings_equal("different", "strings")
assert_bool(!equals_result, "Different strings comparison")

sus not_equals_result lit = strings_not_equal("same", "same")
assert_bool(!not_equals_result, "Same strings not not-equal")

not_equals_result = strings_not_equal("different", "strings")
assert_bool(not_equals_result, "Different strings are not-equal")

fr fr ===== STRING BUILDING TESTS =====

test_group("String Building")

fr fr Test string building functions
sus build_result tea = build_string_two("Part1", "Part2")
assert_string_equals(build_result, "Part1Part2", "Two-part string building")

build_result = build_string_three("A", "B", "C")
assert_string_equals(build_result, "ABC", "Three-part string building")

build_result = build_string_four("1", "2", "3", "4")
assert_string_equals(build_result, "1234", "Four-part string building")

build_result = build_string_five("W", "X", "Y", "Z", "!")
assert_string_equals(build_result, "WXYZ!", "Five-part string building")

fr fr ===== STRING LENGTH TESTS =====

test_group("String Length Operations")

fr fr Test length calculation
sus length_result drip = string_length("")
assert_eq_int(length_result, 0, "Empty string length")

length_result = string_length("Hello")
assert_eq_int(length_result, 5, "Basic string length")

length_result = string_length("Unicode Test: 你好")
assert_true(length_result > 10, "Unicode string length")

fr fr ===== SUBSTRING OPERATIONS TESTS =====

test_group("Substring Operations")

fr fr Test substring extraction
sus substr_result tea = substring("Hello World", 0, 5)
assert_string_equals(substr_result, "Hello", "Substring from beginning")

substr_result = substring("Hello World", 6, 5)
assert_string_equals(substr_result, "World", "Substring from middle")

substr_result = substring("Test", 1, 2)
assert_string_equals(substr_result, "es", "Substring middle extraction")

fr fr Test substring edge cases
substr_result = substring("Short", 0, 10)
assert_string_equals(substr_result, "Short", "Substring longer than string")

substr_result = substring("Test", 4, 5)
assert_string_equals(substr_result, "", "Substring beyond string end")

fr fr ===== CHARACTER OPERATIONS TESTS =====

test_group("Character Operations")

fr fr Test character at position
sus char_result tea = char_at("Hello", 0)
assert_string_equals(char_result, "H", "First character")

char_result = char_at("World", 4)
assert_string_equals(char_result, "d", "Last character")

char_result = char_at("Test", 2)
assert_string_equals(char_result, "s", "Middle character")

fr fr Test character replacement
sus replace_result tea = replace_char_at("Hello", 1, "a")
assert_string_equals(replace_result, "Hallo", "Character replacement")

replace_result = replace_char_at("Test", 0, "B")
assert_string_equals(replace_result, "Best", "First character replacement")

fr fr ===== STRING SEARCH TESTS =====

test_group("String Search Operations")

fr fr Test string contains
sus contains_result lit = string_contains("Hello World", "World")
assert_bool(contains_result, "String contains substring")

contains_result = string_contains("Hello World", "Universe")
assert_bool(!contains_result, "String does not contain substring")

contains_result = string_contains("Test", "Test")
assert_bool(contains_result, "String contains itself")

fr fr Test find operations
sus find_result drip = find_substring("Hello World", "World")
assert_eq_int(find_result, 6, "Find substring position")

find_result = find_substring("Hello World", "NotFound")
assert_eq_int(find_result, -1, "Substring not found")

find_result = find_substring("Test", "Test")
assert_eq_int(find_result, 0, "Find string at beginning")

fr fr Test starts with and ends with
sus starts_result lit = starts_with("Hello World", "Hello")
assert_bool(starts_result, "String starts with prefix")

starts_result = starts_with("Hello World", "World")
assert_bool(!starts_result, "String does not start with suffix")

sus ends_result lit = ends_with("Hello World", "World")
assert_bool(ends_result, "String ends with suffix")

ends_result = ends_with("Hello World", "Hello")
assert_bool(!ends_result, "String does not end with prefix")

fr fr ===== STRING TRANSFORMATION TESTS =====

test_group("String Transformations")

fr fr Test case transformations
sus upper_result tea = to_upper_case("hello world")
assert_string_equals(upper_result, "HELLO WORLD", "Uppercase transformation")

sus lower_result tea = to_lower_case("HELLO WORLD")
assert_string_equals(lower_result, "hello world", "Lowercase transformation")

sus title_result tea = to_title_case("hello world")
assert_string_equals(title_result, "Hello World", "Title case transformation")

fr fr Test string trimming
sus trim_result tea = trim_whitespace("  Hello World  ")
assert_string_equals(trim_result, "Hello World", "Whitespace trimming")

trim_result = trim_left("  Hello World  ")
assert_string_equals(trim_result, "Hello World  ", "Left whitespace trimming")

trim_result = trim_right("  Hello World  ")
assert_string_equals(trim_result, "  Hello World", "Right whitespace trimming")

fr fr Test string padding
sus pad_result tea = pad_left("123", 5, "0")
assert_string_equals(pad_result, "00123", "Left padding")

pad_result = pad_right("123", 5, "0")
assert_string_equals(pad_result, "12300", "Right padding")

pad_result = pad_center("Hi", 6, "*")
assert_string_equals(pad_result, "**Hi**", "Center padding")

fr fr ===== STRING REPLACEMENT TESTS =====

test_group("String Replacement")

fr fr Test string replacement
sus replace_all_result tea = replace_all("Hello World World", "World", "Universe")
assert_string_equals(replace_all_result, "Hello Universe Universe", "Replace all occurrences")

sus replace_first_result tea = replace_first("Hello World World", "World", "Universe")
assert_string_equals(replace_first_result, "Hello Universe World", "Replace first occurrence")

fr fr Test character replacement
sus replace_chars_result tea = replace_all("Hello", "l", "x")
assert_string_equals(replace_chars_result, "Hexxo", "Replace all characters")

fr fr ===== STRING SPLITTING TESTS =====

test_group("String Splitting")

fr fr Test string splitting
sus split_result tea[value] = split_string("Hello,World,Test", ",")
assert_eq_int(array_length(split_result), 3, "Split into three parts")
assert_string_equals(split_result[0], "Hello", "First split part")
assert_string_equals(split_result[1], "World", "Second split part")
assert_string_equals(split_result[2], "Test", "Third split part")

split_result = split_string("NoSeparator", ",")
assert_eq_int(array_length(split_result), 1, "No separator returns original")
assert_string_equals(split_result[0], "NoSeparator", "Original string returned")

fr fr Test split by whitespace
sus whitespace_split tea[value] = split_whitespace("Hello World Test")
assert_eq_int(array_length(whitespace_split), 3, "Whitespace split count")
assert_string_equals(whitespace_split[0], "Hello", "First word")
assert_string_equals(whitespace_split[1], "World", "Second word")
assert_string_equals(whitespace_split[2], "Test", "Third word")

fr fr Test split lines
sus line_split tea[value] = split_lines("Line1\nLine2\nLine3")
assert_eq_int(array_length(line_split), 3, "Line split count")
assert_string_equals(line_split[0], "Line1", "First line")
assert_string_equals(line_split[1], "Line2", "Second line")
assert_string_equals(line_split[2], "Line3", "Third line")

fr fr ===== STRING JOINING TESTS =====

test_group("String Joining")

fr fr Test array joining
sus join_array tea[value] = ["A", "B", "C"]
sus join_result tea = join_strings(join_array, ",")
assert_string_equals(join_result, "A,B,C", "Join with comma separator")

join_result = join_strings(join_array, " - ")
assert_string_equals(join_result, "A - B - C", "Join with multi-char separator")

sus empty_array tea[value] = []
join_result = join_strings(empty_array, ",")
assert_string_equals(join_result, "", "Join empty array")

sus single_array tea[value] = ["Only"]
join_result = join_strings(single_array, ",")
assert_string_equals(join_result, "Only", "Join single element")

fr fr ===== NUMERIC CONVERSION TESTS =====

test_group("Numeric Conversions")

fr fr Test string to number conversions
sus int_result drip = string_to_int("123") fam {
    when err -> {
        assert_fail("String to int conversion failed: " + err)
        damn 0
    }
}
assert_eq_int(int_result, 123, "String to integer conversion")

int_result = string_to_int("-456") fam {
    when err -> {
        assert_fail("Negative string to int failed: " + err)
        damn 0
    }
}
assert_eq_int(int_result, -456, "Negative string to integer")

fr fr Test invalid number conversion
sus invalid_int drip = string_to_int("not_a_number") fam {
    when err -> {
        assert_string_contains(err, "invalid", "Invalid number error")
        damn 999
    }
}
assert_eq_int(invalid_int, 999, "Invalid number conversion handled")

fr fr Test number to string conversions
sus str_result tea = int_to_string(789)
assert_string_equals(str_result, "789", "Integer to string conversion")

str_result = int_to_string(-321)
assert_string_equals(str_result, "-321", "Negative integer to string")

str_result = int_to_string(0)
assert_string_equals(str_result, "0", "Zero to string conversion")

fr fr ===== STRING VALIDATION PATTERNS =====

test_group("String Validation Patterns")

fr fr Test digit validation
sus is_digit_result lit = is_all_digits("12345")
assert_bool(is_digit_result, "All digits validation")

is_digit_result = is_all_digits("123a5")
assert_bool(!is_digit_result, "Mixed digits and letters")

is_digit_result = is_all_digits("")
assert_bool(!is_digit_result, "Empty string not all digits")

fr fr Test alphabetic validation
sus is_alpha_result lit = is_all_alphabetic("Hello")
assert_bool(is_alpha_result, "All alphabetic validation")

is_alpha_result = is_all_alphabetic("Hello123")
assert_bool(!is_alpha_result, "Mixed alphabetic and numeric")

is_alpha_result = is_all_alphabetic("Hello!")
assert_bool(!is_alpha_result, "Alphabetic with punctuation")

fr fr Test alphanumeric validation
sus is_alnum_result lit = is_alphanumeric("Hello123")
assert_bool(is_alnum_result, "Alphanumeric validation")

is_alnum_result = is_alphanumeric("Hello123!")
assert_bool(!is_alnum_result, "Alphanumeric with punctuation")

fr fr ===== UNICODE SUPPORT TESTS =====

test_group("Unicode Support")

fr fr Test Unicode string operations
sus unicode_str tea = "Hello 世界 🌍"
sus unicode_len drip = string_length(unicode_str)
assert_true(unicode_len > 8, "Unicode string length")

sus unicode_upper tea = to_upper_case("hello")
assert_string_equals(unicode_upper, "HELLO", "Unicode uppercase")

sus unicode_contains lit = string_contains(unicode_str, "世界")
assert_bool(unicode_contains, "Unicode substring search")

fr fr ===== PERFORMANCE TESTS =====

test_group("Performance and Large String Tests")

fr fr Test large string operations
sus large_string tea = repeat_string("X", 100)
assert_eq_int(string_length(large_string), 100, "Large string creation")

sus large_find drip = find_substring(large_string, "X")
assert_eq_int(large_find, 0, "Find in large string")

sus large_replace tea = replace_first(large_string, "X", "Y")
assert_string_equals(substring(large_replace, 0, 1), "Y", "Replace in large string")

fr fr Test concatenation performance
sus concat_test tea = ""
sus i drip = 0
bestie (i < 10) {
    concat_test = concat_test + "Part" + int_to_string(i)
    i = i + 1
}
assert_true(string_length(concat_test) > 30, "Concatenation loop result")

fr fr ===== EDGE CASE TESTS =====

test_group("Edge Cases and Error Handling")

fr fr Test empty string operations
sus empty_upper tea = to_upper_case("")
assert_string_equals(empty_upper, "", "Empty string uppercase")

sus empty_find drip = find_substring("", "test")
assert_eq_int(empty_find, -1, "Find in empty string")

sus empty_replace tea = replace_all("", "a", "b")
assert_string_equals(empty_replace, "", "Replace in empty string")

fr fr Test single character operations
sus single_char tea = "A"
sus single_lower tea = to_lower_case(single_char)
assert_string_equals(single_lower, "a", "Single character lowercase")

sus single_repeat tea = repeat_string(single_char, 5)
assert_string_equals(single_repeat, "AAAAA", "Single character repetition")

fr fr Test boundary conditions
sus boundary_substr tea = substring("Test", 0, 0)
assert_string_equals(boundary_substr, "", "Zero length substring")

sus boundary_char tea = char_at("A", 0)
assert_string_equals(boundary_char, "A", "Single char at position 0")

print_test_summary()
