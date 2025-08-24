# Comprehensive Stdlib Functionality Test
# Testing actual implementation vs placeholders

# Core Modules Testing
yeet "vibez"
yeet "mathz"
yeet "stringz" 
yeet "arrayz"

vibez.spill("=== CORE MODULES TEST ===")

# Test vibez module
vibez.spill("Testing vibez.spill - WORKS")
sus test_input tea = vibez.input("Enter test (or press enter): ")
vibez.spill("vibez.input result:", test_input)

# Test mathz module
sus math_result drip = mathz.abs(-42)
vibez.spill("mathz.abs(-42):", math_result)

sus sqrt_result drip = mathz.sqrt(16)
vibez.spill("mathz.sqrt(16):", sqrt_result)

sus pi_val drip = mathz.pi()
vibez.spill("mathz.pi():", pi_val)

sus sin_result drip = mathz.sin(1.57)  # approximately pi/2
vibez.spill("mathz.sin(1.57):", sin_result)

# Test stringz module
sus test_str tea = "Hello World"
sus str_len drip = stringz.len(test_str)
vibez.spill("stringz.len('Hello World'):", str_len)

sus upper_str tea = stringz.upper(test_str)
vibez.spill("stringz.upper:", upper_str)

sus contains_result lit = stringz.contains(test_str, "World")
vibez.spill("stringz.contains('Hello World', 'World'):", contains_result)

sus split_result []tea = stringz.split(test_str, " ")
vibez.spill("stringz.split result length:", stringz.len(split_result[0]))

# Test arrayz module
sus test_array []drip = [1, 2, 3, 4, 5]
sus array_len drip = arrayz.len(test_array)
vibez.spill("arrayz.len([1,2,3,4,5]):", array_len)

sus array_sum drip = arrayz.sum(test_array)
vibez.spill("arrayz.sum([1,2,3,4,5]):", array_sum)

sus array_max drip = arrayz.max(test_array)
vibez.spill("arrayz.max([1,2,3,4,5]):", array_max)

vibez.spill("=== CORE MODULES TEST COMPLETE ===")
