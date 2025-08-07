yeet "stringz"
yeet "mathz"
yeet "arrayz"

# Test basic stdlib functions
sus text tea = "hello"
sus length drip = len_str(text)
vibez.spill("Length of 'hello':", length)

sus num drip = -42
sus abs_val drip = abs_normie(num)
vibez.spill("Absolute value of -42:", abs_val)

# Test user-defined function calling stdlib
slay process_string(s tea) drip {
    sus len drip = len_str(s)
    damn len * 2
}

sus doubled drip = process_string("test")
vibez.spill("Doubled length:", doubled)

# Test nested function calls
slay nested_calc(x drip, y drip) drip {
    sus sum drip = x + y
    damn abs_normie(sum)
}

sus nested_result drip = nested_calc(-10, 5)
vibez.spill("Nested calc result:", nested_result)

# Test function chains
sus chain_result drip = abs_normie(nested_calc(-20, 15))
vibez.spill("Chain result:", chain_result)
