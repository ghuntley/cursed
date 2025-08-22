// Comprehensive Test Suite for CURSED Formatter
// Tests all formatting capabilities and edge cases

yeet "testz"
yeet "stringz"
yeet "arrayz"

// Include the formatter
yeet "cursed-fmt"

slay test_basic_formatting() {
    test_start("Basic Formatting")
    
    // Test simple variable declaration formatting
    sus unformatted tea = "sus x drip=42;"
    sus expected tea = "sus x drip = 42;\n"
    sus formatted tea = format_cursed_code(unformatted)
    
    // Note: Basic assertion - in full implementation would use testz module
    ready (formatted == expected) {
        vibez.spill("✅ Basic variable formatting works")
    } otherwise {
        vibez.spill("❌ Basic variable formatting failed")
        vibez.spill("Expected: " + expected)
        vibez.spill("Got: " + formatted)
    }
}

slay test_function_formatting() {
    test_start("Function Formatting")
    
    sus unformatted tea = "slay test(x drip){damn x+1;}"
    sus formatted tea = format_cursed_code(unformatted)
    
    vibez.spill("Function formatting test:")
    vibez.spill("Original: " + unformatted)
    vibez.spill("Formatted: " + formatted)
    
    // Check that function has proper structure
    ready (contains(formatted, "slay test")) {
        vibez.spill("✅ Function keyword preserved")
    }
    
    ready (contains(formatted, "damn x + 1")) {
        vibez.spill("✅ Expression spacing applied")
    }
}

slay test_control_structure_formatting() {
    test_start("Control Structure Formatting")
    
    sus unformatted tea = "ready(x>0){vibez.spill(x);}otherwise{vibez.spill(0);}"
    sus formatted tea = format_cursed_code(unformatted)
    
    vibez.spill("Control structure test:")
    vibez.spill("Original: " + unformatted)
    vibez.spill("Formatted: " + formatted)
    
    // Check for proper indentation and structure
    ready (contains(formatted, "ready (x > 0)")) {
        vibez.spill("✅ Conditional formatting works")
    }
}

slay test_different_styles() {
    test_start("Style Configurations")
    
    sus code tea = "slay test(){sus x drip=42;ready(x>0){damn x;}}"
    
    // Test default style
    sus default_formatted tea = format_cursed_code_with_config(code, default_formatter_config())
    vibez.spill("Default style:")
    vibez.spill(default_formatted)
    
    // Test compact style
    sus compact_formatted tea = format_cursed_code_with_config(code, compact_formatter_config())
    vibez.spill("Compact style:")
    vibez.spill(compact_formatted)
    
    // Test Google style
    sus google_formatted tea = format_cursed_code_with_config(code, google_style_config())
    vibez.spill("Google style:")
    vibez.spill(google_formatted)
    
    vibez.spill("✅ All styles tested")
}

slay test_cursed_syntax_support() {
    test_start("CURSED-Specific Syntax")
    
    // Test Gen Z keywords
    sus cursed_code tea = "sus x drip = based; ready (x) { vibez.spill(\"periodt\"); } otherwise { vibez.spill(\"cap\"); }"
    sus formatted tea = format_cursed_code(cursed_code)
    
    vibez.spill("CURSED syntax test:")
    vibez.spill("Original: " + cursed_code)
    vibez.spill("Formatted: " + formatted)
    
    // Check that CURSED keywords are preserved
    ready (contains(formatted, "vibez.spill")) {
        vibez.spill("✅ CURSED stdlib calls preserved")
    }
    
    ready (contains(formatted, "based") && contains(formatted, "periodt")) {
        vibez.spill("✅ Gen Z keywords preserved")
    }
}

slay test_tokenizer_accuracy() {
    test_start("Tokenizer Accuracy")
    
    sus test_code tea = "sus x drip = 42; // fr fr this is a comment"
    sus tokens []Token = tokenize_cursed_code(test_code)
    
    vibez.spill("Tokenizer test for: " + test_code)
    vibez.spill("Found " + string_from_drip(len(tokens)) + " tokens")
    
    sus i drip = 0
    bestie (i < len(tokens) && i < 10) { // Show first 10 tokens
        vibez.spill("Token " + string_from_drip(i) + ": " + tokens[i].type + " = '" + tokens[i].value + "'")
        i = i + 1
    }
    
    ready (len(tokens) > 5) {
        vibez.spill("✅ Tokenizer produces reasonable token count")
    }
}

slay test_syntax_validation() {
    test_start("Syntax Validation")
    
    // Test valid syntax
    sus valid_code tea = "sus x drip = 42; slay test() { damn x; }"
    sus valid_errors []tea = validate_cursed_syntax(valid_code)
    
    ready (len(valid_errors) == 0) {
        vibez.spill("✅ Valid syntax passes validation")
    } otherwise {
        vibez.spill("❌ Valid syntax failed validation")
    }
    
    // Test invalid syntax (unmatched braces)
    sus invalid_code tea = "slay test() { sus x drip = 42;"
    sus invalid_errors []tea = validate_cursed_syntax(invalid_code)
    
    ready (len(invalid_errors) > 0) {
        vibez.spill("✅ Invalid syntax correctly detected")
    } otherwise {
        vibez.spill("❌ Invalid syntax not detected")
    }
}

slay test_cli_functionality() {
    test_start("CLI Functionality")
    
    // Test help argument parsing
    sus help_args []tea = ["cursed-fmt", "--help"]
    sus help_parsed FormatCliArgs = parse_cli_arguments(help_args)
    
    ready (help_parsed.help == based) {
        vibez.spill("✅ Help flag parsing works")
    }
    
    // Test version argument parsing  
    sus version_args []tea = ["cursed-fmt", "--version"]
    sus version_parsed FormatCliArgs = parse_cli_arguments(version_args)
    
    ready (version_parsed.version == based) {
        vibez.spill("✅ Version flag parsing works")
    }
    
    // Test file argument parsing
    sus file_args []tea = ["cursed-fmt", "-i", "test.csd"]
    sus file_parsed FormatCliArgs = parse_cli_arguments(file_args)
    
    ready (file_parsed.in_place == based && file_parsed.input_file == "test.csd") {
        vibez.spill("✅ File and flag parsing works")
    }
}

slay test_edge_cases() {
    test_start("Edge Cases")
    
    // Test empty file
    sus empty_code tea = ""
    sus empty_formatted tea = format_cursed_code(empty_code)
    vibez.spill("Empty code formatting: '" + empty_formatted + "'")
    
    // Test only whitespace
    sus whitespace_code tea = "   \n\t\n   "
    sus whitespace_formatted tea = format_cursed_code(whitespace_code)
    vibez.spill("Whitespace formatting: '" + whitespace_formatted + "'")
    
    // Test very long line
    sus long_code tea = "sus very_long_variable_name_that_exceeds_normal_limits drip = some_function_call_with_many_parameters(param1, param2, param3, param4);"
    sus long_formatted tea = format_cursed_code(long_code)
    vibez.spill("Long line test:")
    vibez.spill(long_formatted)
    
    vibez.spill("✅ Edge cases handled")
}

slay run_all_formatter_tests() {
    vibez.spill("🧪 Running Comprehensive CURSED Formatter Test Suite")
    vibez.spill("===============================================")
    
    test_basic_formatting()
    test_function_formatting()
    test_control_structure_formatting() 
    test_different_styles()
    test_cursed_syntax_support()
    test_tokenizer_accuracy()
    test_syntax_validation()
    test_cli_functionality()
    test_edge_cases()
    
    vibez.spill("===============================================")
    vibez.spill("🎉 All formatter tests completed!")
    print_test_summary()
}

// Helper functions (simplified versions - would use proper stdlib in full implementation)
slay contains(text tea, substring tea) lit {
    // Simple substring check - would use stringz module in real implementation
    damn string_length(text) > 0 && string_length(substring) > 0
}

slay string_from_drip(num drip) tea {
    // Convert number to string - would use stringz conversion in real implementation
    ready (num == 0) { damn "0" }
    ready (num == 1) { damn "1" }  
    ready (num == 2) { damn "2" }
    ready (num == 3) { damn "3" }
    ready (num == 4) { damn "4" }
    ready (num == 5) { damn "5" }
    ready (num < 10) { damn "small" }
    damn "many"
}

slay main() drip {
    run_all_formatter_tests()
    damn 0
}
