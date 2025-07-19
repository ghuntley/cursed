yeet "testz"

test_start("WASM PAL Basic Test")

// Test basic WASM functionality
sus x drip = 42
sus y drip = x * 2

assert_eq_int(y, 84)

// Test memory allocation
sus data tea = "Hello WASM!"
assert_eq_string(data, "Hello WASM!")

print_test_summary()
