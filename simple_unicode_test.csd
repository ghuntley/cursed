yeet "unicode"

vibez.spill("Testing Unicode module...")

# Test basic function calls
sus result lit = is_utf8_start_byte(0x41)
vibez.spill("is_utf8_start_byte(0x41): ", result)

sus length normie = utf8_sequence_length(0x41)
vibez.spill("utf8_sequence_length(0x41): ", length)

sus is_letter lit = is_unicode_letter(0x41)
vibez.spill("is_unicode_letter(0x41): ", is_letter)

vibez.spill("Unicode test complete")
