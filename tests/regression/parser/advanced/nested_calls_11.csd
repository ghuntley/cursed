// Nested function calls test 11
slay inner_11(x drip) drip { damn x + 11 }
slay outer_11(y drip) drip { damn inner_11(y * 2) }
slay wrapper_11(z drip) drip { damn outer_11(z + 1) }

sus final drip = wrapper_11(11)
vibez.spill("Nested calls 11:", final)
