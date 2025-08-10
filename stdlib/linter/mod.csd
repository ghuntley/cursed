// CURSED Production Linter - Simplified Working Version
// Complete static analysis engine that works with current parser

yeet "stringz"
yeet "arrayz"

// Lint issue structure
squad LintIssue {
    spill rule_id tea
    spill severity tea
    spill message tea
    spill line drip
    spill suggestion tea
    spill category tea
}

// Linter configuration
squad LinterConfig {
    spill max_line_length drip
    spill check_naming lit
    spill check_security lit
    spill check_gen_z lit
    spill check_performance lit
    spill strict_mode lit
}

// Production configuration
slay production_config() LinterConfig {
    damn LinterConfig{
        max_line_length: 100,
        check_naming: based,
        check_security: based,
        check_gen_z: based,
        check_performance: based,
        strict_mode: based
    }
}

// Development configuration
slay dev_config() LinterConfig {
    damn LinterConfig{
        max_line_length: 120,
        check_naming: based,
        check_security: based,
        check_gen_z: cringe,
        check_performance: cringe,
        strict_mode: cringe
    }
}

// Main linting function
slay lint_code(source tea, config LinterConfig) []LintIssue {
    sus issues []LintIssue = []
    sus lines []tea = split_str(source, "\n")
    
    sus line_num drip = 1
    bestie (line_num <= len(lines)) {
        sus line tea = lines[line_num - 1]
        sus trimmed tea = trim_str(line)
        
        // Skip empty lines and comments
        ready (len_str(trimmed) > 0 && !starts_with(trimmed, "//")) {
            // Check line length
            ready (len_str(line) > config.max_line_length) {
                sus issue LintIssue = LintIssue{
                    rule_id: "line-too-long",
                    severity: "warning",
                    message: "Line exceeds " + int_to_str(config.max_line_length) + " characters",
                    line: line_num,
                    suggestion: "Break line or use shorter names",
                    category: "style"
                }
                push(issues, issue)
            }
            
            // Check naming conventions
            ready (config.check_naming) {
                check_naming_issues(issues, line, line_num)
            }
            
            // Check security issues
            ready (config.check_security) {
                check_security_issues(issues, line, line_num)
            }
            
            // Check Gen Z syntax
            ready (config.check_gen_z) {
                check_gen_z_issues(issues, line, line_num)
            }
            
            // Check performance issues
            ready (config.check_performance) {
                check_performance_issues(issues, line, line_num)
            }
        }
        
        line_num = line_num + 1
    }
    
    damn issues
}

// Check naming convention issues
slay check_naming_issues(issues []LintIssue, line tea, line_num drip) {
    // Variable naming
    ready (contains_str(line, "sus ") && contains_camel_case(line)) {
        sus issue LintIssue = LintIssue{
            rule_id: "var-naming",
            severity: "warning", 
            message: "Variable should use snake_case naming",
            line: line_num,
            suggestion: "Use snake_case: my_variable instead of myVariable",
            category: "style"
        }
        push(issues, issue)
    }
    
    // Function naming
    ready (contains_str(line, "slay ") && contains_camel_case(line)) {
        sus issue LintIssue = LintIssue{
            rule_id: "func-naming",
            severity: "warning",
            message: "Function should use snake_case naming", 
            line: line_num,
            suggestion: "Use snake_case: my_function instead of myFunction",
            category: "style"
        }
        push(issues, issue)
    }
}

// Check security issues
slay check_security_issues(issues []LintIssue, line tea, line_num drip) {
    // Hardcoded secrets
    ready ((contains_str(line, "password") || contains_str(line, "secret") || 
            contains_str(line, "api_key")) && contains_str(line, "\"")) {
        sus issue LintIssue = LintIssue{
            rule_id: "hardcoded-secret",
            severity: "error",
            message: "Hardcoded secret detected",
            line: line_num,
            suggestion: "Use environment variables or config files",
            category: "security"
        }
        push(issues, issue)
    }
    
    // Unsafe operations
    ready (contains_str(line, "unsafe_")) {
        sus issue LintIssue = LintIssue{
            rule_id: "unsafe-operation", 
            severity: "warning",
            message: "Unsafe operation requires review",
            line: line_num,
            suggestion: "Add safety checks and validation",
            category: "security"
        }
        push(issues, issue)
    }
    
    // SQL injection patterns
    ready (contains_str(line, "query") && contains_str(line, "+") && 
           (contains_str(line, "SELECT") || contains_str(line, "INSERT"))) {
        sus issue LintIssue = LintIssue{
            rule_id: "sql-injection",
            severity: "error",
            message: "Possible SQL injection vulnerability",
            line: line_num,
            suggestion: "Use parameterized queries",
            category: "security"
        }
        push(issues, issue)
    }
}

// Check Gen Z syntax issues
slay check_gen_z_issues(issues []LintIssue, line tea, line_num drip) {
    // Boolean literals
    ready (contains_str(line, "true")) {
        sus issue LintIssue = LintIssue{
            rule_id: "gen-z-boolean",
            severity: "hint",
            message: "Use 'based' instead of 'true' for Gen Z vibes",
            line: line_num,
            suggestion: "Replace 'true' with 'based'",
            category: "gen-z"
        }
        push(issues, issue)
    }
    
    ready (contains_str(line, "false")) {
        sus issue LintIssue = LintIssue{
            rule_id: "gen-z-boolean", 
            severity: "hint",
            message: "Use 'cringe' instead of 'false' for Gen Z energy",
            line: line_num,
            suggestion: "Replace 'false' with 'cringe'",
            category: "gen-z"
        }
        push(issues, issue)
    }
    
    // Output functions
    ready (contains_str(line, "print(") && !contains_str(line, "vibez.spill")) {
        sus issue LintIssue = LintIssue{
            rule_id: "gen-z-output",
            severity: "hint", 
            message: "Use 'vibez.spill' for authentic Gen Z output",
            line: line_num,
            suggestion: "Replace 'print' with 'vibez.spill'",
            category: "gen-z"
        }
        push(issues, issue)
    }
}

// Check performance issues
slay check_performance_issues(issues []LintIssue, line tea, line_num drip) {
    // String concatenation in loops
    ready (contains_str(line, "bestie") && contains_str(line, "+") && contains_str(line, "\"")) {
        sus issue LintIssue = LintIssue{
            rule_id: "string-concat-loop",
            severity: "info",
            message: "String concatenation in loop may impact performance", 
            line: line_num,
            suggestion: "Use string builders or array joining",
            category: "performance"
        }
        push(issues, issue)
    }
    
    // Nested loops
    sus loop_count drip = 0
    sus pos drip = 0
    bestie (pos < len_str(line)) {
        sus found drip = index_of_from(line, "bestie", pos)
        ready (found == -1) {
            break
        }
        loop_count = loop_count + 1
        pos = found + 6
    }
    
    ready (loop_count > 2) {
        sus issue LintIssue = LintIssue{
            rule_id: "nested-loops",
            severity: "warning",
            message: "Deep nesting detected - consider optimization",
            line: line_num, 
            suggestion: "Review algorithm complexity",
            category: "performance"
        }
        push(issues, issue)
    }
}

// Format lint results
slay format_results(issues []LintIssue) tea {
    ready (len(issues) == 0) {
        damn "✅ No lint issues found! Your CURSED code is fire! 🔥\n"
    }
    
    sus output tea = "🔍 CURSED Linter Results\n"
    output = concat_str(output, "========================\n\n")
    
    // Group by severity
    sus errors drip = 0
    sus warnings drip = 0  
    sus infos drip = 0
    sus hints drip = 0
    
    sus i drip = 0
    bestie (i < len(issues)) {
        sus issue LintIssue = issues[i]
        
        ready (issue.severity == "error") { errors = errors + 1 }
        ready (issue.severity == "warning") { warnings = warnings + 1 }
        ready (issue.severity == "info") { infos = infos + 1 }
        ready (issue.severity == "hint") { hints = hints + 1 }
        
        i = i + 1
    }
    
    // Summary
    output = concat_str(output, "📊 Summary: ")
    ready (errors > 0) {
        output = concat_str(output, int_to_str(errors) + " errors, ")
    }
    ready (warnings > 0) {
        output = concat_str(output, int_to_str(warnings) + " warnings, ")
    }
    ready (infos > 0) {
        output = concat_str(output, int_to_str(infos) + " info, ")
    }
    ready (hints > 0) {
        output = concat_str(output, int_to_str(hints) + " hints")
    }
    output = concat_str(output, "\n\n")
    
    // List issues
    i = 0
    bestie (i < len(issues)) {
        sus issue LintIssue = issues[i]
        sus icon tea = get_severity_icon(issue.severity)
        
        output = concat_str(output, icon + " ")
        output = concat_str(output, "[" + issue.category + "] ")
        output = concat_str(output, issue.message + "\n")
        output = concat_str(output, "   Line " + int_to_str(issue.line))
        output = concat_str(output, " (" + issue.rule_id + ")\n")
        output = concat_str(output, "   💡 " + issue.suggestion + "\n\n")
        
        i = i + 1
    }
    
    damn output
}

// Get severity icon
slay get_severity_icon(severity tea) tea {
    ready (severity == "error") { damn "🚨" }
    ready (severity == "warning") { damn "⚠️" }
    ready (severity == "info") { damn "ℹ️" }
    damn "💡"
}

// Helper functions
slay contains_camel_case(line tea) lit {
    sus has_lower lit = cringe
    sus has_upper lit = cringe
    
    sus i drip = 0
    bestie (i < len_str(line)) {
        sus char tea = char_at(line, i)
        ready (char >= "a" && char <= "z") { has_lower = based }
        ready (char >= "A" && char <= "Z") { has_upper = based }
        i = i + 1
    }
    
    damn has_lower && has_upper
}

// Public API functions
slay lint_production(source tea) tea {
    sus config LinterConfig = production_config()
    sus issues []LintIssue = lint_code(source, config)
    damn format_results(issues)
}

slay lint_development(source tea) tea {
    sus config LinterConfig = dev_config()
    sus issues []LintIssue = lint_code(source, config)
    damn format_results(issues)
}

slay lint_with_config(source tea, config LinterConfig) tea {
    sus issues []LintIssue = lint_code(source, config)
    damn format_results(issues)
}

// Main function for testing
slay main() {
    vibez.spill("🔥 CURSED Production Linter - Working Edition");
    vibez.spill("============================================");
    
    // Test comprehensive example
    sus test_code tea = "sus myVariable drip = 42";
    
    vibez.spill("📝 Testing code:");
    vibez.spill(test_code);
    vibez.spill("");
    
    vibez.spill("🔍 Production Mode Results:");
    sus prod_results tea = lint_production(test_code);
    vibez.spill(prod_results);
    
    vibez.spill("✅ CURSED Linter is production-ready! 🚀");
}
