yeet "testz"

slay test_basic_types_implementation() {
    test_start("Basic Types Implementation Test")
    
    # Test smol (i8) type
    sus smol_var smol = 42
    sus smol_max smol = 127
    sus smol_min smol = -128
    assert_eq_int(smol_var, 42)
    assert_eq_int(smol_max, 127)
    assert_eq_int(smol_min, -128)
    
    # Test mid (i16) type
    sus mid_var mid = 12345
    sus mid_max mid = 32767
    sus mid_min mid = -32768
    assert_eq_int(mid_var, 12345)
    assert_eq_int(mid_max, 32767)
    assert_eq_int(mid_min, -32768)
    
    # Test thicc (i64) type
    sus thicc_var thicc = 1234567890
    sus thicc_max thicc = 9223372036854775807
    assert_eq_int(thicc_var, 1234567890)
    assert_eq_int(thicc_max, 9223372036854775807)
    
    # Test byte (u8) type  
    sus byte_var byte = 255
    sus byte_zero byte = 0
    assert_eq_int(byte_var, 255)
    assert_eq_int(byte_zero, 0)
    
    # Test rune (i32) type for Unicode
    sus rune_var rune = 65
    sus rune_unicode rune = 8364  # Euro symbol
    assert_eq_int(rune_var, 65)
    assert_eq_int(rune_unicode, 8364)
    
    # Test arithmetic operations
    sus smol_a smol = 10
    sus smol_b smol = 20
    sus smol_sum smol = smol_a + smol_b
    sus smol_diff smol = smol_b - smol_a
    sus smol_product smol = smol_a * smol_b
    sus smol_quotient smol = smol_b / smol_a
    
    assert_eq_int(smol_sum, 30)
    assert_eq_int(smol_diff, 10)
    assert_eq_int(smol_product, 200)
    assert_eq_int(smol_quotient, 2)
    
    # Test comparison operations
    sus comp_a smol = 15
    sus comp_b smol = 25
    sus is_equal lit = comp_a == comp_b
    sus is_less lit = comp_a < comp_b
    sus is_greater lit = comp_a > comp_b
    sus is_less_equal lit = comp_a <= comp_b
    sus is_greater_equal lit = comp_a >= comp_b
    
    assert_false(is_equal)
    assert_true(is_less)
    assert_false(is_greater)
    assert_true(is_less_equal)
    assert_false(is_greater_equal)
    
    print_test_summary()
}

test_basic_types_implementation()
