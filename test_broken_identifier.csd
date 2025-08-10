# Test cases that should cause identifier resolution failures

# Define some variables
sus x drip = 10
sus y drip = 20

# Try to use an undefined variable - this should fail
vibez.spill("Using undefined variable:", undefined_var)

# Try mathematical expression with undefined variable
sus result drip = x + undefined_var

vibez.spill("Result:", result)
