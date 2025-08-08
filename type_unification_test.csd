// Test complex generic types and inference scenarios
yeet "testz"

// Test 1: Complex generic function with type inference
slay generic_map[T, U](arr []T, func slay(T) U) []U {
    sus result []U = []
    sus i drip = 0
    bestie (i < len(arr)) {
        append(result, func(arr[i]))
        i = i + 1
    }
    damn result
}

// Test 2: Nested generic types
slay complex_nested[T](data [][][]T) T {
    damn data[0][0][0]
}

// Test 3: Function with constrained generics
slay add_numeric[T: Numeric](a T, b T) T {
    damn a + b
}

// Test 4: Type inference with constraints
slay test_inference() {
    // These should infer types correctly
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus doubled = generic_map(numbers, slay(x drip) drip { damn x * 2 })
    
    // This should fail at compile time with proper error
    // sus invalid = add_numeric("hello", "world") // Should error: Tea doesn't satisfy Numeric
    
    vibez.spill("Type inference test completed")
}

test_inference()
