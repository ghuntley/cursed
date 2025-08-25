# Simple test for new builtins
yeet "vibez"

vibez.spill("Testing new builtin functions...")

# Test new()
sus obj = new()
vibez.spill("new() result:", obj)

# Test make()
sus arr = make(3)
vibez.spill("make(3) result:", arr)

# Test cap()
sus capacity = cap(arr)
vibez.spill("cap(arr) result:", capacity)

# Test len vs cap
sus length = len(arr)
vibez.spill("len(arr) result:", length)

vibez.spill("All builtin tests completed")
