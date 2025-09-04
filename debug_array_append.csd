vibe main
yeet "vibez"

slay main_character() {
    vibez.spill("=== Array Append Debug Test ===")
    
    fr fr Initialize empty array - using working syntax from existing tests
    sus test_array [normie] = [normie]{}
    vibez.spill("Initial array length:", len(test_array))
    
    fr fr First append
    test_array = append(test_array, 10)
    vibez.spill("After first append - length:", len(test_array), "element:", test_array[0])
    
    fr fr Second append
    test_array = append(test_array, 20)
    vibez.spill("After second append - length:", len(test_array), "elements:", test_array[0], test_array[1])
    
    fr fr Third append
    test_array = append(test_array, 30)
    vibez.spill("After third append - length:", len(test_array), "elements:", test_array[0], test_array[1], test_array[2])
    
    vibez.spill("=== Final Test Results ===")
    vibez.spill("Expected length: 3, Actual length:", len(test_array))
}
