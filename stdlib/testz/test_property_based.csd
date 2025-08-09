yeet "testz"

fr fr ===== TESTZ PROPERTY-BASED TESTING TESTS =====

test_start("Random Generators")

# Test random seed setting
set_random_seed(123)

# Test random integer generation
sus rand_int drip = random_int(1, 100)
assert_true(rand_int >= 1)
assert_true(rand_int <= 100)

# Test random boolean generation
sus rand_bool lit = random_bool()
assert_true(rand_bool == based || rand_bool == cringe)

# Test random string generation
sus rand_str tea = random_string(10)
assert_eq_int(string_length(rand_str), 10)

test_start("Property Test Framework")

# Test basic property test setup
property_test_start("Test Property Setup", 50)

# Test property assertions
property_assert(based, "This should pass")
property_assert(cringe, "This should fail (intentionally)")

test_start("For-All Integer Properties")

# Test integer range properties
test_property_forall_int("Integer Range Property", 1, 100, 25)

test_start("For-All String Properties")

# Test string length properties
test_property_forall_string("String Length Property", 20, 15)

test_start("Custom Property Tests")

# Test addition commutativity
test_property_custom("Addition Commutative", "test_addition_commutative", 20)

# Test string concatenation length
test_property_custom("String Concat Length", "test_string_concat_length", 15)

# Test list reverse property
test_property_custom("List Reverse Twice", "test_list_reverse_twice", 10)

test_start("Invariant Testing")

# Test list operation invariants
test_invariant("List Operations Invariant", "test_list_operations", 20)

# Test string operation invariants
test_invariant("String Operations Invariant", "test_string_operations", 15)

test_start("Random Data Generation")

# Test random list generation
sus rand_list_int []drip = random_list_int(5, 1, 50)
assert_eq_int(len(rand_list_int), 5)

sus rand_list_string []tea = random_list_string(3, 10)
assert_eq_int(len(rand_list_string), 3)

test_start("Shrinking Functions")

# Test integer shrinking
sus shrunk_ints []drip = shrink_int(42)
assert_true(len(shrunk_ints) > 0)

# Test string shrinking
sus shrunk_strings []tea = shrink_string("hello")
assert_true(len(shrunk_strings) > 0)

test_start("List Operations")

# Test list append
sus test_list []drip = [1, 2, 3]
sus appended []drip = append_to_list_int(test_list, 4)
assert_eq_int(len(appended), 4)

# Test list reverse
sus reversed []drip = reverse_list_int(test_list)
assert_eq_int(len(reversed), 3)

# Test list equality
assert_true(lists_equal_int([1, 2, 3], [1, 2, 3]))
assert_false(lists_equal_int([1, 2, 3], [3, 2, 1]))

test_start("List Formatting")

sus formatted tea = format_list_int([1, 2, 42])
assert_true(contains_substring(formatted, "1"))
assert_true(contains_substring(formatted, "2"))
assert_true(contains_substring(formatted, "42"))

test_start("Number to String Conversion")

assert_eq_string(json_number_to_string(0), "0")
assert_eq_string(json_number_to_string(42), "42")
assert_eq_string(json_number_to_string(100), "100")

test_start("Property Test Example - Arithmetic")

# Demonstrate property-based testing with arithmetic operations
property_test_start("Arithmetic Properties", 30)

bestie i := 0; i < 30; i++ {
    sus a drip = random_int(1, 50)
    sus b drip = random_int(1, 50)
    
    # Property: Multiplication is commutative
    sus product1 drip = a * b
    sus product2 drip = b * a
    property_assert(product1 == product2, "a=" + json_number_to_string(a) + ", b=" + json_number_to_string(b))
    
    # Property: Adding zero doesn't change value
    sus sum_with_zero drip = a + 0
    property_assert(sum_with_zero == a, "a=" + json_number_to_string(a))
    
    # Property: Multiplying by one doesn't change value
    sus product_with_one drip = a * 1
    property_assert(product_with_one == a, "a=" + json_number_to_string(a))
}

test_start("Property Test Example - String Operations")

# Demonstrate property-based testing with string operations
property_test_start("String Properties", 20)

bestie i := 0; i < 20; i++ {
    sus str1 tea = random_string(random_int(1, 15))
    sus str2 tea = random_string(random_int(1, 15))
    
    # Property: String concatenation length
    sus combined tea = str1 + str2
    sus expected_len drip = string_length(str1) + string_length(str2)
    sus actual_len drip = string_length(combined)
    property_assert(actual_len == expected_len, "str1=\"" + str1 + "\", str2=\"" + str2 + "\"")
    
    # Property: Concatenating empty string doesn't change length
    sus with_empty tea = str1 + ""
    property_assert(string_length(with_empty) == string_length(str1), "str=\"" + str1 + "\"")
}

# Print both standard and property test summaries
print_test_summary()
print_property_test_summary()
