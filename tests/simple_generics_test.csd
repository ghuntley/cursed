yeet "testz"

// Simple generic function
slay generic_identity<T>(value T) T {
    damn value
}

// Generic function with multiple parameters
slay generic_pair<T, U>(first T, second U) (T, U) {
    damn (first, second)
}

// Generic container
struct Container<T> {
    value T
}

// Test main function
slay main() {
    test_start("Simple Generics Test")
    
    // Test basic generic function
    sus number_result normie = generic_identity<normie>(42)
    assert_eq_int(number_result, 42)
    
    sus string_result tea = generic_identity<tea>("hello")
    assert_eq_string(string_result, "hello")
    
    // Test generic pair
    sus pair (normie, tea) = generic_pair<normie, tea>(123, "world")
    assert_eq_int(pair.0, 123)
    assert_eq_string(pair.1, "world")
    
    // Test generic container
    sus container Container<normie> = Container { value: 456 }
    assert_eq_int(container.value, 456)
    
    print_test_summary()
}
