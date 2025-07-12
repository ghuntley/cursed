yeet "testz"

test_start("Core module basic test")

// Test simple conversion
sus str_val tea = "Hello from core!"
sus int_val normie = 42

// Test basic functionality
assert_eq_string(str_val, "Hello from core!")
assert_eq_int(int_val, 42)

test_pass("Basic core operations work")

print_test_summary()
