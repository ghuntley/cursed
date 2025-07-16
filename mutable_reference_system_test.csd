// Test mutable reference handling and borrowing system integration
yeet "testz"

test_start("Mutable Reference System Tests")

// Test basic mutable reference creation
sus value drip = 42.0
vibez.spill("Original value:", value)

// Test mutable borrowing
sus mutable_ref := &value
*mutable_ref = 84.0
vibez.spill("Modified value:", value)

// Test shared references
sus shared_ref1 := &value
sus shared_ref2 := &value
vibez.spill("Shared ref 1:", *shared_ref1)
vibez.spill("Shared ref 2:", *shared_ref2)

// Test borrowing rules - multiple shared refs should work
assert_eq_float(*shared_ref1, 84.0)
assert_eq_float(*shared_ref2, 84.0)

// Test GC integration with references
sus test_array := [1, 2, 3, 4, 5]
sus array_ref := &test_array
vibez.spill("Array through ref:", *array_ref)

// Test package manager mutable state
sus package_name tea = "test-package"
sus package_version tea = "1.0.0"
vibez.spill("Package:", package_name, "version:", package_version)

// Test runtime value mutable access
sus runtime_value := 100
sus runtime_ref := &runtime_value
*runtime_ref = 200
vibez.spill("Runtime value after mutation:", runtime_value)

assert_true(value > 80.0)
assert_true(runtime_value == 200)

print_test_summary()
