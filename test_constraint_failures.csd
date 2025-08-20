yeet "vibez"

// This should cause constraint violations if the system is working

// Test 1: Try to use a non-numeric type with numeric constraint
slay add_numbers[T: Numeric](a T, b T) T {
    damn a + b
}

// Test 2: Try to compare non-comparable types
slay compare_values[T: Comparable](a T, b T) lit {
    damn a == b  
}

slay main() vibes {
    vibez.spill("Testing constraint violations...")
    
    // This should fail - strings don't satisfy Numeric constraint
    sus bad_result = add_numbers("hello", "world")
    vibez.spill("Bad addition result:", bad_result)
    
    // This should fail - comparing incompatible types
    // sus bad_compare = compare_values(42, "string")
    
    vibez.spill("If you see this, constraint checking is not working properly")
}
