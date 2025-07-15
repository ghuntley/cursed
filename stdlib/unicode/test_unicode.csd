yeet "testz"
yeet "unicode"

# Comprehensive Unicode Module Tests
# Testing all Unicode functionality with extensive coverage

# =====================================
# Test Unicode Normalization Functions
# =====================================

slay test_unicode_normalization() {
    test_start("Unicode Normalization Tests")
    
    # Test NFC normalization
    sus text_nfc tea = normalize_nfc("Café")
    assert_true(text_nfc != "")
    
    # Test NFD normalization  
    sus text_nfd tea = normalize_nfd("Café")
    assert_true(text_nfd != "")
    
    # Test NFKC normalization
    sus text_nfkc tea = normalize_nfkc("Café")
    assert_true(text_nfkc != "")
    
    # Test NFKD normalization
    sus text_nfkd tea = normalize_nfkd("Café")
    assert_true(text_nfkd != "")
    
    # Test canonical decomposition
    sus decomposed tea = canonical_decompose("Café")
    assert_true(decomposed != "")
    
    # Test compatibility decomposition
    sus compat_decomposed tea = compatibility_decompose("Café")
    assert_true(compat_decomposed != "")
    
    # Test canonical composition
    sus composed tea = canonical_compose("Cafe")
    assert_true(composed != "")
    
    vibez.spill("✅ Unicode normalization tests passed")
}

# =====================================
# Test Character Classification Functions
# =====================================

slay test_character_classification() {
    test_start("Character Classification Tests")
    
    # Test general category classification
    sus cat_upper tea = get_general_category(0x0041)  # A
    assert_eq_string(cat_upper, "Lu")
    
    sus cat_lower tea = get_general_category(0x0061)  # a
    assert_eq_string(cat_lower, "Ll")
    
    sus cat_digit tea = get_general_category(0x0030)  # 0
    assert_eq_string(cat_digit, "Nd")
    
    sus cat_space tea = get_general_category(0x0020)  # space
    assert_eq_string(cat_space, "Zs")
    
    # Test letter classification
    assert_true(is_unicode_letter(0x0041))   # A
    assert_true(is_unicode_letter(0x0061))   # a
    assert_true(is_unicode_letter(0x00C0))   # À
    assert_true(is_unicode_letter(0x03B1))   # α
    assert_true(is_unicode_letter(0x0410))   # А (Cyrillic)
    assert_true(is_unicode_letter(0x05D0))   # א (Hebrew)
    assert_true(is_unicode_letter(0x0627))   # ا (Arabic)
    assert_true(is_unicode_letter(0x4E00))   # 一 (CJK)
    assert_false(is_unicode_letter(0x0030))  # 0
    assert_false(is_unicode_letter(0x0020))  # space
    
    # Test digit classification
    assert_true(is_unicode_digit(0x0030))    # 0
    assert_true(is_unicode_digit(0x0039))    # 9
    assert_true(is_unicode_digit(0x0660))    # ٠ (Arabic-Indic)
    assert_true(is_unicode_digit(0x0966))    # ० (Devanagari)
    assert_true(is_unicode_digit(0x0AE6))    # ૦ (Gujarati)
    assert_true(is_unicode_digit(0xFF10))    # ０ (Fullwidth)
    assert_false(is_unicode_digit(0x0041))   # A
    assert_false(is_unicode_digit(0x0020))   # space
    
    # Test whitespace classification
    assert_true(is_unicode_whitespace(0x0009))   # Tab
    assert_true(is_unicode_whitespace(0x000A))   # Line Feed
    assert_true(is_unicode_whitespace(0x0020))   # Space
    assert_true(is_unicode_whitespace(0x00A0))   # No-Break Space
    assert_true(is_unicode_whitespace(0x1680))   # Ogham Space Mark
    assert_true(is_unicode_whitespace(0x2000))   # En Quad
    assert_true(is_unicode_whitespace(0x2028))   # Line Separator
    assert_true(is_unicode_whitespace(0x2029))   # Paragraph Separator
    assert_true(is_unicode_whitespace(0x3000))   # Ideographic Space
    assert_false(is_unicode_whitespace(0x0041))  # A
    assert_false(is_unicode_whitespace(0x0030))  # 0
    
    # Test punctuation classification
    assert_true(is_unicode_punctuation(0x0021))   # !
    assert_true(is_unicode_punctuation(0x002E))   # .
    assert_true(is_unicode_punctuation(0x003F))   # ?
    assert_true(is_unicode_punctuation(0x002C))   # ,
    assert_true(is_unicode_punctuation(0x003A))   # :
    assert_true(is_unicode_punctuation(0x003B))   # ;
    assert_true(is_unicode_punctuation(0x2010))   # Hyphen
    assert_true(is_unicode_punctuation(0x2014))   # Em dash
    assert_true(is_unicode_punctuation(0x201C))   # Left double quotation mark
    assert_false(is_unicode_punctuation(0x0041))  # A
    assert_false(is_unicode_punctuation(0x0030))  # 0
    
    # Test symbol classification
    assert_true(is_unicode_symbol(0x0024))   # $
    assert_true(is_unicode_symbol(0x002B))   # +
    assert_true(is_unicode_symbol(0x003D))   # =
    assert_true(is_unicode_symbol(0x0040))   # @
    assert_true(is_unicode_symbol(0x007E))   # ~
    assert_true(is_unicode_symbol(0x00A2))   # ¢
    assert_true(is_unicode_symbol(0x00A9))   # ©
    assert_true(is_unicode_symbol(0x2190))   # ←
    assert_true(is_unicode_symbol(0x2600))   # ☀
    assert_false(is_unicode_symbol(0x0041))  # A
    assert_false(is_unicode_symbol(0x0030))  # 0
    
    # Test mark classification
    assert_true(is_unicode_mark(0x0300))   # Combining grave accent
    assert_true(is_unicode_mark(0x0301))   # Combining acute accent
    assert_true(is_unicode_mark(0x0302))   # Combining circumflex accent
    assert_true(is_unicode_mark(0x0308))   # Combining diaeresis
    assert_true(is_unicode_mark(0x030A))   # Combining ring above
    assert_true(is_unicode_mark(0x064B))   # Arabic fathatan
    assert_true(is_unicode_mark(0x0951))   # Devanagari stress sign udatta
    assert_true(is_unicode_mark(0x20D0))   # Combining left harpoon above
    assert_false(is_unicode_mark(0x0041))  # A
    assert_false(is_unicode_mark(0x0030))  # 0
    
    vibez.spill("✅ Character classification tests passed")
}

# =====================================
# Test Case Conversion Functions
# =====================================

slay test_case_conversion() {
    test_start("Case Conversion Tests")
    
    # Test uppercase conversion for codepoints
    assert_eq_int(to_unicode_upper(0x0061), 0x0041)  # a -> A
    assert_eq_int(to_unicode_upper(0x007A), 0x005A)  # z -> Z
    assert_eq_int(to_unicode_upper(0x00E0), 0x00C0)  # à -> À
    assert_eq_int(to_unicode_upper(0x00E9), 0x00C9)  # é -> É
    assert_eq_int(to_unicode_upper(0x03B1), 0x0391)  # α -> Α
    assert_eq_int(to_unicode_upper(0x0430), 0x0410)  # а -> А
    assert_eq_int(to_unicode_upper(0x0041), 0x0041)  # A -> A (no change)
    
    # Test lowercase conversion for codepoints
    assert_eq_int(to_unicode_lower(0x0041), 0x0061)  # A -> a
    assert_eq_int(to_unicode_lower(0x005A), 0x007A)  # Z -> z
    assert_eq_int(to_unicode_lower(0x00C0), 0x00E0)  # À -> à
    assert_eq_int(to_unicode_lower(0x00C9), 0x00E9)  # É -> é
    assert_eq_int(to_unicode_lower(0x0391), 0x03B1)  # Α -> α
    assert_eq_int(to_unicode_lower(0x0410), 0x0430)  # А -> а
    assert_eq_int(to_unicode_lower(0x0061), 0x0061)  # a -> a (no change)
    
    # Test title case conversion
    assert_eq_int(to_unicode_title(0x0061), 0x0041)  # a -> A
    assert_eq_int(to_unicode_title(0x0041), 0x0041)  # A -> A (no change)
    
    # Test string case conversion
    sus upper_result tea = string_to_unicode_upper("hello")
    assert_true(upper_result != "")
    
    sus lower_result tea = string_to_unicode_lower("HELLO")
    assert_true(lower_result != "")
    
    sus title_result tea = string_to_unicode_title("hello world")
    assert_true(title_result != "")
    
    vibez.spill("✅ Case conversion tests passed")
}

# =====================================
# Test String Comparison Functions
# =====================================

slay test_string_comparison() {
    test_start("String Comparison Tests")
    
    # Test case-insensitive comparison
    sus compare_result normie = unicode_compare_ignore_case("Hello", "hello")
    assert_eq_int(compare_result, 0)
    
    sus compare_result2 normie = unicode_compare_ignore_case("Hello", "HELLO")
    assert_eq_int(compare_result2, 0)
    
    # Test Unicode collation comparison
    sus collate_result normie = unicode_collate_compare("café", "café")
    assert_eq_int(collate_result, 0)
    
    # Test equality functions
    assert_true(unicode_equal_ignore_case("Hello", "hello"))
    assert_true(unicode_equal_ignore_case("CAFÉ", "café"))
    assert_false(unicode_equal_ignore_case("Hello", "world"))
    
    assert_true(unicode_equal_normalized("café", "café"))
    assert_false(unicode_equal_normalized("café", "coffee"))
    
    vibez.spill("✅ String comparison tests passed")
}

# =====================================
# Test Encoding/Decoding Functions
# =====================================

slay test_encoding_decoding() {
    test_start("Encoding/Decoding Tests")
    
    # Test UTF-8 validation
    assert_true(validate_utf8_string("Hello"))
    assert_true(validate_utf8_string("café"))
    assert_true(validate_utf8_string("こんにちは"))
    assert_true(validate_utf8_string("مرحبا"))
    assert_true(validate_utf8_string("Здравствуйте"))
    
    # Test UTF-16 encoding
    sus utf16_bytes []normie = encode_utf16("Hello")
    assert_true(len(utf16_bytes) > 0)
    
    sus utf16_decoded tea = decode_utf16(utf16_bytes)
    assert_true(utf16_decoded != "")
    
    # Test UTF-32 encoding
    sus utf32_bytes []normie = encode_utf32("Hello")
    assert_true(len(utf32_bytes) > 0)
    
    sus utf32_decoded tea = decode_utf32(utf32_bytes)
    assert_true(utf32_decoded != "")
    
    # Test UTF-8 sequence validation
    sus test_bytes []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F]  # "Hello"
    assert_true(validate_utf8_sequence(test_bytes, 0))
    assert_true(validate_utf8_sequence(test_bytes, 1))
    assert_true(validate_utf8_sequence(test_bytes, 2))
    assert_true(validate_utf8_sequence(test_bytes, 3))
    assert_true(validate_utf8_sequence(test_bytes, 4))
    
    # Test UTF-8 sequence length
    assert_eq_int(utf8_sequence_length(0x48), 1)  # ASCII
    assert_eq_int(utf8_sequence_length(0xC3), 2)  # 2-byte UTF-8
    assert_eq_int(utf8_sequence_length(0xE2), 3)  # 3-byte UTF-8
    assert_eq_int(utf8_sequence_length(0xF0), 4)  # 4-byte UTF-8
    
    # Test codepoint conversion
    sus codepoint_A normie = utf8_to_codepoint([0x41], 0)
    assert_eq_int(codepoint_A, 0x41)
    
    sus utf8_A []normie = codepoint_to_utf8(0x41)
    assert_eq_int(len(utf8_A), 1)
    assert_eq_int(utf8_A[0], 0x41)
    
    # Test UTF-16 codepoint conversion
    sus utf16_A []normie = codepoint_to_utf16(0x41)
    assert_eq_int(len(utf16_A), 2)
    
    # Test UTF-32 codepoint conversion
    sus utf32_A []normie = codepoint_to_utf32(0x41)
    assert_eq_int(len(utf32_A), 4)
    
    # Test character counting
    sus char_count normie = utf8_char_count("Hello")
    assert_eq_int(char_count, 5)
    
    sus char_count_unicode normie = utf8_char_count("café")
    assert_eq_int(char_count_unicode, 4)
    
    vibez.spill("✅ Encoding/decoding tests passed")
}

# =====================================
# Test Grapheme Cluster Functions
# =====================================

slay test_grapheme_clusters() {
    test_start("Grapheme Cluster Tests")
    
    # Test grapheme boundary detection
    assert_true(is_grapheme_boundary(0x0041, 0x0042))    # A, B
    assert_false(is_grapheme_boundary(0x0041, 0x0300))   # A, combining grave
    assert_true(is_grapheme_boundary(0x0041, 0x0020))    # A, space
    assert_true(is_grapheme_boundary(0x0041, 0x000A))    # A, newline
    
    # Test script detection
    sus script_latin tea = get_script(0x0041)
    assert_eq_string(script_latin, "Latin")
    
    sus script_greek tea = get_script(0x03B1)
    assert_eq_string(script_greek, "Greek")
    
    sus script_cyrillic tea = get_script(0x0410)
    assert_eq_string(script_cyrillic, "Cyrillic")
    
    sus script_hebrew tea = get_script(0x05D0)
    assert_eq_string(script_hebrew, "Hebrew")
    
    sus script_arabic tea = get_script(0x0627)
    assert_eq_string(script_arabic, "Arabic")
    
    sus script_devanagari tea = get_script(0x0905)
    assert_eq_string(script_devanagari, "Devanagari")
    
    sus script_han tea = get_script(0x4E00)
    assert_eq_string(script_han, "Han")
    
    sus script_hiragana tea = get_script(0x3042)
    assert_eq_string(script_hiragana, "Hiragana")
    
    sus script_katakana tea = get_script(0x30A2)
    assert_eq_string(script_katakana, "Katakana")
    
    sus script_hangul tea = get_script(0xAC00)
    assert_eq_string(script_hangul, "Hangul")
    
    sus script_common tea = get_script(0x0020)
    assert_eq_string(script_common, "Common")
    
    # Test grapheme cluster counting
    sus cluster_count normie = count_grapheme_clusters("Hello")
    assert_eq_int(cluster_count, 5)
    
    sus cluster_count_unicode normie = count_grapheme_clusters("café")
    assert_true(cluster_count_unicode > 0)
    
    # Test grapheme cluster extraction
    sus cluster_0 tea = get_grapheme_cluster_at("Hello", 0)
    assert_true(cluster_0 != "")
    
    sus cluster_1 tea = get_grapheme_cluster_at("Hello", 1)
    assert_true(cluster_1 != "")
    
    vibez.spill("✅ Grapheme cluster tests passed")
}

# =====================================
# Test Text Segmentation Functions
# =====================================

slay test_text_segmentation() {
    test_start("Text Segmentation Tests")
    
    # Test word segmentation
    sus words []tea = segment_words("Hello world test")
    assert_true(len(words) >= 3)
    
    sus words_punctuation []tea = segment_words("Hello, world! How are you?")
    assert_true(len(words_punctuation) >= 4)
    
    sus words_unicode []tea = segment_words("café résumé naïve")
    assert_true(len(words_unicode) >= 3)
    
    # Test sentence segmentation
    sus sentences []tea = segment_sentences("Hello world. How are you? I am fine!")
    assert_true(len(sentences) >= 3)
    
    sus sentences_complex []tea = segment_sentences("Dr. Smith went to the U.S.A. He was happy.")
    assert_true(len(sentences_complex) >= 1)
    
    # Test line breaking
    sus lines []tea = segment_lines("This is a long line that needs to be broken", 20)
    assert_true(len(lines) >= 2)
    
    sus lines_short []tea = segment_lines("Short", 20)
    assert_true(len(lines_short) >= 1)
    
    sus lines_exact []tea = segment_lines("Exactly twenty chars", 20)
    assert_true(len(lines_exact) >= 1)
    
    vibez.spill("✅ Text segmentation tests passed")
}

# =====================================
# Test Decomposition and Composition
# =====================================

slay test_decomposition_composition() {
    test_start("Decomposition and Composition Tests")
    
    # Test canonical decomposition
    sus decomp_A_grave []normie = get_canonical_decomposition(0x00C0)  # À
    assert_true(len(decomp_A_grave) > 0)
    
    sus decomp_A_acute []normie = get_canonical_decomposition(0x00C1)  # Á
    assert_true(len(decomp_A_acute) > 0)
    
    sus decomp_A_circumflex []normie = get_canonical_decomposition(0x00C2)  # Â
    assert_true(len(decomp_A_circumflex) > 0)
    
    sus decomp_A_tilde []normie = get_canonical_decomposition(0x00C3)  # Ã
    assert_true(len(decomp_A_tilde) > 0)
    
    sus decomp_A_diaeresis []normie = get_canonical_decomposition(0x00C4)  # Ä
    assert_true(len(decomp_A_diaeresis) > 0)
    
    sus decomp_A_ring []normie = get_canonical_decomposition(0x00C5)  # Å
    assert_true(len(decomp_A_ring) > 0)
    
    # Test compatibility decomposition
    sus compat_nbsp []normie = get_compatibility_decomposition(0x00A0)  # No-break space
    assert_true(len(compat_nbsp) > 0)
    
    sus compat_ohm []normie = get_compatibility_decomposition(0x2126)  # Ohm sign
    assert_true(len(compat_ohm) > 0)
    
    sus compat_kelvin []normie = get_compatibility_decomposition(0x212A)  # Kelvin sign
    assert_true(len(compat_kelvin) > 0)
    
    sus compat_angstrom []normie = get_compatibility_decomposition(0x212B)  # Angstrom sign
    assert_true(len(compat_angstrom) > 0)
    
    # Test canonical composition
    sus comp_A_grave normie = get_canonical_composition(0x0041, 0x0300)  # A + grave -> À
    assert_eq_int(comp_A_grave, 0x00C0)
    
    sus comp_A_acute normie = get_canonical_composition(0x0041, 0x0301)  # A + acute -> Á
    assert_eq_int(comp_A_acute, 0x00C1)
    
    sus comp_A_circumflex normie = get_canonical_composition(0x0041, 0x0302)  # A + circumflex -> Â
    assert_eq_int(comp_A_circumflex, 0x00C2)
    
    sus comp_A_tilde normie = get_canonical_composition(0x0041, 0x0303)  # A + tilde -> Ã
    assert_eq_int(comp_A_tilde, 0x00C3)
    
    sus comp_A_diaeresis normie = get_canonical_composition(0x0041, 0x0308)  # A + diaeresis -> Ä
    assert_eq_int(comp_A_diaeresis, 0x00C4)
    
    sus comp_A_ring normie = get_canonical_composition(0x0041, 0x030A)  # A + ring -> Å
    assert_eq_int(comp_A_ring, 0x00C5)
    
    # Test no composition
    sus comp_no_match normie = get_canonical_composition(0x0041, 0x0042)  # A + B -> no composition
    assert_eq_int(comp_no_match, -1)
    
    vibez.spill("✅ Decomposition and composition tests passed")
}

# =====================================
# Test Helper Functions
# =====================================

slay test_helper_functions() {
    test_start("Helper Function Tests")
    
    # Test byte array slicing
    sus test_bytes []normie = [0x48, 0x65, 0x6C, 0x6C, 0x6F]  # "Hello"
    sus sliced []normie = slice_bytes(test_bytes, 1, 4)
    assert_eq_int(len(sliced), 3)
    assert_eq_int(sliced[0], 0x65)  # 'e'
    assert_eq_int(sliced[1], 0x6C)  # 'l'
    assert_eq_int(sliced[2], 0x6C)  # 'l'
    
    # Test byte array appending
    sus dest []normie = [0x48, 0x65]  # "He"
    sus src []normie = [0x6C, 0x6C, 0x6F]  # "llo"
    sus appended []normie = append_bytes(dest, src)
    assert_eq_int(len(appended), 5)
    assert_eq_int(appended[0], 0x48)  # 'H'
    assert_eq_int(appended[1], 0x65)  # 'e'
    assert_eq_int(appended[2], 0x6C)  # 'l'
    assert_eq_int(appended[3], 0x6C)  # 'l'
    assert_eq_int(appended[4], 0x6F)  # 'o'
    
    # Test codepoint appending
    sus dest_cp []normie = [0x48, 0x65]  # "He"
    sus appended_cp []normie = append_codepoint(dest_cp, 0x6C)  # 'l'
    assert_true(len(appended_cp) > 2)
    
    # Test multiple codepoints appending
    sus dest_cps []normie = [0x48, 0x65]  # "He"
    sus codepoints []normie = [0x6C, 0x6C, 0x6F]  # "llo"
    sus appended_cps []normie = append_codepoints(dest_cps, codepoints)
    assert_true(len(appended_cps) > 2)
    
    # Test string array appending
    sus dest_strs []tea = ["Hello"]
    sus appended_strs []tea = append_string(dest_strs, "World")
    assert_eq_int(len(appended_strs), 2)
    
    vibez.spill("✅ Helper function tests passed")
}

# =====================================
# Test Advanced Unicode Features
# =====================================

slay test_advanced_unicode() {
    test_start("Advanced Unicode Feature Tests")
    
    # Test Unicode block detection (from original implementation)
    sus block_latin tea = get_unicode_block(0x0041)
    assert_eq_string(block_latin, "Basic Latin")
    
    sus block_latin1 tea = get_unicode_block(0x00C0)
    assert_eq_string(block_latin1, "Latin-1 Supplement")
    
    sus block_greek tea = get_unicode_block(0x03B1)
    assert_eq_string(block_greek, "Greek and Coptic")
    
    sus block_cyrillic tea = get_unicode_block(0x0410)
    assert_eq_string(block_cyrillic, "Cyrillic")
    
    sus block_hebrew tea = get_unicode_block(0x05D0)
    assert_eq_string(block_hebrew, "Hebrew")
    
    sus block_arabic tea = get_unicode_block(0x0627)
    assert_eq_string(block_arabic, "Arabic")
    
    sus block_devanagari tea = get_unicode_block(0x0905)
    assert_eq_string(block_devanagari, "Devanagari")
    
    sus block_han tea = get_unicode_block(0x4E00)
    assert_eq_string(block_han, "CJK Unified Ideographs")
    
    sus block_hiragana tea = get_unicode_block(0x3042)
    assert_eq_string(block_hiragana, "Hiragana")
    
    sus block_katakana tea = get_unicode_block(0x30A2)
    assert_eq_string(block_katakana, "Katakana")
    
    sus block_hangul tea = get_unicode_block(0xAC00)
    assert_eq_string(block_hangul, "Hangul Syllables")
    
    sus block_unknown tea = get_unicode_block(0x200000)
    assert_eq_string(block_unknown, "Unknown")
    
    vibez.spill("✅ Advanced Unicode feature tests passed")
}

# =====================================
# Test Performance and Edge Cases
# =====================================

slay test_performance_edge_cases() {
    test_start("Performance and Edge Case Tests")
    
    # Test empty string handling
    sus empty_validation lit = validate_utf8_string("")
    assert_true(empty_validation)
    
    sus empty_char_count normie = utf8_char_count("")
    assert_eq_int(empty_char_count, 0)
    
    sus empty_cluster_count normie = count_grapheme_clusters("")
    assert_eq_int(empty_cluster_count, 0)
    
    # Test single character strings
    sus single_char_count normie = utf8_char_count("A")
    assert_eq_int(single_char_count, 1)
    
    sus single_cluster_count normie = count_grapheme_clusters("A")
    assert_eq_int(single_cluster_count, 1)
    
    # Test boundary conditions
    sus boundary_seq_length normie = utf8_sequence_length(0x00)
    assert_eq_int(boundary_seq_length, 1)
    
    sus boundary_seq_length_max normie = utf8_sequence_length(0x7F)
    assert_eq_int(boundary_seq_length_max, 1)
    
    # Test invalid UTF-8 handling
    sus invalid_seq_length normie = utf8_sequence_length(0x80)
    assert_eq_int(invalid_seq_length, 0)
    
    sus invalid_seq_length2 normie = utf8_sequence_length(0xFF)
    assert_eq_int(invalid_seq_length2, 0)
    
    # Test large codepoint handling
    sus large_codepoint_utf8 []normie = codepoint_to_utf8(0x10FFFF)
    assert_eq_int(len(large_codepoint_utf8), 4)
    
    sus large_codepoint_utf16 []normie = codepoint_to_utf16(0x10FFFF)
    assert_eq_int(len(large_codepoint_utf16), 4)  # Surrogate pair
    
    sus large_codepoint_utf32 []normie = codepoint_to_utf32(0x10FFFF)
    assert_eq_int(len(large_codepoint_utf32), 4)
    
    # Test invalid codepoint handling
    sus invalid_codepoint_utf8 []normie = codepoint_to_utf8(0x110000)
    assert_eq_int(len(invalid_codepoint_utf8), 0)
    
    vibez.spill("✅ Performance and edge case tests passed")
}

# =====================================
# Main Test Runner
# =====================================

# Run all Unicode module tests
slay run_all_unicode_tests() {
    vibez.spill("🚀 Starting comprehensive Unicode module tests...")
    
    # Test all major functionality areas
    test_unicode_normalization()
    test_character_classification()
    test_case_conversion()
    test_string_comparison()
    test_encoding_decoding()
    test_grapheme_clusters()
    test_text_segmentation()
    test_decomposition_composition()
    test_helper_functions()
    test_advanced_unicode()
    test_performance_edge_cases()
    
    vibez.spill("🎉 All Unicode module tests completed successfully!")
    vibez.spill("📊 Test Coverage:")
    vibez.spill("  • Unicode Normalization (NFC, NFD, NFKC, NFKD)")
    vibez.spill("  • Character Classification (Letters, Digits, Whitespace, Punctuation, Symbols, Marks)")
    vibez.spill("  • Case Conversion (Upper, Lower, Title)")
    vibez.spill("  • String Comparison (Case-insensitive, Normalized)")
    vibez.spill("  • Encoding/Decoding (UTF-8, UTF-16, UTF-32)")
    vibez.spill("  • Grapheme Cluster Handling")
    vibez.spill("  • Text Segmentation (Words, Sentences, Lines)")
    vibez.spill("  • Advanced Features (Unicode blocks, Scripts)")
    vibez.spill("  • Performance & Edge Cases")
    
    print_test_summary()
}

# Execute all tests
run_all_unicode_tests()
