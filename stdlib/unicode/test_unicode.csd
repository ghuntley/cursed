yeet "testz"
yeet "unicode"

# Comprehensive Unicode and UTF-8 Testing

test_start("UTF-8 sequence length detection")
assert_eq_int(utf8_sequence_length(0x41), 1)     # ASCII 'A'
assert_eq_int(utf8_sequence_length(0xC2), 2)     # 2-byte start
assert_eq_int(utf8_sequence_length(0xE0), 3)     # 3-byte start
assert_eq_int(utf8_sequence_length(0xF0), 4)     # 4-byte start
assert_eq_int(utf8_sequence_length(0xFF), 0)     # Invalid

test_start("UTF-8 byte validation")
assert_true(is_utf8_start_byte(0x41))            # ASCII
assert_true(is_utf8_start_byte(0xC2))            # 2-byte start
assert_true(is_utf8_start_byte(0xE0))            # 3-byte start
assert_true(is_utf8_start_byte(0xF0))            # 4-byte start
assert_false(is_utf8_start_byte(0x80))           # Continuation byte

test_start("UTF-8 continuation byte validation")
assert_true(is_utf8_continuation_byte(0x80))     # Valid continuation
assert_true(is_utf8_continuation_byte(0xBF))     # Valid continuation
assert_false(is_utf8_continuation_byte(0x41))    # ASCII
assert_false(is_utf8_continuation_byte(0xC0))    # Start byte

test_start("Unicode character classification")
assert_true(is_ascii(0x41))                      # 'A'
assert_true(is_ascii(0x7F))                      # DEL
assert_false(is_ascii(0x80))                     # Beyond ASCII
assert_false(is_ascii(0x100))                    # Beyond ASCII

test_start("Latin-1 character classification")
assert_true(is_latin1(0x41))                     # 'A'
assert_true(is_latin1(0xFF))                     # ÿ
assert_false(is_latin1(0x100))                   # Beyond Latin-1

test_start("BMP character classification")
assert_true(is_bmp(0x41))                        # 'A'
assert_true(is_bmp(0xFFFF))                      # Last BMP character
assert_false(is_bmp(0x10000))                    # Beyond BMP

test_start("Valid Unicode codepoint check")
assert_true(is_valid_unicode(0x0))               # Null
assert_true(is_valid_unicode(0x41))              # 'A'
assert_true(is_valid_unicode(0x10FFFF))          # Last valid Unicode
assert_false(is_valid_unicode(0x110000))         # Beyond Unicode

test_start("Unicode digit classification")
assert_true(is_unicode_digit(0x30))              # '0'
assert_true(is_unicode_digit(0x39))              # '9'
assert_true(is_unicode_digit(0x0660))            # Arabic-Indic digit
assert_false(is_unicode_digit(0x41))             # 'A'

test_start("Unicode letter classification")
assert_true(is_unicode_letter(0x41))             # 'A'
assert_true(is_unicode_letter(0x5A))             # 'Z'
assert_true(is_unicode_letter(0x61))             # 'a'
assert_true(is_unicode_letter(0x7A))             # 'z'
assert_true(is_unicode_letter(0x00C0))           # À
assert_false(is_unicode_letter(0x30))            # '0'

test_start("Unicode whitespace classification")
assert_true(is_unicode_whitespace(0x0020))       # Space
assert_true(is_unicode_whitespace(0x0009))       # Tab
assert_true(is_unicode_whitespace(0x000A))       # Line Feed
assert_true(is_unicode_whitespace(0x000D))       # Carriage Return
assert_true(is_unicode_whitespace(0x00A0))       # No-Break Space
assert_true(is_unicode_whitespace(0x3000))       # Ideographic Space
assert_false(is_unicode_whitespace(0x41))        # 'A'

test_start("Unicode case conversion")
assert_eq_int(to_unicode_upper(0x61), 0x41)     # 'a' to 'A'
assert_eq_int(to_unicode_upper(0x7A), 0x5A)     # 'z' to 'Z'
assert_eq_int(to_unicode_upper(0x41), 0x41)     # 'A' unchanged
assert_eq_int(to_unicode_upper(0x00E0), 0x00C0) # à to À

test_start("Unicode case conversion (lower)")
assert_eq_int(to_unicode_lower(0x41), 0x61)     # 'A' to 'a'
assert_eq_int(to_unicode_lower(0x5A), 0x7A)     # 'Z' to 'z'
assert_eq_int(to_unicode_lower(0x61), 0x61)     # 'a' unchanged
assert_eq_int(to_unicode_lower(0x00C0), 0x00E0) # À to à

test_start("UTF-8 codepoint conversion")
assert_eq_int(utf8_to_codepoint([0x41], 0), 0x41)           # ASCII 'A'
assert_eq_int(utf8_to_codepoint([0xC2, 0x80], 0), 0x80)    # 2-byte
assert_eq_int(utf8_to_codepoint([0xE0, 0xA0, 0x80], 0), 0x800) # 3-byte

test_start("Codepoint to UTF-8 conversion")
sus ascii_bytes []normie = codepoint_to_utf8(0x41)
assert_eq_int(len(ascii_bytes), 1)
assert_eq_int(ascii_bytes[0], 0x41)

sus two_byte_result []normie = codepoint_to_utf8(0x80)
assert_eq_int(len(two_byte_result), 2)
assert_eq_int(two_byte_result[0], 0xC2)
assert_eq_int(two_byte_result[1], 0x80)

test_start("Unicode block detection")
assert_eq_string(get_unicode_block(0x41), "Basic Latin")
assert_eq_string(get_unicode_block(0x00C0), "Latin-1 Supplement")
assert_eq_string(get_unicode_block(0x0100), "Latin Extended-A")
assert_eq_string(get_unicode_block(0x0370), "Greek and Coptic")
assert_eq_string(get_unicode_block(0x0400), "Cyrillic")
assert_eq_string(get_unicode_block(0x0590), "Hebrew")
assert_eq_string(get_unicode_block(0x0600), "Arabic")
assert_eq_string(get_unicode_block(0x0900), "Devanagari")
assert_eq_string(get_unicode_block(0x3040), "Hiragana")
assert_eq_string(get_unicode_block(0x30A0), "Katakana")
assert_eq_string(get_unicode_block(0x4E00), "CJK Unified Ideographs")
assert_eq_string(get_unicode_block(0xAC00), "Hangul Syllables")

test_start("Extended Unicode block detection")
assert_eq_string(get_unicode_block(0x2000), "General Punctuation")
assert_eq_string(get_unicode_block(0x2190), "Arrows")
assert_eq_string(get_unicode_block(0x2200), "Mathematical Operators")
assert_eq_string(get_unicode_block(0x2500), "Box Drawing")
assert_eq_string(get_unicode_block(0x2600), "Miscellaneous Symbols")
assert_eq_string(get_unicode_block(0x2800), "Braille Patterns")
assert_eq_string(get_unicode_block(0x1D400), "Mathematical Alphanumeric Symbols")
assert_eq_string(get_unicode_block(0x1F600), "Unknown")

test_start("UTF-8 sequence validation")
sus valid_ascii []normie = [0x41, 0x42, 0x43]
assert_true(validate_utf8_sequence(valid_ascii, 0))
assert_true(validate_utf8_sequence(valid_ascii, 1))
assert_true(validate_utf8_sequence(valid_ascii, 2))

sus valid_two_byte []normie = [0xC2, 0x80, 0x41]
assert_true(validate_utf8_sequence(valid_two_byte, 0))
assert_true(validate_utf8_sequence(valid_two_byte, 2))

sus invalid_sequence []normie = [0x80, 0x41]  # Continuation byte first
assert_false(validate_utf8_sequence(invalid_sequence, 0))

test_start("Comprehensive UTF-8 validation")
# Test various valid UTF-8 sequences
sus simple_ascii []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F]  # "Hello"
assert_true(validate_utf8_sequence(simple_ascii, 0))
assert_true(validate_utf8_sequence(simple_ascii, 1))
assert_true(validate_utf8_sequence(simple_ascii, 2))
assert_true(validate_utf8_sequence(simple_ascii, 3))
assert_true(validate_utf8_sequence(simple_ascii, 4))

# Test 2-byte sequences
sus two_byte_seq []normie = [0xC2, 0xA9, 0x41]  # © + A
assert_true(validate_utf8_sequence(two_byte_seq, 0))
assert_true(validate_utf8_sequence(two_byte_seq, 2))

# Test 3-byte sequences
sus three_byte_seq []normie = [0xE2, 0x82, 0xAC, 0x41]  # € + A
assert_true(validate_utf8_sequence(three_byte_seq, 0))
assert_true(validate_utf8_sequence(three_byte_seq, 3))

# Test 4-byte sequences
sus four_byte_seq []normie = [0xF0, 0x9F, 0x98, 0x80, 0x41]  # 😀 + A
assert_true(validate_utf8_sequence(four_byte_seq, 0))
assert_true(validate_utf8_sequence(four_byte_seq, 4))

test_start("Invalid UTF-8 sequences")
# Test invalid start bytes
sus invalid_start []normie = [0xFF, 0x41]
assert_false(validate_utf8_sequence(invalid_start, 0))

# Test incomplete sequences
sus incomplete_two []normie = [0xC2]  # Missing continuation byte
assert_false(validate_utf8_sequence(incomplete_two, 0))

sus incomplete_three []normie = [0xE2, 0x82]  # Missing continuation byte
assert_false(validate_utf8_sequence(incomplete_three, 0))

# Test invalid continuation bytes
sus invalid_continuation []normie = [0xC2, 0x41]  # ASCII instead of continuation
assert_false(validate_utf8_sequence(invalid_continuation, 0))

test_start("UTF-8 character counting")
# Note: These tests would work with actual string_to_bytes implementation
# For now, testing the logic with mock data

test_start("ASCII string detection")
# These would test actual strings when string_to_bytes is implemented
vibez.spill("ASCII string detection tests require string_to_bytes implementation")

test_start("Unicode normalization")
# Basic normalization test
assert_eq_string(normalize_unicode_nfc("test"), "test")
assert_eq_string(normalize_unicode_nfc(""), "")

test_start("Complex Unicode scenarios")
# Test edge cases and complex Unicode scenarios
assert_true(is_valid_unicode(0x0))           # Null character
assert_true(is_valid_unicode(0x10FFFF))      # Last valid Unicode codepoint
assert_false(is_valid_unicode(0x110000))     # Beyond Unicode range
assert_false(is_valid_unicode(-1))           # Negative codepoint

# Test surrogate pairs (should be invalid in UTF-8)
assert_false(is_valid_unicode(0xD800))       # High surrogate start
assert_false(is_valid_unicode(0xDFFF))       # Low surrogate end

test_start("Unicode boundary conditions")
# Test boundaries between Unicode blocks
assert_eq_string(get_unicode_block(0x007F), "Basic Latin")
assert_eq_string(get_unicode_block(0x0080), "Latin-1 Supplement")
assert_eq_string(get_unicode_block(0x00FF), "Latin-1 Supplement")
assert_eq_string(get_unicode_block(0x0100), "Latin Extended-A")

test_start("UTF-8 encoding edge cases")
# Test edge cases in UTF-8 encoding
sus edge_cases []normie = []

# Test maximum values for each sequence length
assert_eq_int(utf8_sequence_length(0x7F), 1)   # Max 1-byte
assert_eq_int(utf8_sequence_length(0xC1), 2)   # Min 2-byte (technically invalid)
assert_eq_int(utf8_sequence_length(0xDF), 2)   # Max 2-byte
assert_eq_int(utf8_sequence_length(0xE0), 3)   # Min 3-byte
assert_eq_int(utf8_sequence_length(0xEF), 3)   # Max 3-byte
assert_eq_int(utf8_sequence_length(0xF0), 4)   # Min 4-byte
assert_eq_int(utf8_sequence_length(0xF7), 4)   # Max 4-byte

test_start("Unicode case conversion edge cases")
# Test case conversion edge cases
assert_eq_int(to_unicode_upper(0x00DF), 0x00DF)  # ß has no single uppercase
assert_eq_int(to_unicode_lower(0x0130), 0x0130)  # İ special case
assert_eq_int(to_unicode_upper(0x00B5), 0x00B5)  # µ special case

# Test characters that don't have case variants
assert_eq_int(to_unicode_upper(0x30), 0x30)      # '0' unchanged
assert_eq_int(to_unicode_lower(0x30), 0x30)      # '0' unchanged
assert_eq_int(to_unicode_upper(0x2603), 0x2603)  # ☃ (snowman) unchanged
assert_eq_int(to_unicode_lower(0x2603), 0x2603)  # ☃ (snowman) unchanged

test_start("Multi-language character classification")
# Test character classification across languages
assert_true(is_unicode_letter(0x0391))   # Greek Α
assert_true(is_unicode_letter(0x0410))   # Cyrillic А
assert_true(is_unicode_letter(0x05D0))   # Hebrew א
assert_true(is_unicode_letter(0x0627))   # Arabic ا
assert_true(is_unicode_letter(0x0905))   # Devanagari अ
assert_true(is_unicode_letter(0x3042))   # Hiragana あ
assert_true(is_unicode_letter(0x30A2))   # Katakana ア
assert_true(is_unicode_letter(0x4E00))   # CJK 一

# Test digits in various scripts
assert_true(is_unicode_digit(0x0966))    # Devanagari ०
assert_true(is_unicode_digit(0x0660))    # Arabic-Indic ٠
assert_true(is_unicode_digit(0x06F0))    # Extended Arabic-Indic ۰

test_start("Unicode whitespace comprehensive")
# Test various Unicode whitespace characters
assert_true(is_unicode_whitespace(0x0009))   # Tab
assert_true(is_unicode_whitespace(0x000A))   # Line Feed
assert_true(is_unicode_whitespace(0x000B))   # Vertical Tab
assert_true(is_unicode_whitespace(0x000C))   # Form Feed
assert_true(is_unicode_whitespace(0x000D))   # Carriage Return
assert_true(is_unicode_whitespace(0x0020))   # Space
assert_true(is_unicode_whitespace(0x0085))   # Next Line
assert_true(is_unicode_whitespace(0x00A0))   # No-Break Space
assert_true(is_unicode_whitespace(0x1680))   # Ogham Space Mark
assert_true(is_unicode_whitespace(0x2000))   # En Quad
assert_true(is_unicode_whitespace(0x2001))   # Em Quad
assert_true(is_unicode_whitespace(0x2002))   # En Space
assert_true(is_unicode_whitespace(0x2003))   # Em Space
assert_true(is_unicode_whitespace(0x2004))   # Three-Per-Em Space
assert_true(is_unicode_whitespace(0x2005))   # Four-Per-Em Space
assert_true(is_unicode_whitespace(0x2006))   # Six-Per-Em Space
assert_true(is_unicode_whitespace(0x2007))   # Figure Space
assert_true(is_unicode_whitespace(0x2008))   # Punctuation Space
assert_true(is_unicode_whitespace(0x2009))   # Thin Space
assert_true(is_unicode_whitespace(0x200A))   # Hair Space
assert_true(is_unicode_whitespace(0x2028))   # Line Separator
assert_true(is_unicode_whitespace(0x2029))   # Paragraph Separator
assert_true(is_unicode_whitespace(0x202F))   # Narrow No-Break Space
assert_true(is_unicode_whitespace(0x205F))   # Medium Mathematical Space
assert_true(is_unicode_whitespace(0x3000))   # Ideographic Space

test_start("Private Use Area detection")
# Test Private Use Area ranges
assert_eq_string(get_unicode_block(0xE000), "Private Use Area")
assert_eq_string(get_unicode_block(0xF8FF), "Private Use Area")
assert_eq_string(get_unicode_block(0xF0000), "Supplementary Private Use Area-A")
assert_eq_string(get_unicode_block(0xFFFFF), "Supplementary Private Use Area-A")
assert_eq_string(get_unicode_block(0x100000), "Supplementary Private Use Area-B")
assert_eq_string(get_unicode_block(0x10FFFF), "Supplementary Private Use Area-B")

test_start("Surrogate handling")
# Test surrogate ranges (should be detected as invalid)
assert_eq_string(get_unicode_block(0xD800), "High Surrogates")
assert_eq_string(get_unicode_block(0xDB7F), "High Surrogates")
assert_eq_string(get_unicode_block(0xDB80), "High Private Use Surrogates")
assert_eq_string(get_unicode_block(0xDBFF), "High Private Use Surrogates")
assert_eq_string(get_unicode_block(0xDC00), "Low Surrogates")
assert_eq_string(get_unicode_block(0xDFFF), "Low Surrogates")

test_start("Mathematical symbols")
# Test mathematical symbol ranges
assert_eq_string(get_unicode_block(0x2200), "Mathematical Operators")
assert_eq_string(get_unicode_block(0x2300), "Miscellaneous Technical")
assert_eq_string(get_unicode_block(0x27C0), "Miscellaneous Mathematical Symbols-A")
assert_eq_string(get_unicode_block(0x2980), "Miscellaneous Mathematical Symbols-B")
assert_eq_string(get_unicode_block(0x2A00), "Supplemental Mathematical Operators")
assert_eq_string(get_unicode_block(0x1D400), "Mathematical Alphanumeric Symbols")

test_start("CJK ranges")
# Test CJK character ranges
assert_eq_string(get_unicode_block(0x2E80), "CJK Radicals Supplement")
assert_eq_string(get_unicode_block(0x2F00), "Kangxi Radicals")
assert_eq_string(get_unicode_block(0x3000), "CJK Symbols and Punctuation")
assert_eq_string(get_unicode_block(0x3400), "CJK Unified Ideographs Extension A")
assert_eq_string(get_unicode_block(0x4E00), "CJK Unified Ideographs")
assert_eq_string(get_unicode_block(0x20000), "CJK Unified Ideographs Extension B")
assert_eq_string(get_unicode_block(0xF900), "CJK Compatibility Ideographs")
assert_eq_string(get_unicode_block(0x2F800), "CJK Compatibility Ideographs Supplement")

test_start("Hangul ranges")
# Test Hangul character ranges
assert_eq_string(get_unicode_block(0x1100), "Hangul Jamo")
assert_eq_string(get_unicode_block(0x3130), "Hangul Compatibility Jamo")
assert_eq_string(get_unicode_block(0xAC00), "Hangul Syllables")

test_start("Presentation forms")
# Test presentation form ranges
assert_eq_string(get_unicode_block(0xFB00), "Alphabetic Presentation Forms")
assert_eq_string(get_unicode_block(0xFB50), "Arabic Presentation Forms-A")
assert_eq_string(get_unicode_block(0xFE70), "Arabic Presentation Forms-B")
assert_eq_string(get_unicode_block(0xFF00), "Halfwidth and Fullwidth Forms")

test_start("Variation selectors")
# Test variation selector ranges
assert_eq_string(get_unicode_block(0xFE00), "Variation Selectors")
assert_eq_string(get_unicode_block(0xE0100), "Variation Selectors Supplement")

test_start("Special Unicode ranges")
# Test special Unicode ranges
assert_eq_string(get_unicode_block(0x2400), "Control Pictures")
assert_eq_string(get_unicode_block(0x2440), "Optical Character Recognition")
assert_eq_string(get_unicode_block(0x2460), "Enclosed Alphanumerics")
assert_eq_string(get_unicode_block(0x2500), "Box Drawing")
assert_eq_string(get_unicode_block(0x2580), "Block Elements")
assert_eq_string(get_unicode_block(0x25A0), "Geometric Shapes")
assert_eq_string(get_unicode_block(0x2600), "Miscellaneous Symbols")
assert_eq_string(get_unicode_block(0x2700), "Dingbats")
assert_eq_string(get_unicode_block(0xFFF0), "Specials")

print_test_summary()
