// Nested function calls test 20
slay inner_20(x drip) drip { damn x + 20 }
slay outer_20(y drip) drip { damn inner_20(y * 2) }
slay wrapper_20(z drip) drip { damn outer_20(z + 1) }

sus final drip = wrapper_20(20)
vibez.spill("Nested calls 20:", final)
