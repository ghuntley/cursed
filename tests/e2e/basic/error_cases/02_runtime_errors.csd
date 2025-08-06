# This file contains intentional runtime errors for testing error handling

yeet "testz"

test_start("Runtime Error Tests")

# Array out of bounds
sus arr := [1, 2, 3]
vibez.spill("Accessing invalid index...")
# This should cause a runtime error
vibez.spill("Value: " + str(arr[10]))

print_test_summary()
