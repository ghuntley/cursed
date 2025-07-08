yeet "testz"
yeet "spill_facts"

test_start("Basic Spill functionality")
sus result tea = spill_facts.Spill("Hello, world!")
assert_eq_string(result, "Hello, world!")

print_test_summary()
