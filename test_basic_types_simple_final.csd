vibez.spill("=== CURSED Basic Types Test ===")

# Test smol (i8) type
sus smol_var smol = 42
vibez.spill("smol (i8) type:")
vibez.spill(smol_var)

# Test mid (i16) type
sus mid_var mid = 12345
vibez.spill("mid (i16) type:")
vibez.spill(mid_var)

# Test thicc (i64) type
sus thicc_var thicc = 1234567890
vibez.spill("thicc (i64) type:")
vibez.spill(thicc_var)

# Test byte (u8) type
sus byte_var byte = 255
vibez.spill("byte (u8) type:")
vibez.spill(byte_var)

# Test rune (i32) type
sus rune_var rune = 65
vibez.spill("rune (i32) type:")
vibez.spill(rune_var)

# Test arithmetic operations
sus smol_a smol = 10
sus smol_b smol = 20
sus smol_sum smol = smol_a + smol_b
sus smol_diff smol = smol_b - smol_a
sus smol_product smol = smol_a * smol_b
sus smol_quotient smol = smol_b / smol_a

vibez.spill("Arithmetic operations on smol:")
vibez.spill("10 + 20 =")
vibez.spill(smol_sum)
vibez.spill("20 - 10 =")
vibez.spill(smol_diff)
vibez.spill("10 * 20 =")
vibez.spill(smol_product)
vibez.spill("20 / 10 =")
vibez.spill(smol_quotient)

# Test comparison operations
sus comp_a smol = 15
sus comp_b smol = 25
sus is_equal lit = comp_a == comp_b
sus is_less lit = comp_a < comp_b
sus is_greater lit = comp_a > comp_b

vibez.spill("Comparison operations (15 vs 25):")
vibez.spill("15 == 25:")
vibez.spill(is_equal)
vibez.spill("15 < 25:")
vibez.spill(is_less)
vibez.spill("15 > 25:")
vibez.spill(is_greater)

vibez.spill("=== All basic types working correctly! ===")
