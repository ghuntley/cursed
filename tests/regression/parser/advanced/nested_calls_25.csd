// Nested function calls test 25
slay inner_25(x drip) drip { damn x + 25 }
slay outer_25(y drip) drip { damn inner_25(y * 2) }
slay wrapper_25(z drip) drip { damn outer_25(z + 1) }

sus final drip = wrapper_25(25)
vibez.spill("Nested calls 25:", final)
