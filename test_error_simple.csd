vibez.spill("Testing basic error_drip functionality")

# Test simple tuple creation
sus test_tuple := ("type", "message", "wrapped", "severity")
vibez.spill("Tuple created successfully")

# Test tuple destructuring
sus (t, m, w, s) := test_tuple
vibez.spill("Type: " + t)
vibez.spill("Message: " + m)

vibez.spill("Basic tuple operations work")
