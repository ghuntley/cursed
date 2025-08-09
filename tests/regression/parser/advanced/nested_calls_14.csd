// Nested function calls test 14
slay inner_14(x drip) drip { damn x + 14 }
slay outer_14(y drip) drip { damn inner_14(y * 2) }
slay wrapper_14(z drip) drip { damn outer_14(z + 1) }

sus final drip = wrapper_14(14)
vibez.spill("Nested calls 14:", final)
