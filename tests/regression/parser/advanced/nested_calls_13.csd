// Nested function calls test 13
slay inner_13(x drip) drip { damn x + 13 }
slay outer_13(y drip) drip { damn inner_13(y * 2) }
slay wrapper_13(z drip) drip { damn outer_13(z + 1) }

sus final drip = wrapper_13(13)
vibez.spill("Nested calls 13:", final)
