yeet "testz"
yeet "unicode"

slay test_unicode_basic() {
    test_start("Unicode Basic Tests") fr fr Test letter classification
    assert_true(is_unicode_letter(0x0041)) fr fr A
    assert_true(is_unicode_letter(0x0061)) fr fr a
    assert_false(is_unicode_letter(0x0030)) fr fr 0 fr fr Test digit classification  
    assert_true(is_unicode_digit(0x0030)) fr fr 0
    assert_false(is_unicode_digit(0x0041)) fr fr A fr fr Test whitespace
    assert_true(is_unicode_whitespace(0x0020)) fr fr space
    assert_false(is_unicode_whitespace(0x0041)) fr fr A fr fr Test case conversion
    assert_eq_int(to_unicode_upper(0x0061), 0x0041) fr fr a -> A
    assert_eq_int(to_unicode_lower(0x0041), 0x0061) fr fr A -> a fr fr Test general category
    sus cat_upper tea = get_general_category(0x0041)
    assert_eq_string(cat_upper, "Lu")
    
    sus cat_lower tea = get_general_category(0x0061)
    assert_eq_string(cat_lower, "Ll") fr fr Test normalization (simplified)
    sus normalized tea = normalize_nfc("test")
    assert_eq_string(normalized, "test") fr fr Test UTF-8 validation
    assert_true(validate_utf8_string("Hello")) fr fr Test UTF-8 sequence length
    assert_eq_int(utf8_sequence_length(0x48), 1) fr fr ASCII
    assert_eq_int(utf8_sequence_length(0xC3), 2) fr fr 2-byte UTF-8 fr fr Test additional character types
    assert_true(is_unicode_punctuation(0x21)) fr fr !
    assert_true(is_unicode_symbol(0x24)) fr fr $
    assert_true(is_unicode_mark(0x0300)) fr fr combining grave fr fr Test script detection
    sus script_latin tea = get_script(0x0041)
    assert_eq_string(script_latin, "Latin") fr fr Test Unicode blocks
    sus block_latin tea = get_unicode_block(0x0041)
    assert_eq_string(block_latin, "Basic Latin") fr fr Test string comparison
    assert_true(unicode_equal_ignore_case("test", "test"))
    
    vibez.spill("✅ Unicode basic tests passed")
}

slay run_unicode_tests() {
    vibez.spill("🚀 Starting Unicode module tests...")
    test_unicode_basic()
    vibez.spill("🎉 Unicode tests completed!")
    print_test_summary()
}

run_unicode_tests()
