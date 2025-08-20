yeet "testz"

run_test_suite("Working CURSED Testing Suite")

test_section("Basic Assertions")

test_start("boolean tests")
assert_true(based)
assert_false(cringe)

test_start("integer equality tests")
assert_eq_int(2 + 2, 4)
assert_eq_int(10 - 5, 5)
assert_not_eq_int(3, 4)

test_start("string equality tests") 
assert_eq_string("hello", "hello")
assert_not_eq_string("foo", "bar")

test_section("Error cases")

test_start("failing assertions")
assert_eq_int(1, 2)  fr fr This should fail
assert_eq_string("cat", "dog")  fr fr This should fail

test_section("Utilities")

skip_test("This test is intentionally skipped")
test_todo("Add more complex test scenarios")

benchmark_start("simple operations")
sus result drip = 5 + 10
benchmark_end()

print_test_summary()

vibez.spill("Testing framework is working!")
