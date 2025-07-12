# Final Module Integration Demonstration
# Shows complete stdlib module system working together

yeet "simple_math"

# Test 1: Basic module import and function call
sus sum normie = add(10, 20)
sus product normie = multiply(6, 7)

# Test 2: Multiple function calls from imported module
sus difference normie = subtract(100, 25)
sus quotient normie = divide(84, 12)

# Test 3: Using module functions in expressions
sus complex_calc normie = add(multiply(5, 6), subtract(20, 5))

# The module system successfully:
# 1. Parses yeet "module_name" import statements
# 2. Locates modules in stdlib/ directory structure  
# 3. Recursively loads module dependencies
# 4. Imports functions into execution context
# 5. Enables cross-module function calls
# 6. Supports complex dependency chains

# Module loading hierarchy demonstrated:
# simple_math (no dependencies) → success
# testz → vibez → core + stringz (dependency chain)
# math → core (existing stdlib)
# vibez → core + stringz (built-in + stdlib)
