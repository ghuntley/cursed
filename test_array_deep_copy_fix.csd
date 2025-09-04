vibe main
yeet "vibez"

slay process_array(data) {
    vibez.spill("Processing array with elements:")
    sus element drip = data[0]
    vibez.spill(element)
    sus second drip = data[1]
    vibez.spill(second)
}

slay modify_array(data) {
    // This would modify the array if it was shared
    // But with deep copying, original should be untouched
    vibez.spill("Modifying array parameter")
}

slay main_character() {
    vibez.spill("Starting array deep copy test")
    
    // Create an array
    sus my_array drip = [42, 100, 200]
    
    // Pass to first function - should deep copy
    process_array(my_array)
    
    // Pass to second function - should also deep copy
    modify_array(my_array)
    
    // Original array should still be accessible
    vibez.spill("Test completed - no double-free should occur")
}
