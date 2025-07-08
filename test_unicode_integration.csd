yeet "unicode"
yeet "string_integration"

# Unicode Integration Test

vibez.spill("=== Unicode Integration Test ===")

# Test basic Unicode validation and classification
vibez.spill("Testing Unicode character classification...")

# Test ASCII characters
sus ascii_a normie = 0x41  # 'A'
vibez.spill("Character 'A' (0x41):")
vibez.spill("  Is ASCII: ", is_ascii(ascii_a))
vibez.spill("  Is letter: ", is_unicode_letter(ascii_a))
vibez.spill("  Is digit: ", is_unicode_digit(ascii_a))
vibez.spill("  Unicode block: ", get_unicode_block(ascii_a))

# Test digits
sus digit_0 normie = 0x30  # '0'
vibez.spill("\nCharacter '0' (0x30):")
vibez.spill("  Is ASCII: ", is_ascii(digit_0))
vibez.spill("  Is letter: ", is_unicode_letter(digit_0))
vibez.spill("  Is digit: ", is_unicode_digit(digit_0))
vibez.spill("  Unicode block: ", get_unicode_block(digit_0))

# Test Unicode characters beyond ASCII
sus greek_alpha normie = 0x0391  # Α (Greek capital alpha)
vibez.spill("\nGreek Alpha (0x0391):")
vibez.spill("  Is ASCII: ", is_ascii(greek_alpha))
vibez.spill("  Is letter: ", is_unicode_letter(greek_alpha))
vibez.spill("  Is BMP: ", is_bmp(greek_alpha))
vibez.spill("  Unicode block: ", get_unicode_block(greek_alpha))

# Test Arabic characters
sus arabic_alif normie = 0x0627  # ا (Arabic letter alif)
vibez.spill("\nArabic Alif (0x0627):")
vibez.spill("  Is ASCII: ", is_ascii(arabic_alif))
vibez.spill("  Is letter: ", is_unicode_letter(arabic_alif))
vibez.spill("  Unicode block: ", get_unicode_block(arabic_alif))

# Test CJK characters
sus cjk_char normie = 0x4E00  # 一 (CJK unified ideograph)
vibez.spill("\nCJK Character (0x4E00):")
vibez.spill("  Is ASCII: ", is_ascii(cjk_char))
vibez.spill("  Is letter: ", is_unicode_letter(cjk_char))
vibez.spill("  Unicode block: ", get_unicode_block(cjk_char))

# Test UTF-8 encoding
vibez.spill("\nTesting UTF-8 encoding...")

# ASCII character encoding
sus ascii_bytes []normie = codepoint_to_utf8(ascii_a)
vibez.spill("ASCII 'A' UTF-8 bytes: ", len(ascii_bytes))
bestie len(ascii_bytes) > 0 {
    vibez.spill("  Byte 0: ", ascii_bytes[0])
}

# 2-byte UTF-8 character
sus latin_a_grave normie = 0x00C0  # À
sus latin_bytes []normie = codepoint_to_utf8(latin_a_grave)
vibez.spill("Latin À UTF-8 bytes: ", len(latin_bytes))
bestie len(latin_bytes) > 1 {
    vibez.spill("  Byte 0: ", latin_bytes[0])
    vibez.spill("  Byte 1: ", latin_bytes[1])
}

# 3-byte UTF-8 character (Greek)
sus greek_bytes []normie = codepoint_to_utf8(greek_alpha)
vibez.spill("Greek Α UTF-8 bytes: ", len(greek_bytes))
bestie len(greek_bytes) > 2 {
    vibez.spill("  Byte 0: ", greek_bytes[0])
    vibez.spill("  Byte 1: ", greek_bytes[1])
    vibez.spill("  Byte 2: ", greek_bytes[2])
}

# Test case conversion
vibez.spill("\nTesting case conversion...")

sus lowercase_a normie = 0x61  # 'a'
sus uppercase_result normie = to_unicode_upper(lowercase_a)
vibez.spill("'a' to uppercase: ", uppercase_result, " (expected: 65)")

sus uppercase_a normie = 0x41  # 'A'
sus lowercase_result normie = to_unicode_lower(uppercase_a)
vibez.spill("'A' to lowercase: ", lowercase_result, " (expected: 97)")

# Test Latin-1 case conversion
sus latin_a_grave_lower normie = 0x00E0  # à
sus latin_upper_result normie = to_unicode_upper(latin_a_grave_lower)
vibez.spill("'à' to uppercase: ", latin_upper_result, " (expected: 192)")

# Test whitespace classification
vibez.spill("\nTesting whitespace classification...")

sus space normie = 0x0020  # Regular space
vibez.spill("Space is whitespace: ", is_unicode_whitespace(space))

sus tab normie = 0x0009  # Tab
vibez.spill("Tab is whitespace: ", is_unicode_whitespace(tab))

sus newline normie = 0x000A  # Line feed
vibez.spill("Newline is whitespace: ", is_unicode_whitespace(newline))

sus ideographic_space normie = 0x3000  # Ideographic space
vibez.spill("Ideographic space is whitespace: ", is_unicode_whitespace(ideographic_space))

sus letter_not_space normie = 0x41  # 'A'
vibez.spill("Letter 'A' is whitespace: ", is_unicode_whitespace(letter_not_space))

# Test validation
vibez.spill("\nTesting UTF-8 validation...")

# Test valid sequences
sus valid_ascii []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F]  # "Hello"
vibez.spill("Validating ASCII 'Hello': ", validate_utf8_sequence(valid_ascii, 0))

sus valid_2byte []normie = [0xC2, 0xA9]  # © copyright symbol
vibez.spill("Validating 2-byte sequence: ", validate_utf8_sequence(valid_2byte, 0))

sus valid_3byte []normie = [0xE2, 0x82, 0xAC]  # € euro symbol
vibez.spill("Validating 3-byte sequence: ", validate_utf8_sequence(valid_3byte, 0))

# Test invalid sequences
sus invalid_start []normie = [0xFF]  # Invalid start byte
vibez.spill("Validating invalid start byte: ", validate_utf8_sequence(invalid_start, 0))

sus incomplete []normie = [0xC2]  # Incomplete 2-byte sequence
vibez.spill("Validating incomplete sequence: ", validate_utf8_sequence(incomplete, 0))

# Test sequence length detection
vibez.spill("\nTesting sequence length detection...")
vibez.spill("ASCII sequence length: ", utf8_sequence_length(0x41))
vibez.spill("2-byte sequence length: ", utf8_sequence_length(0xC2))
vibez.spill("3-byte sequence length: ", utf8_sequence_length(0xE0))
vibez.spill("4-byte sequence length: ", utf8_sequence_length(0xF0))
vibez.spill("Invalid byte length: ", utf8_sequence_length(0xFF))

# Test Unicode boundaries
vibez.spill("\nTesting Unicode boundaries...")

vibez.spill("Max ASCII (0x7F) is ASCII: ", is_ascii(0x7F))
vibez.spill("Beyond ASCII (0x80) is ASCII: ", is_ascii(0x80))
vibez.spill("Max Latin-1 (0xFF) is Latin-1: ", is_latin1(0xFF))
vibez.spill("Beyond Latin-1 (0x100) is Latin-1: ", is_latin1(0x100))
vibez.spill("Max BMP (0xFFFF) is BMP: ", is_bmp(0xFFFF))
vibez.spill("Beyond BMP (0x10000) is BMP: ", is_bmp(0x10000))
vibez.spill("Max Unicode (0x10FFFF) is valid: ", is_valid_unicode(0x10FFFF))
vibez.spill("Beyond Unicode (0x110000) is valid: ", is_valid_unicode(0x110000))

# Test extended Unicode blocks
vibez.spill("\nTesting extended Unicode blocks...")

vibez.spill("Cyrillic block: ", get_unicode_block(0x0410))  # А
vibez.spill("Hebrew block: ", get_unicode_block(0x05D0))    # א
vibez.spill("Arabic block: ", get_unicode_block(0x0627))    # ا
vibez.spill("Devanagari block: ", get_unicode_block(0x0905)) # अ
vibez.spill("Hiragana block: ", get_unicode_block(0x3042))  # あ
vibez.spill("Katakana block: ", get_unicode_block(0x30A2))  # ア
vibez.spill("Hangul block: ", get_unicode_block(0xAC00))    # 가

# Test mathematical symbols
vibez.spill("\nTesting mathematical symbols...")

vibez.spill("Math operators block: ", get_unicode_block(0x2200))  # ∀
vibez.spill("Arrows block: ", get_unicode_block(0x2190))          # ←
vibez.spill("Box drawing block: ", get_unicode_block(0x2500))     # ─

vibez.spill("\n=== Unicode Integration Test Complete ===")
