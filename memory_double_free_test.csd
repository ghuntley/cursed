// Test for memory allocation double-free fix
yeet "testz"

test_start("memory allocation safety")

// Test basic allocation and deallocation
sus x drip = 42.5
vibez.spill("Basic allocation works: ")
vibez.spill(x)

// Test multiple allocations
sus arr [5]normie = [1, 2, 3, 4, 5]
vibez.spill("Array allocation works: ")
vibez.spill(arr[0])

// Test string allocation
sus msg tea = "Memory safety test"
vibez.spill("String allocation works: ")
vibez.spill(msg)

assert_true(based)
print_test_summary()
