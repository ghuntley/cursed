# P0 Basic Functionality Test
yeet "vibez"
yeet "testz"

test_start("P0_Basic_Test")

# Test 1: Basic Variables
sus basic_int drip = 42
sus basic_string tea = "Hello"
sus basic_bool lit = based

vibez.spill("Basic test:", basic_int, basic_string, basic_bool)
assert_eq_int(basic_int, 42)

# Test 2: Simple Function
slay simple_add(a drip, b drip) drip {
    damn a + b
}

sus result drip = simple_add(10, 20)
assert_eq_int(result, 30)

# Test 3: Array Operations
sus numbers []drip = [1, 2, 3]
sus first drip = numbers[0]
assert_eq_int(first, 1)

print_test_summary()
