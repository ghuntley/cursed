vibe main

yeet "vibez"
yeet "mathz"
yeet "stringz"

fr fr Test: Standard library import and usage validation
fr fr Purpose: Ensure all stdlib modules can be imported and used together
fr fr Expected: All imported modules should be accessible and functional

damn main() {
    vibez.spill("=== Testing Multiple Stdlib Imports ===");
    
fr fr Test vibez module
    vibez.spill("Testing vibez output");
    
fr fr Test mathz module
    sus num1: i32 = -15;
    sus num2: i32 = 25;
    sus abs_result: i32 = mathz.abs(num1);
    sus max_result: i32 = mathz.max(num1, num2);
    
    vibez.spill("mathz.abs({}) = {}", num1, abs_result);
    vibez.spill("mathz.max({}, {}) = {}", num1, num2, max_result);
    
fr fr Test stringz module (assuming it exists)
    sus test_string: string = "Hello CURSED";
    sus string_length: i32 = stringz.length(test_string);
    sus upper_string: string = stringz.upper(test_string);
    
    vibez.spill("stringz.length('{}') = {}", test_string, string_length);
    vibez.spill("stringz.upper('{}') = '{}'", test_string, upper_string);
    
fr fr Test combining multiple modules in expressions
    sus combined_result: i32 = mathz.abs(stringz.length("test") - 2);
    vibez.spill("Combined modules result: {}", combined_result);
    
fr fr Test module functions in conditionals
    if mathz.abs(-10) > stringz.length("short") {
        vibez.spill("Module functions work in conditions");
    }
    
fr fr Test nested module calls
    sus text: string = "CURSED";
    sus processed: i32 = mathz.max(stringz.length(text), 3);
    vibez.spill("Nested module calls: mathz.max(stringz.length('{}'), 3) = {}", text, processed);
    
    return 0;
}
