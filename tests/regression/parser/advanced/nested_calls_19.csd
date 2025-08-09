// Nested function calls test 19
slay inner_19(x drip) drip { damn x + 19 }
slay outer_19(y drip) drip { damn inner_19(y * 2) }
slay wrapper_19(z drip) drip { damn outer_19(z + 1) }

sus final drip = wrapper_19(19)
vibez.spill("Nested calls 19:", final)
