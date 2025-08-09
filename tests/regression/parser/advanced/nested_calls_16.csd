// Nested function calls test 16
slay inner_16(x drip) drip { damn x + 16 }
slay outer_16(y drip) drip { damn inner_16(y * 2) }
slay wrapper_16(z drip) drip { damn outer_16(z + 1) }

sus final drip = wrapper_16(16)
vibez.spill("Nested calls 16:", final)
