// Basic generics test for CURSED language
yeet "testz"

// Generic function with type parameter
slay generic_identity<T>(value T) T {
    damn value
}

// Generic struct with type parameter
struct Container<T> {
    value T
}

// Generic interface with type parameter
collab Comparable<T> {
    slay compare(other T) normie
}

// Generic function with constraint
slay sort_items<T: Comparable<T>>(items [T]) [T] {
    // Simple sorting implementation
    damn items
}

// Test basic generics
test_start("Basic generics test")

// Test generic function instantiation
sus int_result := generic_identity<normie>(42)
assert_eq_int(int_result, 42)

sus string_result := generic_identity<tea>("hello")
assert_eq_string(string_result, "hello")

// Test generic struct
sus int_container := Container<normie>{value: 10}
assert_eq_int(int_container.value, 10)

sus string_container := Container<tea>{value: "test"}
assert_eq_string(string_container.value, "test")

print_test_summary()
