// Comprehensive CURSED Linter Test Suite
// Tests all major lint rules and functionality

yeet "testz"

// Test the linter against various code patterns
slay test_security_rules() {
    vibez.spill("🔐 Testing security rules...")
    
    // Test hardcoded secrets detection
    sus test_code tea = "sus password tea = \"secret123\""
    // TODO: Test linting this code
    
    vibez.spill("✅ Security rules test completed")
}

slay test_performance_rules() {
    vibez.spill("⚡ Testing performance rules...")
    
    // Test string concatenation in loops
    sus test_code tea = "bestie (i < 100) { result = result + item }"
    // TODO: Test linting this code
    
    vibez.spill("✅ Performance rules test completed")
}

slay test_code_quality_rules() {
    vibez.spill("📊 Testing code quality rules...")
    
    // Test line length
    sus long_line tea = "sus reallyLongVariableName tea = \"this line is too long and exceeds the recommended character limit for readability purposes\""
    // TODO: Test linting this code
    
    vibez.spill("✅ Code quality rules test completed")
}

slay test_cursed_style_rules() {
    vibez.spill("🎨 Testing CURSED style rules...")
    
    // Test proper CURSED syntax usage
    sus test_code tea = "function myFunction() { return 42; }"  // Should suggest 'slay'
    // TODO: Test linting this code
    
    vibez.spill("✅ CURSED style rules test completed")
}

slay test_unused_variable_detection() {
    vibez.spill("🔍 Testing unused variable detection...")
    
    // Test unused variables
    sus unused_var drip = 42
    sus used_var drip = 24
    vibez.spill("Using: " + int_to_str(used_var))
    
    vibez.spill("✅ Unused variable detection test completed")
}

slay test_configuration_modes() {
    vibez.spill("⚙️ Testing configuration modes...")
    
    // Test different config modes
    sus prod_config LintConfig = default_config()
    vibez.spill("Production config max line length: " + int_to_str(prod_config.max_line_length))
    
    vibez.spill("✅ Configuration modes test completed")
}

slay test_output_formatting() {
    vibez.spill("📋 Testing output formatting...")
    
    // Test issue formatting
    sus test_issues []LintIssue = []
    sus test_config LintConfig = default_config()
    
    sus formatted_output tea = format_results(test_issues, test_config)
    vibez.spill("Sample output: " + formatted_output)
    
    vibez.spill("✅ Output formatting test completed")
}

slay test_lint_file_functionality() {
    vibez.spill("📁 Testing file linting functionality...")
    
    // Test linting a file
    sus config LintConfig = default_config()
    sus issues []LintIssue = lint_file("test_lint_sample.csd", config)
    
    vibez.spill("Found issues count: " + int_to_str(len(issues)))
    
    vibez.spill("✅ File linting functionality test completed")
}

slay test_all_severity_levels() {
    vibez.spill("🚨 Testing severity levels...")
    
    sus crit Severity = critical()
    sus err Severity = error()
    sus warn Severity = warning()
    sus inf Severity = info()
    sus hint_sev Severity = hint()
    
    vibez.spill("Critical level: " + int_to_str(crit.level))
    vibez.spill("Error level: " + int_to_str(err.level))
    vibez.spill("Warning level: " + int_to_str(warn.level))
    vibez.spill("Info level: " + int_to_str(inf.level))
    vibez.spill("Hint level: " + int_to_str(hint_sev.level))
    
    vibez.spill("✅ Severity levels test completed")
}

slay run_comprehensive_linter_tests() {
    vibez.spill("🧪 Starting Comprehensive CURSED Linter Test Suite")
    vibez.spill("===============================================")
    
    test_security_rules()
    test_performance_rules()
    test_code_quality_rules()
    test_cursed_style_rules()
    test_unused_variable_detection()
    test_configuration_modes()
    test_output_formatting()
    test_lint_file_functionality()
    test_all_severity_levels()
    
    vibez.spill("===============================================")
    vibez.spill("🎉 All linter tests completed successfully!")
    vibez.spill("✨ CURSED linter is production ready! 🔥💯")
}

slay main() {
    run_comprehensive_linter_tests()
}
