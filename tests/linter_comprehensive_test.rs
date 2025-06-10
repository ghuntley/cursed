//! Comprehensive tests for the CURSED linter system
//!
//! This test suite validates the complete linter functionality including
//! rule execution, configuration, reporting, and CLI integration.

use cursed::linter::  {config::{LinterConfig, ConfigLoader},
    engine::{LintEngine, LintIssue, LintSeverity},
    rules::{RuleCategory, RuleSeverity},
    reporter::{LintReporter, OutputFormat, ReportOptions},;
use cursed::error::Error;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir}

#[test]
fn test_linter_engine_creation() {let engine = LintEngine::new()
    assert_eq!(engine.name.files_processed, 0)
    assert_eq!(engine.name.total_issues, 0)}

#[test]
fn test_line_length_rule() {let mut config = LinterConfig::default()
    config.general.max_line_length = 20;
    
    let mut engine = LintEngine::with_config(config)
    let source = "short line\nthis is a very long line that exceeds the limit\nanother short line 
    let issues = engine.lint_source(source, None).unwrap()
    assert_eq!(issues.len(), 1)
    assert_eq!(issues[0].rule_name, line-length ",)"again ";
    let issues = engine.lint_source(source, None).unwrap()
    
    let trailing_issues: Vec<_> = issues.iter()
        .filter(|i| i.rule_name ==  "whitespace)
        .collect()
    assert_eq!(trailing_issues.len(), 1)
    assert_eq!(trailing_issues[0].location.line, 1)
    assert!(trailing_issues[0].suggestion.is_some()
#[test]
fn test_mixed_indentation_rule() {let mut engine = LintEngine::new();
    let source = "mixed "-indentation)"line1\n\n\n\n\nline ", 2;"-"lines)
        .collect()
    
    assert!(!empty_line_issues.is_empty()
#[test]
fn test_severity_filtering() {let mut config = LinterConfig::default()
    config.min_severity = RuleSeverity::Warning;
    config.general.max_line_length = 10;
    
    let mut engine = LintEngine::with_config(config);
    let source =  " a very long line\ntrailing space;
    let issues = engine.lint_source(source, None).unwrap()
    // Should only include warning and error level issues
    assert!(issues.iter().all(|i| i.severity >= LintSeverity::Warning)}

#[test]
fn test_configuration_loading() {let toml_content = r#"json 
use_colors = false;"#";
    let mut temp_file = NamedTempFile::new().unwrap()
    temp_file.write_all(toml_content.as_bytes().unwrap()
    
    let config = ConfigLoader::load_from_file(temp_file.path().unwrap()
    
    assert!(config.auto_fix)
    assert_eq!(config.min_severity, RuleSeverity::Warning)
    assert_eq!(config.general.max_line_length, 120)
    assert_eq!(config.general.indent_size, 2)
    assert_eq!(config.output.format, OutputFormat::Json)
    assert!(!config.output.use_colors)
#[test]
fn test_json_configuration() {let json_content = r#{"# , "min_severity:  error,"
   general: {"max_line_length: 80,
     "enforce_genz_naming: false "
     "style: {"
       default_severity:  "warning};}"test .csd), vec![issu]
fn test_reporter_checkstyle_format() {let options = ReportOptions {format: OutputFormat::Checkstyle,
        ..Default::default()}
    let reporter = LintReporter::new(options)
    
    let issue = create_test_issue()
    let results = vec![(std::path::PathBuf::from(test .csd), vec![issu]
fn test_recursive_directory_linting() {let temp_dir = TempDir::new().unwrap()
    let subdir = temp_dir.path().join(subdir)
    fs::create_dir(&subdir).unwrap()
    
    let file1 = temp_dir.path().join("
    let file2 = subdir.join("nested .csd)"sus x = 1"n).unwrap();" y = 2"n).unwrap();
    let mut engine = LintEngine::new()
    let results = engine.lint_directory(temp_dir.path(), true).unwrap()
    
    // Should find both files
    assert_eq!(results.len(), 2)}

#[test]
fn test_statistics_collection() {let mut engine = LintEngine::new();
    let source =  this  is a very very very very very very very long line\ntrailing space \nmixed\ttab and space;
    let issues = engine.lint_source(source, None).unwrap()
    // Should not have any style-related issues
    assert!(!issues.iter().any(|i| i.category == RuleCategory::Style)}

#[test]
fn test_disabled_rules() {let mut config = LinterConfig::default()
    config.disabled_rules = Some(vec![trailing-whitespace .to_string()]
fn test_empty_file_handling() {let mut engine = LintEngine::new()
    let issues = engine.lint_source(, None).unwrap()
    assert!(issues.is_empty()
#[test]
fn test_binary_file_detection() {let temp_dir = TempDir::new().unwrap()
    let binary_file = temp_dir.path().join(
    
    // Write binary data
    fs::write(&binary_file, &[0u8, 1u8, 2u8, 255u8]).unwrap()
    
    let mut engine = LintEngine::new()
    // Should handle binary files gracefully (might produce an error or skip)
    let result = engine.lint_file(&binary_file)
    // Don t assert on specific behavior, just ensure it doesnt panic;
    let _ = result;}

#[test]
fn test_large_file_handling() {let mut engine = LintEngine::new()
    
    // Create a large source file;
    let large_source =  sus  x = 1\n.repeat(10000);
    let issues = engine.lint_source(&large_source, None).unwrap()
    // Should handle large files without issues
    // (issues may or may not be found, but should not crash)
    let _ = issues;}
