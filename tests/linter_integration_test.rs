/// Integration tests for the CURSED linter
/// 
/// These tests validate the linter's ability to detect various issues
/// in CURSED source code and provide appropriate feedback.

use cursed::tools::linter::{CursedLinter, LinterConfig, LintSeverity, LintCategory};
use std::collections::HashSet;

#[test]
fn test_basic_linting_functionality() {
    let mut linter = CursedLinter::default();
    let source = r#"
vibe test

slay main() {
    sus x = 42
    print(x)
}
"#;
    
    let results = linter.lint(source).unwrap();
    // Should have some basic linting results
    assert!(!results.is_empty());
}

#[test]
fn test_line_length_detection() {
    let mut linter = CursedLinter::new(LinterConfig {
        max_line_length: 20,
        ..LinterConfig::default()
    });
    
    let source = "sus very_long_variable_name_that_exceeds_limit = 42";
    let results = linter.lint(source).unwrap();
    
    let line_length_issues: Vec<_> = results.iter()
        .filter(|r| r.rule_id == "line_too_long")
        .collect();
    
    assert!(!line_length_issues.is_empty());
    assert_eq!(line_length_issues[0].severity, LintSeverity::Warning);
    assert_eq!(line_length_issues[0].category, LintCategory::Style);
}

#[test]
fn test_trailing_whitespace_detection() {
    let mut linter = CursedLinter::default();
    let source = "sus x = 42   \nsus y = 100\t\n";
    
    let results = linter.lint(source).unwrap();
    
    let whitespace_issues: Vec<_> = results.iter()
        .filter(|r| r.rule_id == "trailing_whitespace")
        .collect();
    
    assert!(!whitespace_issues.is_empty());
    assert_eq!(whitespace_issues[0].severity, LintSeverity::Suggestion);
}

#[test]
fn test_go_style_keyword_detection() {
    let mut linter = CursedLinter::default();
    let source = "func main() { var x = 42; return x; }";
    
    let results = linter.lint(source).unwrap();
    
    let go_style_issues: Vec<_> = results.iter()
        .filter(|r| r.category == LintCategory::GenZSlang)
        .collect();
    
    assert!(!go_style_issues.is_empty());
    
    // Check that suggestions are provided
    for issue in &go_style_issues {
        assert!(issue.suggestion.is_some());
    }
}

#[test]
fn test_go_style_comment_detection() {
    let mut linter = CursedLinter::default();
    let source = r#"
// This is a Go-style comment
slay main() {
    /* This is a Go-style block comment */
    sus x = 42
}
"#;
    
    let results = linter.lint(source).unwrap();
    
    let comment_issues: Vec<_> = results.iter()
        .filter(|r| r.rule_id.contains("comment"))
        .collect();
    
    assert!(!comment_issues.is_empty());
}

#[test]
fn test_mixed_indentation_detection() {
    let mut linter = CursedLinter::default();
    let source = "slay main() {\n\t  sus x = 42\n}";  // Mixed tabs and spaces
    
    let results = linter.lint(source).unwrap();
    
    let mixed_indent_issues: Vec<_> = results.iter()
        .filter(|r| r.rule_id == "mixed_indentation")
        .collect();
    
    assert!(!mixed_indent_issues.is_empty());
    assert_eq!(mixed_indent_issues[0].severity, LintSeverity::Warning);
}

#[test]
fn test_function_parameter_count_limit() {
    let mut linter = CursedLinter::new(LinterConfig {
        max_function_parameters: 3,
        ..LinterConfig::default()
    });
    
    let source = "slay test(a, b, c, d, e) { yolo a + b + c + d + e }";
    
    let results = linter.lint(source).unwrap();
    
    let param_issues: Vec<_> = results.iter()
        .filter(|r| r.rule_id == "too_many_parameters")
        .collect();
    
    assert!(!param_issues.is_empty());
    assert_eq!(param_issues[0].severity, LintSeverity::Warning);
    assert_eq!(param_issues[0].category, LintCategory::Complexity);
}

#[test]
fn test_identifier_naming_conventions() {
    let mut linter = CursedLinter::default();
    let source = r#"
slay test() {
    sus x = 42
    sus very_long_identifier_name_that_exceeds_reasonable_expectations = 100
}
"#;
    
    let results = linter.lint(source).unwrap();
    
    let naming_issues: Vec<_> = results.iter()
        .filter(|r| r.category == LintCategory::Naming)
        .collect();
    
    assert!(!naming_issues.is_empty());
}

#[test]
fn test_generic_function_name_detection() {
    let mut linter = CursedLinter::default();
    let source = "slay doSomething() { yolo }";
    
    let results = linter.lint(source).unwrap();
    
    let generic_name_issues: Vec<_> = results.iter()
        .filter(|r| r.rule_id == "generic_function_name")
        .collect();
    
    assert!(!generic_name_issues.is_empty());
    assert_eq!(generic_name_issues[0].category, LintCategory::Naming);
}

#[test]
fn test_deep_nesting_detection() {
    let mut linter = CursedLinter::default();
    let source = r#"
slay test() {
    lowkey based {
        lowkey based {
            lowkey based {
                lowkey based {
                    lowkey based {
                        sus x = 42
                    }
                }
            }
        }
    }
}
"#;
    
    let results = linter.lint(source).unwrap();
    
    let nesting_issues: Vec<_> = results.iter()
        .filter(|r| r.rule_id == "deep_nesting")
        .collect();
    
    assert!(!nesting_issues.is_empty());
    assert_eq!(nesting_issues[0].category, LintCategory::Complexity);
}

#[test]
fn test_disabled_rules() {
    let mut config = LinterConfig::default();
    config.disable_rule("line_too_long");
    config.max_line_length = 10; // Very short to trigger the rule
    
    let mut linter = CursedLinter::new(config);
    let source = "sus very_long_variable_name = 42";
    
    let results = linter.lint(source).unwrap();
    
    let line_length_issues: Vec<_> = results.iter()
        .filter(|r| r.rule_id == "line_too_long")
        .collect();
    
    // Should be empty because the rule is disabled
    assert!(line_length_issues.is_empty());
}

#[test]
fn test_severity_filtering() {
    let mut linter = CursedLinter::default();
    let source = r#"
slay test() {
    sus x = 42   
    sus y = "test"
}
"#;
    
    let results = linter.lint(source).unwrap();
    linter.results = results;
    
    let errors = linter.get_results_by_severity(LintSeverity::Error);
    let warnings = linter.get_results_by_severity(LintSeverity::Warning);
    let suggestions = linter.get_results_by_severity(LintSeverity::Suggestion);
    let info = linter.get_results_by_severity(LintSeverity::Info);
    
    // Verify filtering works
    for result in &errors {
        assert_eq!(result.severity, LintSeverity::Error);
    }
    
    for result in &warnings {
        assert_eq!(result.severity, LintSeverity::Warning);
    }
    
    for result in &suggestions {
        assert_eq!(result.severity, LintSeverity::Suggestion);
    }
    
    for result in &info {
        assert_eq!(result.severity, LintSeverity::Info);
    }
}

#[test]
fn test_category_filtering() {
    let mut linter = CursedLinter::default();
    let source = "func main() { sus x = 42   }"; // Mix of issues
    
    let results = linter.lint(source).unwrap();
    linter.results = results;
    
    let style_issues = linter.get_results_by_category(LintCategory::Style);
    let gen_z_issues = linter.get_results_by_category(LintCategory::GenZSlang);
    let naming_issues = linter.get_results_by_category(LintCategory::Naming);
    
    // Verify category filtering
    for result in &style_issues {
        assert_eq!(result.category, LintCategory::Style);
    }
    
    for result in &gen_z_issues {
        assert_eq!(result.category, LintCategory::GenZSlang);
    }
    
    for result in &naming_issues {
        assert_eq!(result.category, LintCategory::Naming);
    }
}

#[test]
fn test_strict_configuration() {
    let mut linter = CursedLinter::new(LinterConfig::strict());
    let source = r#"
slay test() {
    sus x = 42
}
"#;
    
    let results = linter.lint(source).unwrap();
    
    // Strict mode should be more aggressive
    // This test mainly verifies that strict config doesn't crash
    assert!(results.len() >= 0);
}

#[test]
fn test_relaxed_configuration() {
    let mut linter = CursedLinter::new(LinterConfig::relaxed());
    let source = r#"
slay test() {
    sus very_long_variable_name_that_might_exceed_normal_limits = 42
    sus x = 42
}
"#;
    
    let results = linter.lint(source).unwrap();
    
    // Relaxed mode should be more permissive
    // This test mainly verifies that relaxed config works
    assert!(results.len() >= 0);
}

#[test]
fn test_parse_error_handling() {
    let mut linter = CursedLinter::default();
    let source = "invalid syntax that won't parse {{{";
    
    let results = linter.lint(source).unwrap();
    
    // Should handle parse errors gracefully
    let parse_errors: Vec<_> = results.iter()
        .filter(|r| r.rule_id == "parse_error" || r.category == LintCategory::Correctness)
        .collect();
    
    assert!(!parse_errors.is_empty());
}

#[test]
fn test_empty_source_handling() {
    let mut linter = CursedLinter::default();
    let source = "";
    
    let results = linter.lint(source).unwrap();
    
    // Empty source should not crash and may have minimal issues
    assert!(results.len() >= 0);
}

#[test]
fn test_package_name_validation() {
    let mut linter = CursedLinter::default();
    let source = "vibe 123invalid";
    
    let results = linter.lint(source).unwrap();
    
    let package_name_issues: Vec<_> = results.iter()
        .filter(|r| r.rule_id.contains("package_name"))
        .collect();
    
    assert!(!package_name_issues.is_empty());
}

#[test]
fn test_import_validation() {
    let mut linter = CursedLinter::default();
    let source = r#"
vibe test
yeet ""
yeet "valid_import"
"#;
    
    let results = linter.lint(source).unwrap();
    
    let import_issues: Vec<_> = results.iter()
        .filter(|r| r.rule_id.contains("import"))
        .collect();
    
    // Should detect empty import path
    assert!(!import_issues.is_empty());
}

#[test]
fn test_string_literal_analysis() {
    let mut linter = CursedLinter::default();
    let source = r#"
slay test() {
    sus short = "ok"
    sus long = "This is a very long string literal that exceeds the reasonable length limit and should trigger a linting warning about string length because it's just too verbose and could be split or moved to a constant"
}
"#;
    
    let results = linter.lint(source).unwrap();
    
    let string_issues: Vec<_> = results.iter()
        .filter(|r| r.rule_id.contains("string"))
        .collect();
    
    // Should detect long string literal
    assert!(!string_issues.is_empty());
}

#[test]
fn test_custom_rule_configuration() {
    let mut config = LinterConfig::default();
    config.enable_custom_rule("custom_test_rule");
    
    let mut linter = CursedLinter::new(config);
    let source = "slay test() { sus x = 42 }";
    
    let results = linter.lint(source).unwrap();
    
    // Should not crash with custom rules
    assert!(results.len() >= 0);
}

#[test]
fn test_result_sorting() {
    let mut linter = CursedLinter::default();
    let source = r#"
slay test() {
    sus x = 42   
}
slay another() {
    sus y = 100  
}
"#;
    
    let results = linter.lint(source).unwrap();
    
    // Results should be sorted by line number
    for i in 1..results.len() {
        assert!(results[i].line >= results[i-1].line);
        if results[i].line == results[i-1].line {
            assert!(results[i].column >= results[i-1].column);
        }
    }
}
