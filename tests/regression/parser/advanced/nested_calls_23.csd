// Nested function calls test 23
slay inner_23(x drip) drip { damn x + 23 }
slay outer_23(y drip) drip { damn inner_23(y * 2) }
slay wrapper_23(z drip) drip { damn outer_23(z + 1) }

sus final drip = wrapper_23(23)
vibez.spill("Nested calls 23:", final)
