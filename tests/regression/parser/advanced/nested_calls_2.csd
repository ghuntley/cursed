// Nested function calls test 2
slay inner_2(x drip) drip { damn x + 2 }
slay outer_2(y drip) drip { damn inner_2(y * 2) }
slay wrapper_2(z drip) drip { damn outer_2(z + 1) }

sus final drip = wrapper_2(2)
vibez.spill("Nested calls 2:", final)
