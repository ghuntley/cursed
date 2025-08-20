# Test edge cases that might break variable dereferencing

# Test 1: Variables in nested expressions
sus a drip = 10
sus b drip = 20
sus nested_result drip = (a + b) * 2
vibez.spill("nested result:", nested_result)

# Test 2: Variables in function calls
slay test_func(value drip) drip {
    damn value * 2
}

sus func_param drip = 15
sus func_result drip = test_func(func_param)
vibez.spill("function result:", func_result)

# Test 3: Variables in array context
sus arr_size drip = 3
sus my_array []drip = [a, b, arr_size]
vibez.spill("array:", my_array)

# Test 4: Undefined variable (should fail gracefully)
sus undefined_var drip = nonexistent_variable + 1
vibez.spill("undefined test:", undefined_var)
