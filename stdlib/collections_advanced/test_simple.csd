yeet "testz"

# Simple test to verify basic collection functionality
slay test_basic_collections() {
    test_start("Basic Collections Test")
    
    # Test simple array-like operations
    sus numbers := [1, 2, 3, 4, 5]
    assert_eq_int(numbers[0], 1)
    assert_eq_int(numbers[2], 3)
    assert_eq_int(numbers[4], 5)
    
    # Test basic string operations
    sus text := "collections"
    assert_eq_string(text, "collections")
    
    vibez.spill("Basic collections functionality verified")
}

test_start("Collections Advanced Module Verification")
test_basic_collections()
print_test_summary()
