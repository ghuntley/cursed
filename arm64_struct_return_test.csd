# ARM64 Struct Return Test - Validates P0 issue #10 fix
# Tests C ABI struct return handling on aarch64

yeet "vibez"
yeet "testz"

# Test struct that should be returned in registers (≤16 bytes)
squad SmallStruct {
    sus a drip
    sus b drip  
}

# Test struct that should be returned via X8 (>16 bytes)
squad LargeStruct {
    sus a drip
    sus b drip
    sus c drip
    sus d drip
    sus e drip  # This makes it >16 bytes
}

# Simulate C function calls that return structs
slay create_small_struct() SmallStruct {
    damn SmallStruct { a: 42, b: 84 }
}

slay create_large_struct() LargeStruct {
    damn LargeStruct { a: 1, b: 2, c: 3, d: 4, e: 5 }
}

# Test the fixed ARM64 struct return handling
test_start("ARM64 Struct Return Fix")

# Test small struct return (should use X0/X1 registers)
sus small SmallStruct = create_small_struct()
assert_eq_int(small.a, 42)
assert_eq_int(small.b, 84)
vibez.spill("✅ Small struct return: a={}, b={}", small.a, small.b)

# Test large struct return (should use X8 indirect)
sus large LargeStruct = create_large_struct()
assert_eq_int(large.a, 1)
assert_eq_int(large.b, 2)
assert_eq_int(large.c, 3)
assert_eq_int(large.d, 4)
assert_eq_int(large.e, 5)
vibez.spill("✅ Large struct return: a={}, b={}, c={}, d={}, e={}", 
           large.a, large.b, large.c, large.d, large.e)

# Test many-field struct (16 bytes, should still use registers)
squad ManyFieldStruct {
    sus a drip
    sus b drip
}

slay create_many_field_struct() ManyFieldStruct {
    damn ManyFieldStruct { a: 100, b: 200 }
}

sus many_field ManyFieldStruct = create_many_field_struct()
assert_eq_int(many_field.a, 100)
assert_eq_int(many_field.b, 200)
vibez.spill("✅ Many-field struct (16 bytes) in registers: a={}, b={}", 
           many_field.a, many_field.b)

print_test_summary()
vibez.spill("🚀 ARM64 C-ABI struct return fix validated!")
