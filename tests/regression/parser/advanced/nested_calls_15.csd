// Nested function calls test 15
slay inner_15(x drip) drip { damn x + 15 }
slay outer_15(y drip) drip { damn inner_15(y * 2) }
slay wrapper_15(z drip) drip { damn outer_15(z + 1) }

sus final drip = wrapper_15(15)
vibez.spill("Nested calls 15:", final)
