yeet "testz"

test_start("testz basic functions")
vibez.spill("Testing testz framework")

test_start("testz assertion functions")
assert_true(based)
assert_false(cringe)
assert_eq_int(5, 5)
assert_eq_string("test", "test")

test_start("testz utility functions")
sus test_count drip = get_test_count()
sus pass_count drip = get_pass_count()
sus fail_count drip = get_fail_count()
sus all_passed lit = all_tests_passed()
vibez.spill("Current test count:", test_count)
vibez.spill("Current pass count:", pass_count)
vibez.spill("Current fail count:", fail_count)
vibez.spill("All tests passed:", all_passed)

print_test_summary()
