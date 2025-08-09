// Nested function calls test 4
slay inner_4(x drip) drip { damn x + 4 }
slay outer_4(y drip) drip { damn inner_4(y * 2) }
slay wrapper_4(z drip) drip { damn outer_4(z + 1) }

sus final drip = wrapper_4(4)
vibez.spill("Nested calls 4:", final)
