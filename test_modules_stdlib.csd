# CURSED Module System and Standard Library Test

# Test import statements
yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "arrayz"
yeet "testz"

# Test vibez module (I/O)
spill("=== MODULE SYSTEM TEST ===")
vibez.spill("Testing vibez module")

# Test mathz module if available
sus pi_val drip = mathz.pi()
sus sqrt_val drip = mathz.sqrt(16)
sus max_val drip = mathz.max(10, 20)
spill("Pi value:", pi_val)
spill("Square root of 16:", sqrt_val)
spill("Max of 10, 20:", max_val)

# Test stringz module
sus test_str tea = "CURSED Language"
sus upper_str tea = stringz.upper(test_str)
sus lower_str tea = stringz.lower(test_str)
sus str_len drip = stringz.len(test_str)
spill("Original string:", test_str)
spill("Upper case:", upper_str)
spill("Lower case:", lower_str)
spill("String length:", str_len)

# Test arrayz module
sus test_array []drip = [5, 2, 8, 1, 9]
sus sorted_array []drip = arrayz.sort(test_array)
sus sum_val drip = arrayz.sum(test_array)
spill("Original array:", test_array)
spill("Sorted array:", sorted_array)
spill("Sum of array:", sum_val)

# Test testz module
testz.test_start("Module System Test")
testz.assert_eq_int(2 + 2, 4)
testz.assert_eq_str("hello", "hello")
testz.print_test_summary()
