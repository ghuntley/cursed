// Nested function calls test 3
slay inner_3(x drip) drip { damn x + 3 }
slay outer_3(y drip) drip { damn inner_3(y * 2) }
slay wrapper_3(z drip) drip { damn outer_3(z + 1) }

sus final drip = wrapper_3(3)
vibez.spill("Nested calls 3:", final)
