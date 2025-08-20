yeet "testz"

run_test_suite("Comprehensive CURSED Testing Suite")

test_section("Basic Assertions")

test_start("boolean assertions")
assert_true(based)
assert_false(cringe)

test_start("integer equality")
assert_eq_int(2 + 2, 4)
assert_eq_int(10 - 5, 5)
assert_not_eq_int(3, 4)

test_start("string equality")
assert_eq_string("hello", "hello")
assert_not_eq_string("foo", "bar")

test_section("Array Testing")

test_start("integer arrays")
sus expected_ints []drip = [1, 2, 3]
sus actual_ints []drip = [1, 2, 3]
assert_eq_array_int(actual_ints, expected_ints)

test_start("string arrays")
sus expected_strings []tea = ["hello", "world"]
sus actual_strings []tea = ["hello", "world"]
assert_eq_array_string(actual_strings, expected_strings)

test_section("Property Testing")

property_test_start("basic property test")
property_assert(5 > 3, "5 should be greater than 3")
property_assert(based == based, "based should equal based")

test_section("Benchmarking")

benchmark_start("simple arithmetic")
sus result drip = 10 + 20 + 30
benchmark_end()
assert_eq_int(result, 60)

test_section("Utilities")

skip_test("This test is intentionally skipped")
test_todo("Implement advanced concurrency tests")

print_test_summary()
print_property_test_summary()
print_final_summary()
