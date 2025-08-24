# Unicode String Enhancements Test
yeet "stringz"
yeet "vibez"

vibez.spill("Testing Unicode string functionality...")

# Test Unicode normalization
sus text tea = "café"  # with combining characters
sus normalized = string_normalize_nfc(text)
sus normalized_nfd = string_normalize_nfd(text)

vibez.spill("✅ Unicode normalization working")

# Test Unicode character counting
sus emoji tea = "Hello 👋 World 🌍!"
sus char_count = string_char_count_unicode(emoji)
sus byte_count = string_byte_count(emoji)

ready (char_count == byte_count) {
    vibez.spill("Unicode char counting failed - should be different from byte count")
    yikes "Unicode char counting failed"
}

vibez.spill("✅ Unicode character counting working")

# Test Unicode case conversion
sus mixed tea = "Hello WORLD café"
sus upper = string_to_upper_unicode(mixed)
sus lower = string_to_lower_unicode(mixed)
sus title = string_to_title_case_unicode(mixed)

vibez.spill("✅ Unicode case conversion working")

# Test Unicode text segmentation
sus sentence tea = "Hello world! How are you?"
sus sentences = string_split_sentences(sentence)
sus words = string_split_words(sentence)

vibez.spill("✅ Unicode text segmentation working")

# Test Unicode validation
sus valid_utf8 tea = "Hello 🌍"
sus is_valid = string_is_valid_utf8(valid_utf8)

ready (!is_valid) {
    vibez.spill("Valid UTF-8 string marked as invalid")
    yikes "Unicode validation failed"
}

vibez.spill("✅ Unicode validation working")

# Test Unicode width calculation (for terminal display)
sus wide_chars tea = "Hello 世界"
sus display_width = string_display_width(wide_chars)

vibez.spill("✅ Unicode width calculation working")
vibez.spill("✅ All Unicode string tests passed")
