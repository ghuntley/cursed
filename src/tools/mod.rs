/// CURSED Language Tools
/// 
/// This module contains various development tools for the CURSED programming language,
/// including linters, formatters, and other analysis utilities.

pub mod linter;
pub mod formatter;

// Re-export main types for convenience
pub use linter::{
    CursedLinter, LinterConfig, LintResult, LintSeverity, LintCategory
};
pub use formatter::{
    CursedFormatter, FormatterConfig, FormatterResult, BraceStyle, OperatorSpacing, CommaSpacing
};
