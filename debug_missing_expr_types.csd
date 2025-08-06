slay test_all_expressions() {
    // Test array access
    sus arr = [1, 2, 3]
    sus first = arr[0]
    
    // Test slice access  
    sus slice = arr[1:3]
    
    // Test type assertion
    sus value normie = 42
    sus asserted = value.(drip)
    
    // Test increment/decrement (if supported)
    value++
    value--
    
    vibez.spill("Testing advanced expressions")
}
