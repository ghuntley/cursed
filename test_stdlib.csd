# Test 4: Standard library - Module imports (yeet), stdlib function calls

# Import standard library modules
yeet "vibez"
yeet "mathz" 
yeet "stringz"
yeet "arrayz"

# Test vibez (I/O operations)
vibez.spill("Testing stdlib functions")

# Test mathz functions (if available)
sus number drip = 16
sus sqrt_result drip = mathz.sqrt(number)
vibez.spill("Square root of", number, "is", sqrt_result)

sus abs_result drip = mathz.abs(-42)
vibez.spill("Absolute value of -42 is", abs_result)

sus max_result drip = mathz.max(10, 25)
vibez.spill("Max of 10 and 25 is", max_result)

# Test stringz functions (if available)
sus text tea = "Hello World"
sus length drip = stringz.len(text)
vibez.spill("Length of '", text, "' is", length)

sus upper_text tea = stringz.upper(text)
vibez.spill("Uppercase:", upper_text)

sus lower_text tea = stringz.lower(text)
vibez.spill("Lowercase:", lower_text)

# Test arrayz functions (if available)
sus numbers []drip = [1, 2, 3, 4, 5]
sus array_len drip = arrayz.len(numbers)
vibez.spill("Array length:", array_len)

sus sum_result drip = arrayz.sum(numbers)
vibez.spill("Array sum:", sum_result)

sus first_element drip = numbers[0]
vibez.spill("First element:", first_element)

# Test more complex stdlib operations
sus greeting tea = stringz.concat("Hello ", "from ", "CURSED!")
vibez.spill("Concatenated:", greeting)
