# Test undefined variable in assignment
sus x drip = 10
sus y drip = x + undefined_var  # This should fail
vibez.spill("Should not reach here:", y)
