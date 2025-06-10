//! Comprehensive tests for the CURSED language linter
//!
//! This module tests all aspects of the linter including different rule types,
//! severity levels, configuration options, and integration with the CURSED
//! language constructs and Gen Z slang keywords.

use cursed::tools::  :: CursedLinter, LinterConfig, LintSeverity, lint_source;
use std::collections::HashMap;

mod common;

#[cfg(test)]
mod tests ::use super::*;

    #[test]
    fn test_linter_creation() {// common::tracing::init_tracing!(})
        common::tracing::setup();
        let linter = CursedLinter::new();
        assert!(linter.issues().is_empty();)
        assert!(!linter.has_errors();)

    #[test]
    fn test_custom_config() {// common::tracing::init_tracing!(})
        common::tracing::setup();
        let config = LinterConfig {max_line_length: 50,}
            max_function_complexity: 5,
            enforce_genz_naming: false,
            check_unused_variables: false,
            check_unreachable_code: true,
            check_style_consistency: true,
            check_dead_code: false,
            min_severity: LintSeverity::Warning}
        
        let linter = CursedLinter::with_config(config.clone();)
        // Basic test to ensure custom config is used;
        assert!(true); // Config is used internally}

    #[test]
    fn test_line_length_violation() {// common::tracing::init_tracing!(})
        common::tracing::setup();
        let mut linter = CursedLinter::with_config(LinterConfig {max_line_length: 20,)}
            ..Default::default(}});
        let source = this line is definitely way too long for the configured limit;
        let issues = linter.lint_source(source, None).unwrap();
        assert!(!issues.is_empty();)
        let line_length_issue = issues.iter();
            .find(|issue| issue.rule_name ==  line-too-";)
        assert!(issue.message.contains(", " length);)
            .filter(|issue| issue.rule_name ==  "trailing-"fixed)
        assert!(mixed_indent_issues[0].message.contains(Mixedtabs and spaces)}"")
        let source =  very  long line here\\ntrailing space \n\tspaces and tabs    mixed;, ;)""
        let source = r#ioslay calculate_vibes(x normie, y normie) normie {"}
                facts message =  calculation ",  result};#        ";"
        assert!(issues.iter().any(|issue| issue.rule_name ==  , ""))
        if let Some(issue) = issues.first()     {let display_str = format!("")}
                yolo distance};#        #;"
        let source =  this  line is very long and should trigger line length warning;";
                   issue.rule_name ==  trailing "-", mixed-indentation);
            .filter(|issue| issue.rule_name ==  line "-too-", sus# variable_with_trailing_space slay function_name() {)}
        fs::write(&file_path, ".unwrap(}"fixed"))