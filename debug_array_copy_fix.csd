vibe main
yeet "collections"

slay test_array_parameter(arr) {
    facts("Function received array with length:", collections.length(arr))
    facts("First element:", arr[0])
}

slay main_character() {
    sus my_array drip = ["hello", "world", "test", "array", "copy"]
    facts("Original array length:", collections.length(my_array))
    
    // Pass array to function - this should deep copy to avoid double-free
    test_array_parameter(my_array)
    
    facts("Original array still valid:", collections.length(my_array))
    facts("Test completed successfully!")
}
