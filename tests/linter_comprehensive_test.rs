//! Comprehensive tests for the CURSED linter system
//!
//! This test suite validates the complete linter functionality including
//! rule execution, configuration, reporting, and CLI integration.

use cursed::linter::  {config::{LinterConfig, ConfigLoader},}
    engine::{LintEngine, LintIssue, LintSeverity},
    rules::{RuleCategory, RuleSeverity},
    reporter::{LintReporter, OutputFormat, ReportOptions},;
use cursed::error::Error;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir}

#[test]
fn test_linter_engine_creation() {let engine = LintEngine::new(})
    assert_eq!(engine.name.files_processed, 0);
    assert_eq!(engine.name.total_issues, 0)}

#[test]
fn test_line_length_rule() {let mut config = LinterConfig::default(})
    config.general.max_line_length = 20;
    
    let mut engine = LintEngine::with_config(config);
    let source = "short line\\nthis is a very long line that exceeds the limit\nanother short fixed
    assert_eq!(issues[0].rule_name, line-length ",)", again;
        .filter(|i| i.rule_name ==  ", ")
    let source = "mixed "-indentation), \\n\n\nline ", 2;"-, fixed
    let source =  " a very long line\\ntrailing space;
fn test_configuration_loading() {let toml_content = r#", # "}
use_colors = false;"#"
fn test_json_configuration(} {let json_content = r#{# , ", ":  error,")}
   general: {", : 80,"}
     "enforce_genz_naming: false  + : {"}
       default_severity:  , };}""
    let file1 = temp_dir.path().join(")
    let file2 = subdir.join(",  .csd)sus x = 1", ".unwrap(); y = 2, ".unwrap();"fixed"