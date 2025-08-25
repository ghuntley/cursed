yeet "vibez"
yeet "testz"

# Load the regex engine functions
yeet "../regex_real_engine"

# Test the fixed Unicode processing functions
slay test_string_to_codepoint() {
    vibez.spill("=== Testing string_to_codepoint() ===")
    
    # Test ASCII character
    sus ascii_result drip = string_to_codepoint("A")
    assert_eq_int(ascii_result, 65)
    vibez.spill("✓ ASCII 'A' -> 65")
    
    # Test UTF-8 2-byte sequence (é = U+00E9)
    sus utf8_2byte drip = string_to_codepoint("é")
    assert_eq_int(utf8_2byte, 233)  # 0x00E9 = 233
    vibez.spill("✓ UTF-8 'é' -> 233 (U+00E9)")
    
    # Test UTF-8 3-byte sequence (€ = U+20AC)
    sus utf8_3byte drip = string_to_codepoint("€")
    assert_eq_int(utf8_3byte, 8364)  # 0x20AC = 8364
    vibez.spill("✓ UTF-8 '€' -> 8364 (U+20AC)")
    
    # Test UTF-8 4-byte sequence (🚀 = U+1F680)
    sus utf8_4byte drip = string_to_codepoint("🚀")
    assert_eq_int(utf8_4byte, 128640)  # 0x1F680 = 128640
    vibez.spill("✓ UTF-8 '🚀' -> 128640 (U+1F680)")
    
    # Test empty string
    sus empty_result drip = string_to_codepoint("")
    assert_eq_int(empty_result, 0)
    vibez.spill("✓ Empty string -> 0")
    
    vibez.spill("All string_to_codepoint tests passed!")
}

slay test_text_to_codepoints_real() {
    vibez.spill("=== Testing text_to_codepoints_real() ===")
    
    # Test ASCII text
    sus ascii_codepoints []drip = text_to_codepoints_real("ABC")
    assert_eq_int(len(ascii_codepoints), 3)
    assert_eq_int(ascii_codepoints[0], 65)   # A
    assert_eq_int(ascii_codepoints[1], 66)   # B
    assert_eq_int(ascii_codepoints[2], 67)   # C
    vibez.spill("✓ ASCII 'ABC' -> [65, 66, 67]")
    
    # Test mixed ASCII and UTF-8
    sus mixed_codepoints []drip = text_to_codepoints_real("Hé€🚀")
    assert_eq_int(len(mixed_codepoints), 4)
    assert_eq_int(mixed_codepoints[0], 72)      # H
    assert_eq_int(mixed_codepoints[1], 233)     # é (U+00E9)
    assert_eq_int(mixed_codepoints[2], 8364)    # € (U+20AC) 
    assert_eq_int(mixed_codepoints[3], 128640)  # 🚀 (U+1F680)
    vibez.spill("✓ Mixed 'Hé€🚀' -> [72, 233, 8364, 128640]")
    
    # Test Chinese characters (测试 = U+6D4B U+8BD5)
    sus chinese_codepoints []drip = text_to_codepoints_real("测试")
    assert_eq_int(len(chinese_codepoints), 2)
    assert_eq_int(chinese_codepoints[0], 27979)  # 测 (U+6D4B)
    assert_eq_int(chinese_codepoints[1], 35797)  # 试 (U+8BD5)
    vibez.spill("✓ Chinese '测试' -> [27979, 35797]")
    
    # Test empty string
    sus empty_codepoints []drip = text_to_codepoints_real("")
    assert_eq_int(len(empty_codepoints), 0)
    vibez.spill("✓ Empty string -> []")
    
    vibez.spill("All text_to_codepoints_real tests passed!")
}

slay test_substring_by_codepoints() {
    vibez.spill("=== Testing substring_by_codepoints() ===")
    
    # Test ASCII substring
    sus ascii_sub tea = substring_by_codepoints("ABCDEF", 1, 4)
    assert_eq_string(ascii_sub, "BCD")
    vibez.spill("✓ ASCII substring 'ABCDEF'[1:4] -> 'BCD'")
    
    # Test Unicode substring
    sus unicode_sub tea = substring_by_codepoints("Hé€🚀!", 1, 4)
    assert_eq_string(unicode_sub, "é€🚀")
    vibez.spill("✓ Unicode substring 'Hé€🚀!'[1:4] -> 'é€🚀'")
    
    # Test Chinese substring
    sus chinese_sub tea = substring_by_codepoints("测试编程", 1, 3)
    assert_eq_string(chinese_sub, "试编")
    vibez.spill("✓ Chinese substring '测试编程'[1:3] -> '试编'")
    
    # Test boundary conditions
    sus boundary1 tea = substring_by_codepoints("ABC", 0, 2)
    assert_eq_string(boundary1, "AB")
    vibez.spill("✓ Boundary test 'ABC'[0:2] -> 'AB'")
    
    sus boundary2 tea = substring_by_codepoints("ABC", 2, 5)  # end > length
    assert_eq_string(boundary2, "C")
    vibez.spill("✓ Boundary test 'ABC'[2:5] -> 'C'")
    
    # Test invalid ranges
    sus invalid1 tea = substring_by_codepoints("ABC", 5, 10)  # start >= length
    assert_eq_string(invalid1, "")
    vibez.spill("✓ Invalid range 'ABC'[5:10] -> ''")
    
    sus invalid2 tea = substring_by_codepoints("ABC", 2, 1)   # end <= start
    assert_eq_string(invalid2, "")
    vibez.spill("✓ Invalid range 'ABC'[2:1] -> ''")
    
    # Test negative start
    sus negative_start tea = substring_by_codepoints("ABC", -1, 2)
    assert_eq_string(negative_start, "AB")
    vibez.spill("✓ Negative start 'ABC'[-1:2] -> 'AB'")
    
    vibez.spill("All substring_by_codepoints tests passed!")
}

slay test_codepoints_to_utf8_string() {
    vibez.spill("=== Testing codepoints_to_utf8_string() ===")
    
    # Test ASCII codepoints
    sus ascii_codepoints []drip = [65, 66, 67]  # A, B, C
    sus ascii_string tea = codepoints_to_utf8_string(ascii_codepoints)
    assert_eq_string(ascii_string, "ABC")
    vibez.spill("✓ [65, 66, 67] -> 'ABC'")
    
    # Test mixed codepoints
    sus mixed_codepoints []drip = [72, 233, 8364, 128640]  # H, é, €, 🚀
    sus mixed_string tea = codepoints_to_utf8_string(mixed_codepoints)
    assert_eq_string(mixed_string, "Hé€🚀")
    vibez.spill("✓ [72, 233, 8364, 128640] -> 'Hé€🚀'")
    
    # Test Chinese codepoints
    sus chinese_codepoints []drip = [27979, 35797]  # 测, 试
    sus chinese_string tea = codepoints_to_utf8_string(chinese_codepoints)
    assert_eq_string(chinese_string, "测试")
    vibez.spill("✓ [27979, 35797] -> '测试'")
    
    # Test empty array
    sus empty_codepoints []drip = []
    sus empty_string tea = codepoints_to_utf8_string(empty_codepoints)
    assert_eq_string(empty_string, "")
    vibez.spill("✓ [] -> ''")
    
    vibez.spill("All codepoints_to_utf8_string tests passed!")
}

slay test_regex_unicode_integration() {
    vibez.spill("=== Testing Regex Unicode Integration ===")
    
    # Test that Unicode functions work together for regex processing
    sus test_text tea = "Hello 世界! 🌍"
    sus codepoints []drip = text_to_codepoints_real(test_text)
    
    # Verify we get the correct number of characters (not bytes)
    assert_eq_int(len(codepoints), 10)  # H,e,l,l,o, ,世,界,!, ,🌍
    vibez.spill("✓ 'Hello 世界! 🌍' has 10 Unicode characters")
    
    # Test substring extraction
    sus world_substr tea = substring_by_codepoints(test_text, 6, 8)  # Extract "世界"
    assert_eq_string(world_substr, "世界")
    vibez.spill("✓ Extracted '世界' from position 6-8")
    
    # Test round-trip: text -> codepoints -> text
    sus reconstructed tea = codepoints_to_utf8_string(codepoints)
    assert_eq_string(reconstructed, test_text)
    vibez.spill("✓ Round-trip conversion preserves text")
    
    # Test individual character extraction
    sus space_char tea = substring_by_codepoints(test_text, 5, 6)
    assert_eq_string(space_char, " ")
    vibez.spill("✓ Extracted space character correctly")
    
    sus emoji_char tea = substring_by_codepoints(test_text, 9, 10)
    assert_eq_string(emoji_char, "🌍")
    vibez.spill("✓ Extracted emoji character correctly")
    
    vibez.spill("All regex Unicode integration tests passed!")
}

# Run all tests
slay run_all_unicode_tests() {
    test_start("Unicode Regex Engine Tests")
    
    test_string_to_codepoint()
    test_text_to_codepoints_real()
    test_substring_by_codepoints()
    test_codepoints_to_utf8_string()
    test_regex_unicode_integration()
    
    vibez.spill("\n🎉 All Unicode regex processing tests PASSED!")
    vibez.spill("✅ Unicode support is now working correctly")
    vibez.spill("✅ Regex engine can now handle non-ASCII text properly")
    
    print_test_summary()
}

# Main execution
run_all_unicode_tests()
