// Nested function calls test 1
slay inner_1(x drip) drip { damn x + 1 }
slay outer_1(y drip) drip { damn inner_1(y * 2) }
slay wrapper_1(z drip) drip { damn outer_1(z + 1) }

sus final drip = wrapper_1(1)
vibez.spill("Nested calls 1:", final)
