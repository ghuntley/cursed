yeet "testz"
yeet "vibez_optimized"

# Comprehensive test suite for vibez_optimized module
# Real functional tests for optimized I/O operations

test_start("test_string_concat_optimized")
# Test optimized string concatenation with multiple strings
sus parts []tea = ["Hello", " ", "World", "!"]
sus result tea = string_concat_optimized(parts)
assert_eq_tea(result, "Hello World!")

# Test with empty array
sus empty_parts []tea = []
sus empty_result tea = string_concat_optimized(empty_parts)
assert_eq_tea(empty_result, "")

# Test with single string
sus single_parts []tea = ["OnlyString"]
sus single_result tea = string_concat_optimized(single_parts)
assert_eq_tea(single_result, "OnlyString")

# Test with empty strings
sus with_empty []tea = ["Start", "", "End"]
sus with_empty_result tea = string_concat_optimized(with_empty)
assert_eq_tea(with_empty_result, "StartEnd")
print_test_summary()

test_start("test_string_find_optimized")
# Test finding substring in string
sus haystack tea = "Hello World Programming"
sus needle tea = "World"
sus position drip = string_find_optimized(haystack, needle)
assert_eq_int(position, 6)

# Test with non-existent substring
sus not_found drip = string_find_optimized(haystack, "xyz")
assert_eq_int(not_found, -1)

# Test with empty needle
sus empty_needle drip = string_find_optimized(haystack, "")
assert_eq_int(empty_needle, 0)

# Test needle longer than haystack
sus long_needle drip = string_find_optimized("short", "very long needle")
assert_eq_int(long_needle, -1)

# Test exact match
sus exact_match drip = string_find_optimized("test", "test")
assert_eq_int(exact_match, 0)
print_test_summary()

test_start("test_initialize_string_pool")
# Test string pool initialization
sus result lit = initialize_string_pool()
assert_true(result)

# Test re-initialization (should succeed)
sus result2 lit = initialize_string_pool()
assert_true(result2)
print_test_summary()

test_start("test_get_pooled_string")
# Initialize pool first
initialize_string_pool()

# Test getting pooled string of various sizes
sus small_string tea = get_pooled_string(16)
assert_true(len(small_string) == 16)

sus medium_string tea = get_pooled_string(64)
assert_true(len(medium_string) == 64)

sus large_string tea = get_pooled_string(256)
assert_true(len(large_string) == 256)

# Test with zero size
sus zero_string tea = get_pooled_string(0)
assert_true(len(zero_string) == 0)
print_test_summary()

test_start("test_spillf_optimized")
# Test optimized formatted printing
sus result lit = spillf_optimized("Hello %s, you are %d years old", "John", 25)
assert_true(result)

# Test with no format specifiers
sus simple_result lit = spillf_optimized("Simple message")
assert_true(simple_result)

# Test with multiple format specifiers
sus complex_result lit = spillf_optimized("Name: %s, Age: %d, Score: %f, Active: %b", "Alice", 30, 95.5, based)
assert_true(complex_result)
print_test_summary()

test_start("test_parse_format_string")
# Test parsing format string with specifiers
sus format_info lit = parse_format_string("Hello %s, you are %d")
assert_true(format_info)

# Test with no format specifiers
sus no_format_info lit = parse_format_string("Plain text")
assert_true(no_format_info)

# Test with complex format
sus complex_format_info lit = parse_format_string("%s: %d%% complete, %f average")
assert_true(complex_format_info)
print_test_summary()

test_start("test_execute_cached_format")
# Test executing cached format operations
sus cache_result lit = execute_cached_format("user_format", "User: %s", "John")
assert_true(cache_result)

# Test with different format
sus cache_result2 lit = execute_cached_format("number_format", "Value: %d", 42)
assert_true(cache_result2)
print_test_summary()

test_start("test_string_replace_optimized")
# Test optimized string replacement
sus original tea = "Hello World World"
sus replaced tea = string_replace_optimized(original, "World", "Universe")
assert_true(len(replaced) > 0)

# Test with no matches
sus no_match tea = string_replace_optimized("Hello", "xyz", "abc")
assert_eq_tea(no_match, "Hello")

# Test replacing with empty string
sus empty_replace tea = string_replace_optimized("Hello World", "World", "")
assert_eq_tea(empty_replace, "Hello ")
print_test_summary()

test_start("test_build_kmp_table")
# Test building KMP failure table
sus pattern tea = "ABABCABAB"
sus kmp_table []drip = build_kmp_table(pattern)
assert_true(len(kmp_table) == len(pattern))

# Test with simple pattern
sus simple_pattern tea = "AAA"
sus simple_table []drip = build_kmp_table(simple_pattern)
assert_true(len(simple_table) == 3)
print_test_summary()

test_start("test_build_bad_char_table")
# Test building Boyer-Moore bad character table
sus pattern tea = "EXAMPLE"
sus bad_char_table []drip = build_bad_char_table(pattern)
assert_true(len(bad_char_table) > 0)

# Test with repeating characters
sus repeat_pattern tea = "AAA"
sus repeat_table []drip = build_bad_char_table(repeat_pattern)
assert_true(len(repeat_table) > 0)
print_test_summary()

test_start("test_vectorized_string_copy")
# Test vectorized string copying
sus dest tea = get_pooled_string(100)
sus source tea = "This is a test string for vectorized copying"
vectorized_string_copy(dest, 0, source)
assert_true(len(dest) > 0)

# Test copying to different positions
sus dest2 tea = get_pooled_string(200)
vectorized_string_copy(dest2, 10, "offset test")
assert_true(len(dest2) > 0)
print_test_summary()

test_start("test_int_to_string_optimized")
# Test optimized integer to string conversion
sus result tea = int_to_string_optimized(12345)
assert_eq_tea(result, "12345")

# Test with negative number
sus negative tea = int_to_string_optimized(-789)
assert_eq_tea(negative, "-789")

# Test with zero
sus zero tea = int_to_string_optimized(0)
assert_eq_tea(zero, "0")

# Test with large number
sus large tea = int_to_string_optimized(2147483647)
assert_true(len(large) > 0)
print_test_summary()

test_start("test_spill_buffered")
# Test buffered output
sus result lit = spill_buffered("Test message 1")
assert_true(result)

sus result2 lit = spill_buffered("Test message 2")
assert_true(result2)

# Test with large message
sus large_message tea = "This is a very long message that should test the buffering system properly"
sus large_result lit = spill_buffered(large_message)
assert_true(large_result)
print_test_summary()

test_start("test_flush_output_buffer")
# Fill buffer first
spill_buffered("Message 1")
spill_buffered("Message 2")

# Test flushing
sus flush_result lit = flush_output_buffer()
assert_true(flush_result)

# Test flushing empty buffer
sus empty_flush_result lit = flush_output_buffer()
assert_true(empty_flush_result)
print_test_summary()

test_start("test_spill")
# Test basic spill functionality
sus result lit = spill("Hello World")
assert_true(result)

# Test with number
sus number_result lit = spill(42)
assert_true(number_result)

# Test with boolean
sus bool_result lit = spill(based)
assert_true(bool_result)
print_test_summary()

# Performance and stress tests
test_start("performance_string_operations")
initialize_string_pool()

# Test rapid string concatenation
bestie i := 0; i < 100; i++ {
    sus parts []tea = ["Part", string(i), "End"]
    sus result tea = string_concat_optimized(parts)
    assert_true(len(result) > 0)
}

# Test rapid string searches
bestie i := 0; i < 50; i++ {
    sus haystack tea = "Search in this long string for patterns " + string(i)
    sus position drip = string_find_optimized(haystack, "patterns")
    assert_true(position > 0)
}
print_test_summary()

# Edge case testing
test_start("edge_cases_vibez_optimized")
# Test with very long strings
sus long_string tea = ""
bestie i := 0; i < 1000; i++ {
    long_string = long_string + "x"
}
sus long_parts []tea = [long_string, "end"]
sus long_result tea = string_concat_optimized(long_parts)
assert_true(len(long_result) > 1000)

# Test format with many specifiers
sus many_specifiers tea = "%s %s %s %s %s %s %s %s %s %s"
sus many_format_result lit = spillf_optimized(many_specifiers, "1", "2", "3", "4", "5", "6", "7", "8", "9", "10")
assert_true(many_format_result)

# Test buffer overflow protection
bestie i := 0; i < 10; i++ {
    spill_buffered("Buffer test " + string(i))
}
sus overflow_flush lit = flush_output_buffer()
assert_true(overflow_flush)
print_test_summary()

# Integration test
test_start("integration_optimized_workflow")
# Initialize system
initialize_string_pool()

# Complex workflow test
sus base_text tea = get_pooled_string(256)
sus parts []tea = ["Processing", " item ", "42", " of ", "100"]
sus combined tea = string_concat_optimized(parts)

sus position drip = string_find_optimized(combined, "item")
assert_true(position > 0)

sus replaced tea = string_replace_optimized(combined, "42", "43")
assert_true(len(replaced) > 0)

spillf_optimized("Result: %s", replaced)
flush_output_buffer()
print_test_summary()
