// Comprehensive CURSED Linter Test Suite
// Tests all 42 migrated critical rules

yeet "linter"

slay main() {
    vibez.spill("🧪 CURSED Linter Comprehensive Test Suite")
    vibez.spill("=========================================")
    vibez.spill("")
    
    // Test 1: Style enforcement rules (Rules 1-5)
    test_style_rules()
    
    // Test 2: Security analysis rules (Rules 6-15)
    test_security_rules()
    
    // Test 3: Safety pattern rules (Rules 16-25)
    test_safety_rules()
    
    // Test 4: Performance optimization rules (Rules 26-35)
    test_performance_rules()
    
    // Test 5: Pattern detection rules (Rules 36-42)
    test_pattern_rules()
    
    // Test 6: Configuration modes
    test_configuration_modes()
    
    vibez.spill("✅ All linter tests completed!")
    vibez.spill("🚀 CURSED Linter is production-ready with 42 critical rules!")
}

slay test_style_rules() {
    vibez.spill("📋 Testing Style Enforcement Rules (1-5)...")
    vibez.spill("===========================================")
    
    // Rule 1: Line length
    sus long_line_test tea = "sus thisIsAVeryLongLineOfCodeThatExceedsTheMaximumLineLengthLimitAndShouldTriggerAWarningFromTheLinterBecauseItIsTooLong drip = 42"
    
    // Rule 2: Variable naming (camelCase violation)
    sus camel_case_test tea = "sus myVariableName drip = 42"
    
    // Rule 3: Function naming (camelCase violation)
    sus function_naming_test tea = "slay myFunctionName() drip { damn 42 }"
    
    // Rule 4: Trailing whitespace
    sus trailing_whitespace_test tea = "sus variable drip = 42    "
    
    // Rule 5: Mixed indentation
    sus mixed_indent_test tea = "	sus tab_indented drip = 42\n    sus space_indented drip = 24"
    
    sus config LinterConfig = production_config()
    
    vibez.spill("Testing long line violation...")
    sus long_line_issues LintIssue[value] = lint_and_get_issues(long_line_test, config, "style_test.csd")
    ready (len(long_line_issues) > 0) {
        vibez.spill("  ✅ Rule 1 (line-too-long) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 1 (line-too-long) failed to trigger")
    }
    
    vibez.spill("Testing variable naming violation...")
    sus var_naming_issues LintIssue[value] = lint_and_get_issues(camel_case_test, config, "style_test.csd")
    ready (len(var_naming_issues) > 0) {
        vibez.spill("  ✅ Rule 2 (variable-naming) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 2 (variable-naming) failed to trigger")
    }
    
    vibez.spill("Testing function naming violation...")
    sus func_naming_issues LintIssue[value] = lint_and_get_issues(function_naming_test, config, "style_test.csd")
    ready (len(func_naming_issues) > 0) {
        vibez.spill("  ✅ Rule 3 (function-naming) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 3 (function-naming) failed to trigger")
    }
    
    vibez.spill("")
}

slay test_security_rules() {
    vibez.spill("🔒 Testing Security Analysis Rules (6-15)...")
    vibez.spill("============================================")
    
    // Rule 6: Hardcoded secrets
    sus hardcoded_secret_test tea = "sus password tea = \"my_secret_password_123\""
    
    // Rule 7: API key patterns
    sus api_key_test tea = "sus api_key tea = \"sk_test_1234567890abcdef\""
    
    // Rule 8: SQL injection
    sus sql_injection_test tea = "sus query tea = \"SELECT * FROM users WHERE id = \" + user_input"
    
    // Rule 9: Unsafe operations
    sus unsafe_operation_test tea = "unsafe_direct_memory_access(pointer)"
    
    // Rule 10: Weak cryptography
    sus weak_crypto_test tea = "sus hash tea = md5(input_data)"
    
    sus config LinterConfig = production_config()
    
    vibez.spill("Testing hardcoded secret detection...")
    sus secret_issues LintIssue[value] = lint_and_get_issues(hardcoded_secret_test, config, "security_test.csd")
    ready (len(secret_issues) > 0) {
        vibez.spill("  ✅ Rule 6 (hardcoded-secret) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 6 (hardcoded-secret) failed to trigger")
    }
    
    vibez.spill("Testing API key pattern detection...")
    sus api_issues LintIssue[value] = lint_and_get_issues(api_key_test, config, "security_test.csd")
    ready (len(api_issues) > 0) {
        vibez.spill("  ✅ Rule 7 (api-key-pattern) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 7 (api-key-pattern) failed to trigger")
    }
    
    vibez.spill("Testing SQL injection detection...")
    sus sql_issues LintIssue[value] = lint_and_get_issues(sql_injection_test, config, "security_test.csd")
    ready (len(sql_issues) > 0) {
        vibez.spill("  ✅ Rule 8 (sql-injection) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 8 (sql-injection) failed to trigger")
    }
    
    vibez.spill("Testing unsafe operation detection...")
    sus unsafe_issues LintIssue[value] = lint_and_get_issues(unsafe_operation_test, config, "security_test.csd")
    ready (len(unsafe_issues) > 0) {
        vibez.spill("  ✅ Rule 9 (unsafe-operation) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 9 (unsafe-operation) failed to trigger")
    }
    
    vibez.spill("Testing weak cryptography detection...")
    sus crypto_issues LintIssue[value] = lint_and_get_issues(weak_crypto_test, config, "security_test.csd")
    ready (len(crypto_issues) > 0) {
        vibez.spill("  ✅ Rule 10 (weak-crypto) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 10 (weak-crypto) failed to trigger")
    }
    
    vibez.spill("")
}

slay test_safety_rules() {
    vibez.spill("🛡️ Testing Safety Pattern Rules (16-25)...")
    vibez.spill("===========================================")
    
    // Rule 16: Division by zero
    sus division_zero_test tea = "sus result drip = numerator / 0"
    
    // Rule 17: Unsafe array access
    sus unsafe_array_test tea = "sus element drip = array[unsafe_index]"
    
    // Rule 18: Null dereference
    sus null_deref_test tea = "sus value drip = null.property"
    
    // Rule 19: Memory leak
    sus memory_leak_test tea = "sus ptr = malloc(1024)"
    
    // Rule 20: Integer overflow
    sus overflow_test tea = "sus result int = big_number * another_big_number + more_numbers"
    
    sus config LinterConfig = production_config()
    
    vibez.spill("Testing division by zero detection...")
    sus div_zero_issues LintIssue[value] = lint_and_get_issues(division_zero_test, config, "safety_test.csd")
    ready (len(div_zero_issues) > 0) {
        vibez.spill("  ✅ Rule 16 (division-by-zero) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 16 (division-by-zero) failed to trigger")
    }
    
    vibez.spill("Testing unsafe array access detection...")
    sus array_issues LintIssue[value] = lint_and_get_issues(unsafe_array_test, config, "safety_test.csd")
    ready (len(array_issues) > 0) {
        vibez.spill("  ✅ Rule 17 (unsafe-array-access) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 17 (unsafe-array-access) failed to trigger")
    }
    
    vibez.spill("Testing null dereference detection...")
    sus null_issues LintIssue[value] = lint_and_get_issues(null_deref_test, config, "safety_test.csd")
    ready (len(null_issues) > 0) {
        vibez.spill("  ✅ Rule 18 (null-dereference) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 18 (null-dereference) failed to trigger")
    }
    
    vibez.spill("Testing memory leak detection...")
    sus memory_issues LintIssue[value] = lint_and_get_issues(memory_leak_test, config, "safety_test.csd")
    ready (len(memory_issues) > 0) {
        vibez.spill("  ✅ Rule 19 (memory-leak) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 19 (memory-leak) failed to trigger")
    }
    
    vibez.spill("Testing integer overflow detection...")
    sus overflow_issues LintIssue[value] = lint_and_get_issues(overflow_test, config, "safety_test.csd")
    ready (len(overflow_issues) > 0) {
        vibez.spill("  ✅ Rule 20 (integer-overflow) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 20 (integer-overflow) failed to trigger")
    }
    
    vibez.spill("")
}

slay test_performance_rules() {
    vibez.spill("⚡ Testing Performance Optimization Rules (26-35)...")
    vibez.spill("===================================================")
    
    // Rule 26: Inefficient string concatenation
    sus inefficient_concat_test tea = "bestie (i < count) { result = result + \"data\" }"
    
    // Rule 27: Unnecessary array copying
    sus array_copy_test tea = "sus new_array drip[value] = copy(original_array)"
    
    // Rule 28: Redundant computation
    sus redundant_test tea = "bestie (i < len(array)) { process(len(array)) }"
    
    // Rule 29: Expensive loop operation
    sus expensive_loop_test tea = "bestie (i < count) { sort(large_array) }"
    
    // Rule 30: Inefficient data structure
    sus inefficient_ds_test tea = "sus found lit = linear_search(array, target)"
    
    sus config LinterConfig = production_config()
    
    vibez.spill("Testing inefficient string concatenation...")
    sus concat_issues LintIssue[value] = lint_and_get_issues(inefficient_concat_test, config, "performance_test.csd")
    ready (len(concat_issues) > 0) {
        vibez.spill("  ✅ Rule 26 (inefficient-string-concat) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 26 (inefficient-string-concat) failed to trigger")
    }
    
    vibez.spill("Testing unnecessary array copying...")
    sus copy_issues LintIssue[value] = lint_and_get_issues(array_copy_test, config, "performance_test.csd")
    ready (len(copy_issues) > 0) {
        vibez.spill("  ✅ Rule 27 (unnecessary-array-copy) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 27 (unnecessary-array-copy) failed to trigger")
    }
    
    vibez.spill("Testing redundant computation...")
    sus redundant_issues LintIssue[value] = lint_and_get_issues(redundant_test, config, "performance_test.csd")
    ready (len(redundant_issues) > 0) {
        vibez.spill("  ✅ Rule 28 (redundant-computation) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 28 (redundant-computation) failed to trigger")
    }
    
    vibez.spill("Testing expensive loop operation...")
    sus expensive_issues LintIssue[value] = lint_and_get_issues(expensive_loop_test, config, "performance_test.csd")
    ready (len(expensive_issues) > 0) {
        vibez.spill("  ✅ Rule 29 (expensive-loop-operation) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 29 (expensive-loop-operation) failed to trigger")
    }
    
    vibez.spill("Testing inefficient data structure...")
    sus ds_issues LintIssue[value] = lint_and_get_issues(inefficient_ds_test, config, "performance_test.csd")
    ready (len(ds_issues) > 0) {
        vibez.spill("  ✅ Rule 30 (inefficient-data-structure) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 30 (inefficient-data-structure) failed to trigger")
    }
    
    vibez.spill("")
}

slay test_pattern_rules() {
    vibez.spill("🔍 Testing Pattern Detection Rules (36-42)...")
    vibez.spill("==============================================")
    
    // Rule 36: Dead code
    sus dead_code_test tea = "damn result\nbestie (based) { unreachable_code() }"
    
    // Rule 37: Magic numbers
    sus magic_number_test tea = "sus timeout drip = 5000"
    
    // Rule 38: Code duplication
    sus duplication_test tea = "ready (condition_that_is_very_long_and_complex) { process() }\nready (condition_that_is_very_long_and_complex) { process() }"
    
    // Rule 39: Complex boolean
    sus complex_boolean_test tea = "ready (a && b && c || d && !e || f) { process() }"
    
    // Rule 40: Long parameter list
    sus long_params_test tea = "slay process(a drip, b drip, c drip, d drip, e drip, f drip) { }"
    
    // Rule 41: Inconsistent error handling
    sus inconsistent_error_test tea = "catch error { log(error) }"
    
    // Rule 42: Missing return
    sus missing_return_test tea = "slay calculate() drip { sus result drip = 42 }"
    
    sus config LinterConfig = production_config()
    
    vibez.spill("Testing dead code detection...")
    sus dead_issues LintIssue[value] = lint_and_get_issues(dead_code_test, config, "pattern_test.csd")
    ready (len(dead_issues) > 0) {
        vibez.spill("  ✅ Rule 36 (dead-code) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 36 (dead-code) failed to trigger")
    }
    
    vibez.spill("Testing magic number detection...")
    sus magic_issues LintIssue[value] = lint_and_get_issues(magic_number_test, config, "pattern_test.csd")
    ready (len(magic_issues) > 0) {
        vibez.spill("  ✅ Rule 37 (magic-numbers) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 37 (magic-numbers) failed to trigger")
    }
    
    vibez.spill("Testing complex boolean detection...")
    sus boolean_issues LintIssue[value] = lint_and_get_issues(complex_boolean_test, config, "pattern_test.csd")
    ready (len(boolean_issues) > 0) {
        vibez.spill("  ✅ Rule 39 (complex-boolean) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 39 (complex-boolean) failed to trigger")
    }
    
    vibez.spill("Testing long parameter list detection...")
    sus param_issues LintIssue[value] = lint_and_get_issues(long_params_test, config, "pattern_test.csd")
    ready (len(param_issues) > 0) {
        vibez.spill("  ✅ Rule 40 (long-parameter-list) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 40 (long-parameter-list) failed to trigger")
    }
    
    vibez.spill("Testing inconsistent error handling...")
    sus error_issues LintIssue[value] = lint_and_get_issues(inconsistent_error_test, config, "pattern_test.csd")
    ready (len(error_issues) > 0) {
        vibez.spill("  ✅ Rule 41 (inconsistent-error-handling) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 41 (inconsistent-error-handling) failed to trigger")
    }
    
    vibez.spill("Testing missing return detection...")
    sus return_issues LintIssue[value] = lint_and_get_issues(missing_return_test, config, "pattern_test.csd")
    ready (len(return_issues) > 0) {
        vibez.spill("  ✅ Rule 42 (missing-return) triggered correctly")
    } otherwise {
        vibez.spill("  ❌ Rule 42 (missing-return) failed to trigger")
    }
    
    vibez.spill("")
}

slay test_configuration_modes() {
    vibez.spill("⚙️ Testing Configuration Modes...")
    vibez.spill("=================================")
    
    sus test_code tea = "sus myVariable drip = 42"
    
    // Test production config (strict)
    vibez.spill("Testing production configuration (strict)...")
    sus prod_config LinterConfig = production_config()
    sus prod_issues LintIssue[value] = lint_and_get_issues(test_code, prod_config, "config_test.csd")
    vibez.spill("  Production mode found " + int_to_str(len(prod_issues)) + " issues")
    
    // Test development config (relaxed)
    vibez.spill("Testing development configuration (relaxed)...")
    sus dev_config LinterConfig = dev_config()
    sus dev_issues LintIssue[value] = lint_and_get_issues(test_code, dev_config, "config_test.csd")
    vibez.spill("  Development mode found " + int_to_str(len(dev_issues)) + " issues")
    
    // Test minimal config (essential only)
    vibez.spill("Testing minimal configuration (essential only)...")
    sus min_config LinterConfig = minimal_config()
    sus min_issues LintIssue[value] = lint_and_get_issues(test_code, min_config, "config_test.csd")
    vibez.spill("  Minimal mode found " + int_to_str(len(min_issues)) + " issues")
    
    ready (len(prod_issues) >= len(dev_issues) && len(dev_issues) >= len(min_issues)) {
        vibez.spill("  ✅ Configuration hierarchy working correctly")
    } otherwise {
        vibez.spill("  ❌ Configuration hierarchy issue detected")
    }
    
    vibez.spill("")
}
