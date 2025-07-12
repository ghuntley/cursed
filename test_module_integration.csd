# Comprehensive Module Integration Test
# Tests stdlib modules importing and using each other

yeet "testz"
yeet "core"  
yeet "vibez"
yeet "math"
yeet "stringz"

# Test basic imports work
test_start("Module Import Integration Test")

# Test core module functions
sus num normie = 42
sus str_result tea = core.tea(num)
assert_eq_string(str_result, "42")

# Test vibez module functions
vibez.spill("Testing vibez module import")
assert_true(based)

# Test math module functions
sus math_result normie = math.add(5, 10)
assert_eq_int(math_result, 15)

# Test stringz module functions  
sus concat_result tea = stringz.concat("Hello", " World")
assert_eq_string(concat_result, "Hello World")

# Test cross-module dependency
# vibez uses core.tea() internally for formatting
sus format_test tea = vibez.spillstr("Number: %d", core.tea(123))
assert_eq_string(format_test, "Number: 123")

print_test_summary()
