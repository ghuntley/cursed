// Nested function calls test 9
slay inner_9(x drip) drip { damn x + 9 }
slay outer_9(y drip) drip { damn inner_9(y * 2) }
slay wrapper_9(z drip) drip { damn outer_9(z + 1) }

sus final drip = wrapper_9(9)
vibez.spill("Nested calls 9:", final)
