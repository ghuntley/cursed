yeet "unicode"

# Unicode Module Demo

vibez.spill("=== Unicode Module Demo ===")

# Test basic UTF-8 validation
vibez.spill("Testing UTF-8 validation...")
vibez.spill("ASCII validation: ", is_utf8_start_byte(0x41))
vibez.spill("2-byte start: ", is_utf8_start_byte(0xC2))
vibez.spill("Continuation byte: ", is_utf8_continuation_byte(0x80))

# Test character classification
vibez.spill("\nTesting character classification...")
vibez.spill("Is 'A' ASCII: ", is_ascii(0x41))
vibez.spill("Is '9' digit: ", is_unicode_digit(0x39))
vibez.spill("Is 'Z' letter: ", is_unicode_letter(0x5A))
vibez.spill("Is space whitespace: ", is_unicode_whitespace(0x20))

# Test case conversion
vibez.spill("\nTesting case conversion...")
vibez.spill("'a' to upper: ", to_unicode_upper(0x61))
vibez.spill("'A' to lower: ", to_unicode_lower(0x41))

# Test Unicode ranges
vibez.spill("\nTesting Unicode ranges...")
vibez.spill("Is 0x41 BMP: ", is_bmp(0x41))
vibez.spill("Is 0x10FFFF valid: ", is_valid_unicode(0x10FFFF))
vibez.spill("Is 0x110000 valid: ", is_valid_unicode(0x110000))

# Test Unicode blocks
vibez.spill("\nTesting Unicode blocks...")
vibez.spill("Block for 'A': ", get_unicode_block(0x41))
vibez.spill("Block for Greek: ", get_unicode_block(0x0391))
vibez.spill("Block for Arabic: ", get_unicode_block(0x0627))

# Test UTF-8 sequence lengths
vibez.spill("\nTesting UTF-8 sequence lengths...")
vibez.spill("ASCII length: ", utf8_sequence_length(0x41))
vibez.spill("2-byte length: ", utf8_sequence_length(0xC2))
vibez.spill("3-byte length: ", utf8_sequence_length(0xE0))
vibez.spill("4-byte length: ", utf8_sequence_length(0xF0))

# Test codepoint conversion
vibez.spill("\nTesting codepoint conversion...")
sus utf8_bytes []normie = codepoint_to_utf8(0x41)
vibez.spill("UTF-8 bytes for 'A': ", len(utf8_bytes), " bytes")
vibez.spill("First byte: ", utf8_bytes[0])

# Test 2-byte conversion
sus two_byte []normie = codepoint_to_utf8(0x80)
vibez.spill("UTF-8 bytes for U+0080: ", len(two_byte), " bytes")
vibez.spill("Bytes: ", two_byte[0], ", ", two_byte[1])

vibez.spill("\n=== Unicode Module Demo Complete ===")
