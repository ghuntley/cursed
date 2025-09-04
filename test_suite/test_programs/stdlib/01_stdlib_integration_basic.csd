vibe main
yeet "vibez"
yeet "mathz"
yeet "stringz"

fr fr Stdlib Integration Test - Basic Multi-module Usage
fr fr Tests: Integration between vibez, mathz, and stringz modules
fr fr Expected: Successful operations across all modules

slay main_character() {
    vibez.spill("=== Stdlib Integration Test ===")
    
    vibez.spill("Testing mathz operations...")
    sus num1 = 42
    sus num2 = -17
    
    vibez.spill("abs(-17) =", mathz.abs_normie(num2))
    vibez.spill("max(42, 17) =", mathz.max(num1, 17))
    vibez.spill("min(42, 17) =", mathz.min(num1, 17))
    
    vibez.spill("Testing stringz operations...")
    sus text1 = "Hello"
    sus text2 = "World"
    
    vibez.spill("Length of 'Hello':", stringz.length(text1))
    vibez.spill("Uppercase:", stringz.to_upper(text1))
    vibez.spill("Lowercase:", stringz.to_lower(text2))
    
    vibez.spill("Combined operations...")
    sus abs_num = mathz.abs_normie(num2)
    sus num_str = stringz.from_int(abs_num)
    sus final_text = stringz.concat(text1, num_str)
    
    vibez.spill("Final result:", final_text)
    vibez.spill("Final length:", stringz.length(final_text))
    
    vibez.spill("Stdlib integration test completed")
}
