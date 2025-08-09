// Nested function calls test 10
slay inner_10(x drip) drip { damn x + 10 }
slay outer_10(y drip) drip { damn inner_10(y * 2) }
slay wrapper_10(z drip) drip { damn outer_10(z + 1) }

sus final drip = wrapper_10(10)
vibez.spill("Nested calls 10:", final)
