// Nested function calls test 17
slay inner_17(x drip) drip { damn x + 17 }
slay outer_17(y drip) drip { damn inner_17(y * 2) }
slay wrapper_17(z drip) drip { damn outer_17(z + 1) }

sus final drip = wrapper_17(17)
vibez.spill("Nested calls 17:", final)
