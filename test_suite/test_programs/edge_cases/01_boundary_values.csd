vibe main
yeet "vibez"
yeet "mathz"
yeet "stringz"

// Test boundary values and edge cases
slay test_numeric_boundaries() {
    vibez.spill("=== Numeric Boundary Tests ===")
    
    // Test zero values
    vibez.spill("Zero value tests:")
    sus zero = 0
    vibez.spill("Zero:", zero)
    vibez.spill("Zero abs:", mathz.abs(zero))
    vibez.spill("Zero power 0:", mathz.pow(zero, 0))
    vibez.spill("Zero power 1:", mathz.pow(zero, 1))
    
    // Test negative values
    vibez.spill("Negative value tests:")
    sus negative = -999
    vibez.spill("Negative:", negative)
    vibez.spill("Negative abs:", mathz.abs(negative))
    vibez.spill("Negative squared:", mathz.pow(negative, 2))
    
    // Test large values
    vibez.spill("Large value tests:")
    sus large = 999999
    vibez.spill("Large value:", large)
    vibez.spill("Large abs:", mathz.abs(large))
    
    // Test edge arithmetic
    vibez.spill("Edge arithmetic tests:")
    vibez.spill("1 + 0 =", 1 + 0)
    vibez.spill("1 * 0 =", 1 * 0)
    vibez.spill("0 * 999 =", 0 * 999)
    vibez.spill("1 - 1 =", 1 - 1)
}

slay test_string_boundaries() {
    vibez.spill("=== String Boundary Tests ===")
    
    // Test empty strings
    vibez.spill("Empty string tests:")
    sus empty_str = ""
    vibez.spill("Empty string length:", stringz.length(empty_str))
    
    // Test single character strings
    vibez.spill("Single character tests:")
    sus single_char = "a"
    vibez.spill("Single char:", single_char)
    vibez.spill("Single char length:", stringz.length(single_char))
    vibez.spill("Single char upper:", stringz.to_upper(single_char))
    vibez.spill("Single char lower:", stringz.to_lower(single_char))
    
    // Test special characters
    vibez.spill("Special character tests:")
    sus special = "!@#$%^&*()"
    vibez.spill("Special chars:", special)
    vibez.spill("Special length:", stringz.length(special))
    
    // Test repeated characters
    vibez.spill("Repeated character tests:")
    sus repeated = "aaaaaaaaaa"
    vibez.spill("Repeated 'a':", repeated)
    vibez.spill("Repeated length:", stringz.length(repeated))
    
    // Test whitespace
    vibez.spill("Whitespace tests:")
    sus whitespace = "   "
    vibez.spill("Whitespace length:", stringz.length(whitespace))
}

slay test_array_boundaries() {
    vibez.spill("=== Array Boundary Tests ===")
    
    // Test empty arrays
    vibez.spill("Empty array tests:")
    sus empty_array []normie = []normie{}
    vibez.spill("Empty array length:", len(empty_array))
    
    // Test single element arrays
    vibez.spill("Single element tests:")
    sus single_array []normie = []normie{42}
    vibez.spill("Single array length:", len(single_array))
    vibez.spill("Single element:", single_array[0])
    
    // Test array with zeros
    vibez.spill("Array with zeros:")
    sus zero_array []normie = []normie{0, 0, 0, 0, 0}
    vibez.spill("Zero array length:", len(zero_array))
    finna i normie = 0; i < len(zero_array); i++ {
        vibez.spill("Zero element", i, ":", zero_array[i])
    }
    
    // Test array growth patterns
    vibez.spill("Array growth tests:")
    sus growing_array []normie = []normie{}
    finna i normie = 0; i < 5; i++ {
        growing_array = append(growing_array, i)
        vibez.spill("After append", i, "length:", len(growing_array))
    }
}

slay test_edge_case_combinations() {
    vibez.spill("=== Edge Case Combinations ===")
    
    // Combine edge values
    sus zero = 0
    sus negative = -1
    sus positive = 1
    
    vibez.spill("Edge combinations:")
    vibez.spill("0 + (-1) =", zero + negative)
    vibez.spill("0 * (-1) =", zero * negative) 
    vibez.spill("(-1) * 1 =", negative * positive)
    vibez.spill("1 - 1 =", positive - positive)
    
    // Edge string operations
    sus empty = ""
    sus space = " "
    sus letter = "x"
    
    vibez.spill("String edge combinations:")
    sus concat1 = stringz.concat(empty, letter)
    sus concat2 = stringz.concat(space, letter)
    sus concat3 = stringz.concat(letter, empty)
    
    vibez.spill("Empty + letter:", concat1, "(len:", stringz.length(concat1), ")")
    vibez.spill("Space + letter:", concat2, "(len:", stringz.length(concat2), ")")  
    vibez.spill("Letter + empty:", concat3, "(len:", stringz.length(concat3), ")")
}

slay test_mathematical_edge_cases() {
    vibez.spill("=== Mathematical Edge Cases ===")
    
    // Division by powers
    vibez.spill("Division edge cases:")
    vibez.spill("8 / 1 =", 8 / 1)
    vibez.spill("8 / 2 =", 8 / 2)
    vibez.spill("8 / 4 =", 8 / 4) 
    vibez.spill("8 / 8 =", 8 / 8)
    
    // Powers
    vibez.spill("Power edge cases:")
    vibez.spill("1 ^ 100 =", mathz.pow(1, 100))
    vibez.spill("2 ^ 0 =", mathz.pow(2, 0))
    vibez.spill("0 ^ 2 =", mathz.pow(0, 2))
    
    // Modulo operations  
    vibez.spill("Modulo edge cases:")
    vibez.spill("0 % 5 =", 0 % 5)
    vibez.spill("1 % 1 =", 1 % 1)
    vibez.spill("7 % 1 =", 7 % 1)
    vibez.spill("7 % 7 =", 7 % 7)
}

slay main() {
    vibez.spill("=== Comprehensive Edge Case Tests ===")
    
    test_numeric_boundaries()
    test_string_boundaries() 
    test_array_boundaries()
    test_edge_case_combinations()
    test_mathematical_edge_cases()
    
    vibez.spill("All edge case tests completed")
}
