// Test complete IR node coverage - Oracle Priority 3 validation

sus main_character() {
    // Test ternary operator: condition ? true_value : false_value
    sus age drip = 25
    sus status tea = age > 18 ? "adult" : "minor"
    vibez.spill("Status: " + status)
    
    // Test slice operations: array[start:end]
    facts numbers []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    facts slice []drip = numbers[2:6]  // [3, 4, 5, 6]
    vibez.spill("Slice length: " + slice.len)
    
    // Test tuple access: tuple.index
    facts point (drip, drip) = (10, 20)
    sus x drip = point.0
    sus y drip = point.1
    vibez.spill("Point: (" + x + ", " + y + ")")
    
    // Test defer statements (LIFO execution)
    defer vibez.spill("Defer 3: Last executed")
    defer vibez.spill("Defer 2: Second to last")
    defer vibez.spill("Defer 1: Third to last")
    
    // Test question mark operator for error propagation
    sus result drip = risky_operation()? // Early return on error
    vibez.spill("Result: " + result)
    
    vibez.spill("Main function body complete")
    // Implicit return - function ends without explicit return
}

slay risky_operation() -> (drip, error?) {
    // Returns a result that could have an error
    damn (42, null) // Success case
}

// Test complex expression with multiple IR nodes
slay complex_demo() {
    facts data []drip = [1, 2, 3, 4, 5]
    
    // Combine ternary, slice, and tuple access
    facts processed = data.len > 3 ? data[1:4] : data[0:2]
    facts tuple_result = (processed[0], processed.len)
    
    defer vibez.spill("Cleanup complete")
    
    vibez.spill("Complex demo result: " + tuple_result.0)
}
