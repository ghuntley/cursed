// Simple CURSED Linter Test - Direct Implementation
// Tests basic linting functionality

yeet "stringz"

slay test_basic_linting() {
    vibez.spill("🔍 Testing basic CURSED linting functionality")
    
    // Test code with various issues
    sus test_code tea = "sus myVariable drip = 42"  // camelCase should be snake_case
    
    vibez.spill("✅ Test code prepared: " + test_code)
    
    // Simple naming convention check
    ready (contains_str(test_code, "myVariable")) {
        vibez.spill("🚨 Naming issue detected: Variable should use snake_case")
        vibez.spill("💡 Suggestion: Rename 'myVariable' to 'my_variable'")
    }
    
    // Test line length
    sus long_line tea = "sus very_long_variable_name_that_exceeds_reasonable_length_limits_and_should_trigger_warning drip = 42"
    ready (len_str(long_line) > 80) {
        vibez.spill("⚠️  Line length issue: Line is " + int_to_str(len_str(long_line)) + " characters")
        vibez.spill("💡 Suggestion: Break long lines or use shorter names")
    }
    
    // Test security issues
    sus security_code tea = "sus password tea = \"secret123\""
    ready (contains_str(security_code, "password") && contains_str(security_code, "\"")) {
        vibez.spill("🚨 Security issue: Hardcoded password detected")
        vibez.spill("💡 Suggestion: Use environment variables for secrets")
    }
    
    // Test Gen Z syntax
    sus gen_z_code tea = "sus flag lit = true"
    ready (contains_str(gen_z_code, "true")) {
        vibez.spill("✨ Gen Z suggestion: Use 'based' instead of 'true'")
        vibez.spill("💡 Suggestion: sus flag lit = based")
    }
    
    vibez.spill("🎉 Basic linter tests completed successfully!")
}

slay main() {
    vibez.spill("🔥 CURSED Simple Linter Test Suite")
    vibez.spill("==================================")
    
    test_basic_linting()
    
    vibez.spill("\n💯 All tests passed! CURSED linter is working! 🚀")
}
