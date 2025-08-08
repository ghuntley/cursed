// Test suite for CURSED Code Linter

yeet "testz"
yeet "linter"

slay test_basic_linting() {
    test_start("Basic linting functionality")
    
    sus clean_code tea = "sus x drip = 42;"
    sus config LinterConfig = default_linter_config()
    sus issues []LintIssue = lint_cursed_code(clean_code, config)
    
    assert_eq_int(len(issues), 0)
}

slay test_missing_semicolon() {
    test_start("Missing semicolon detection")
    
    sus code tea = "sus x drip = 42"  // Missing semicolon
    sus config LinterConfig = default_linter_config()
    sus issues []LintIssue = lint_cursed_code(code, config)
    
    assert_true(len(issues) > 0)
    assert_eq_string(issues[0].rule_id, "missing-semicolon")
    assert_eq_int(issues[0].severity.level, 1)  // Error
}

slay test_line_length_check() {
    test_start("Line length checking")
    
    sus long_line tea = "sus very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_long_variable_name drip = 42;"
    sus config LinterConfig = default_linter_config()
    config.max_line_length = 50
    sus issues []LintIssue = lint_cursed_code(long_line, config)
    
    assert_true(len(issues) > 0)
    assert_eq_string(issues[0].rule_id, "line-too-long")
    assert_eq_int(issues[0].severity.level, 2)  // Warning
}

slay test_naming_conventions() {
    test_start("Naming convention checking")
    
    sus code tea = "sus camelCaseVariable drip = 42;\nslay PascalCaseFunction() { damn 0; }"
    sus config LinterConfig = default_linter_config()
    sus issues []LintIssue = lint_cursed_code(code, config)
    
    sus naming_issues drip = 0
    sus i drip = 0
    bestie (i < len(issues)) {
        ready (issues[i].rule_id == "naming-convention") {
            naming_issues = naming_issues + 1
        }
        i = i + 1
    }
    
    assert_true(naming_issues > 0)
}

slay test_security_issues() {
    test_start("Security issue detection")
    
    sus code tea = "sus password tea = \"secret123\";"
    sus config LinterConfig = default_linter_config()
    sus issues []LintIssue = lint_cursed_code(code, config)
    
    sus security_issues drip = 0
    sus i drip = 0
    bestie (i < len(issues)) {
        ready (issues[i].rule_id == "hardcoded-secret") {
            security_issues = security_issues + 1
        }
        i = i + 1
    }
    
    assert_true(security_issues > 0)
    assert_eq_string(issues[0].category, "security")
}

slay test_gen_z_syntax() {
    test_start("Gen Z syntax suggestions")
    
    sus code tea = "sus flag lit = true;\nprint(\"hello\");"
    sus config LinterConfig = default_linter_config()
    sus issues []LintIssue = lint_cursed_code(code, config)
    
    sus gen_z_issues drip = 0
    sus i drip = 0
    bestie (i < len(issues)) {
        ready (issues[i].category == "gen-z") {
            gen_z_issues = gen_z_issues + 1
        }
        i = i + 1
    }
    
    assert_true(gen_z_issues > 0)
}

slay test_unused_variables() {
    test_start("Unused variable detection")
    
    sus code tea = "sus unused_var drip = 42;\nsus used_var drip = 24;\nvibez.spill(used_var);"
    sus config LinterConfig = default_linter_config()
    sus issues []LintIssue = lint_cursed_code(code, config)
    
    sus unused_issues drip = 0
    sus i drip = 0
    bestie (i < len(issues)) {
        ready (issues[i].rule_id == "unused-variable") {
            unused_issues = unused_issues + 1
        }
        i = i + 1
    }
    
    assert_true(unused_issues > 0)
}

slay test_function_length() {
    test_start("Function length checking")
    
    // Create a long function
    sus long_function tea = "slay long_function() {\n"
    sus i drip = 0
    bestie (i < 60) {  // More than default max of 50
        long_function = concat_str(long_function, "vibez.spill(\"line " + int_to_str(i) + "\");\n")
        i = i + 1
    }
    long_function = concat_str(long_function, "}")
    
    sus config LinterConfig = default_linter_config()
    sus issues []LintIssue = lint_cursed_code(long_function, config)
    
    sus length_issues drip = 0
    sus j drip = 0
    bestie (j < len(issues)) {
        ready (issues[j].rule_id == "function-too-long") {
            length_issues = length_issues + 1
        }
        j = j + 1
    }
    
    assert_true(length_issues > 0)
}

slay test_performance_warnings() {
    test_start("Performance issue detection")
    
    sus code tea = "slay test_function() {\nbestie (i < 100) {\nsus result tea = concat_str(result, \"data\");\n}\n}"
    sus config LinterConfig = default_linter_config()
    sus issues []LintIssue = lint_cursed_code(code, config)
    
    sus perf_issues drip = 0
    sus i drip = 0
    bestie (i < len(issues)) {
        ready (issues[i].category == "performance") {
            perf_issues = perf_issues + 1
        }
        i = i + 1
    }
    
    assert_true(perf_issues > 0)
}

slay test_severity_levels() {
    test_start("Severity level validation")
    
    sus error_sev Severity = error_severity()
    sus warning_sev Severity = warning_severity()
    sus info_sev Severity = info_severity()
    sus hint_sev Severity = hint_severity()
    
    assert_eq_int(error_sev.level, 1)
    assert_eq_int(warning_sev.level, 2)
    assert_eq_int(info_sev.level, 3)
    assert_eq_int(hint_sev.level, 4)
    
    assert_eq_string(error_sev.name, "error")
    assert_eq_string(warning_sev.name, "warning")
    assert_eq_string(info_sev.name, "info")
    assert_eq_string(hint_sev.name, "hint")
}

slay test_config_options() {
    test_start("Configuration options")
    
    sus config LinterConfig = default_linter_config()
    
    // Test disabling checks
    config.check_unused_variables = cringe
    config.check_naming_conventions = cringe
    config.enforce_gen_z_syntax = cringe
    
    sus code tea = "sus camelCase drip = 42;\nsus unused drip = 0;\nsus flag lit = true;"
    sus issues []LintIssue = lint_cursed_code(code, config)
    
    // Should have fewer issues with checks disabled
    sus specific_issues drip = 0
    sus i drip = 0
    bestie (i < len(issues)) {
        ready (issues[i].rule_id == "unused-variable" || 
               issues[i].rule_id == "naming-convention" ||
               issues[i].category == "gen-z") {
            specific_issues = specific_issues + 1
        }
        i = i + 1
    }
    
    assert_eq_int(specific_issues, 0)
}

slay test_lint_output_formatting() {
    test_start("Lint output formatting")
    
    sus code tea = "sus x drip = 42"  // Missing semicolon
    sus issues []LintIssue = lint_cursed_code(code, default_linter_config())
    sus formatted tea = format_lint_results(issues)
    
    assert_true(contains_str(formatted, "CURSED Linter Results"))
    assert_true(contains_str(formatted, "missing-semicolon"))
    assert_true(contains_str(formatted, "Line 1"))
}

slay test_clean_code_output() {
    test_start("Clean code output message")
    
    sus clean_code tea = "sus x drip = 42;"
    sus issues []LintIssue = lint_cursed_code(clean_code, default_linter_config())
    sus formatted tea = format_lint_results(issues)
    
    assert_true(contains_str(formatted, "No lint issues found"))
    assert_true(contains_str(formatted, "fresh"))
}

slay main() {
    vibez.spill("Running CURSED Linter Tests...")
    
    test_basic_linting()
    test_missing_semicolon()
    test_line_length_check()
    test_naming_conventions()
    test_security_issues()
    test_gen_z_syntax()
    test_unused_variables()
    test_function_length()
    test_performance_warnings()
    test_severity_levels()
    test_config_options()
    test_lint_output_formatting()
    test_clean_code_output()
    
    print_test_summary()
    
    vibez.spill("\nLinter tests completed!")
}
