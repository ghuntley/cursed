vibe main
yeet "vibez"
yeet "mathz"
yeet "stringz"

fr fr Stdlib Cross-Module Test - Simple Integration
fr fr Tests: Basic operations across multiple stdlib modules
fr fr Expected: Successful integration between modules

slay main() {
    vibez.spill("=== Stdlib Cross-Module Test ===")
    
    vibez.spill("Testing mathz and vibez integration...")
    sus number1 = -42
    sus number2 = 17
    
    sus abs_result = mathz.abs(number1)
    vibez.spill("abs(-42) =", abs_result)
    
    sus max_result = mathz.max(abs_result, number2)
    vibez.spill("max(42, 17) =", max_result)
    
    vibez.spill("Testing stringz and vibez integration...")
    sus text1 = "hello"
    sus text2 = "WORLD"
    
    sus upper_text = stringz.to_upper(text1)
    sus lower_text = stringz.to_lower(text2)
    
    vibez.spill("Upper:", upper_text)
    vibez.spill("Lower:", lower_text)
    
    vibez.spill("Testing combined operations...")
    sus length1 = stringz.length(text1)
    sus length2 = stringz.length(text2)
    
    sus max_length = mathz.max(length1, length2)
    vibez.spill("Max string length:", max_length)
    
    vibez.spill("Testing mathematical string operations...")
    sus combined_length = length1 + length2
    sus doubled_length = mathz.multiply(combined_length, 2)
    
    vibez.spill("Combined length:", combined_length)
    vibez.spill("Doubled length:", doubled_length)
    
    vibez.spill("Cross-module test completed")
}
