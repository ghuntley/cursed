// CURSED Code Linter - Pure CURSED Implementation
// Self-hosting static analysis tool for code quality

yeet "stringz"
yeet "arrayz"
yeet "testz"

// Lint rule severity levels
squad Severity {
    spill level drip
    spill name tea
}

slay error_severity() Severity {
    damn Severity{level: 1, name: "error"}
}

slay warning_severity() Severity {
    damn Severity{level: 2, name: "warning"}
}

slay info_severity() Severity {
    damn Severity{level: 3, name: "info"}
}

slay hint_severity() Severity {
    damn Severity{level: 4, name: "hint"}
}

// Lint issue representation
squad LintIssue {
    spill rule_id tea
    spill severity Severity
    spill message tea
    spill line drip
    spill column drip
    spill suggestion tea
    spill category tea
}

// Linter configuration
squad LinterConfig {
    spill max_line_length drip
    spill max_function_length drip
    spill enforce_gen_z_syntax lit
    spill check_unused_variables lit
    spill check_naming_conventions lit
    spill check_security_issues lit
    spill check_performance_issues lit
}

// Default linter configuration
slay default_linter_config() LinterConfig {
    damn LinterConfig{
        max_line_length: 100,
        max_function_length: 50,
        enforce_gen_z_syntax: based,
        check_unused_variables: based,
        check_naming_conventions: based,
        check_security_issues: based,
        check_performance_issues: based
    }
}

// Variable tracking for unused variable detection
squad VariableInfo {
    spill name tea
    spill line drip
    spill declared_line drip
    spill used lit
    spill is_parameter lit
}

// Linter state
squad Linter {
    spill config LinterConfig
    spill issues LintIssue[value]
    spill variables VariableInfo[value]
    spill current_line drip
    spill in_function lit
    spill function_start_line drip
}

// Initialize linter
slay init_linter(config LinterConfig) Linter {
    damn Linter{
        config: config,
        issues: [],
        variables: [],
        current_line: 1,
        in_function: cringe,
        function_start_line: 0
    }
}

// Add lint issue
slay add_issue(linter Linter, rule_id tea, severity Severity, message tea, line drip, column drip, suggestion tea, category tea) {
    sus issue LintIssue = LintIssue{
        rule_id: rule_id,
        severity: severity,
        message: message,
        line: line,
        column: column,
        suggestion: suggestion,
        category: category
    }
    push(linter.issues, issue)
}

// Check line length
slay check_line_length(linter Linter, line tea, line_num drip) {
    ready (len_str(line) > linter.config.max_line_length) {
        sus message tea = "Line exceeds maximum length of " + int_to_str(linter.config.max_line_length) + " characters"
        sus suggestion tea = "Break line into multiple lines or shorten variable names"
        add_issue(linter, "line-too-long", warning_severity(), message, line_num, len_str(line), suggestion, "style")
    }
}

// Check naming conventions
slay check_naming_conventions(linter Linter, line tea, line_num drip) {
    ready (!linter.config.check_naming_conventions) {
        damn
    }
    
    // Variable names should be snake_case
    ready (contains_str(line, "sus ")) {
        sus parts tea[value] = split_str(line, " ")
        ready (len(parts) >= 2) {
            sus var_name tea = parts[1]
            ready (contains_str(var_name, "camelCase") || contains_str(var_name, "PascalCase")) {
                sus message tea = "Variable '" + var_name + "' should use snake_case naming"
                sus suggestion tea = "Use snake_case: " + to_snake_case(var_name)
                add_issue(linter, "naming-convention", warning_severity(), message, line_num, 0, suggestion, "style")
            }
        }
    }
    
    // Function names should be snake_case
    ready (contains_str(line, "slay ")) {
        sus parts tea[value] = split_str(line, " ")
        ready (len(parts) >= 2) {
            sus func_name tea = parts[1]
            sus clean_name tea = substring(func_name, 0, index_of(func_name, "("))
            ready (contains_str(clean_name, "camelCase") || contains_str(clean_name, "PascalCase")) {
                sus message tea = "Function '" + clean_name + "' should use snake_case naming"
                sus suggestion tea = "Use snake_case: " + to_snake_case(clean_name)
                add_issue(linter, "naming-convention", warning_severity(), message, line_num, 0, suggestion, "style")
            }
        }
    }
}

// Check for security issues
slay check_security_issues(linter Linter, line tea, line_num drip) {
    ready (!linter.config.check_security_issues) {
        damn
    }
    
    // Check for hardcoded secrets
    ready (contains_str(line, "password") || contains_str(line, "secret") || contains_str(line, "api_key")) {
        ready (contains_str(line, "=") && contains_str(line, "\"")) {
            sus message tea = "Potential hardcoded secret detected"
            sus suggestion tea = "Use environment variables or configuration files for secrets"
            add_issue(linter, "hardcoded-secret", error_severity(), message, line_num, 0, suggestion, "security")
        }
    }
    
    // Check for unsafe operations
    ready (contains_str(line, "unsafe_")) {
        sus message tea = "Unsafe operation detected - review for security implications"
        sus suggestion tea = "Consider using safe alternatives or add safety checks"
        add_issue(linter, "unsafe-operation", warning_severity(), message, line_num, 0, suggestion, "security")
    }
}

// Check for performance issues
slay check_performance_issues(linter Linter, line tea, line_num drip) {
    ready (!linter.config.check_performance_issues) {
        damn
    }
    
    // Check for string concatenation in loops
    ready (contains_str(line, "bestie") && linter.in_function) {
        ready (contains_str(line, "concat_str") || contains_str(line, "+")) {
            sus message tea = "String concatenation in loop may cause performance issues"
            sus suggestion tea = "Consider using string builders or array joining"
            add_issue(linter, "string-concat-loop", info_severity(), message, line_num, 0, suggestion, "performance")
        }
    }
    
    // Check for nested loops
    ready (contains_str(line, "bestie") && count_occurrences(line, "bestie") > 1) {
        sus message tea = "Nested loops detected - consider algorithmic optimization"
        sus suggestion tea = "Review algorithm complexity and consider alternatives"
        add_issue(linter, "nested-loops", info_severity(), message, line_num, 0, suggestion, "performance")
    }
}

// Check Gen Z syntax compliance
slay check_gen_z_syntax(linter Linter, line tea, line_num drip) {
    ready (!linter.config.enforce_gen_z_syntax) {
        damn
    }
    
    // Encourage using 'based' instead of 'true'
    ready (contains_str(line, "true")) {
        sus message tea = "Use 'based' instead of 'true' for authentic Gen Z vibes"
        sus suggestion tea = replace_str(line, "true", "based")
        add_issue(linter, "gen-z-boolean", hint_severity(), message, line_num, 0, suggestion, "gen-z")
    }
    
    // Encourage using 'cringe' instead of 'false'
    ready (contains_str(line, "false")) {
        sus message tea = "Use 'cringe' instead of 'false' for authentic Gen Z vibes"
        sus suggestion tea = replace_str(line, "false", "cringe")
        add_issue(linter, "gen-z-boolean", hint_severity(), message, line_num, 0, suggestion, "gen-z")
    }
    
    // Suggest Gen Z alternatives for common patterns
    ready (contains_str(line, "print")) {
        sus message tea = "Consider using 'vibez.spill' for that Gen Z energy"
        sus suggestion tea = replace_str(line, "print", "vibez.spill")
        add_issue(linter, "gen-z-output", hint_severity(), message, line_num, 0, suggestion, "gen-z")
    }
}

// Track variable usage
slay track_variable_usage(linter Linter, line tea, line_num drip) {
    ready (!linter.config.check_unused_variables) {
        damn
    }
    
    // Track variable declarations
    ready (contains_str(line, "sus ")) {
        sus parts tea[value] = split_str(line, " ")
        ready (len(parts) >= 2) {
            sus var_name tea = parts[1]
            sus var_info VariableInfo = VariableInfo{
                name: var_name,
                line: line_num,
                declared_line: line_num,
                used: cringe,
                is_parameter: cringe
            }
            push(linter.variables, var_info)
        }
    }
    
    // Track variable usage
    sus i drip = 0
    bestie (i < len(linter.variables)) {
        ready (contains_str(line, linter.variables[i].name) && line_num != linter.variables[i].declared_line) {
            linter.variables[i].used = based
        }
        i = i + 1
    }
}

// Check function length
slay check_function_length(linter Linter, line tea, line_num drip) {
    ready (contains_str(line, "slay ")) {
        linter.in_function = based
        linter.function_start_line = line_num
    }
    
    ready (linter.in_function && contains_str(line, "}") && !contains_str(line, "{")) {
        sus function_length drip = line_num - linter.function_start_line
        ready (function_length > linter.config.max_function_length) {
            sus message tea = "Function is too long (" + int_to_str(function_length) + " lines). Consider breaking it up."
            sus suggestion tea = "Split into smaller, focused functions"
            add_issue(linter, "function-too-long", warning_severity(), message, linter.function_start_line, 0, suggestion, "maintainability")
        }
        linter.in_function = cringe
    }
}

// Check for missing semicolons
slay check_missing_semicolons(linter Linter, line tea, line_num drip) {
    sus trimmed tea = trim_str(line)
    
    // Statements that should end with semicolons
    ready ((contains_str(trimmed, "sus ") || contains_str(trimmed, "vibez.spill") || 
            contains_str(trimmed, "damn ") || contains_str(trimmed, "yeet ")) &&
           !ends_with(trimmed, ";") && !ends_with(trimmed, "{") && !ends_with(trimmed, "}")) {
        sus message tea = "Missing semicolon at end of statement"
        sus suggestion tea = trimmed + ";"
        add_issue(linter, "missing-semicolon", error_severity(), message, line_num, len_str(trimmed), suggestion, "syntax")
    }
}

// Main lint function
slay lint_cursed_code(source tea, config LinterConfig) LintIssue[value]{
    sus linter Linter = init_linter(config)
    sus lines tea[value] = split_str(source, "\n")
    
    sus line_num drip = 1
    bestie (line_num <= len(lines)) {
        sus line tea = lines[line_num - 1]
        linter.current_line = line_num
        
        // Skip empty lines and comments
        ready (len_str(trim_str(line)) == 0 || starts_with(trim_str(line), "//")) {
            line_num = line_num + 1
            continue
        }
        
        // Run all checks
        check_line_length(linter, line, line_num)
        check_naming_conventions(linter, line, line_num)
        check_security_issues(linter, line, line_num)
        check_performance_issues(linter, line, line_num)
        check_gen_z_syntax(linter, line, line_num)
        track_variable_usage(linter, line, line_num)
        check_function_length(linter, line, line_num)
        check_missing_semicolons(linter, line, line_num)
        
        line_num = line_num + 1
    }
    
    // Check for unused variables
    check_unused_variables(linter)
    
    damn linter.issues
}

// Check for unused variables at end of analysis
slay check_unused_variables(linter Linter) {
    sus i drip = 0
    bestie (i < len(linter.variables)) {
        ready (!linter.variables[i].used && !linter.variables[i].is_parameter) {
            sus message tea = "Variable '" + linter.variables[i].name + "' is declared but never used"
            sus suggestion tea = "Remove unused variable or use it in your code"
            add_issue(linter, "unused-variable", warning_severity(), message, linter.variables[i].line, 0, suggestion, "cleanup")
        }
        i = i + 1
    }
}

// Format lint results
slay format_lint_results(issues LintIssue[value]) tea {
    sus output tea = ""
    sus i drip = 0
    
    ready (len(issues) == 0) {
        damn "✅ No lint issues found! Your code is looking fresh! 💯"
    }
    
    output = concat_str(output, "🔍 CURSED Linter Results:\n\n")
    
    bestie (i < len(issues)) {
        sus issue LintIssue = issues[i]
        sus severity_icon tea = get_severity_icon(issue.severity)
        
        output = concat_str(output, severity_icon + " ")
        output = concat_str(output, "[" + issue.category + "] ")
        output = concat_str(output, issue.message + "\n")
        output = concat_str(output, "   Line " + int_to_str(issue.line))
        ready (issue.column > 0) {
            output = concat_str(output, ", Column " + int_to_str(issue.column))
        }
        output = concat_str(output, " (" + issue.rule_id + ")\n")
        
        ready (len_str(issue.suggestion) > 0) {
            output = concat_str(output, "   💡 Suggestion: " + issue.suggestion + "\n")
        }
        output = concat_str(output, "\n")
        
        i = i + 1
    }
    
    output = concat_str(output, "📊 Total issues: " + int_to_str(len(issues)))
    damn output
}

// Helper functions
slay get_severity_icon(severity Severity) tea {
    ready (severity.level == 1) { damn "🚨" }  // Error
    ready (severity.level == 2) { damn "⚠️" }   // Warning  
    ready (severity.level == 3) { damn "ℹ️" }   // Info
    damn "💡"  // Hint
}

slay to_snake_case(name tea) tea {
    // Simplified snake_case conversion
    sus result tea = ""
    sus i drip = 0
    bestie (i < len_str(name)) {
        sus char tea = char_at(name, i)
        ready (char >= "A" && char <= "Z") {
            ready (i > 0) {
                result = concat_str(result, "_")
            }
            result = concat_str(result, to_lower(char))
        } otherwise {
            result = concat_str(result, char)
        }
        i = i + 1
    }
    damn result
}

slay count_occurrences(text tea, pattern tea) drip {
    sus count drip = 0
    sus pos drip = 0
    bestie (pos < len_str(text)) {
        sus found drip = index_of_from(text, pattern, pos)
        ready (found == -1) {
            break
        }
        count = count + 1
        pos = found + len_str(pattern)
    }
    damn count
}

// Public API
slay lint_code(source tea) tea {
    sus config LinterConfig = default_linter_config()
    sus issues LintIssue[value] = lint_cursed_code(source, config)
    damn format_lint_results(issues)
}

slay lint_code_with_config(source tea, config LinterConfig) tea {
    sus issues LintIssue[value] = lint_cursed_code(source, config)
    damn format_lint_results(issues)
}

slay main_character() {
    vibez.spill("CURSED Code Linter - Self-Hosting Edition")
    
    // Example usage
    sus sample_code tea = "sus x drip=42\nslay veryLongFunctionName(){vibez.spill(x)password=\"secret123\";}"
    vibez.spill("Analyzing code:")
    vibez.spill(sample_code)
    vibez.spill("")
    
    sus results tea = lint_code(sample_code)
    vibez.spill(results)
}
