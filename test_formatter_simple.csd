fr fr Simple Formatter Test - Basic functionality test
fr fr Testing the AST-based formatter without complex dependencies

yeet "stringz"
yeet "formatter"

slay main() {
    vibez.spill("🎨 CURSED AST-Based Code Formatter Test")
    vibez.spill("Testing comprehensive formatting capabilities")
    vibez.spill("")
    
    fr fr Test basic variable formatting
    vibez.spill("=== TEST 1: Basic Variable Formatting ===")
    sus input1 tea = "sus x drip=42;"
    vibez.spill("Input:  " + input1)
    sus output1 tea = format_cursed_code_ast(input1)
    vibez.spill("Output: " + output1)
    vibez.spill("")
    
    fr fr Test function formatting
    vibez.spill("=== TEST 2: Function Formatting ===")
    sus input2 tea = "slay test(){damn 42;}"
    vibez.spill("Input:  " + input2)
    sus output2 tea = format_cursed_code_ast(input2)
    vibez.spill("Output: " + output2)
    vibez.spill("")
    
    fr fr Test configuration
    vibez.spill("=== TEST 3: Configuration Test ===")
    sus config FormatterConfig = compact_formatter_config()
    sus input3 tea = "slay test(){damn 42;}"
    vibez.spill("Input:  " + input3)
    sus output3 tea = format_cursed_code_with_config_ast(input3, config)
    vibez.spill("Output (compact): " + output3)
    vibez.spill("")
    
    fr fr Test diff generation
    vibez.spill("=== TEST 4: Diff Generation ===")
    sus original tea = "sus x drip=42;"
    sus diff_output tea = format_with_diff(original, default_formatter_config())
    vibez.spill("Diff output:")
    vibez.spill(diff_output)
    vibez.spill("")
    
    fr fr Test syntax validation
    vibez.spill("=== TEST 5: Syntax Validation ===")
    sus valid_code tea = "sus x drip = 42;"
    sus errors []tea = validate_syntax(valid_code)
    vibez.spill("Valid code errors: " + int_to_string(len(errors)))
    
    sus invalid_code tea = "sus x drip = ;"
    sus errors2 []tea = validate_syntax(invalid_code)
    vibez.spill("Invalid code errors: " + int_to_string(len(errors2)))
    vibez.spill("")
    
    fr fr Test advanced formatting
    vibez.spill("=== TEST 6: Complex Code Formatting ===")
    sus complex tea = "ready(x>0){vibez.spill(x);}otherwise{vibez.spill(0);}"
    vibez.spill("Input:  " + complex)
    sus complex_output tea = format_cursed_code_ast(complex)
    vibez.spill("Output: " + complex_output)
    vibez.spill("")
    
    vibez.spill("✅ Formatter tests completed!")
}
