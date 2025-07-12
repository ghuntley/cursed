yeet "testz"

# Test basic CURSED types: smol, mid, thicc, byte, rune, extra

slay test_basic_types_comprehensive() {
    test_start("Basic Types Comprehensive Test")
    
    # Test smol (i8) type
    sus smol_var smol = 42
    sus smol_max smol = 127
    sus smol_min smol = -128
    vibez.spill("smol tests:")
    vibez.spill(smol_var)
    vibez.spill(smol_max)
    vibez.spill(smol_min)
    
    # Test mid (i16) type
    sus mid_var mid = 12345
    sus mid_max mid = 32767
    sus mid_min mid = -32768
    vibez.spill("mid tests:")
    vibez.spill(mid_var)
    vibez.spill(mid_max)
    vibez.spill(mid_min)
    
    # Test thicc (i64) type
    sus thicc_var thicc = 1234567890
    sus thicc_max thicc = 9223372036854775807
    vibez.spill("thicc tests:")
    vibez.spill(thicc_var)
    vibez.spill(thicc_max)
    
    # Test byte (u8) type
    sus byte_var byte = 255
    sus byte_zero byte = 0
    vibez.spill("byte tests:")
    vibez.spill(byte_var)
    vibez.spill(byte_zero)
    
    # Test rune (i32) type for Unicode
    sus rune_var rune = 65
    sus rune_unicode rune = 8364  # Euro symbol
    vibez.spill("rune tests:")
    vibez.spill(rune_var)
    vibez.spill(rune_unicode)
    
    # Test type conversions
    sus convert_test normie = 42
    sus smol_converted smol = smol(convert_test)
    sus mid_converted mid = mid(convert_test)
    sus thicc_converted thicc = thicc(convert_test)
    sus byte_converted byte = byte(convert_test)
    sus rune_converted rune = rune(convert_test)
    
    vibez.spill("Type conversion tests:")
    vibez.spill(smol_converted)
    vibez.spill(mid_converted)
    vibez.spill(thicc_converted)
    vibez.spill(byte_converted)
    vibez.spill(rune_converted)
    
    # Test arithmetic operations
    sus smol_a smol = 10
    sus smol_b smol = 20
    sus smol_sum smol = smol_a + smol_b
    sus smol_diff smol = smol_b - smol_a
    sus smol_product smol = smol_a * smol_b
    sus smol_quotient smol = smol_b / smol_a
    
    vibez.spill("Arithmetic operations:")
    vibez.spill(smol_sum)
    vibez.spill(smol_diff)
    vibez.spill(smol_product)
    vibez.spill(smol_quotient)
    
    # Test comparison operations
    sus comp_a smol = 15
    sus comp_b smol = 25
    sus is_equal lit = comp_a == comp_b
    sus is_less lit = comp_a < comp_b
    sus is_greater lit = comp_a > comp_b
    sus is_less_equal lit = comp_a <= comp_b
    sus is_greater_equal lit = comp_a >= comp_b
    
    vibez.spill("Comparison operations:")
    vibez.spill(is_equal)
    vibez.spill(is_less)
    vibez.spill(is_greater)
    vibez.spill(is_less_equal)
    vibez.spill(is_greater_equal)
    
    # Test complex (extra) type - this might not be implemented yet
    # sus complex_var extra = (3.14, 2.71)
    # vibez.spill("Complex number test:")
    # vibez.spill(complex_var)
    
    print_test_summary()
}

test_basic_types_comprehensive()
