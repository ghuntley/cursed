yeet "testz"
yeet "unicode/string_processing"

test_start("Unicode String Processing Test Suite")

# Test UTF-8 encoding/decoding
test_start("UTF-8 Encoding/Decoding")

# Test ASCII character
sus ascii_bytes []byte = utf8_encode_char(65)  # 'A'
assert_eq_int(len(ascii_bytes), 1)
assert_eq_int(normie(ascii_bytes[0]), 65)

# Test 2-byte character
sus two_byte_bytes []byte = utf8_encode_char(0x00E9)  # 'é'
assert_eq_int(len(two_byte_bytes), 2)

# Test 3-byte character  
sus three_byte_bytes []byte = utf8_encode_char(0x20AC)  # '€'
assert_eq_int(len(three_byte_bytes), 3)

# Test 4-byte character
sus four_byte_bytes []byte = utf8_encode_char(0x1F600)  # Emoji
assert_eq_int(len(four_byte_bytes), 4)

# Test decoding
sus decoded_char, byte_count = utf8_decode_char(ascii_bytes, 0)
assert_eq_int(decoded_char, 65)
assert_eq_int(byte_count, 1)

test_start("Unicode Character Classification")

# Test ASCII letters
assert_true(is_unicode_letter(65))   # 'A'
assert_true(is_unicode_letter(97))   # 'a'
assert_false(is_unicode_letter(48))  # '0'

# Test ASCII digits
assert_true(is_unicode_digit(48))    # '0'
assert_true(is_unicode_digit(57))    # '9'
assert_false(is_unicode_digit(65))   # 'A'

# Test whitespace
assert_true(is_unicode_whitespace(32))   # space
assert_true(is_unicode_whitespace(9))    # tab
assert_false(is_unicode_whitespace(65))  # 'A'

# Test punctuation
assert_true(is_unicode_punctuation(33))  # '!'
assert_true(is_unicode_punctuation(46))  # '.'
assert_false(is_unicode_punctuation(65)) # 'A'

test_start("Case Conversion")

# Test uppercase conversion
assert_eq_int(unicode_to_upper_codepoint(97), 65)    # 'a' -> 'A'
assert_eq_int(unicode_to_upper_codepoint(122), 90)   # 'z' -> 'Z'
assert_eq_int(unicode_to_upper_codepoint(65), 65)    # 'A' -> 'A' (no change)

# Test lowercase conversion
assert_eq_int(unicode_to_lower_codepoint(65), 97)    # 'A' -> 'a'
assert_eq_int(unicode_to_lower_codepoint(90), 122)   # 'Z' -> 'z'
assert_eq_int(unicode_to_lower_codepoint(97), 97)    # 'a' -> 'a' (no change)

# Test Greek case conversion
assert_eq_int(unicode_to_upper_codepoint(0x03B1), 0x0391)  # α -> Α
assert_eq_int(unicode_to_lower_codepoint(0x0391), 0x03B1)  # Α -> α

test_start("Unicode Categories")

# Test category classification
assert_eq_int(get_unicode_category(65), UNICODE_CATEGORY_LU)   # 'A' uppercase
assert_eq_int(get_unicode_category(97), UNICODE_CATEGORY_LL)   # 'a' lowercase
assert_eq_int(get_unicode_category(48), UNICODE_CATEGORY_ND)   # '0' digit
assert_eq_int(get_unicode_category(32), UNICODE_CATEGORY_ZS)   # ' ' space

# Test CJK characters
assert_eq_int(get_unicode_category(0x4E00), UNICODE_CATEGORY_LO)  # CJK ideograph

# Test Arabic characters
assert_eq_int(get_unicode_category(0x0627), UNICODE_CATEGORY_LO)  # Arabic letter

# Test Emoji
assert_eq_int(get_unicode_category(0x1F600), UNICODE_CATEGORY_SO)  # Emoji

test_start("String Validation")

# Test valid UTF-8 detection (simplified test)
assert_true(is_valid_utf8_string("hello"))
assert_true(is_nfc_normalized("hello"))

test_start("Word Boundaries")

# Test word boundary detection
assert_true(is_word_boundary("hello", 0))    # Start of string
assert_true(is_word_boundary("hello", 5))    # End of string
assert_true(is_grapheme_boundary("hello", 1)) # Between characters

test_start("String Length")

# Test character counting
assert_eq_int(unicode_string_length("hello"), 5)
assert_eq_int(grapheme_count("hello"), 5)

print_test_summary()
