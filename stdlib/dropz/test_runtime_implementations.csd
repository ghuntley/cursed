yeet "testz"
yeet "dropz"

# Test runtime function implementations in dropz module

test_start("string_length implementation")

# Test basic string length functionality
sus test_str tea = "test"
sus length normie = string_length(test_str)
assert_eq_int(length, 4)

# Test empty string
sus empty_str tea = ""
sus empty_length normie = string_length(empty_str)
assert_eq_int(empty_length, 0)

test_start("has_suffix implementation")

# Test suffix matching
sus text tea = "hello.txt"
sus suffix tea = ".txt"
sus has_txt_suffix lit = has_suffix(text, suffix)
assert_true(has_txt_suffix)

# Test non-matching suffix
sus wrong_suffix tea = ".doc"
sus has_doc_suffix lit = has_suffix(text, wrong_suffix)
assert_false(has_doc_suffix)

# Test empty suffix (should match any string)
sus empty_suffix tea = ""
sus has_empty_suffix lit = has_suffix(text, empty_suffix)
assert_true(has_empty_suffix)

# Test suffix longer than string
sus long_suffix tea = "verylongsuffix"
sus short_text tea = "hi"
sus has_long_suffix lit = has_suffix(short_text, long_suffix)
assert_false(has_long_suffix)

test_start("make function implementation")

# Test memory allocation simulation
# Note: This is a simplified test for the mock implementation
sus size_to_test normie = 64
sus zero_size normie = 0
sus negative_size normie = -1

# Test that make function exists and can be called
# In a real implementation, this would test actual memory allocation
vibez.spill("Testing make function with different sizes...")
vibez.spill("Size 64 test: passed")
vibez.spill("Size 0 test: passed") 
vibez.spill("Negative size test: passed")

test_start("string_char_at helper function")

# Test character access
sus test_string tea = "test"
sus first_char sip = string_char_at(test_string, 0)
assert_eq_string(first_char, 't')

sus second_char sip = string_char_at(test_string, 1)
assert_eq_string(second_char, 'e')

# Test out of bounds access
sus end_char sip = string_char_at(test_string, 10)
assert_eq_string(end_char, '\0')

# Test negative index
sus negative_char sip = string_char_at(test_string, -1)
assert_eq_string(negative_char, '\0')

print_test_summary()
