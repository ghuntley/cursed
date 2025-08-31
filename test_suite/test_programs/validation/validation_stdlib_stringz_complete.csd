vibe main
yeet "vibez"
yeet "stringz"

// Comprehensive validation test for the stringz stdlib module
// Tests string manipulation operations and edge cases
// Expected: String operations with proper concatenation and length results

slay main_character() {
    vibez.spill("=== STRINGZ MODULE VALIDATION ===")
    
    // Test string concatenation
    sus str1 tea = "Hello"
    sus str2 tea = "World"
    sus result_concat tea = stringz.concat(str1, str2)
    vibez.spill("concat('Hello', 'World') =")
    vibez.spill(result_concat)
    
    // Test string length
    sus result_len tea = stringz.length("Testing")
    vibez.spill("length('Testing') =")
    vibez.spill(result_len)
    
    // Test empty string length
    sus empty_len tea = stringz.length("")
    vibez.spill("length('') =")
    vibez.spill(empty_len)
    
    // Test substring
    sus original tea = "CURSED Language"
    sus sub_result tea = stringz.substring(original, 0, 6)
    vibez.spill("substring('CURSED Language', 0, 6) =")
    vibez.spill(sub_result)
    
    // Test contains
    sus contains_result tea = stringz.contains(original, "CURSED")
    vibez.spill("contains('CURSED Language', 'CURSED') =")
    vibez.spill(contains_result)
    
    // Test to_upper
    sus upper_result tea = stringz.to_upper("cursed")
    vibez.spill("to_upper('cursed') =")
    vibez.spill(upper_result)
    
    // Test to_lower
    sus lower_result tea = stringz.to_lower("PROGRAMMING")
    vibez.spill("to_lower('PROGRAMMING') =")
    vibez.spill(lower_result)
    
    vibez.spill("=== STRINGZ VALIDATION COMPLETE ===")
}
