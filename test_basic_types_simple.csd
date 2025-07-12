# Test basic CURSED types: smol, mid, thicc, byte, rune

vibez.spill("Testing basic types:")

# Test smol (i8) type
sus smol_var smol = 42
vibez.spill("smol value:")
vibez.spill(smol_var)

# Test mid (i16) type  
sus mid_var mid = 12345
vibez.spill("mid value:")
vibez.spill(mid_var)

# Test thicc (i64) type
sus thicc_var thicc = 1234567890
vibez.spill("thicc value:")
vibez.spill(thicc_var)

# Test byte (u8) type
sus byte_var byte = 255
vibez.spill("byte value:")
vibez.spill(byte_var)

# Test rune (i32) type
sus rune_var rune = 65
vibez.spill("rune value:")
vibez.spill(rune_var)

vibez.spill("Basic types test complete")
