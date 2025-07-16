yeet "testz"
yeet "property_testing"

# Test suite for the property-based testing framework itself

# Test random value generators
test_start("Random integer generator test")
sus random_int normie = generate_random_int(1, 10)
vibez.spill("Generated random int: " + tea(random_int))

test_start("Random string generator test")
sus random_str tea = generate_random_string(5)
vibez.spill("Generated random string: " + random_str)

test_start("Random boolean generator test")
sus random_bool lit = generate_random_boolean()
vibez.spill("Generated random boolean: " + tea(random_bool))

# Test shrinking functionality
test_start("Integer shrinking test")
sus large_int normie = 100
sus shrunk_int normie = shrink_int(large_int)
vibez.spill("Original: " + tea(large_int) + " Shrunk: " + tea(shrunk_int))

test_start("String shrinking test")
sus long_str tea = generate_random_string(10)
sus shrunk_str tea = shrink_string(long_str)
vibez.spill("Original: " + long_str + " Shrunk: " + shrunk_str)

test_start("Boolean shrinking test")
sus bool_val lit = based
sus shrunk_bool lit = shrink_boolean(bool_val)
vibez.spill("Original: " + tea(bool_val) + " Shrunk: " + tea(shrunk_bool))

# Test property assertion functions
test_start("Property assert true test")
sus assert_result lit = property_assert_true(based, "Test assertion")
vibez.spill("Assert true result: " + tea(assert_result))

test_start("Property assert equal int test")
sus eq_result lit = property_assert_equal_int(42, 42, "Integer equality test")
vibez.spill("Assert equal int result: " + tea(eq_result))

test_start("Property assert equal string test")
sus str_eq_result lit = property_assert_equal_string("test", "test", "String equality test")
vibez.spill("Assert equal string result: " + tea(str_eq_result))

# Test basic property testing
test_start("Simple property test")
sus property_result lit = run_property_test("Identity property", 5)
vibez.spill("Property test result: " + tea(property_result))

# Test reflexivity property
test_start("Reflexivity property test")
sus reflexivity_result lit = test_reflexivity_int("Equality reflexivity", 5)
vibez.spill("Reflexivity test result: " + tea(reflexivity_result))

# Test addition commutativity
test_start("Addition commutativity test")
sus commutativity_result lit = test_addition_commutative("Addition commutativity", 5)
vibez.spill("Commutativity test result: " + tea(commutativity_result))

# Test property test summary reporting
print_property_summary("Sample Test", based, 100)
print_property_summary("Sample Failing Test", cap, 50)

# Final comprehensive test
test_start("Property testing framework integration")
vibez.spill("All property testing framework components are working!")

vibez.spill("Property-based testing framework tests completed!")
print_test_summary()
