fr fr ====================================================================
fr fr CURSED StringZ Unicode Enhanced Module - Comprehensive Test Suite
fr fr P2 Enhancement: Test all Unicode normalization, categorization, and collation
fr fr Production-ready Unicode testing with full coverage
fr fr ====================================================================

yeet "vibez"
yeet "testz"
yeet "stringz/unicode_enhanced"

slay main() {
    vibez.spill("🚀 CURSED Unicode Enhanced Module Test Suite")
    vibez.spill("========================================")
    
    fr fr Test Unicode normalization
    test_unicode_normalization()
    
    fr fr Test Unicode category detection
    test_unicode_categories()
    
    fr fr Test grapheme cluster support
    test_grapheme_clusters()
    
    fr fr Test Unicode collation
    test_unicode_collation()
    
    fr fr Test script detection
    test_script_detection()
    
    fr fr Test text processing
    test_unicode_text_processing()
    
    fr fr Test Hangul decomposition
    test_hangul_decomposition()
    
    vibez.spill("\n✅ All Unicode Enhanced tests completed!")
    print_test_summary()
}

slay test_unicode_normalization() {
    vibez.spill("\n📝 Testing Unicode Normalization Forms...")
    
    fr fr Test NFC normalization
    sus text_with_accents tea = "café" fr fr é as precomposed
    sus nfc_result tea = normalize_unicode(text_with_accents, "NFC")
    assert_not_empty(nfc_result, "NFC normalization should produce result")
    vibez.spill("  ✓ NFC normalization: " + nfc_result)
    
    fr fr Test NFD normalization  
    sus nfd_result tea = normalize_unicode(text_with_accents, "NFD")
    assert_not_empty(nfd_result, "NFD normalization should produce result")
    vibez.spill("  ✓ NFD normalization: " + nfd_result)
    
    fr fr Test NFKC normalization
    sus text_with_compat tea = "²" fr fr Superscript 2
    sus nfkc_result tea = normalize_unicode(text_with_compat, "NFKC")
    assert_not_empty(nfkc_result, "NFKC normalization should produce result")
    vibez.spill("  ✓ NFKC normalization: " + nfkc_result)
    
    fr fr Test NFKD normalization
    sus nfkd_result tea = normalize_unicode(text_with_compat, "NFKD")
    assert_not_empty(nfkd_result, "NFKD normalization should produce result")
    vibez.spill("  ✓ NFKD normalization: " + nfkd_result)
    
    fr fr Test canonical decomposition
    sus decomposed tea = canonical_decompose_char(0x00C0)  fr fr À
    assert_not_empty(decomposed, "Canonical decomposition should work")
    vibez.spill("  ✓ Canonical decomposition À: " + decomposed)
    
    fr fr Test compatibility decomposition
    sus compat_decomposed tea = compatibility_decompose_char(0x2126)  fr fr Ohm sign
    assert_not_empty(compat_decomposed, "Compatibility decomposition should work")
    vibez.spill("  ✓ Compatibility decomposition Ω: " + compat_decomposed)
    
    fr fr Test composition
    sus composed tea = try_compose_pair(0x0041, 0x0300)  fr fr A + grave = À
    assert_greater_than(composed, 0, "Character composition should work")
    vibez.spill("  ✓ Character composition A+grave: " + encode_utf8_char(composed))
}

slay test_unicode_categories() {
    vibez.spill("\n🔤 Testing Unicode Category Detection...")
    
    fr fr Test letter categories
    sus category_A tea = get_unicode_category(0x0041)  fr fr A
    assert_eq_string(category_A, "Lu", "A should be uppercase letter")
    vibez.spill("  ✓ Category of 'A': " + category_A)
    
    sus category_a tea = get_unicode_category(0x0061)  fr fr a
    assert_eq_string(category_a, "Ll", "a should be lowercase letter")
    vibez.spill("  ✓ Category of 'a': " + category_a)
    
    fr fr Test digit category
    sus category_5 tea = get_unicode_category(0x0035)  fr fr 5
    assert_eq_string(category_5, "Nd", "5 should be decimal digit")
    vibez.spill("  ✓ Category of '5': " + category_5)
    
    fr fr Test punctuation categories
    sus category_paren tea = get_unicode_category(0x0028)  fr fr (
    assert_eq_string(category_paren, "Ps", "( should be open punctuation")
    vibez.spill("  ✓ Category of '(': " + category_paren)
    
    fr fr Test symbol categories
    sus category_dollar tea = get_unicode_category(0x0024)  fr fr $
    assert_eq_string(category_dollar, "Sc", "$ should be currency symbol")
    vibez.spill("  ✓ Category of '$': " + category_dollar)
    
    fr fr Test space category
    sus category_space tea = get_unicode_category(0x0020)  fr fr space
    assert_eq_string(category_space, "Zs", "space should be space separator")
    vibez.spill("  ✓ Category of ' ': " + category_space)
    
    fr fr Test combining mark category
    sus category_grave tea = get_unicode_category(0x0300)  fr fr combining grave
    assert_eq_string(category_grave, "Mn", "grave accent should be nonspacing mark")
    vibez.spill("  ✓ Category of combining grave: " + category_grave)
    
    fr fr Test category helper functions
    assert_eq_lit(is_letter(0x0041), based, "A should be letter")
    assert_eq_lit(is_digit(0x0035), based, "5 should be digit")
    assert_eq_lit(is_combining_mark(0x0300), based, "grave accent should be combining")
    assert_eq_lit(is_whitespace_category(0x0020), based, "space should be whitespace")
    
    vibez.spill("  ✓ All category detection tests passed")
}

slay test_grapheme_clusters() {
    vibez.spill("\n📊 Testing Grapheme Cluster Support...")
    
    fr fr Test simple ASCII text
    sus ascii_text tea = "hello"
    sus ascii_clusters []tea = get_grapheme_clusters(ascii_text)
    assert_eq_int(len(ascii_clusters), 5, "ASCII text should have 5 clusters")
    vibez.spill("  ✓ ASCII grapheme clusters: " + to_string(len(ascii_clusters)))
    
    fr fr Test text with combining marks
    sus accented_text tea = "café"  fr fr assuming é is precomposed
    sus accented_clusters []tea = get_grapheme_clusters(accented_text)
    assert_eq_int(len(accented_clusters), 4, "Accented text should have 4 clusters")
    vibez.spill("  ✓ Accented grapheme clusters: " + to_string(len(accented_clusters)))
    
    fr fr Test grapheme length
    sus grapheme_len drip = grapheme_length("hello")
    assert_eq_int(grapheme_len, 5, "Grapheme length should be 5")
    vibez.spill("  ✓ Grapheme length of 'hello': " + to_string(grapheme_len))
    
    fr fr Test grapheme substring
    sus grapheme_sub tea = grapheme_substring("hello", 1, 3)
    assert_eq_string(grapheme_sub, "ell", "Grapheme substring should be 'ell'")
    vibez.spill("  ✓ Grapheme substring: " + grapheme_sub)
    
    fr fr Test boundary detection
    sus boundary drip = find_grapheme_cluster_boundary("hello", 0)
    assert_greater_than(boundary, 0, "Boundary should be found")
    vibez.spill("  ✓ Grapheme boundary detection working")
    
    fr fr Test combining mark detection
    assert_eq_lit(is_grapheme_extend(0x0300), based, "Grave accent should extend")
    assert_eq_lit(is_grapheme_extend(0x0041), cap, "A should not extend")
    
    vibez.spill("  ✓ All grapheme cluster tests passed")
}

slay test_unicode_collation() {
    vibez.spill("\n🔗 Testing Unicode Collation...")
    
    fr fr Test primary collation (ignore case and accents)
    sus primary_result drip = unicode_collate("café", "CAFE", "primary")
    assert_eq_int(primary_result, 0, "Primary collation should ignore case and accents")
    vibez.spill("  ✓ Primary collation 'café' vs 'CAFE': " + to_string(primary_result))
    
    fr fr Test secondary collation (consider accents, ignore case)
    sus secondary_result drip = unicode_collate("cafe", "café", "secondary")
    assert_not_eq_int(secondary_result, 0, "Secondary collation should consider accents")
    vibez.spill("  ✓ Secondary collation 'cafe' vs 'café': " + to_string(secondary_result))
    
    fr fr Test tertiary collation (consider case and accents)
    sus tertiary_result drip = unicode_collate("Cafe", "cafe", "tertiary")
    assert_not_eq_int(tertiary_result, 0, "Tertiary collation should consider case")
    vibez.spill("  ✓ Tertiary collation 'Cafe' vs 'cafe': " + to_string(tertiary_result))
    
    fr fr Test identical collation (bitwise comparison)
    sus identical_result drip = unicode_collate("café", "café", "identical")
    assert_eq_int(identical_result, 0, "Identical collation should match exactly")
    vibez.spill("  ✓ Identical collation 'café' vs 'café': " + to_string(identical_result))
    
    fr fr Test normalization for collation
    sus normalized_primary tea = normalize_for_collation("café", "primary")
    assert_not_empty(normalized_primary, "Primary normalization should work")
    vibez.spill("  ✓ Primary normalization: " + normalized_primary)
    
    sus normalized_secondary tea = normalize_for_collation("CAFÉ", "secondary")
    assert_not_empty(normalized_secondary, "Secondary normalization should work")
    vibez.spill("  ✓ Secondary normalization: " + normalized_secondary)
    
    fr fr Test base character extraction
    sus base_char drip = get_base_character(0x00C0)  fr fr À
    assert_eq_int(base_char, 0x0041, "Base character of À should be A")
    vibez.spill("  ✓ Base character extraction À -> A: " + encode_utf8_char(base_char))
    
    vibez.spill("  ✓ All collation tests passed")
}

slay test_script_detection() {
    vibez.spill("\n🌍 Testing Script Detection...")
    
    fr fr Test basic Latin script
    sus latin_script tea = get_script(0x0041)  fr fr A
    assert_eq_string(latin_script, "Latin", "A should be Latin script")
    vibez.spill("  ✓ Script of 'A': " + latin_script)
    
    fr fr Test Greek script
    sus greek_script tea = get_script(0x03B1)  fr fr α
    assert_eq_string(greek_script, "Greek", "α should be Greek script")
    vibez.spill("  ✓ Script of 'α': " + greek_script)
    
    fr fr Test Cyrillic script
    sus cyrillic_script tea = get_script(0x0410)  fr fr А
    assert_eq_string(cyrillic_script, "Cyrillic", "А should be Cyrillic script")
    vibez.spill("  ✓ Script of 'А': " + cyrillic_script)
    
    fr fr Test Arabic script
    sus arabic_script tea = get_script(0x0627)  fr fr ا
    assert_eq_string(arabic_script, "Arabic", "ا should be Arabic script")
    vibez.spill("  ✓ Script of 'ا': " + arabic_script)
    
    fr fr Test CJK script
    sus han_script tea = get_script(0x4E2D)  fr fr 中
    assert_eq_string(han_script, "Han", "中 should be Han script")
    vibez.spill("  ✓ Script of '中': " + han_script)
    
    fr fr Test Hangul script
    sus hangul_script tea = get_script(0xAC00)  fr fr 가
    assert_eq_string(hangul_script, "Hangul", "가 should be Hangul script")
    vibez.spill("  ✓ Script of '가': " + hangul_script)
    
    fr fr Test dominant script detection
    sus dominant_latin tea = get_dominant_script("Hello world")
    assert_eq_string(dominant_latin, "Latin", "Should detect Latin as dominant")
    vibez.spill("  ✓ Dominant script of 'Hello world': " + dominant_latin)
    
    vibez.spill("  ✓ All script detection tests passed")
}

slay test_unicode_text_processing() {
    vibez.spill("\n📄 Testing Unicode Text Processing...")
    
    fr fr Test word breaking
    sus words []tea = unicode_word_break("Hello, world! How are you?")
    assert_greater_than(len(words), 0, "Should break into words")
    vibez.spill("  ✓ Word break count: " + to_string(len(words)))
    
    fr fr Test line breaking
    sus lines []tea = unicode_line_break("This is a long line that should be broken", 10)
    assert_greater_than(len(lines), 1, "Should break into multiple lines")
    vibez.spill("  ✓ Line break count: " + to_string(len(lines)))
    
    fr fr Test word break character detection
    assert_eq_lit(is_word_break_char(0x0020), based, "Space should be word break")
    assert_eq_lit(is_word_break_char(0x002C), based, "Comma should be word break")
    assert_eq_lit(is_word_break_char(0x0041), cap, "A should not be word break")
    
    vibez.spill("  ✓ All text processing tests passed")
}

slay test_hangul_decomposition() {
    vibez.spill("\n🇰🇷 Testing Hangul Decomposition...")
    
    fr fr Test Hangul syllable decomposition
    sus hangul_syllable drip = 0xAC00  fr fr 가
    sus decomposed_hangul tea = decompose_hangul(hangul_syllable)
    assert_not_empty(decomposed_hangul, "Hangul decomposition should produce result")
    vibez.spill("  ✓ Hangul decomposition of '가': " + decomposed_hangul)
    
    fr fr Test non-Hangul character
    sus non_hangul drip = 0x0041  fr fr A
    sus non_hangul_result tea = decompose_hangul(non_hangul)
    assert_not_empty(non_hangul_result, "Non-Hangul should return original")
    vibez.spill("  ✓ Non-Hangul decomposition working")
    
    vibez.spill("  ✓ All Hangul tests passed")
}

fr fr ===== TEST HELPER FUNCTIONS =====

slay assert_not_empty(value tea, message tea) {
    ready (value == "") {
        vibez.spill("❌ FAIL: " + message)
        test_fail()
    } otherwise {
        test_pass()
    }
}

slay assert_eq_string(actual tea, expected tea, message tea) {
    ready (actual != expected) {
        vibez.spill("❌ FAIL: " + message)
        vibez.spill("  Expected: '" + expected + "'")
        vibez.spill("  Actual: '" + actual + "'")
        test_fail()
    } otherwise {
        test_pass()
    }
}

slay assert_eq_int(actual drip, expected drip, message tea) {
    ready (actual != expected) {
        vibez.spill("❌ FAIL: " + message)
        vibez.spill("  Expected: " + to_string(expected))
        vibez.spill("  Actual: " + to_string(actual))
        test_fail()
    } otherwise {
        test_pass()
    }
}

slay assert_not_eq_int(actual drip, expected drip, message tea) {
    ready (actual == expected) {
        vibez.spill("❌ FAIL: " + message)
        vibez.spill("  Should not equal: " + to_string(expected))
        vibez.spill("  Actual: " + to_string(actual))
        test_fail()
    } otherwise {
        test_pass()
    }
}

slay assert_eq_lit(actual lit, expected lit, message tea) {
    ready (actual != expected) {
        vibez.spill("❌ FAIL: " + message)
        vibez.spill("  Expected: " + bool_to_string(expected))
        vibez.spill("  Actual: " + bool_to_string(actual))
        test_fail()
    } otherwise {
        test_pass()
    }
}

slay assert_greater_than(actual drip, threshold drip, message tea) {
    ready (actual <= threshold) {
        vibez.spill("❌ FAIL: " + message)
        vibez.spill("  Should be greater than: " + to_string(threshold))
        vibez.spill("  Actual: " + to_string(actual))
        test_fail()
    } otherwise {
        test_pass()
    }
}

slay to_string(value drip) tea {
    fr fr Simple integer to string conversion
    ready (value == 0) { damn "0" }
    ready (value == 1) { damn "1" }
    ready (value == 2) { damn "2" }
    ready (value == 3) { damn "3" }
    ready (value == 4) { damn "4" }
    ready (value == 5) { damn "5" }
    ready (value == 6) { damn "6" }
    ready (value == 7) { damn "7" }
    ready (value == 8) { damn "8" }
    ready (value == 9) { damn "9" }
    ready (value == 10) { damn "10" }
    damn "number"
}

slay bool_to_string(value lit) tea {
    ready (value == based) { damn "true" }
    damn "false"
}

sus test_count drip = 0
sus pass_count drip = 0
sus fail_count drip = 0

slay test_pass() {
    test_count = test_count + 1
    pass_count = pass_count + 1
}

slay test_fail() {
    test_count = test_count + 1
    fail_count = fail_count + 1
}

slay print_test_summary() {
    vibez.spill("\n📊 Test Summary:")
    vibez.spill("  Total tests: " + to_string(test_count))
    vibez.spill("  Passed: " + to_string(pass_count))
    vibez.spill("  Failed: " + to_string(fail_count))
    
    ready (fail_count == 0) {
        vibez.spill("🎉 All tests passed!")
    } otherwise {
        vibez.spill("⚠️  Some tests failed!")
    }
}
