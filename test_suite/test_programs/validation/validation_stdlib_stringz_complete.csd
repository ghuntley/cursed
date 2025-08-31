vibe main
yeet "vibez"
yeet "stringz"

fr fr Comprehensive validation test for the stringz stdlib module
fr fr Tests tea manipulation operations and edge cases
fr fr Expected: String operations with proper concatenation and length results

slay main_character() {
    vibez.spill("=== STRINGZ MODULE VALIDATION ===")
    
    fr fr Test tea concatenation
    sus str1 tea = "Hello"
    sus str2 tea = "World"
    sus result_concat tea = stringz.concat(str1, str2)
    vibez.spill("concat('Hello', 'World') =")
    vibez.spill(result_concat)
    
    fr fr Test tea length
    sus result_len tea = stringz.length("Testing")
    vibez.spill("length('Testing') =")
    vibez.spill(result_len)
    
    fr fr Test empty tea length
    sus empty_len tea = stringz.length("")
    vibez.spill("length('') =")
    vibez.spill(empty_len)
    
    fr fr Test substring
    sus original tea = "CURSED Language"
    sus sub_result tea = stringz.substring(original, 0, 6)
    vibez.spill("substring('CURSED Language', 0, 6) =")
    vibez.spill(sub_result)
    
    fr fr Test contains
    sus contains_result tea = stringz.contains(original, "CURSED")
    vibez.spill("contains('CURSED Language', 'CURSED') =")
    vibez.spill(contains_result)
    
    fr fr Test to_upper
    sus upper_result tea = stringz.to_upper("cursed")
    vibez.spill("to_upper('cursed') =")
    vibez.spill(upper_result)
    
    fr fr Test to_lower
    sus lower_result tea = stringz.to_lower("PROGRAMMING")
    vibez.spill("to_lower('PROGRAMMING') =")
    vibez.spill(lower_result)
    
    vibez.spill("=== STRINGZ VALIDATION COMPLETE ===")
}
