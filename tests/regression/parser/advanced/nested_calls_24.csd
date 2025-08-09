// Nested function calls test 24
slay inner_24(x drip) drip { damn x + 24 }
slay outer_24(y drip) drip { damn inner_24(y * 2) }
slay wrapper_24(z drip) drip { damn outer_24(z + 1) }

sus final drip = wrapper_24(24)
vibez.spill("Nested calls 24:", final)
