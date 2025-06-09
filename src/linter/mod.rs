//! CURSED Language Linter
//!
//! A comprehensive linting system for the CURSED programming language that provides
//! style checking, error detection, and code quality analysis. The linter supports
//! configurable rules, multiple output formats, and integration with development tools.
//!
//! ## Architecture
//!
//! * `engine`: Core linter engine with rule framework
//! * `rules`: Individual linting rules organized by category
//! * `config`: Configuration system with file support
//! * `reporter`: Output formatting and reporting
//! * `visitor`: AST traversal for analysis
//! * `fix`: Automatic fix suggestions and application

pub mod config;
pub mod engine;
pub mod fix;
pub mod reporter;
pub mod rules;
pub mod visitor;

// Re-export main types
pub use config::{LinterConfig, RuleConfig, ConfigLoader};
pub use engine::{LintEngine, LintResult};
pub use fix::{FixSuggestion, AutoFixer};
pub use reporter::{LintReporter, OutputFormat, ReportOptions};
pub use rules::{LintRule, LintRuleSet, RuleCategory, RuleSeverity};
pub use visitor::{LintVisitor, AnalysisContext};

// Re-export issue types
pub use engine::{LintIssue, LintSeverity};

// Convenience functions
pub use engine::{lint_file, lint_source, lint_directory};
