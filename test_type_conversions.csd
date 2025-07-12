vibez.spill("Testing type conversions:")

# Test type conversion functions
sus base_val normie = 42

# Test conversion to different types
sus smol_val smol = smol(base_val)
sus mid_val mid = mid(base_val)
sus thicc_val thicc = thicc(base_val)
sus byte_val byte = byte(base_val)
sus rune_val rune = rune(base_val)

vibez.spill("Original value:")
vibez.spill(base_val)

vibez.spill("Converted to smol:")
vibez.spill(smol_val)

vibez.spill("Converted to mid:")
vibez.spill(mid_val)

vibez.spill("Converted to thicc:")
vibez.spill(thicc_val)

vibez.spill("Converted to byte:")
vibez.spill(byte_val)

vibez.spill("Converted to rune:")
vibez.spill(rune_val)

# Test arithmetic operations on typed values
sus smol_a smol = 10
sus smol_b smol = 20
sus smol_sum smol = smol_a + smol_b
sus smol_diff smol = smol_b - smol_a
sus smol_product smol = smol_a * smol_b

vibez.spill("Arithmetic on smol types:")
vibez.spill(smol_sum)
vibez.spill(smol_diff)
vibez.spill(smol_product)

# Test comparisons
sus is_equal lit = smol_a == smol_b
sus is_less lit = smol_a < smol_b
sus is_greater lit = smol_a > smol_b

vibez.spill("Comparison operations:")
vibez.spill(is_equal)
vibez.spill(is_less)
vibez.spill(is_greater)
