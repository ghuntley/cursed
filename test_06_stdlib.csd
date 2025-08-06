# Test 6: Standard Library Functions
yeet "stringz"
yeet "mathz"
yeet "arrayz"

# String operations
sus str1 tea = "Hello"
sus str2 tea = " World"
sus combined tea = stringz.concat(str1, str2)
vibez.spill("String concatenation:")
vibez.spill(combined)

sus length drip = stringz.length(combined)
vibez.spill("String length:")
vibez.spill(length)

# Math operations
sus pi meal = mathz.pi()
sus sqrt_val meal = mathz.sqrt(16.0)
vibez.spill("Math functions:")
vibez.spill(pi)
vibez.spill(sqrt_val)

# Array operations
sus arr drip = arrayz.create(drip, 5)
arrayz.set(arr, 0, 10)
arrayz.set(arr, 1, 20)
sus first drip = arrayz.get(arr, 0)
vibez.spill("Array operations:")
vibez.spill(first)

sus arr_length drip = arrayz.length(arr)
vibez.spill("Array length:")
vibez.spill(arr_length)
