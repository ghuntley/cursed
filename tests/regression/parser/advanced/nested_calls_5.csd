// Nested function calls test 5
slay inner_5(x drip) drip { damn x + 5 }
slay outer_5(y drip) drip { damn inner_5(y * 2) }
slay wrapper_5(z drip) drip { damn outer_5(z + 1) }

sus final drip = wrapper_5(5)
vibez.spill("Nested calls 5:", final)
