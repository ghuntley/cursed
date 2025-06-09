//! Core linting engine for the CURSED programming language
//!
//! This module provides the main `LintEngine` that coordinates rule execution,
//! AST analysis, and issue collection. It serves as the central orchestrator
//! for all linting operations.

use crate::ast::*;
use crate::error::{Error, SourceLocation};
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use crate::linter::{
    config::LinterConfig,
    rules::{LintRule, LintRuleSet, RuleCategory, RuleSeverity},
    visitor::{LintVisitor, AnalysisContext},
    reporter::{LintReporter, OutputFormat, ReportOptions},
    fix::{FixSuggestion, AutoFixer}
};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{debug, error, info, instrument, warn};

/// Severity level for lint issues
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LintSeverity {
    Info,
    Warning,
    Error,
}

impl fmt::Display for LintSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LintSeverity::Info => write!(f, "info"),
            LintSeverity::Warning => write!(f, "warning"),
            LintSeverity::Error => write!(f, "error"),
        }
    }
}

impl From<RuleSeverity> for LintSeverity {
    fn from(severity: RuleSeverity) -> Self {
        match severity {
            RuleSeverity::Info => LintSeverity::Info,
            RuleSeverity::Warning => LintSeverity::Warning,
            RuleSeverity::Error => LintSeverity::Error,
        }
    }
}

/// A lint issue found in the code
#[derive(Debug, Clone)]
pub struct LintIssue {
    pub severity: LintSeverity,
    pub rule_name: String,
    pub category: RuleCategory,
    pub message: String,
    pub location: SourceLocation,
    pub suggestion: Option<FixSuggestion>,
    pub context: HashMap<String, String>,
}

impl LintIssue {
    /// Create a new lint issue
    pub fn new(
        severity: LintSeverity,
        rule_name: String,
        category: RuleCategory,
        message: String,
        location: SourceLocation,
    ) -> Self {
        Self {
            severity,
            rule_name,
            category,
            message,
            location,
            suggestion: None,
            context: HashMap::new(),
        }
    }

    /// Add a fix suggestion to this issue
    pub fn with_suggestion(mut self, suggestion: FixSuggestion) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    /// Add context information to this issue
    pub fn with_context<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }

    /// Get the unique identifier for this issue type
    pub fn issue_id(&self) -> String {
        format!("{}:{}", self.category, self.rule_name)
    }
}

impl fmt::Display for LintIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} [{}] - {}",
            self.severity, self.location, self.rule_name, self.message
        )?;
        if let Some(suggestion) = &self.suggestion {
            write!(f, "\n  Suggestion: {}", suggestion.description)?;
        }
        Ok(())
    }
}

/// Statistics about linting run
#[derive(Debug, Clone, Default)]
pub struct LintStatistics {
    pub files_processed: usize,
    pub total_issues: usize,
    pub issues_by_severity: HashMap<LintSeverity, usize>,
    pub issues_by_category: HashMap<RuleCategory, usize>,
    pub issues_by_rule: HashMap<String, usize>,
    pub auto_fixable_issues: usize,
    pub processing_time_ms: u128,
}

impl LintStatistics {
    /// Add an issue to the statistics
    pub fn add_issue(&mut self, issue: &LintIssue) {
        self.total_issues += 1;
        *self.issues_by_severity.entry(issue.severity.clone()).or_insert(0) += 1;
        *self.issues_by_category.entry(issue.category.clone()).or_insert(0) += 1;
        *self.issues_by_rule.entry(issue.rule_name.clone()).or_insert(0) += 1;
        
        if issue.suggestion.is_some() {
            self.auto_fixable_issues += 1;
        }
    }

    /// Check if there are any error-level issues
    pub fn has_errors(&self) -> bool {
        self.issues_by_severity.get(&LintSeverity::Error).unwrap_or(&0) > &0
    }

    /// Generate a summary string
    pub fn summary(&self) -> String {
        let errors = self.issues_by_severity.get(&LintSeverity::Error).unwrap_or(&0);
        let warnings = self.issues_by_severity.get(&LintSeverity::Warning).unwrap_or(&0);
        let info = self.issues_by_severity.get(&LintSeverity::Info).unwrap_or(&0);
        let fixable = self.auto_fixable_issues;

        format!(
            "Processed {} files in {}ms: {} errors, {} warnings, {} info ({} auto-fixable)",
            self.files_processed,
            self.processing_time_ms,
            errors,
            warnings,
            info,
            fixable
        )
    }
}

/// The main linting engine
pub struct LintEngine {
    config: LinterConfig,
    rule_set: LintRuleSet,
    reporter: LintReporter,
    auto_fixer: AutoFixer,
    statistics: LintStatistics,
}

impl LintEngine {
    /// Create a new lint engine with default configuration
    pub fn new() -> Self {
        Self::with_config(LinterConfig::default())
    }

    /// Create a new lint engine with custom configuration
    pub fn with_config(config: LinterConfig) -> Self {
        let rule_set = LintRuleSet::from_config(&config);
        let report_options = ReportOptions {
            format: config.output.format.clone(),
            show_rule_names: config.output.show_rule_names,
            show_severity: config.output.show_severity,
            show_suggestions: config.output.show_suggestions,
            use_colors: config.output.use_colors,
            max_issues_per_file: config.output.max_issues_per_file,
        };
        let reporter = LintReporter::new(report_options);
        let auto_fixer = AutoFixer::new();
        
        Self {
            config,
            rule_set,
            reporter,
            auto_fixer,
            statistics: LintStatistics::default(),
        }
    }

    /// Lint a single file
    #[instrument(skip(self))]
    pub fn lint_file<P: AsRef<Path> + std::fmt::Debug>(&mut self, file_path: P) -> LintResult<Vec<LintIssue>> {
        let path = file_path.as_ref();
        let start_time = std::time::Instant::now();
        
        debug!("Linting file: {}", path.display());
        
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::IoError(e))?;
        
        let mut issues = self.lint_source_internal(&content, Some(path.to_string_lossy().to_string()))?;
        
        // Apply auto-fixes if enabled
        if self.config.auto_fix {
            issues = self.auto_fixer.apply_fixes(&content, issues)?;
        }
        
        self.statistics.files_processed += 1;
        self.statistics.processing_time_ms += start_time.elapsed().as_millis();
        
        for issue in &issues {
            self.statistics.add_issue(issue);
        }
        
        debug!("Found {} issues in {}", issues.len(), path.display());
        Ok(issues)
    }

    /// Lint source code
    #[instrument(skip(self, source))]
    pub fn lint_source(&mut self, source: &str, file_name: Option<String>) -> LintResult<Vec<LintIssue>> {
        let start_time = std::time::Instant::now();
        let issues = self.lint_source_internal(source, file_name)?;
        
        self.statistics.processing_time_ms += start_time.elapsed().as_millis();
        for issue in &issues {
            self.statistics.add_issue(issue);
        }
        
        Ok(issues)
    }

    /// Internal source linting implementation
    fn lint_source_internal(&mut self, source: &str, file_name: Option<String>) -> LintResult<Vec<LintIssue>> {
        debug!("Starting lint analysis");
        
        let mut issues = Vec::new();
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();

        // Create analysis context
        let mut context = AnalysisContext::new(source_lines.clone(), file_name.clone());

        // Phase 1: Text-based analysis (before parsing)
        self.run_text_rules(source, &file_name, &mut issues)?;

        // Phase 2: Token-based analysis
        let tokens = self.tokenize_source(source)?;
        self.run_token_rules(&tokens, &mut context, &mut issues)?;

        // Phase 3: AST-based analysis
        match self.parse_source(source) {
            Ok(program) => {
                self.run_ast_rules(&program, &mut context, &mut issues)?;
            }
            Err(e) => {
                warn!("Failed to parse source for AST analysis: {:?}", e);
                // Continue with partial analysis
            }
        }

        // Filter issues by minimum severity
        let filtered_issues: Vec<LintIssue> = issues
            .into_iter()
            .filter(|issue| issue.severity >= self.config.min_severity.clone().into())
            .collect();

        info!("Found {} lint issues", filtered_issues.len());
        Ok(filtered_issues)
    }

    /// Run text-based linting rules
    fn run_text_rules(&self, source: &str, file_name: &Option<String>, issues: &mut Vec<LintIssue>) -> LintResult<()> {
        for rule in self.rule_set.text_rules() {
            if self.is_rule_enabled(&rule.name()) {
                let rule_issues = rule.check_text(source, file_name.as_deref())?;
                issues.extend(rule_issues);
            }
        }
        Ok(())
    }

    /// Run token-based linting rules
    fn run_token_rules(&self, tokens: &[Token], context: &mut AnalysisContext, issues: &mut Vec<LintIssue>) -> LintResult<()> {
        for rule in self.rule_set.token_rules() {
            if self.is_rule_enabled(&rule.name()) {
                let rule_issues = rule.check_tokens(tokens, context)?;
                issues.extend(rule_issues);
            }
        }
        Ok(())
    }

    /// Run AST-based linting rules
    fn run_ast_rules(&self, program: &Program, context: &mut AnalysisContext, issues: &mut Vec<LintIssue>) -> LintResult<()> {
        // Create visitor for AST traversal
        let mut visitor = LintVisitor::new(context);
        
        // First pass: collect symbols and declarations
        visitor.visit_program(program)?;
        
        // Run AST rules
        for rule in self.rule_set.ast_rules() {
            if self.is_rule_enabled(&rule.name()) {
                let rule_issues = rule.check_ast(program, &visitor.context())?;
                issues.extend(rule_issues);
            }
        }
        
        Ok(())
    }

    /// Check if a rule is enabled in the configuration
    fn is_rule_enabled(&self, rule_name: &str) -> bool {
        if let Some(disabled_rules) = &self.config.disabled_rules {
            !disabled_rules.contains(&rule_name.to_string())
        } else {
            true
        }
    }

    /// Tokenize source code
    fn tokenize_source(&self, source: &str) -> LintResult<Vec<Token>> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    if matches!(token, Token::Eof) {
                        break;
                    }
                    tokens.push(token);
                }
                Err(e) => {
                    warn!("Tokenization error: {:?}", e);
                    break;
                }
            }
        }
        
        Ok(tokens)
    }

    /// Parse source code into AST
    fn parse_source(&self, source: &str) -> LintResult<Program> {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer).map_err(|e| {
            Error::Parser {
                location: SourceLocation::new(0, 0),
                message: format!("Failed to create parser: {:?}", e),
            }
        })?;
        
        parser.parse_program().map_err(|e| {
            Error::Parser {
                location: SourceLocation::new(0, 0),
                message: format!("Failed to parse: {:?}", e),
            }
        })
    }

    /// Lint multiple files
    #[instrument(skip(self))]
    pub fn lint_files<P: AsRef<Path> + std::fmt::Debug>(&mut self, file_paths: &[P]) -> LintResult<Vec<(PathBuf, Vec<LintIssue>)>> {
        let mut results = Vec::new();
        
        for file_path in file_paths {
            let path = file_path.as_ref().to_path_buf();
            match self.lint_file(&path) {
                Ok(issues) => results.push((path, issues)),
                Err(e) => {
                    error!("Failed to lint {}: {}", path.display(), e);
                    // Continue with other files
                }
            }
        }
        
        Ok(results)
    }

    /// Lint a directory recursively
    #[instrument(skip(self))]
    pub fn lint_directory<P: AsRef<Path> + std::fmt::Debug>(&mut self, dir_path: P, recursive: bool) -> LintResult<Vec<(PathBuf, Vec<LintIssue>)>> {
        let files = self.collect_cursed_files(dir_path.as_ref(), recursive)?;
        self.lint_files(&files)
    }

    /// Collect CURSED files from a directory
    fn collect_cursed_files(&self, dir_path: &Path, recursive: bool) -> LintResult<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        for entry in std::fs::read_dir(dir_path).map_err(Error::IoError)? {
            let entry = entry.map_err(Error::IoError)?;
            let path = entry.path();
            
            if path.is_file() && self.is_cursed_file(&path) {
                files.push(path);
            } else if path.is_dir() && recursive {
                let mut subdir_files = self.collect_cursed_files(&path, recursive)?;
                files.append(&mut subdir_files);
            }
        }
        
        Ok(files)
    }

    /// Check if a file is a CURSED source file
    fn is_cursed_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("csd"))
            .unwrap_or(false)
    }

    /// Get linting statistics
    pub fn statistics(&self) -> &LintStatistics {
        &self.statistics
    }

    /// Reset statistics
    pub fn reset_statistics(&mut self) {
        self.statistics = LintStatistics::default();
    }

    /// Generate a report
    pub fn generate_report(&self, results: &[(PathBuf, Vec<LintIssue>)]) -> LintResult<String> {
        self.reporter.generate_report(results, &self.statistics)
    }

    /// Print issues to stdout
    pub fn print_issues(&self, results: &[(PathBuf, Vec<LintIssue>)]) {
        self.reporter.print_issues(results);
    }
}

impl Default for LintEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Result type for linting operations
pub type LintResult<T> = Result<T, Error>;

/// Convenience function to lint a single file
pub fn lint_file<P: AsRef<Path> + std::fmt::Debug>(file_path: P) -> LintResult<Vec<LintIssue>> {
    let mut engine = LintEngine::new();
    engine.lint_file(file_path)
}

/// Convenience function to lint source code
pub fn lint_source(source: &str) -> LintResult<Vec<LintIssue>> {
    let mut engine = LintEngine::new();
    engine.lint_source(source, None)
}

/// Convenience function to lint a directory
pub fn lint_directory<P: AsRef<Path> + std::fmt::Debug>(dir_path: P, recursive: bool) -> LintResult<Vec<(PathBuf, Vec<LintIssue>)>> {
    let mut engine = LintEngine::new();
    engine.lint_directory(dir_path, recursive)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint_engine_creation() {
        let engine = LintEngine::new();
        assert_eq!(engine.statistics().files_processed, 0);
        assert_eq!(engine.statistics().total_issues, 0);
    }

    #[test]
    fn test_lint_severity_ordering() {
        assert!(LintSeverity::Error > LintSeverity::Warning);
        assert!(LintSeverity::Warning > LintSeverity::Info);
    }

    #[test]
    fn test_lint_issue_creation() {
        let location = SourceLocation::new(1, 5);
        let issue = LintIssue::new(
            LintSeverity::Warning,
            "test-rule".to_string(),
            RuleCategory::Style,
            "Test message".to_string(),
            location,
        );
        
        assert_eq!(issue.severity, LintSeverity::Warning);
        assert_eq!(issue.rule_name, "test-rule");
        assert_eq!(issue.category, RuleCategory::Style);
        assert!(issue.suggestion.is_none());
    }

    #[test]
    fn test_statistics_accumulation() {
        let mut stats = LintStatistics::default();
        
        let issue1 = LintIssue::new(
            LintSeverity::Error,
            "rule1".to_string(),
            RuleCategory::Correctness,
            "Error message".to_string(),
            SourceLocation::new(1, 1),
        );
        
        let issue2 = LintIssue::new(
            LintSeverity::Warning,
            "rule2".to_string(),
            RuleCategory::Style,
            "Warning message".to_string(),
            SourceLocation::new(2, 1),
        );
        
        stats.add_issue(&issue1);
        stats.add_issue(&issue2);
        
        assert_eq!(stats.total_issues, 2);
        assert_eq!(stats.issues_by_severity.get(&LintSeverity::Error), Some(&1));
        assert_eq!(stats.issues_by_severity.get(&LintSeverity::Warning), Some(&1));
        assert!(stats.has_errors());
    }

    #[test]
    fn test_is_cursed_file() {
        let engine = LintEngine::new();
        
        assert!(engine.is_cursed_file(Path::new("test.csd")));
        assert!(engine.is_cursed_file(Path::new("TEST.CSD")));
        assert!(!engine.is_cursed_file(Path::new("test.rs")));
        assert!(!engine.is_cursed_file(Path::new("test")));
    }
}
