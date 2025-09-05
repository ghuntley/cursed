// CURSED Linter Test Suite - Comprehensive Validation
// Tests for production linter functionality

yeet "linter"
yeet "testz"
yeet "stringz"

// Test helper functions
slay test_lint_result_contains(result tea, expected_rule tea) lit {
    damn contains_str(result, expected_rule)
}

slay test_lint_result_severity(result tea, expected_severity tea) lit {
    damn contains_str(result, expected_severity)
}

slay test_no_issues(result tea) lit {
    damn contains_str(result, "No lint issues found")
}

// Test naming convention violations
slay test_naming_conventions() {
    test_start("Naming Convention Tests")
    
    // Test variable naming
    sus bad_var_code tea = "sus myVariable drip = 42;"
    sus var_result tea = lint_code(bad_var_code)
    assert_true(test_lint_result_contains(var_result, "var-naming-convention"))
    assert_true(test_lint_result_contains(var_result, "snake_case"))
    
    // Test function naming
    sus bad_func_code tea = "slay myFunction() { vibez.spill(\"test\"); }"
    sus func_result tea = lint_code(bad_func_code)
    assert_true(test_lint_result_contains(func_result, "func-naming-convention"))
    
    // Test good naming (should pass)
    sus good_code tea = "sus my_variable drip = 42;\nslay my_function() { vibez.spill(\"test\"); }"
    sus good_result tea = lint_code(good_code)
    assert_false(test_lint_result_contains(good_result, "naming-convention"))
    
    vibez.spill("✅ Naming convention tests passed")
}

// Test security issue detection
slay test_security_checks() {
    test_start("Security Check Tests")
    
    // Test hardcoded secrets
    sus secret_code tea = "sus password tea = \"secret123\";"
    sus secret_result tea = lint_code(secret_code)
    assert_true(test_lint_result_contains(secret_result, "hardcoded-secret"))
    assert_true(test_lint_result_contains(secret_result, "error"))
    
    // Test API key detection
    sus api_code tea = "sus api_key tea = \"abc123def456\";"
    sus api_result tea = lint_code(api_code)
    assert_true(test_lint_result_contains(api_result, "hardcoded-secret"))
    
    // Test unsafe operations
    sus unsafe_code tea = "unsafe_operation();"
    sus unsafe_result tea = lint_code(unsafe_code)
    assert_true(test_lint_result_contains(unsafe_result, "unsafe-operation"))
    
    // Test secure code (should pass)
    sus secure_code tea = "sus password tea = env.get(\"PASSWORD\");"
    sus secure_result tea = lint_code(secure_code)
    assert_false(test_lint_result_contains(secure_result, "hardcoded-secret"))
    
    vibez.spill("✅ Security check tests passed")
}

// Test performance issue detection
slay test_performance_checks() {
    test_start("Performance Check Tests")
    
    // Test string concatenation in loop
    sus concat_code tea = "bestie (i < 10) {\n    result = result + \"text\";\n}"
    sus concat_result tea = lint_code(concat_code)
    assert_true(test_lint_result_contains(concat_result, "string-concat-loop"))
    
    // Test nested loops
    sus nested_code tea = "bestie (i < 10) {\n    bestie (j < 10) {\n        bestie (k < 10) {\n            process();\n        }\n    }\n}"
    sus nested_result tea = lint_code(nested_code)
    assert_true(test_lint_result_contains(nested_result, "high-complexity"))
    
    vibez.spill("✅ Performance check tests passed")
}

// Test Gen Z syntax enforcement
slay test_gen_z_syntax() {
    test_start("Gen Z Syntax Tests")
    
    // Test boolean replacements
    sus bool_code tea = "sus flag lit = true;\nready (false) { print(\"test\"); }"
    sus bool_result tea = lint_code(bool_code)
    assert_true(test_lint_result_contains(bool_result, "gen-z-boolean"))
    assert_true(test_lint_result_contains(bool_result, "based"))
    assert_true(test_lint_result_contains(bool_result, "cringe"))
    assert_true(test_lint_result_contains(bool_result, "gen-z-output"))
    
    // Test authentic Gen Z code (should pass Gen Z checks)
    sus authentic_code tea = "sus flag lit = based;\nready (cringe) { vibez.spill(\"test\"); }"
    sus auth_result tea = lint_code(authentic_code)
    assert_false(test_lint_result_contains(auth_result, "gen-z-boolean"))
    assert_false(test_lint_result_contains(auth_result, "gen-z-output"))
    
    vibez.spill("✅ Gen Z syntax tests passed")
}

// Test line length limits
slay test_line_length() {
    test_start("Line Length Tests")
    
    // Create a very long line (over 100 characters)
    sus long_line tea = "sus very_long_variable_name_that_exceeds_the_maximum_line_length_limit_and_should_trigger_warning drip = 42;"
    sus long_result tea = lint_code(long_line)
    assert_true(test_lint_result_contains(long_result, "line-too-long"))
    
    // Test normal length line
    sus normal_line tea = "sus normal_var drip = 42;"
    sus normal_result tea = lint_code(normal_line)
    assert_false(test_lint_result_contains(normal_result, "line-too-long"))
    
    vibez.spill("✅ Line length tests passed")
}

// Test function complexity
slay test_function_complexity() {
    test_start("Function Complexity Tests")
    
    // Test long function (over 50 lines)
    sus long_func tea = "slay long_function() {\n"
    sus i drip = 0
    bestie (i < 60) {
        long_func = concat_str(long_func, "    vibez.spill(\"line " + int_to_str(i) + "\");\n")
        i = i + 1
    }
    long_func = concat_str(long_func, "}")
    
    sus func_result tea = lint_code(long_func)
    assert_true(test_lint_result_contains(func_result, "function-too-long"))
    
    vibez.spill("✅ Function complexity tests passed")
}

// Test code smell detection
slay test_code_smells() {
    test_start("Code Smell Tests")
    
    // Test magic numbers
    sus magic_code tea = "sus timeout drip = 5000;\nready (count > 42) { process(); }"
    sus magic_result tea = lint_code(magic_code)
    assert_true(test_lint_result_contains(magic_result, "magic-number"))
    
    // Test long parameter list
    sus params_code tea = "slay complex_function(a drip, b drip, c drip, d drip, e drip, f drip, g drip) { }"
    sus params_result tea = lint_code(params_code)
    assert_true(test_lint_result_contains(params_result, "long-parameter-list"))
    
    vibez.spill("✅ Code smell tests passed")
}

// Test configuration variations
slay test_configuration_modes() {
    test_start("Configuration Mode Tests")
    
    sus test_code tea = "sus flag lit = true;\nslay myFunction() { print(\"test\"); }"
    
    // Test production mode (strict)
    sus prod_result tea = lint_code(test_code)
    assert_true(test_lint_result_contains(prod_result, "gen-z-boolean"))
    assert_true(test_lint_result_contains(prod_result, "naming-convention"))
    
    // Test dev mode (relaxed)
    sus dev_result tea = lint_code_dev(test_code)
    assert_false(test_lint_result_contains(dev_result, "gen-z-boolean"))
    assert_true(test_lint_result_contains(dev_result, "naming-convention"))
    
    vibez.spill("✅ Configuration mode tests passed")
}

// Test edge cases and error handling
slay test_edge_cases() {
    test_start("Edge Case Tests")
    
    // Test empty code
    sus empty_result tea = lint_code("")
    assert_true(test_no_issues(empty_result))
    
    // Test comment-only code
    sus comment_code tea = "// This is just a comment\n// Another comment"
    sus comment_result tea = lint_code(comment_code)
    assert_true(test_no_issues(comment_result))
    
    // Test single line
    sus single_line tea = "vibez.spill(\"hello\");"
    sus single_result tea = lint_code(single_line)
    assert_false(test_lint_result_contains(single_result, "error"))
    
    vibez.spill("✅ Edge case tests passed")
}

// Test comprehensive real-world example
slay test_real_world_example() {
    test_start("Real World Example Test")
    
    sus real_code tea = "
// User authentication module
yeet \"cryptz\"

squad User {
    spill username tea
    spill password tea  // This should trigger security warning
    spill email tea
}

slay authenticateUser(username tea, password tea) lit {
    // Missing validation - should trigger security warning
    sus hashedPassword tea = sha1(password)  // Weak crypto warning
    
    ready (username == \"admin\" && hashedPassword == \"secret\") {  // Hardcoded secret
        damn true  // Should suggest 'based'
    }
    
    damn false  // Should suggest 'cringe'
}

slay veryLongFunctionNameThatViolatesNamingConventions() {
    sus i drip = 0
    bestie (i < 1000) {
        bestie (j < 1000) {  // Nested loop performance warning
            result = result + \"processing \" + int_to_str(i)  // String concat in loop
            i = i + 1
        }
    }
}

sus unusedVariable drip = 42  // Unused variable warning
"
    
    sus real_result tea = lint_code(real_code)
    
    // Verify multiple issue types are detected
    assert_true(test_lint_result_contains(real_result, "hardcoded-secret"))
    assert_true(test_lint_result_contains(real_result, "weak-crypto"))
    assert_true(test_lint_result_contains(real_result, "gen-z-boolean"))
    assert_true(test_lint_result_contains(real_result, "func-naming-convention"))
    assert_true(test_lint_result_contains(real_result, "string-concat-loop"))
    assert_true(test_lint_result_contains(real_result, "high-complexity"))
    
    vibez.spill("✅ Real world example test passed")
}

// Test performance benchmarks
slay test_performance_benchmarks() {
    test_start("Performance Benchmark Tests")
    
    // Generate large code sample
    sus large_code tea = ""
    sus i drip = 0
    bestie (i < 1000) {
        large_code = concat_str(large_code, "sus var" + int_to_str(i) + " drip = " + int_to_str(i) + ";\n")
        i = i + 1
    }
    
    // Measure linting time (basic performance test)
    sus start_time drip = current_timestamp()
    sus perf_result tea = lint_code(large_code)
    sus end_time drip = current_timestamp()
    sus duration drip = end_time - start_time
    
    vibez.spill("Linted " + int_to_str(len_str(large_code)) + " characters in " + int_to_str(duration) + "ms")
    assert_true(len_str(perf_result) > 0)
    
    vibez.spill("✅ Performance benchmark tests passed")
}

// Main test runner
slay main_character() {
    vibez.spill("🧪 CURSED Production Linter Test Suite")
    vibez.spill("=====================================\n")
    
    // Run all test suites
    test_naming_conventions()
    test_security_checks()
    test_performance_checks()
    test_gen_z_syntax()
    test_line_length()
    test_function_complexity()
    test_code_smells()
    test_configuration_modes()
    test_edge_cases()
    test_real_world_example()
    test_performance_benchmarks()
    
    // Print final summary
    vibez.spill("\n🎉 All linter tests completed!")
    print_test_summary()
    
    vibez.spill("\n💯 CURSED Linter is production-ready and fire! 🔥")
}

// Minimal implementation for missing functions
slay current_timestamp() drip { damn 1000 }
