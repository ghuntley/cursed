# Comprehensive Module Integration Test
# Tests all critical stdlib modules working together

yeet "testz"
yeet "core" 
yeet "vibez"
yeet "math"

# Test 1: testz module functions
test_start("Basic testz functionality")
assert_true(based)
assert_false(cap)
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")

# Test 2: core module type conversions
test_start("Core module integration")
sus num normie = 123
sus str_val tea = core.tea(num)
assert_eq_string(str_val, "123")

sus bool_val lit = core.lit(1)
assert_true(bool_val)

# Test 3: math module operations
test_start("Math module integration")
sus result normie = math.add(10, 20)
assert_eq_int(result, 30)

sus product normie = math.multiply(6, 7)
assert_eq_int(product, 42)

# Test 4: vibez module output
test_start("Vibez module integration")
vibez.spill("Testing vibez output from module")
assert_true(based)

# Test 5: Cross-module dependencies
test_start("Cross-module dependency test")
# Test that modules can use functions from other modules
sus formatted tea = vibez.spillstr("Result: %s", core.tea(42))
assert_eq_string(formatted, "Result: 42")

print_test_summary()
