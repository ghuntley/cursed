# Comprehensive Test Suite for Enhanced StringZ Module
# Tests all migrated functionality from Rust stdlib modules

yeet "testz"
yeet "stringz"

# Start comprehensive testing
test_start("StringZ Enhanced Module Comprehensive Tests")

# ===== CORE STRING OPERATIONS TESTS =====

test_start("Core String Operations")

# Test string_length
assert_eq_int(string_length(""), 0)
assert_eq_int(string_length("hello"), 5)
assert_eq_int(string_length("🚀"), 1)  # Unicode handling

# Test string_is_empty
assert_true(string_is_empty(""))
assert_false(string_is_empty("test"))

# Test string_concat
assert_eq_string(string_concat("hello", " world"), "hello world")
assert_eq_string(string_concat("", "test"), "test")
assert_eq_string(string_concat("test", ""), "test")

# Test string_reverse
assert_eq_string(string_reverse("hello"), "olleh")
assert_eq_string(string_reverse(""), "")
assert_eq_string(string_reverse("a"), "a")

vibez.spill("✅ Core operations tests passed")

# ===== SEARCH AND MATCHING TESTS =====

test_start("Search and Matching Operations")

# Test string_contains
assert_true(string_contains("hello world", "world"))
assert_true(string_contains("hello world", "hello"))
assert_false(string_contains("hello world", "xyz"))
assert_true(string_contains("test", ""))  # Empty string contained in any string

# Test string_index_of
assert_eq_int(string_index_of("hello world", "world"), 6)
assert_eq_int(string_index_of("hello world", "hello"), 0)
assert_eq_int(string_index_of("hello world", "xyz"), -1)
assert_eq_int(string_index_of("test", ""), 0)  # Empty string at position 0

# Test string_last_index_of  
assert_eq_int(string_last_index_of("hello hello", "hello"), 6)
assert_eq_int(string_last_index_of("hello world", "o"), 7)
assert_eq_int(string_last_index_of("test", "xyz"), -1)

# Test string_count_occurrences
assert_eq_int(string_count_occurrences("hello hello hello", "hello"), 3)
assert_eq_int(string_count_occurrences("test", "t"), 2)
assert_eq_int(string_count_occurrences("test", "xyz"), 0)

vibez.spill("✅ Search and matching tests passed")

# ===== PREFIX AND SUFFIX TESTS =====

test_start("Prefix and Suffix Operations")

# Test string_has_prefix
assert_true(string_has_prefix("hello world", "hello"))
assert_false(string_has_prefix("hello world", "world"))
assert_true(string_has_prefix("test", ""))  # Empty prefix

# Test string_has_suffix
assert_true(string_has_suffix("hello world", "world"))
assert_false(string_has_suffix("hello world", "hello"))
assert_true(string_has_suffix("test", ""))  # Empty suffix

vibez.spill("✅ Prefix and suffix tests passed")

# ===== CASE CONVERSION TESTS =====

test_start("Case Conversion Operations")

# Test char_to_lower and char_to_upper
assert_eq_string(char_to_lower('A') + "", "a")
assert_eq_string(char_to_upper('a') + "", "A")
assert_eq_string(char_to_lower('5') + "", "5")  # Non-letter unchanged

# Test string_to_lower and string_to_upper
assert_eq_string(string_to_lower("HELLO WORLD"), "hello world")
assert_eq_string(string_to_upper("hello world"), "HELLO WORLD")
assert_eq_string(string_to_lower("Test123"), "test123")

# Test string_to_title_case
assert_eq_string(string_to_title_case("hello world"), "Hello World")
assert_eq_string(string_to_title_case("test case"), "Test Case")

vibez.spill("✅ Case conversion tests passed")

# ===== WHITESPACE AND TRIMMING TESTS =====

test_start("Whitespace and Trimming Operations")

# Test is_whitespace
assert_true(is_whitespace(' '))
assert_true(is_whitespace('\t'))
assert_true(is_whitespace('\n'))
assert_false(is_whitespace('a'))

# Test string_trim_left
assert_eq_string(string_trim_left("   hello"), "hello")
assert_eq_string(string_trim_left("hello   "), "hello   ")
assert_eq_string(string_trim_left("\t\n  test"), "test")

# Test string_trim_right
assert_eq_string(string_trim_right("hello   "), "hello")
assert_eq_string(string_trim_right("   hello"), "   hello")
assert_eq_string(string_trim_right("test  \t\n"), "test")

# Test string_trim (both sides)
assert_eq_string(string_trim("  hello  "), "hello")
assert_eq_string(string_trim("\t\nhello\t\n"), "hello")

vibez.spill("✅ Whitespace and trimming tests passed")

# ===== SUBSTRING AND SLICING TESTS =====

test_start("Substring and Slicing Operations")

# Test string_substring
assert_eq_string(string_substring("hello world", 0, 5), "hello")
assert_eq_string(string_substring("hello world", 6, 5), "world")
assert_eq_string(string_substring("test", 2, 2), "st")
assert_eq_string(string_substring("test", 10, 5), "")  # Out of bounds

# Test string_slice
assert_eq_string(string_slice("hello world", 0, 5), "hello")
assert_eq_string(string_slice("hello world", 6, 11), "world")
assert_eq_string(string_slice("test", -2, -1), "s")  # Negative indices

vibez.spill("✅ Substring and slicing tests passed")

# ===== SPLITTING AND JOINING TESTS =====

test_start("Splitting and Joining Operations")

# Test string_split
sus split_result [tea] = string_split("hello,world,test", ",")
assert_eq_int(len(split_result), 3)
assert_eq_string(split_result[0], "hello")
assert_eq_string(split_result[1], "world")
assert_eq_string(split_result[2], "test")

# Test string_join
sus parts [tea] = ["hello", "world", "test"]
assert_eq_string(string_join(parts, ","), "hello,world,test")
assert_eq_string(string_join(parts, " "), "hello world test")

# Test string_split_lines
sus line_result [tea] = string_split_lines("line1\nline2\rline3\r\nline4")
assert_eq_int(len(line_result), 4)
assert_eq_string(line_result[0], "line1")
assert_eq_string(line_result[1], "line2")

vibez.spill("✅ Splitting and joining tests passed")

# ===== REPLACEMENT TESTS =====

test_start("Replacement Operations")

# Test string_replace_first
assert_eq_string(string_replace_first("hello world hello", "hello", "hi"), "hi world hello")
assert_eq_string(string_replace_first("test", "xyz", "abc"), "test")  # No match

# Test string_replace_all
assert_eq_string(string_replace_all("hello world hello", "hello", "hi"), "hi world hi")
assert_eq_string(string_replace_all("test test test", "test", "pass"), "pass pass pass")

# Test string_replace_at_index
assert_eq_string(string_replace_at_index("hello", 1, "XYZ"), "hXYZllo")
assert_eq_string(string_replace_at_index("test", 10, "x"), "test")  # Invalid index

vibez.spill("✅ Replacement tests passed")

# ===== REPETITION AND PADDING TESTS =====

test_start("Repetition and Padding Operations")

# Test string_repeat
assert_eq_string(string_repeat("hello", 3), "hellohellohello")
assert_eq_string(string_repeat("test", 0), "")
assert_eq_string(string_repeat("a", 5), "aaaaa")

# Test string_pad_left
assert_eq_string(string_pad_left("test", 8, ' '), "    test")
assert_eq_string(string_pad_left("hello", 3, 'X'), "hello")  # Already longer

# Test string_pad_right
assert_eq_string(string_pad_right("test", 8, ' '), "test    ")
assert_eq_string(string_pad_right("hello", 3, 'X'), "hello")  # Already longer

# Test string_center
assert_eq_string(string_center("test", 8, ' '), "  test  ")
assert_eq_string(string_center("hi", 5, 'X'), "XhiXX")  # Odd padding

vibez.spill("✅ Repetition and padding tests passed")

# ===== VALIDATION TESTS =====

test_start("Validation and Classification Operations")

# Test string_is_numeric
assert_true(string_is_numeric("12345"))
assert_false(string_is_numeric("123a5"))
assert_false(string_is_numeric(""))
assert_false(string_is_numeric("12.34"))  # Decimal not numeric

# Test string_is_alpha
assert_true(string_is_alpha("hello"))
assert_true(string_is_alpha("Hello"))
assert_false(string_is_alpha("hello123"))
assert_false(string_is_alpha(""))

# Test string_is_alphanumeric
assert_true(string_is_alphanumeric("hello123"))
assert_true(string_is_alphanumeric("Test"))
assert_false(string_is_alphanumeric("hello world"))  # Space not alphanumeric
assert_false(string_is_alphanumeric(""))

# Test string_is_lower and string_is_upper
assert_true(string_is_lower("hello world"))
assert_false(string_is_lower("Hello world"))
assert_true(string_is_upper("HELLO WORLD"))
assert_false(string_is_upper("Hello WORLD"))

vibez.spill("✅ Validation tests passed")

# ===== ADVANCED OPERATIONS TESTS =====

test_start("Advanced String Operations")

# Test string_common_prefix
assert_eq_string(string_common_prefix("hello", "help"), "hel")
assert_eq_string(string_common_prefix("test", "temp"), "te")
assert_eq_string(string_common_prefix("abc", "xyz"), "")

# Test string_common_suffix
assert_eq_string(string_common_suffix("testing", "running"), "ing")
assert_eq_string(string_common_suffix("hello", "world"), "")
assert_eq_string(string_common_suffix("test", "west"), "est")

# Test string_distance_levenshtein (simplified)
assert_true(string_distance_levenshtein("hello", "hello") == 0)
assert_true(string_distance_levenshtein("hello", "hallo") >= 1)
assert_true(string_distance_levenshtein("test", "best") >= 1)

vibez.spill("✅ Advanced operations tests passed")

# ===== FORMAT AND ENCODING TESTS =====

test_start("Format and Encoding Operations")

# Test string_escape_special_chars
assert_eq_string(string_escape_special_chars("hello\nworld"), "hello\\nworld")
assert_eq_string(string_escape_special_chars("test\"quote"), "test\\\"quote")
assert_eq_string(string_escape_special_chars("path\\file"), "path\\\\file")

# Test string_unescape_special_chars
assert_eq_string(string_unescape_special_chars("hello\\nworld"), "hello\nworld")
assert_eq_string(string_unescape_special_chars("test\\\"quote"), "test\"quote")
assert_eq_string(string_unescape_special_chars("path\\\\file"), "path\\file")

vibez.spill("✅ Format and encoding tests passed")

# ===== COMPATIBILITY ALIAS TESTS =====

test_start("Compatibility Alias Functions")

# Test backward compatibility functions
assert_true(Contains("hello world", "world"))
assert_true(HasPrefix("hello", "hel"))
assert_true(HasSuffix("world", "rld"))
assert_eq_string(ToLower("HELLO"), "hello")
assert_eq_string(ToUpper("hello"), "HELLO")
assert_eq_string(Trim("  test  "), "test")
assert_eq_string(Replace("hello world", "world", "universe"), "hello universe")
assert_eq_int(Length("hello"), 5)
assert_true(IsEmpty(""))
assert_false(IsEmpty("test"))

vibez.spill("✅ Compatibility alias tests passed")

# ===== EDGE CASE TESTS =====

test_start("Edge Cases and Error Handling")

# Test empty string handling
assert_eq_string(string_to_upper(""), "")
assert_eq_string(string_to_lower(""), "")
assert_eq_string(string_trim(""), "")
assert_eq_string(string_reverse(""), "")

# Test single character strings
assert_eq_string(string_to_upper("a"), "A")
assert_eq_string(string_reverse("x"), "x")
assert_true(string_contains("a", "a"))

# Test boundary conditions
assert_eq_string(string_substring("test", 0, 0), "")
assert_eq_string(string_substring("test", 4, 0), "")
assert_eq_string(string_slice("test", 0, 0), "")

vibez.spill("✅ Edge case tests passed")

# ===== PERFORMANCE AND STRESS TESTS =====

test_start("Performance and Stress Tests")

# Test with moderately large strings
sus large_string tea = string_repeat("test", 100)
assert_eq_int(string_length(large_string), 400)
assert_true(string_contains(large_string, "test"))

# Test with many operations
sus test_string tea = "The Quick Brown Fox Jumps Over The Lazy Dog"
sus lower_string tea = string_to_lower(test_string)
sus upper_string tea = string_to_upper(test_string)
sus trimmed tea = string_trim("  " + test_string + "  ")
assert_eq_string(trimmed, test_string)

vibez.spill("✅ Performance and stress tests passed")

# Print final test summary
print_test_summary()
vibez.spill("🎉 StringZ Enhanced Module - All comprehensive tests completed!")
vibez.spill("📊 Total functions tested: 50+ core string operations")
vibez.spill("🔬 Coverage: Core, Search, Case, Trim, Slice, Split, Replace, Pad, Validate, Advanced, Format")
vibez.spill("✅ Migration from Rust stdlib modules successful!")
