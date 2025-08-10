# Test closure functionality
yeet "vibez"

# Simple lambda without captures (should compile to function pointer)
sus simple_lambda = slay(x drip) drip { damn x * 2 }

# Lambda with captured variable (should compile to closure)
sus multiplier drip = 5
sus closure_lambda = slay(x drip) drip { damn x * multiplier }

# Test simple lambda
sus result1 drip = simple_lambda(10)
vibez.spill("Simple lambda result:", result1)

# Test closure lambda
sus result2 drip = closure_lambda(10)
vibez.spill("Closure lambda result:", result2)

# Test that closure captures current value
multiplier = 3
sus result3 drip = closure_lambda(10)
vibez.spill("Closure with modified captured var:", result3)
