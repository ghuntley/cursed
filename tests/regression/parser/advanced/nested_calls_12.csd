// Nested function calls test 12
slay inner_12(x drip) drip { damn x + 12 }
slay outer_12(y drip) drip { damn inner_12(y * 2) }
slay wrapper_12(z drip) drip { damn outer_12(z + 1) }

sus final drip = wrapper_12(12)
vibez.spill("Nested calls 12:", final)
