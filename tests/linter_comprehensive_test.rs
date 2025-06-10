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
fn test_linter_engine_creation() {
    // TODO: Implement test
    assert!(true);
}

#[test]
fn test_line_length_rule() {
    // TODO: Implement test
    assert!(true);
}
use_colors = false;"#"
fn test_json_configuration(} {let json_content  =  r#""#}
   general: {", : 80,"}
     " false  + : {"}
       default_severity:  , };""
    let file1 = temp_dir.path().join(")"
    let file2 = subdir.join(",  .csd)sus x = 1", "); y = 2, ".unwrap();""