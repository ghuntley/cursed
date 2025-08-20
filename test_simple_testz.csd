yeet "testz"

vibez.spill("Before test_start")
test_start("simple test")
vibez.spill("After test_start")

vibez.spill("Before assert_true")
assert_true(based)
vibez.spill("After assert_true")

vibez.spill("Before print_test_summary")
print_test_summary()
vibez.spill("After print_test_summary")
