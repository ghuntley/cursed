vibez.spill("Testing basic types without conversions:")

# Test typed variable declarations
sus smol_var smol = 42
sus mid_var mid = 12345
sus thicc_var thicc = 1234567890
sus byte_var byte = 255
sus rune_var rune = 65

vibez.spill("smol value:")
vibez.spill(smol_var)

vibez.spill("mid value:")
vibez.spill(mid_var)

vibez.spill("thicc value:")
vibez.spill(thicc_var)

vibez.spill("byte value:")
vibez.spill(byte_var)

vibez.spill("rune value:")
vibez.spill(rune_var)

# Test arithmetic operations
sus smol_a smol = 10
sus smol_b smol = 20
sus smol_sum smol = smol_a + smol_b
sus smol_diff smol = smol_b - smol_a
sus smol_product smol = smol_a * smol_b

vibez.spill("Arithmetic operations:")
vibez.spill(smol_sum)
vibez.spill(smol_diff)
vibez.spill(smol_product)

# Test comparisons  
sus comp_a smol = 15
sus comp_b smol = 25
sus is_equal lit = comp_a == comp_b
sus is_less lit = comp_a < comp_b
sus is_greater lit = comp_a > comp_b

vibez.spill("Comparison operations:")
vibez.spill(is_equal)
vibez.spill(is_less)
vibez.spill(is_greater)

vibez.spill("Basic types test complete!")
