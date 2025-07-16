// Simple test for mutable reference handling
yeet "testz"

test_start("Simple Borrowing Test")

// Test basic values
sus value normie = 42
vibez.spill("Original value:", value)

// Test basic mutation
value = 84
vibez.spill("Modified value:", value)

// Test string handling
sus name tea = "test-package"
vibez.spill("Package name:", name)

// Test boolean
sus flag lit = based
vibez.spill("Flag:", flag)

// Test assertions
assert_true(value == 84)
assert_true(flag == based)

print_test_summary()
