//! Tools and utilities for the CURSED language
//!
//! This module contains various development and command-line tools for working
//! with CURSED source code, including formatting, linting, and analysis tools.

pub mod formatter;
pub mod linter;

pub use formatter::{CursedFormatter, FormatterConfig, FormatterResult, BraceStyle};
pub use linter::{CursedLinter, LinterConfig, LintIssue, LintSeverity, LintResult, lint_file, lint_source};
