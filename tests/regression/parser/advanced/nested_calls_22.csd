// Nested function calls test 22
slay inner_22(x drip) drip { damn x + 22 }
slay outer_22(y drip) drip { damn inner_22(y * 2) }
slay wrapper_22(z drip) drip { damn outer_22(z + 1) }

sus final drip = wrapper_22(22)
vibez.spill("Nested calls 22:", final)
