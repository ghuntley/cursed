vibe main_character

yeet "vibez"
yeet "mathz"
yeet "stringz"

fr fr Test: Standard library import and usage validation
fr fr Purpose: Ensure all stdlib modules can be imported and used together
fr fr Expected: All imported modules should be accessible and functional

slay main_character() {
    vibez.spill("=== Testing Multiple Stdlib Imports ===")
    
fr fr Test vibez module
    vibez.spill("Testing vibez output")
    
fr fr Test mathz module
    sus num1 normie = -15
    sus num2 normie = 25
    sus abs_result normie = mathz.abs(num1)
    sus max_result normie = mathz.max(num1, num2)
    
    vibez.spill("mathz.abs({}) = {}", num1, abs_result)
    vibez.spill("mathz.max({}, {}) = {}", num1, num2, max_result)
    
fr fr Test stringz module (assuming it exists)
    sus test_string tea = "Hello CURSED"
    sus string_length normie = stringz.length(test_string)
    sus upper_string tea = stringz.upper(test_string)
    
    vibez.spill("stringz.length('{}') = {}", test_string, string_length)
    vibez.spill("stringz.upper('{}') = '{}'", test_string, upper_string)
    
fr fr Test combining multiple modules in expressions
    sus combined_result normie = mathz.abs(stringz.length("test") - 2)
    vibez.spill("Combined modules result: {}", combined_result)
    
fr fr Test module functions in conditionals
    ready mathz.abs(-10) > stringz.length("short") {
        vibez.spill("Module functions work in conditions")
    }
    
fr fr Test nested module calls
    sus text tea = "CURSED"
    sus processed normie = mathz.max(stringz.length(text), 3)
    vibez.spill("Nested module calls: mathz.max(stringz.length('{}'), 3) = {}", text, processed)
    
    damn 0
}
