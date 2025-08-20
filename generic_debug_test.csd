// Debug test for generic instantiation issues
yeet "testz"

// Simple generic identity function
slay identity[T](value T) T {
    damn value
}

// Generic pair function
slay make_pair[T, U](first T, second U) {
    damn (first, second)
}

// Generic container struct
squad Container[T] {
    value T
}

slay main() {
    test_start("Generic Debug Test")
    
    // Test 1: Generic function with explicit type
    vibez.spill("Testing identity[tea]...")
    sus str_result tea = identity[tea]("hello")
    vibez.spill("Result:", str_result)
    assert_eq_string(str_result, "hello")
    
    // Test 2: Generic function with number type
    vibez.spill("Testing identity[drip]...")
    sus num_result drip = identity[drip](42)
    vibez.spill("Result:", num_result)
    assert_eq_int(num_result, 42)
    
    // Test 3: Generic struct instantiation
    vibez.spill("Testing Container[drip]...")
    sus container Container[drip] = Container { value: 123 }
    vibez.spill("Container value:", container.value)
    assert_eq_int(container.value, 123)
    
    print_test_summary()
}
