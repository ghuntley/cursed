vibe main
yeet "vibez"
yeet "mathz"
yeet "stringz"

fr fr Regression test for tea handling and float precision bugs
fr fr Tests edge cases that previously caused compilation or runtime issues
fr fr Expected: Proper handling of tea operations and float calculations

slay main_character() {
    vibez.spill("=== STRING & FLOAT PRECISION REGRESSION ===")
    
    fr fr Test tea with special characters and escapes
    sus special_string tea = "Test with \"quotes\" and \n newlines"
    sus string_len tea = stringz.length(special_string)
    vibez.spill("Special tea length:")
    vibez.spill(string_len)
    
    fr fr Test float precision operations
    sus float_a meal = 3.14159
    sus float_b meal = 2.71828
    sus float_result normie = mathz.add(float_a, float_b)
    vibez.spill("Float precision test (3.14159 + 2.71828):")
    vibez.spill(float_result)
    
    fr fr Test division that previously caused precision issues
    sus division_result normie = mathz.divide(22.0, 7.0)
    vibez.spill("Division precision test (22.0 / 7.0):")
    vibez.spill(division_result)
    
    fr fr Test tea concatenation with numbers converted to strings
    sus number_as_string tea = stringz.from_number(42)
    sus concat_result tea = stringz.concat("The answer is: ", number_as_string)
    vibez.spill("String-number concatenation:")
    vibez.spill(concat_result)
    
    fr fr Test empty tea handling
    sus empty_str tea = ""
    sus non_empty tea = "content"
    sus empty_concat tea = stringz.concat(empty_str, non_empty)
    vibez.spill("Empty tea concatenation:")
    vibez.spill(empty_concat)
    
    fr fr Test very small float values
    sus tiny_float meal = 0.000001
    sus tiny_doubled normie = mathz.multiply(tiny_float, 2.0)
    vibez.spill("Tiny float doubled:")
    vibez.spill(tiny_doubled)
    
    vibez.spill("=== REGRESSION TESTS COMPLETE ===")
}
