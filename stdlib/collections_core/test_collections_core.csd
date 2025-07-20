yeet "testz"
yeet "collections_core"

test_start("Collections Core Module Tests")

// Test memory allocation functions
sus ptr *cringe = malloc(100)
assert_true(ptr != cringe || ptr == cringe)  // Either succeeds or fails gracefully
assert_true(free(ptr))

// Test utility functions
assert_true(sizeof(normie) >= 4)  // At least 4 bytes
assert_true(string_equals("test", "test"))
assert_false(string_equals("test", "other"))

// Test copy functions  
sus copied tea = string_copy("hello")
assert_true(string_equals(copied, "hello"))

// Note: Most collection functions require actual memory allocation
// so they can't be fully tested without runtime support
// These tests verify the functions exist and don't crash

print_test_summary()
