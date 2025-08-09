// Nested function calls test 7
slay inner_7(x drip) drip { damn x + 7 }
slay outer_7(y drip) drip { damn inner_7(y * 2) }
slay wrapper_7(z drip) drip { damn outer_7(z + 1) }

sus final drip = wrapper_7(7)
vibez.spill("Nested calls 7:", final)
