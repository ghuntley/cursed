// Nested function calls test 21
slay inner_21(x drip) drip { damn x + 21 }
slay outer_21(y drip) drip { damn inner_21(y * 2) }
slay wrapper_21(z drip) drip { damn outer_21(z + 1) }

sus final drip = wrapper_21(21)
vibez.spill("Nested calls 21:", final)
