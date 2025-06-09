//! Reporting and output formatting for linting results

use crate::linter::{engine::{LintIssue, LintStatistics}, rules::RuleCategory};
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

/// Output format options
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum OutputFormat {
    #[serde(rename = "human")]
    Human,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "checkstyle")]
    Checkstyle,
    #[serde(rename = "sarif")]
    Sarif,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Human
    }
}

/// Reporter configuration
#[derive(Debug, Clone)]
pub struct ReportOptions {
    pub format: OutputFormat,
    pub show_rule_names: bool,
    pub show_severity: bool,
    pub show_suggestions: bool,
    pub use_colors: bool,
    pub max_issues_per_file: Option<usize>,
}

impl Default for ReportOptions {
    fn default() -> Self {
        Self {
            format: OutputFormat::Human,
            show_rule_names: true,
            show_severity: true,
            show_suggestions: true,
            use_colors: true,
            max_issues_per_file: None,
        }
    }
}

/// Main reporter for linting results
#[derive(Debug)]
pub struct LintReporter {
    options: ReportOptions,
}

impl LintReporter {
    pub fn new(options: ReportOptions) -> Self {
        Self { options }
    }

    /// Generate a complete report
    pub fn generate_report(
        &self,
        results: &[(PathBuf, Vec<LintIssue>)],
        statistics: &LintStatistics,
    ) -> Result<String, Error> {
        match self.options.format {
            OutputFormat::Human => self.generate_human_report(results, statistics),
            OutputFormat::Json => self.generate_json_report(results, statistics),
            OutputFormat::Checkstyle => self.generate_checkstyle_report(results, statistics),
            OutputFormat::Sarif => self.generate_sarif_report(results, statistics),
        }
    }

    /// Print issues to stdout
    pub fn print_issues(&self, results: &[(PathBuf, Vec<LintIssue>)]) {
        for (file_path, issues) in results {
            if !issues.is_empty() {
                self.print_file_issues(file_path, issues);
            }
        }
    }

    fn print_file_issues(&self, file_path: &PathBuf, issues: &[LintIssue]) {
        match self.options.format {
            OutputFormat::Human => self.print_human_issues(file_path, issues),
            OutputFormat::Json => self.print_json_issues(file_path, issues),
            OutputFormat::Checkstyle => self.print_checkstyle_issues(file_path, issues),
            OutputFormat::Sarif => (), // SARIF is typically generated as a complete report
        }
    }

    fn generate_human_report(
        &self,
        results: &[(PathBuf, Vec<LintIssue>)],
        statistics: &LintStatistics,
    ) -> Result<String, Error> {
        let mut output = String::new();

        for (file_path, issues) in results {
            if !issues.is_empty() {
                output.push_str(&format!("\n{}\n", file_path.display()));
                for issue in issues {
                    output.push_str(&format!("  {}\n", self.format_human_issue(issue)));
                }
            }
        }

        output.push_str(&format!("\n{}\n", statistics.summary()));
        Ok(output)
    }

    fn generate_json_report(
        &self,
        results: &[(PathBuf, Vec<LintIssue>)],
        statistics: &LintStatistics,
    ) -> Result<String, Error> {
        let json_results: Vec<_> = results
            .iter()
            .map(|(path, issues)| JsonFileResult {
                file: path.to_string_lossy().to_string(),
                issues: issues.iter().map(|i| self.issue_to_json(i)).collect(),
            })
            .collect();

        let report = JsonReport {
            results: json_results,
            statistics: JsonStatistics::from(statistics),
        };

        serde_json::to_string_pretty(&report)
            .map_err(|e| Error::Configuration(format!("JSON serialization error: {}", e)))
    }

    fn generate_checkstyle_report(
        &self,
        results: &[(PathBuf, Vec<LintIssue>)],
        _statistics: &LintStatistics,
    ) -> Result<String, Error> {
        let mut output = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        output.push_str("\n<checkstyle version=\"1.0\">\n");

        for (file_path, issues) in results {
            output.push_str(&format!(r#"  <file name="{}">"#, file_path.display()));
            output.push('\n');

            for issue in issues {
                output.push_str(&format!(
                    r#"    <error line="{}" column="{}" severity="{}" message="{}" source="{}"/>"#,
                    issue.location.line + 1,
                    issue.location.column + 1,
                    issue.severity,
                    html_escape(&issue.message),
                    issue.rule_name
                ));
                output.push('\n');
            }

            output.push_str("  </file>\n");
        }

        output.push_str("</checkstyle>\n");
        Ok(output)
    }

    fn generate_sarif_report(
        &self,
        results: &[(PathBuf, Vec<LintIssue>)],
        _statistics: &LintStatistics,
    ) -> Result<String, Error> {
        // SARIF (Static Analysis Results Interchange Format) implementation
        // This is a simplified version
        let sarif_results: Vec<_> = results
            .iter()
            .flat_map(|(path, issues)| {
                issues.iter().map(move |issue| SarifResult {
                    rule_id: issue.rule_name.clone(),
                    level: match issue.severity {
                        crate::linter::engine::LintSeverity::Error => "error".to_string(),
                        crate::linter::engine::LintSeverity::Warning => "warning".to_string(),
                        crate::linter::engine::LintSeverity::Info => "note".to_string(),
                    },
                    message: SarifMessage {
                        text: issue.message.clone(),
                    },
                    locations: vec![SarifLocation {
                        physical_location: SarifPhysicalLocation {
                            artifact_location: SarifArtifactLocation {
                                uri: path.to_string_lossy().to_string(),
                            },
                            region: SarifRegion {
                                start_line: issue.location.line + 1,
                                start_column: issue.location.column + 1,
                            },
                        },
                    }],
                })
            })
            .collect();

        let sarif_report = SarifReport {
            schema: Some("https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json".to_string()),
            version: "2.1.0".to_string(),
            runs: vec![SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: "cursed-lint".to_string(),
                        version: "1.0.0".to_string(),
                    },
                },
                results: sarif_results,
            }],
        };

        serde_json::to_string_pretty(&sarif_report)
            .map_err(|e| Error::Configuration(format!("SARIF serialization error: {}", e)))
    }

    fn print_human_issues(&self, file_path: &PathBuf, issues: &[LintIssue]) {
        println!("\n{}", file_path.display());
        for issue in issues {
            println!("  {}", self.format_human_issue(issue));
        }
    }

    fn print_json_issues(&self, file_path: &PathBuf, issues: &[LintIssue]) {
        for issue in issues {
            let json_issue = self.issue_to_json(issue);
            if let Ok(json_str) = serde_json::to_string(&json_issue) {
                println!("{}", json_str);
            }
        }
    }

    fn print_checkstyle_issues(&self, file_path: &PathBuf, issues: &[LintIssue]) {
        if !issues.is_empty() {
            println!(r#"<file name="{}">"#, file_path.display());
            for issue in issues {
                println!(
                    r#"  <error line="{}" column="{}" severity="{}" message="{}" source="{}"/>"#,
                    issue.location.line + 1,
                    issue.location.column + 1,
                    issue.severity,
                    html_escape(&issue.message),
                    issue.rule_name
                );
            }
            println!("</file>");
        }
    }

    fn format_human_issue(&self, issue: &LintIssue) -> String {
        let mut formatted = String::new();

        if self.options.show_severity {
            formatted.push_str(&format!("{}: ", issue.severity));
        }

        formatted.push_str(&format!("{}:{}: ", issue.location.line + 1, issue.location.column + 1));

        formatted.push_str(&issue.message);

        if self.options.show_rule_names {
            formatted.push_str(&format!(" [{}]", issue.rule_name));
        }

        if self.options.show_suggestions {
            if let Some(suggestion) = &issue.suggestion {
                formatted.push_str(&format!("\n    Suggestion: {}", suggestion.description));
            }
        }

        formatted
    }

    fn issue_to_json(&self, issue: &LintIssue) -> JsonIssue {
        JsonIssue {
            rule: issue.rule_name.clone(),
            category: format!("{}", issue.category),
            severity: format!("{}", issue.severity),
            message: issue.message.clone(),
            line: issue.location.line + 1,
            column: issue.location.column + 1,
            suggestion: issue.suggestion.as_ref().map(|s| s.description.clone()),
        }
    }
}

// JSON serialization structures
#[derive(Serialize)]
struct JsonReport {
    results: Vec<JsonFileResult>,
    statistics: JsonStatistics,
}

#[derive(Serialize)]
struct JsonFileResult {
    file: String,
    issues: Vec<JsonIssue>,
}

#[derive(Serialize)]
struct JsonIssue {
    rule: String,
    category: String,
    severity: String,
    message: String,
    line: usize,
    column: usize,
    suggestion: Option<String>,
}

#[derive(Serialize)]
struct JsonStatistics {
    files_processed: usize,
    total_issues: usize,
    errors: usize,
    warnings: usize,
    info: usize,
    auto_fixable: usize,
    processing_time_ms: u128,
}

impl From<&LintStatistics> for JsonStatistics {
    fn from(stats: &LintStatistics) -> Self {
        Self {
            files_processed: stats.files_processed,
            total_issues: stats.total_issues,
            errors: stats.issues_by_severity.get(&crate::linter::engine::LintSeverity::Error).copied().unwrap_or(0),
            warnings: stats.issues_by_severity.get(&crate::linter::engine::LintSeverity::Warning).copied().unwrap_or(0),
            info: stats.issues_by_severity.get(&crate::linter::engine::LintSeverity::Info).copied().unwrap_or(0),
            auto_fixable: stats.auto_fixable_issues,
            processing_time_ms: stats.processing_time_ms,
        }
    }
}

// SARIF serialization structures
#[derive(Serialize)]
struct SarifReport {
    #[serde(rename = "$schema")]
    schema: Option<String>,
    version: String,
    runs: Vec<SarifRun>,
}

#[derive(Serialize)]
struct SarifRun {
    tool: SarifTool,
    results: Vec<SarifResult>,
}

#[derive(Serialize)]
struct SarifTool {
    driver: SarifDriver,
}

#[derive(Serialize)]
struct SarifDriver {
    name: String,
    version: String,
}

#[derive(Serialize)]
struct SarifResult {
    #[serde(rename = "ruleId")]
    rule_id: String,
    level: String,
    message: SarifMessage,
    locations: Vec<SarifLocation>,
}

#[derive(Serialize)]
struct SarifMessage {
    text: String,
}

#[derive(Serialize)]
struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    physical_location: SarifPhysicalLocation,
}

#[derive(Serialize)]
struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    artifact_location: SarifArtifactLocation,
    region: SarifRegion,
}

#[derive(Serialize)]
struct SarifArtifactLocation {
    uri: String,
}

#[derive(Serialize)]
struct SarifRegion {
    #[serde(rename = "startLine")]
    start_line: usize,
    #[serde(rename = "startColumn")]
    start_column: usize,
}

/// Escape HTML entities
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::SourceLocation, linter::{engine::LintSeverity, rules::RuleCategory}};

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("test"), "test");
        assert_eq!(html_escape("a < b"), "a &lt; b");
        assert_eq!(html_escape("a & b"), "a &amp; b");
        assert_eq!(html_escape("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_output_format_serialization() {
        assert_eq!(serde_json::to_string(&OutputFormat::Human).unwrap(), "\"human\"");
        assert_eq!(serde_json::to_string(&OutputFormat::Json).unwrap(), "\"json\"");
        assert_eq!(serde_json::to_string(&OutputFormat::Checkstyle).unwrap(), "\"checkstyle\"");
    }

    #[test]
    fn test_reporter_creation() {
        let options = ReportOptions::default();
        let reporter = LintReporter::new(options);
        // Test that reporter is created successfully
    }

    #[test]
    fn test_issue_to_json() {
        let reporter = LintReporter::new(ReportOptions::default());
        let issue = crate::linter::engine::LintIssue::new(
            LintSeverity::Warning,
            "test-rule".to_string(),
            RuleCategory::Style,
            "Test message".to_string(),
            SourceLocation::new(5, 10),
        );

        let json_issue = reporter.issue_to_json(&issue);
        assert_eq!(json_issue.rule, "test-rule");
        assert_eq!(json_issue.severity, "warning");
        assert_eq!(json_issue.line, 6); // 1-based
        assert_eq!(json_issue.column, 11); // 1-based
    }
}
