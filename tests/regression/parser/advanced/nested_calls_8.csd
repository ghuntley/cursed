// Nested function calls test 8
slay inner_8(x drip) drip { damn x + 8 }
slay outer_8(y drip) drip { damn inner_8(y * 2) }
slay wrapper_8(z drip) drip { damn outer_8(z + 1) }

sus final drip = wrapper_8(8)
vibez.spill("Nested calls 8:", final)
