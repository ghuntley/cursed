yeet "testz"
yeet "unicode/string_processing"

test_start("Unicode String Processing Test Suite")

fr fr Test UTF-8 encoding/decoding
test_start("UTF-8 Encoding/Decoding")

fr fr Test ASCII character
sus ascii_bytes byte[value] = utf8_encode_char(65) fr fr 'A'
assert_eq_int(len(ascii_bytes), 1)
assert_eq_int(normie(ascii_bytes[0]), 65)

fr fr Test 2-byte character
sus two_byte_bytes byte[value] = utf8_encode_char(0x00E9) fr fr 'é'
assert_eq_int(len(two_byte_bytes), 2)

fr fr Test 3-byte character  
sus three_byte_bytes byte[value] = utf8_encode_char(0x20AC) fr fr '€'
assert_eq_int(len(three_byte_bytes), 3)

fr fr Test 4-byte character
sus four_byte_bytes byte[value] = utf8_encode_char(0x1F600) fr fr Emoji
assert_eq_int(len(four_byte_bytes), 4)

fr fr Test decoding
sus decoded_char, byte_count = utf8_decode_char(ascii_bytes, 0)
assert_eq_int(decoded_char, 65)
assert_eq_int(byte_count, 1)

test_start("Unicode Character Classification")

fr fr Test ASCII letters
assert_true(is_unicode_letter(65)) fr fr 'A'
assert_true(is_unicode_letter(97)) fr fr 'a'
assert_false(is_unicode_letter(48)) fr fr '0'

fr fr Test ASCII digits
assert_true(is_unicode_digit(48)) fr fr '0'
assert_true(is_unicode_digit(57)) fr fr '9'
assert_false(is_unicode_digit(65)) fr fr 'A'

fr fr Test whitespace
assert_true(is_unicode_whitespace(32)) fr fr space
assert_true(is_unicode_whitespace(9)) fr fr tab
assert_false(is_unicode_whitespace(65)) fr fr 'A'

fr fr Test punctuation
assert_true(is_unicode_punctuation(33)) fr fr '!'
assert_true(is_unicode_punctuation(46)) fr fr '.'
assert_false(is_unicode_punctuation(65)) fr fr 'A'

test_start("Case Conversion")

fr fr Test uppercase conversion
assert_eq_int(unicode_to_upper_codepoint(97), 65) fr fr 'a' -> 'A'
assert_eq_int(unicode_to_upper_codepoint(122), 90) fr fr 'z' -> 'Z'
assert_eq_int(unicode_to_upper_codepoint(65), 65) fr fr 'A' -> 'A' (no change)

fr fr Test lowercase conversion
assert_eq_int(unicode_to_lower_codepoint(65), 97) fr fr 'A' -> 'a'
assert_eq_int(unicode_to_lower_codepoint(90), 122) fr fr 'Z' -> 'z'
assert_eq_int(unicode_to_lower_codepoint(97), 97) fr fr 'a' -> 'a' (no change)

fr fr Test Greek case conversion
assert_eq_int(unicode_to_upper_codepoint(0x03B1), 0x0391) fr fr α -> Α
assert_eq_int(unicode_to_lower_codepoint(0x0391), 0x03B1) fr fr Α -> α

test_start("Unicode Categories")

fr fr Test category classification
assert_eq_int(get_unicode_category(65), UNICODE_CATEGORY_LU) fr fr 'A' uppercase
assert_eq_int(get_unicode_category(97), UNICODE_CATEGORY_LL) fr fr 'a' lowercase
assert_eq_int(get_unicode_category(48), UNICODE_CATEGORY_ND) fr fr '0' digit
assert_eq_int(get_unicode_category(32), UNICODE_CATEGORY_ZS) fr fr ' ' space

fr fr Test CJK characters
assert_eq_int(get_unicode_category(0x4E00), UNICODE_CATEGORY_LO) fr fr CJK ideograph

fr fr Test Arabic characters
assert_eq_int(get_unicode_category(0x0627), UNICODE_CATEGORY_LO) fr fr Arabic letter

fr fr Test Emoji
assert_eq_int(get_unicode_category(0x1F600), UNICODE_CATEGORY_SO) fr fr Emoji

test_start("String Validation")

fr fr Test valid UTF-8 detection (simplified test)
assert_true(is_valid_utf8_string("hello"))
assert_true(is_nfc_normalized("hello"))

test_start("Word Boundaries")

fr fr Test word boundary detection
assert_true(is_word_boundary("hello", 0)) fr fr Start of string
assert_true(is_word_boundary("hello", 5)) fr fr End of string
assert_true(is_grapheme_boundary("hello", 1)) fr fr Between characters

test_start("String Length")

fr fr Test character counting
assert_eq_int(unicode_string_length("hello"), 5)
assert_eq_int(grapheme_count("hello"), 5)

print_test_summary()
