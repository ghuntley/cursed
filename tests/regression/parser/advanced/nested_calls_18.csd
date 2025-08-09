// Nested function calls test 18
slay inner_18(x drip) drip { damn x + 18 }
slay outer_18(y drip) drip { damn inner_18(y * 2) }
slay wrapper_18(z drip) drip { damn outer_18(z + 1) }

sus final drip = wrapper_18(18)
vibez.spill("Nested calls 18:", final)
