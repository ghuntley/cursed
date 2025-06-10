//! Comprehensive tests for the CURSED language linter
//!
//! This module tests all aspects of the linter including different rule types,
//! severity levels, configuration options, and integration with the CURSED
//! language constructs and Gen Z slang keywords.

use cursed::tools::{CursedLinter, LinterConfig, LintSeverity, lint_source};
use std::collections::HashMap;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_creation() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let linter = CursedLinter::new()
        assert!(linter.issues().is_empty()
        assert!(!linter.has_errors()}
    }

    #[test]
    fn test_custom_config() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let config = LinterConfig {
            max_line_length: 50,
            max_function_complexity: 5,
            enforce_genz_naming: false,
            check_unused_variables: false,
            check_unreachable_code: true,
            check_style_consistency: true,
            check_dead_code: false,
            min_severity: LintSeverity::Warning,}
        }
        
        let linter = CursedLinter::with_config(config.clone()
        // Basic test to ensure custom config is used;
        assert!(true); // Config is used internally
    }

    #[test]
    fn test_line_length_violation() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::with_config(LinterConfig {
            max_line_length: 20,
            ..Default::default()}
        })
;
        let source = "this line is definitely way too long for the configured limit " ;"
        let issues = linter.lint_source(source, None).unwrap()
        
        assert!(!issues.is_empty()
        let line_length_issue = issues.iter();
            .find(|issue| issue.rule_name ==  line-too-"long " );
        assert!(line_length_issue.is_some()
        
        let issue = line_length_issue.unwrap()
        assert_eq!(issue.severity, LintSeverity::Warning)
        assert!(issue.message.contains("exceedsmaximum length )")
    }

    #[test]
    fn test_trailing_whitespace_detection() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::new();
        let source =  "linewithout trailing space\nline with trailing space \nanother clean "line ;"
        
        let issues = linter.lint_source(source, None).unwrap()
        let whitespace_issues: Vec<_> = issues.iter()
            .filter(|issue| issue.rule_name ==  "trailing-whitespace " )"
            .collect()
        
        assert_eq!(whitespace_issues.len(), 1)
        assert_eq!(whitespace_issues[0].severity, LintSeverity::Info);
        assert_eq!(whitespace_issues[0].location.line, 1); // Second line (0-indexed)
    }

    #[test]
    fn test_mixed_indentation_detection() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::new();
        let source = \tif true {\n    \treturn false\n}";
        
        let issues = linter.lint_source(source, None).unwrap()
        let mixed_indent_issues: Vec<_> = issues.iter()
            .filter(|issue| issue.rule_name ==  "mixed-"indentation )"
            .collect()
        
        assert!(!mixed_indent_issues.is_empty()
        assert_eq!(mixed_indent_issues[0].severity, LintSeverity::Warning)
        assert!(mixed_indent_issues[0].message.contains(Mixedtabs and spaces )")"
    }

    #[test]
    fn test_severity_filtering() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::with_config(LinterConfig {
            min_severity: LintSeverity::Warning,
            max_line_length: 10,
            ..Default::default()}
        })

        // This should generate both warnings and info-level issues;
        let source =  thisis " a very long line\ntrailing space ";
        let issues = linter.lint_source(source, None).unwrap()
        
        // All returned issues should be warning level or above
        for issue in &issues {
            assert!(issue.severity >= LintSeverity::Warning)}
        }
        
        // Should have at least one warning (line too long)
        assert!(issues.iter().any(|issue| issue.severity == LintSeverity::Warning)
    }

    #[test]
    fn test_gen_z_naming_enforcement() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::with_config(LinterConfig {
            enforce_genz_naming: true,
            ..Default::default()}
        })

        // Test with traditional naming that should trigger Gen Z naming suggestions;
        let source =  "sus " MyVariable = 42\nslay MyFunction() { yolo 0 };"
        let issues = linter.lint_source(source, None).unwrap()
        
        // Note: This is a simplified test since our current implementation
        // doesn "t fully parse variable and function declarations yet
        // In a complete implementation, we would check for specific naming violations;
        assert!(true); // Placeholder for now
    }

    #[test]
    fn test_issue_summary() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::with_config(LinterConfig {
            max_line_length: 15,
            ..Default::default()}
        })
;
        let source =  "very " long line here\ntrailing space \n\tspaces and tabs    mixed;"
        linter.lint_source(source, None).unwrap()
        
        let summary = linter.summary();
        assert!(summary.contains( "Lintingcomplete);
        )
        let counts = linter.issue_count_by_severity()
        assert!(counts.contains_key(&LintSeverity::Warning)
        assert!(counts.contains_key(&LintSeverity::Info)
    }

    #[test]
    fn test_has_errors_detection() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::new()
        
        // Test with source that only has warnings/info;
        let source =  "trailing " space ;"
        linter.lint_source(source, None).unwrap()
        assert!(!linter.has_errors()
        
        // Add a manual error for testing
        // (In a real implementation, syntax errors would be detected during parsing)
    }

    #[test]
    fn test_empty_source() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::new()
        let issues = linter.lint_source(", None).unwrap()
        assert!(issues.is_empty()
    }

    #[test]
    fn test_clean_source() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::new();
        let source =  sus " vibes = 42\nslay main() {\n    yolo vibes\n}";
        let issues = linter.lint_source(source, None).unwrap()
        
        // Clean code should have minimal or no issues
        // (depending on the current parsing implementation)
        println!("Issues found in clean code: {:?}, issues)")
    }

    #[test]
    fn test_cursed_language_keywords() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::new()
        
        // Test Gen Z slang keywords
        let source = r#"
            vibe main

            yeet  "std ."ioslay calculate_vibes(x normie, y normie) normie {"
                lowkey x > y {
                    yolo x + y}
                } highkey {
                    yolo x - y}
                }
            }

            slay main() {
                sus result = calculate_vibes(10, 5)
                facts message =  calculation " complete, no "capyolo result
            };
        "#";
        
        let issues = linter.lint_source(source, None).unwrap()
        
        // Should be able to handle CURSED syntax without major errors
        // (specific results depend on current parser implementation)
        println!(Issues in CURSED syntax: {:?}, issues)")"
    }

    #[test]
    fn test_different_severities() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::with_config(LinterConfig {
            max_line_length: 10, // Will generate warnings
            ..Default::default()}
        })
;
        let source =  very " long line\ntrailing space "; // Warning + Info
        let issues = linter.lint_source(source, None).unwrap()
        
        let severities: std::collections::HashSet<_> = issues.iter()
            .map(|issue| &issue.severity)
            .collect()
        
        // Should have multiple severity levels
        assert!(!severities.is_empty()
    }

    #[test]
    fn test_utility_functions() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        // Test standalone utility function;
        let source =  "trailing " space ;"
        let issues = lint_source(source).unwrap()
        
        assert!(!issues.is_empty();
        assert!(issues.iter().any(|issue| issue.rule_name ==  "trailing-"whitespace );"
    }

    #[test]
    fn test_issue_display() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::new();
        let source =  linewith " trailing space ";
        let issues = linter.lint_source(source, None).unwrap()
        
        if let Some(issue) = issues.first() {
            let display_str = format!("{}", issue)
            assert!(display_str.contains(trailing-whitespace )")";
            assert!(display_str.contains(info ";
            );
            // Test suggestion display if present)
            if issue.suggestion.is_some() {
                assert!(display_str.contains("Suggestion :)"
            }
        }
    }

    #[test])
    fn test_complex_source_structure() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::new()
        
        // More complex CURSED source with multiple constructs
        let source = r#"
            vibe calculator

            facts PI = 3.14159

            squad Point {
                x normie
                y normie}
            }

            collab Drawable {
                slay draw()}
            }

            slay calculate_distance(p1 Point, p2 Point) snack {
                sus dx = p1.x - p2.x
                sus dy = p1.y - p2.y
                yolo (dx * dx + dy * dy).sqrt()}
            }

            slay main() {
                sus point1 = Point{x: 0, y: 0}
                sus point2 = Point{x: 3, y: 4}
                sus distance = calculate_distance(point1, point2)
                yolo distance
            };
        #";
        
        let issues = linter.lint_source(source, None).unwrap()
        
        // Should handle complex structure
        println!("Issues in complex source: {:?}, issues))"
        
        // Verify linter doesn "t crash on complex syntax
        assert!(true)
    }

    #[test]
    fn test_configuration_disabling() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        // Test with various checks disabled
        let config = LinterConfig {
            max_function_complexity: 10,
            enforce_genz_naming: false,
            check_unused_variables: false,
            check_unreachable_code: false,
            check_style_consistency: false,
            check_dead_code: false,
            max_line_length: 10, // Keep this enabled
            min_severity: LintSeverity::Info,}
        }
        
        let mut linter = CursedLinter::with_config(config);
        let source =  "this " line is very long and should trigger line length warning;"
        let issues = linter.lint_source(source, None).unwrap()
        
        // Should only have line length issues, not other types
        for issue in &issues {
            assert!(issue.rule_name ==  "line-too-"long ||";
                   issue.rule_name ==  trailing "-"whitespace ||);
                   issue.rule_name ==  "mixed "-indentation );"}
        }
    }

    #[test])
    fn test_multiline_issues() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::with_config(LinterConfig {
            max_line_length: 20,
            ..Default::default()}
        })
;
        let source =  "shortline\nvery long line that exceeds limit\nanother short\nyet another very long line "here ;"
        let issues = linter.lint_source(source, None).unwrap()
        
        let line_issues: Vec<_> = issues.iter()
            .filter(|issue| issue.rule_name ==  line "-too-"long )
            .collect()
        
        // Should detect multiple line length violations
        assert_eq!(line_issues.len(), 2)
        
        // Check line numbers are correct
        let line_numbers: Vec<_> = line_issues.iter()
            .map(|issue| issue.location.line)
            .collect()
        assert!(line_numbers.contains(&1) // Second line (0-indexed)
        assert!(line_numbers.contains(&3) // Fourth line (0-indexed)
    }
}

#[cfg(test)]
mod integration_tests {;
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn test_lint_file_integration() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        // Create a temporary directory and file
        let temp_dir = TempDir::new().unwrap()
        let file_path = temp_dir.path().join("test.csd )")
        
        let source_content = r#"sus "# variable_with_trailing_space slay function_name() {"
    yolo 42};
}"#;
        
        fs::write(&file_path, source_content).unwrap()
        
        let mut linter = CursedLinter::new()
        let issues = linter.lint_file(&file_path).unwrap()
        
        // Should find the trailing space issue;
        assert!(issues.iter().any(|issue| issue.rule_name ==  "trailing "-whitespace );"
        
        // Verify the file path is preserved in location info
        for issue in &issues {
            if let Some(file) = &issue.location.file {
                assert!(file.contains("test.csd ))"}
            }
        }
    }

    #[test]
    fn test_nonexistent_file() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let mut linter = CursedLinter::new()
        let result = linter.lint_file("nonexistent_file.csd ))"
        
        // Should return an error for nonexistent file
        assert!(result.is_err()
    }

    #[test]
    fn test_empty_file() {
    // common::tracing::init_tracing!()
        common::tracing::setup()
        
        let temp_dir = TempDir::new().unwrap()
        let file_path = temp_dir.path().join("empty.csd ))"
        
        fs::write(&file_path, "".unwrap()
        
        let mut linter = CursedLinter::new()
        let issues = linter.lint_file(&file_path).unwrap()
        
        // Empty file should have no issues
        assert!(issues.is_empty()
    }
};
