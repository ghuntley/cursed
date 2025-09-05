yeet "testz"
yeet "property_test"

fr fr Simple test suite for property-based testing framework

fr fr Test basic generator functionality
test_start("Random number generator produces values in range")
set_seed(42)
sus test_passed lit = based
sus i normie = 0
bestie i < 10 {
    sus value normie = gen_int(10, 20)
    vibes value < 10 || value > 20 {
        test_passed = cap
        ghosted
    }
    i = i + 1
}
assert_true(test_passed)

fr fr Test boolean generator
test_start("Boolean generator works")
set_seed(123)
sus bool_val lit = gen_boolean()
assert_true(bool_val == based || bool_val == cap)

fr fr Test positive integer generator
test_start("Positive integer generator produces positive values")
set_seed(456)
sus pos_val normie = gen_positive_int()
assert_true(pos_val > 0)

fr fr Test string generator
test_start("String generator produces strings")
set_seed(789)
sus str_val tea = gen_small_string()
assert_true(str_val != "")

fr fr Test simple property
test_start("Simple property test works")
set_seed(101)
sus simple_result lit = run_simple_property_test("numbers equal themselves")
assert_true(simple_result)

fr fr Test mathematical properties
set_seed(202)
sus comm_result lit = test_addition_commutative()
assert_true(comm_result)

set_seed(303)
sus identity_result lit = test_identity_property()
assert_true(identity_result)

fr fr Test configuration
test_start("Configuration works")
sus original_count normie = property_test_count
set_test_count(5)
assert_eq_int(property_test_count, 5)
set_test_count(original_count)

print_test_summary()
