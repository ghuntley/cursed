fr fr ====================================================================
fr fr CURSED StringZ Unicode Tests - Comprehensive Unicode String Operation Tests  
fr fr Issue #6 Fix Validation: Testing proper Unicode support vs ASCII-only operations
fr fr ====================================================================

yeet "testz"
yeet "unicode_stringz"

fr fr ===== BASIC UNICODE LENGTH TESTS =====

slay test_unicode_length() {
    testz.test_start("Unicode Length Tests")
    
    fr fr ASCII strings should work the same
    testz.assert_eq_int(unicode_length("hello"), 5)
    testz.assert_eq_int(unicode_length(""), 0)
    testz.assert_eq_int(unicode_length("a"), 1)
    
    fr fr Multi-byte Unicode characters
    testz.assert_eq_int(unicode_length("café"), 4)      fr fr é is one character
    testz.assert_eq_int(unicode_length("naïve"), 5)     fr fr ï is one character
    testz.assert_eq_int(unicode_length("José"), 4)      fr fr é is one character
    testz.assert_eq_int(unicode_length("Björk"), 5)     fr fr ö is one character
    
    fr fr Emojis should count as single characters
    testz.assert_eq_int(unicode_length("🚀"), 1)
    testz.assert_eq_int(unicode_length("Hello 🌍!"), 9)
    testz.assert_eq_int(unicode_length("👨‍💻"), 1)        fr fr Complex emoji with ZWJ
    
    fr fr Asian characters
    testz.assert_eq_int(unicode_length("こんにちは"), 5)   fr fr Japanese hiragana
    testz.assert_eq_int(unicode_length("你好"), 2)         fr fr Chinese characters
    testz.assert_eq_int(unicode_length("안녕하세요"), 5)   fr fr Korean characters
    
    fr fr Mixed scripts
    testz.assert_eq_int(unicode_length("Hello世界"), 7)
    testz.assert_eq_int(unicode_length("café🚀世界"), 6)
    
    testz.test_pass("Unicode length counting works correctly")
}

fr fr ===== UNICODE CASE CONVERSION TESTS =====

slay test_unicode_case_conversion() {
    testz.test_start("Unicode Case Conversion Tests")
    
    fr fr Basic ASCII (should still work)
    testz.assert_eq_string(unicode_to_lowercase("HELLO"), "hello")
    testz.assert_eq_string(unicode_to_uppercase("world"), "WORLD")
    testz.assert_eq_string(unicode_to_lowercase("MiXeD"), "mixed")
    
    fr fr Latin accented characters
    testz.assert_eq_string(unicode_to_lowercase("CAFÉ"), "café")
    testz.assert_eq_string(unicode_to_uppercase("naïve"), "NAÏVE") 
    testz.assert_eq_string(unicode_to_lowercase("JOSÉ"), "josé")
    testz.assert_eq_string(unicode_to_uppercase("björk"), "BJÖRK")
    
    fr fr German special characters
    testz.assert_eq_string(unicode_to_lowercase("STRAßE"), "straße")
    testz.assert_eq_string(unicode_to_uppercase("größe"), "GRÖßE")
    
    fr fr Greek characters
    testz.assert_eq_string(unicode_to_lowercase("ΑΛΦΑ"), "αλφα")
    testz.assert_eq_string(unicode_to_uppercase("βητα"), "ΒΗΤΑ")
    
    fr fr Cyrillic characters 
    testz.assert_eq_string(unicode_to_lowercase("ПРИВЕТ"), "привет")
    testz.assert_eq_string(unicode_to_uppercase("мир"), "МИР")
    
    fr fr Characters without case should remain unchanged
    testz.assert_eq_string(unicode_to_lowercase("123"), "123")
    testz.assert_eq_string(unicode_to_uppercase("🚀"), "🚀")
    testz.assert_eq_string(unicode_to_lowercase("你好"), "你好")
    
    fr fr Mixed case and scripts
    testz.assert_eq_string(unicode_to_lowercase("Hello世界"), "hello世界")
    testz.assert_eq_string(unicode_to_uppercase("café🚀"), "CAFÉ🚀")
    
    testz.test_pass("Unicode case conversion works correctly")
}

fr fr ===== UNICODE CHARACTER ACCESS TESTS =====

slay test_unicode_char_access() {
    testz.test_start("Unicode Character Access Tests")
    
    fr fr ASCII access should work normally
    testz.assert_eq_string(unicode_char_at("hello", 0), "h")
    testz.assert_eq_string(unicode_char_at("hello", 4), "o")
    testz.assert_eq_string(unicode_char_at("hello", 5), "")  fr fr Out of bounds
    
    fr fr Multi-byte character access
    testz.assert_eq_string(unicode_char_at("café", 0), "c")
    testz.assert_eq_string(unicode_char_at("café", 1), "a")
    testz.assert_eq_string(unicode_char_at("café", 2), "f")
    testz.assert_eq_string(unicode_char_at("café", 3), "é")  fr fr Multi-byte character
    testz.assert_eq_string(unicode_char_at("café", 4), "")   fr fr Out of bounds
    
    fr fr Emoji character access
    testz.assert_eq_string(unicode_char_at("🚀🌍", 0), "🚀")
    testz.assert_eq_string(unicode_char_at("🚀🌍", 1), "🌍")
    testz.assert_eq_string(unicode_char_at("🚀🌍", 2), "")
    
    fr fr Mixed script access
    testz.assert_eq_string(unicode_char_at("Hello世界", 0), "H")
    testz.assert_eq_string(unicode_char_at("Hello世界", 5), "世")
    testz.assert_eq_string(unicode_char_at("Hello世界", 6), "界")
    
    testz.test_pass("Unicode character access works correctly")
}

fr fr ===== UNICODE SUBSTRING TESTS =====

slay test_unicode_substring() {
    testz.test_start("Unicode Substring Tests")
    
    fr fr ASCII substrings should work normally
    testz.assert_eq_string(unicode_substring("hello", 0, 3), "hel")
    testz.assert_eq_string(unicode_substring("hello", 2, 2), "ll")
    testz.assert_eq_string(unicode_substring("hello", 1, 10), "ello")  fr fr Beyond end
    
    fr fr Multi-byte character substrings
    testz.assert_eq_string(unicode_substring("café", 0, 2), "ca")
    testz.assert_eq_string(unicode_substring("café", 2, 2), "fé")
    testz.assert_eq_string(unicode_substring("naïve", 1, 3), "aïv")
    
    fr fr Emoji substrings
    testz.assert_eq_string(unicode_substring("🚀🌍🎉", 0, 1), "🚀")
    testz.assert_eq_string(unicode_substring("🚀🌍🎉", 1, 2), "🌍🎉")
    testz.assert_eq_string(unicode_substring("Hello🚀", 5, 1), "🚀")
    
    fr fr Mixed script substrings
    testz.assert_eq_string(unicode_substring("Hello世界", 0, 5), "Hello")
    testz.assert_eq_string(unicode_substring("Hello世界", 5, 2), "世界")
    testz.assert_eq_string(unicode_substring("café🚀世界", 4, 2), "🚀世")
    
    fr fr Edge cases
    testz.assert_eq_string(unicode_substring("test", -1, 2), "")     fr fr Negative start
    testz.assert_eq_string(unicode_substring("test", 2, 0), "")      fr fr Zero length
    testz.assert_eq_string(unicode_substring("test", 2, -1), "")     fr fr Negative length
    testz.assert_eq_string(unicode_substring("", 0, 1), "")          fr fr Empty string
    
    testz.test_pass("Unicode substring operations work correctly")
}

fr fr ===== UNICODE STRING COMPARISON TESTS =====

slay test_unicode_comparison() {
    testz.test_start("Unicode String Comparison Tests")
    
    fr fr Basic equality tests
    testz.assert_true(unicode_equals("hello", "hello"))
    testz.assert_false(unicode_equals("hello", "world"))
    testz.assert_false(unicode_equals("hello", "Hello"))
    
    fr fr Unicode character equality
    testz.assert_true(unicode_equals("café", "café"))
    testz.assert_false(unicode_equals("café", "cafe"))    fr fr ASCII e vs é
    testz.assert_true(unicode_equals("naïve", "naïve"))
    testz.assert_false(unicode_equals("naïve", "naive"))  fr fr ASCII i vs ï
    
    fr fr Emoji equality
    testz.assert_true(unicode_equals("🚀", "🚀"))
    testz.assert_false(unicode_equals("🚀", "🌍"))
    testz.assert_true(unicode_equals("Hello🚀", "Hello🚀"))
    
    fr fr Mixed script equality
    testz.assert_true(unicode_equals("Hello世界", "Hello世界"))
    testz.assert_false(unicode_equals("Hello世界", "Hello世"))
    testz.assert_true(unicode_equals("café🚀世界", "café🚀世界"))
    
    fr fr Empty and single character comparisons
    testz.assert_true(unicode_equals("", ""))
    testz.assert_false(unicode_equals("", "a"))
    testz.assert_false(unicode_equals("a", ""))
    testz.assert_true(unicode_equals("é", "é"))
    
    testz.test_pass("Unicode string comparison works correctly")
}

fr fr ===== UNICODE SEARCH TESTS =====

slay test_unicode_search() {
    testz.test_start("Unicode Search Tests")
    
    fr fr Contains tests with Unicode
    testz.assert_true(unicode_contains("café", "é"))
    testz.assert_true(unicode_contains("naïve", "ïv"))
    testz.assert_false(unicode_contains("café", "e"))    fr fr ASCII e should not match é
    testz.assert_true(unicode_contains("Hello世界", "世"))
    testz.assert_true(unicode_contains("🚀🌍🎉", "🌍"))
    
    fr fr Starts with tests
    testz.assert_true(unicode_starts_with("café", "ca"))
    testz.assert_true(unicode_starts_with("café", "café"))
    testz.assert_false(unicode_starts_with("café", "af"))
    testz.assert_true(unicode_starts_with("🚀hello", "🚀"))
    testz.assert_true(unicode_starts_with("世界hello", "世界"))
    
    fr fr Ends with tests  
    testz.assert_true(unicode_ends_with("café", "é"))
    testz.assert_true(unicode_ends_with("café", "fé"))
    testz.assert_true(unicode_ends_with("café", "café"))
    testz.assert_false(unicode_ends_with("café", "e"))    fr fr ASCII e should not match é
    testz.assert_true(unicode_ends_with("hello🚀", "🚀"))
    testz.assert_true(unicode_ends_with("hello世界", "世界"))
    
    fr fr Edge cases
    testz.assert_true(unicode_contains("test", ""))       fr fr Empty substring
    testz.assert_true(unicode_starts_with("test", ""))
    testz.assert_true(unicode_ends_with("test", ""))
    testz.assert_false(unicode_contains("", "a"))         fr fr Empty string, non-empty substr
    
    testz.test_pass("Unicode search operations work correctly")
}

fr fr ===== UNICODE REVERSE TESTS =====

slay test_unicode_reverse() {
    testz.test_start("Unicode Reverse Tests")
    
    fr fr ASCII reverse should work normally
    testz.assert_eq_string(unicode_reverse("hello"), "olleh")
    testz.assert_eq_string(unicode_reverse("abc"), "cba")
    testz.assert_eq_string(unicode_reverse(""), "")
    testz.assert_eq_string(unicode_reverse("a"), "a")
    
    fr fr Unicode character reversal
    testz.assert_eq_string(unicode_reverse("café"), "éfac")
    testz.assert_eq_string(unicode_reverse("naïve"), "evïan")
    testz.assert_eq_string(unicode_reverse("josé"), "ésoj")
    
    fr fr Emoji reversal
    testz.assert_eq_string(unicode_reverse("🚀🌍"), "🌍🚀")
    testz.assert_eq_string(unicode_reverse("a🚀b"), "b🚀a")
    testz.assert_eq_string(unicode_reverse("🎉🚀🌍"), "🌍🚀🎉")
    
    fr fr Mixed script reversal
    testz.assert_eq_string(unicode_reverse("Hello世界"), "界世olleH")
    testz.assert_eq_string(unicode_reverse("café🚀世"), "世🚀éfac")
    testz.assert_eq_string(unicode_reverse("a世b界c"), "c界b世a")
    
    testz.test_pass("Unicode reverse operations work correctly")
}

fr fr ===== UNICODE WHITESPACE TESTS =====

slay test_unicode_whitespace() {
    testz.test_start("Unicode Whitespace Tests")
    
    fr fr ASCII whitespace detection
    testz.assert_true(is_unicode_whitespace(" "))      fr fr Regular space
    testz.assert_true(is_unicode_whitespace("\t"))     fr fr Tab
    testz.assert_true(is_unicode_whitespace("\n"))     fr fr Newline
    testz.assert_true(is_unicode_whitespace("\r"))     fr fr Carriage return
    testz.assert_false(is_unicode_whitespace("a"))     fr fr Letter
    testz.assert_false(is_unicode_whitespace("1"))     fr fr Digit
    
    fr fr Unicode whitespace detection
    testz.assert_true(is_unicode_whitespace(" "))      fr fr Non-breaking space (U+00A0)
    testz.assert_true(is_unicode_whitespace(" "))      fr fr En quad (U+2000)  
    testz.assert_true(is_unicode_whitespace("　"))      fr fr Ideographic space (U+3000)
    
    fr fr Unicode whitespace trimming
    testz.assert_eq_string(unicode_trim_whitespace("  hello  "), "hello")
    testz.assert_eq_string(unicode_trim_whitespace("\thello\n"), "hello")
    testz.assert_eq_string(unicode_trim_whitespace("  café  "), "café")
    testz.assert_eq_string(unicode_trim_whitespace("　hello世界　"), "hello世界")
    testz.assert_eq_string(unicode_trim_whitespace("   "), "")  fr fr All whitespace
    testz.assert_eq_string(unicode_trim_whitespace(""), "")     fr fr Empty string
    testz.assert_eq_string(unicode_trim_whitespace("hello"), "hello")  fr fr No whitespace
    
    testz.test_pass("Unicode whitespace handling works correctly")
}

fr fr ===== EMOJI AND SYMBOL TESTS =====

slay test_emoji_support() {
    testz.test_start("Emoji and Symbol Support Tests")
    
    fr fr Emoji counting
    testz.assert_eq_int(count_emojis("🚀"), 1)
    testz.assert_eq_int(count_emojis("🚀🌍🎉"), 3)
    testz.assert_eq_int(count_emojis("Hello🚀World"), 1)
    testz.assert_eq_int(count_emojis("hello"), 0)
    testz.assert_eq_int(count_emojis("café🚀世界🎉"), 2)
    
    fr fr Emoji detection
    testz.assert_true(is_emoji_codepoint(0x1F680))    fr fr 🚀 rocket
    testz.assert_true(is_emoji_codepoint(0x1F30D))    fr fr 🌍 earth
    testz.assert_true(is_emoji_codepoint(0x1F389))    fr fr 🎉 party
    testz.assert_false(is_emoji_codepoint(65))        fr fr A
    testz.assert_false(is_emoji_codepoint(228))       fr fr ä
    
    fr fr Emoji stripping
    testz.assert_eq_string(strip_emojis("Hello🚀World"), "HelloWorld")
    testz.assert_eq_string(strip_emojis("🚀🌍🎉"), "")
    testz.assert_eq_string(strip_emojis("café🚀世界🎉test"), "café世界test")
    testz.assert_eq_string(strip_emojis("hello"), "hello")  fr fr No emojis
    testz.assert_eq_string(strip_emojis(""), "")            fr fr Empty string
    
    testz.test_pass("Emoji and symbol support works correctly")
}

fr fr ===== RTL SCRIPT SUPPORT TESTS =====

slay test_rtl_script_support() {
    testz.test_start("Right-to-Left Script Support Tests")
    
    fr fr Arabic script detection
    testz.assert_true(is_rtl_script(0x0627))     fr fr Arabic letter alef
    testz.assert_true(is_rtl_script(0x0628))     fr fr Arabic letter beh
    testz.assert_true(is_rtl_script(0x0629))     fr fr Arabic letter teh marbuta
    
    fr fr Hebrew script detection  
    testz.assert_true(is_rtl_script(0x05D0))     fr fr Hebrew letter alef
    testz.assert_true(is_rtl_script(0x05D1))     fr fr Hebrew letter bet
    testz.assert_true(is_rtl_script(0x05E9))     fr fr Hebrew letter shin
    
    fr fr LTR script detection (should be false)
    testz.assert_false(is_rtl_script(65))        fr fr Latin A
    testz.assert_false(is_rtl_script(0x03B1))    fr fr Greek alpha
    testz.assert_false(is_rtl_script(0x4E00))    fr fr CJK character
    
    fr fr RTL text detection in strings
    testz.assert_true(has_rtl_text("مرحبا"))     fr fr Arabic "hello"  
    testz.assert_true(has_rtl_text("שלום"))      fr fr Hebrew "hello"
    testz.assert_false(has_rtl_text("hello"))    fr fr English
    testz.assert_false(has_rtl_text("café"))     fr fr French
    testz.assert_false(has_rtl_text("世界"))     fr fr Chinese
    
    fr fr Mixed LTR/RTL text
    testz.assert_true(has_rtl_text("Hello مرحبا"))  fr fr English + Arabic
    testz.assert_true(has_rtl_text("café שלום"))    fr fr French + Hebrew
    
    testz.test_pass("Right-to-left script support works correctly")
}

fr fr ===== UTF-8 VALIDATION TESTS =====

slay test_utf8_validation() {
    testz.test_start("UTF-8 Validation Tests")
    
    fr fr Valid UTF-8 strings
    testz.assert_true(is_valid_utf8("hello"))         fr fr ASCII
    testz.assert_true(is_valid_utf8("café"))          fr fr Latin-1 supplement
    testz.assert_true(is_valid_utf8("🚀"))            fr fr Emoji
    testz.assert_true(is_valid_utf8("Hello世界"))      fr fr Mixed scripts
    testz.assert_true(is_valid_utf8(""))              fr fr Empty string
    testz.assert_true(is_valid_utf8("naïve"))         fr fr Diacritical marks
    
    fr fr Valid multi-byte sequences
    testz.assert_true(is_valid_utf8("Ω"))             fr fr Greek omega (3 bytes)
    testz.assert_true(is_valid_utf8("€"))             fr fr Euro symbol (3 bytes)
    testz.assert_true(is_valid_utf8("𝓤𝓷𝓲𝓬𝓸𝓭𝓮"))        fr fr Mathematical script (4 bytes)
    
    testz.test_pass("UTF-8 validation works correctly")
}

fr fr ===== UNICODE NORMALIZATION TESTS =====

slay test_unicode_normalization() {
    testz.test_start("Unicode Normalization Tests")
    
    fr fr Basic normalization (NFC)
    sus composed tea = "é"        fr fr Precomposed é (U+00E9)
    sus decomposed tea = "e" + encode_utf8_char(769)  fr fr e + combining acute (U+0065 U+0301)
    
    fr fr Both should normalize to the same result
    sus normalized1 tea = unicode_normalize_nfc(composed)
    sus normalized2 tea = unicode_normalize_nfc(decomposed)
    testz.assert_eq_string(normalized1, normalized2)
    
    fr fr Test various accented characters
    testz.assert_true(unicode_length(unicode_normalize_nfc("café")) > 0)
    testz.assert_true(unicode_length(unicode_normalize_nfc("naïve")) > 0)
    testz.assert_true(unicode_length(unicode_normalize_nfc("résumé")) > 0)
    
    fr fr Decomposition of common characters
    testz.assert_true(unicode_length(unicode_decompose("À")) >= 2)  fr fr Should be A + combining grave
    testz.assert_true(unicode_length(unicode_decompose("é")) >= 2)  fr fr Should be e + combining acute
    testz.assert_true(unicode_length(unicode_decompose("ñ")) >= 2)  fr fr Should be n + combining tilde
    
    testz.test_pass("Unicode normalization works correctly")
}

fr fr ===== PERFORMANCE AND EDGE CASE TESTS =====

slay test_unicode_edge_cases() {
    testz.test_start("Unicode Edge Cases and Performance Tests")
    
    fr fr Empty string handling
    testz.assert_eq_int(unicode_length(""), 0)
    testz.assert_eq_string(unicode_to_lowercase(""), "")
    testz.assert_eq_string(unicode_to_uppercase(""), "")
    testz.assert_eq_string(unicode_char_at("", 0), "")
    testz.assert_eq_string(unicode_substring("", 0, 1), "")
    testz.assert_eq_string(unicode_reverse(""), "")
    
    fr fr Single character strings
    testz.assert_eq_int(unicode_length("é"), 1)
    testz.assert_eq_string(unicode_char_at("é", 0), "é")
    testz.assert_eq_string(unicode_substring("é", 0, 1), "é")
    testz.assert_eq_string(unicode_reverse("é"), "é")
    
    fr fr Very long strings with mixed content
    sus long_mixed tea = "Hello" + "café" + "🚀🌍" + "世界" + "مرحبا" + "שלום"
    testz.assert_true(unicode_length(long_mixed) > 10)
    testz.assert_true(unicode_length(unicode_to_lowercase(long_mixed)) > 10)
    testz.assert_true(unicode_contains(long_mixed, "🚀"))
    testz.assert_true(unicode_contains(long_mixed, "世界"))
    
    fr fr Boundary conditions
    testz.assert_eq_string(unicode_char_at("test", -1), "")      fr fr Negative index
    testz.assert_eq_string(unicode_char_at("test", 100), "")     fr fr Way beyond length
    testz.assert_eq_string(unicode_substring("test", 10, 5), "") fr fr Start beyond length
    
    fr fr Null character handling (if supported)
    testz.assert_true(unicode_length("test") > 0)    fr fr Basic sanity check
    
    testz.test_pass("Unicode edge cases handled correctly")
}

fr fr ===== MAIN TEST RUNNER =====

slay run_all_unicode_tests() {
    testz.test_suite_start("Complete Unicode String Operations Test Suite")
    
    test_unicode_length()
    test_unicode_case_conversion()  
    test_unicode_char_access()
    test_unicode_substring()
    test_unicode_comparison()
    test_unicode_search()
    test_unicode_reverse()
    test_unicode_whitespace()
    test_emoji_support()
    test_rtl_script_support()
    test_utf8_validation()
    test_unicode_normalization()
    test_unicode_edge_cases()
    
    testz.test_suite_end()
    testz.print_test_summary()
}

fr fr Run all tests
run_all_unicode_tests()
