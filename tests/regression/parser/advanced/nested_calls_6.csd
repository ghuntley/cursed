// Nested function calls test 6
slay inner_6(x drip) drip { damn x + 6 }
slay outer_6(y drip) drip { damn inner_6(y * 2) }
slay wrapper_6(z drip) drip { damn outer_6(z + 1) }

sus final drip = wrapper_6(6)
vibez.spill("Nested calls 6:", final)
