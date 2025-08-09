# Comprehensive stdlib memory corruption test
# Loads all major stdlib modules and uses their functions extensively

# Load all stdlib modules
yeet "mathz"
yeet "stringz" 
yeet "arrayz"
yeet "cryptz"
yeet "testz"

# Test mathz functions
sus math_test1 drip = abs_normie(-100)
sus math_test2 drip = max_normie(50, 75)
sus math_test3 drip = power_int(3, 4)
sus math_test4 drip = factorial(5)

# Test string functions
sus str1 tea = "Hello"
sus str2 tea = "World"
sus concat_result tea = concat_strings(str1, str2)

# Test array functions
sus test_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
sus array_sum drip = sum_array(test_array)
sus array_max drip = find_max(test_array)
sus array_min drip = find_min(test_array)

# Test crypto functions (basic ones)
sus random_val drip = crypto_secure_random_u32()

# Test testz functions
test_start("Memory Corruption Test")
assert_true(math_test1 > 0)
assert_true(array_sum > 0)
print_test_summary()

vibez.spill("Stdlib memory corruption test completed!")
