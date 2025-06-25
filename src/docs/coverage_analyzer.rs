/// Documentation Coverage Analysis System
/// 
/// Provides comprehensive analysis of documentation coverage across CURSED source files,
/// including missing documentation detection, quality metrics, and improvement suggestions.

use crate::error::{CursedError, SourceLocation};
use crate::lexer::{Lexer, TokenType};
use crate::parser::{Parser, ParsedProgram};
use crate::ast::{AstNode, Statement, Expression, Declaration};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;

/// Documentation coverage analyzer
#[derive(Debug)]
pub struct CoverageAnalyzer {
    /// Configuration for coverage analysis
    /// Collected coverage statistics
    /// Quality metrics
/// Configuration for coverage analysis
#[derive(Debug, Clone)]
pub struct CoverageConfig {
    /// Minimum documentation length threshold
    /// Require documentation for public items
    /// Require documentation for private items
    /// Minimum quality score threshold
    /// Enable spelling/grammar checks
    /// Files to exclude from analysis
    /// Item types to require documentation
impl Default for CoverageConfig {
    fn default() -> Self {
        let mut required_items = HashSet::new();
        required_items.insert(ItemType::Function);
        required_items.insert(ItemType::Struct);
        required_items.insert(ItemType::Interface);
        required_items.insert(ItemType::Module);
        
        Self {
        }
    }
/// Types of documentable items
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
/// Documentation coverage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageStatistics {
    /// Total items found
    /// Items with documentation
    /// Items missing documentation
    /// Coverage percentage
    /// Coverage by item type
    /// Coverage by file
    /// Quality distribution
impl Default for CoverageStatistics {
    fn default() -> Self {
        Self {
        }
    }
/// Coverage statistics for a specific item type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeCoverage {
/// Coverage statistics for a specific file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
/// Missing documentation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingDocumentation {
/// Item visibility
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
/// Quality metrics for documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Average documentation length
    /// Spelling errors found
    /// Grammar issues found
    /// Consistency score (0.0 - 1.0)
    /// Completeness score (0.0 - 1.0)
    /// Overall quality score (0.0 - 1.0)
impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
        }
    }
/// Quality score distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityDistribution {
    pub excellent: usize,   // 0.9 - 1.0
    pub good: usize,        // 0.7 - 0.9
    pub fair: usize,        // 0.5 - 0.7
    pub poor: usize,        // 0.0 - 0.5
impl Default for QualityDistribution {
    fn default() -> Self {
        Self {
        }
    }
/// Documentation item information
#[derive(Debug, Clone)]
pub struct DocumentationItem {
/// Coverage analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
/// Documentation improvement suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationSuggestion {
/// Types of documentation suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
/// Suggestion priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
impl CoverageAnalyzer {
    /// Create a new coverage analyzer
    pub fn new(config: CoverageConfig) -> Self {
        Self {
        }
    }

    /// Analyze documentation coverage for a set of files
    pub fn analyze_files(&mut self, files: &[PathBuf]) -> crate::error::Result<()> {
        self.stats = CoverageStatistics::default();
        self.quality_metrics = QualityMetrics::default();

        let mut all_items = Vec::new();
        let mut file_coverages = HashMap::new();

        for file_path in files {
            if self.config.excluded_files.contains(file_path) {
                continue;
            let file_content = fs::read_to_string(file_path)
                .map_err(|e| CursedError::SystemError(format!("Failed to read file {}: {}", file_path.display(), e)))?;

            let items = self.extract_items_from_file(&file_content, file_path)?;
            let file_coverage = self.analyze_file_coverage(&items, file_path);
            
            file_coverages.insert(file_path.clone(), file_coverage);
            all_items.extend(items);
        self.stats.coverage_by_file = file_coverages;
        self.calculate_overall_statistics(&all_items);
        self.calculate_quality_metrics(&all_items);

        let suggestions = self.generate_suggestions(&all_items);
        let analysis_summary = self.generate_analysis_summary();

        Ok(CoverageReport {
        })
    /// Extract documentable items from a file
    fn extract_items_from_file(&self, content: &str, file_path: &Path) -> crate::error::Result<()> {
        let mut items = Vec::new();

        // Parse the file to get AST
        let mut lexer = Lexer::new(content.to_string());
        let tokens = lexer.tokenize()
            .map_err(|e| CursedError::SystemError(format!("Failed to tokenize file {}: {:?}", file_path.display(), e)))?;

        let mut parser = Parser::new(tokens);
        let program = parser.parse()
            .map_err(|e| CursedError::SystemError(format!("Failed to parse file {}: {:?}", file_path.display(), e)))?;

        // Extract items from AST
        self.extract_items_from_program(&program, &mut items);

        // Extract documentation for each item
        for item in &mut items {
            item.documentation = self.extract_documentation_for_item(content, &item.location);
            item.quality_score = self.calculate_item_quality(&item);
        Ok(items)
    /// Extract items from parsed program
    fn extract_items_from_program(&self, program: &ParsedProgram, items: &mut Vec<DocumentationItem>) {
        for statement in &program.statements {
            self.extract_items_from_statement(statement, items);
        }
    }

    /// Extract items from AST statement
    fn extract_items_from_statement(&self, statement: &dyn Statement, items: &mut Vec<DocumentationItem>) {
        match statement {
            Statement::FunctionDeclaration { name, location, .. } => {
                items.push(DocumentationItem {
                    visibility: Visibility::Public, // Default assumption
                });
            }
            Statement::StructDeclaration { name, location, .. } => {
                items.push(DocumentationItem {
                });
            }
            Statement::InterfaceDeclaration { name, location, .. } => {
                items.push(DocumentationItem {
                });
            }
            Statement::VariableDeclaration { name, location, .. } => {
                items.push(DocumentationItem {
                    visibility: Visibility::Private, // Default assumption
                });
            }
            _ => {
                // Handle other statement types as needed
            }
        }
    /// Extract documentation for a specific item
    fn extract_documentation_for_item(&self, content: &str, location: &SourceLocation) -> Option<String> {
        let lines: Vec<&str> = content.split("\n").collect();
        
        if location.line == 0 || location.line > lines.len() {
            return None;
        let mut doc_lines = Vec::new();
        let mut line_idx = location.line.saturating_sub(1);

        // Look backwards for documentation comments
        while line_idx > 0 {
            line_idx -= 1;
            let line = lines[line_idx].trim();
            
            if line.starts_with("///") {
                doc_lines.insert(0, line.trim_start_matches("///").trim());
            } else if line.starts_with("/**") {
                // Multi-line comment start
                let mut comment = line.trim_start_matches("/**").trim().to_string();
                if line.ends_with("*/") {
                    comment = comment.trim_end_matches("*/").trim().to_string();
                    doc_lines.insert(0, &comment);
                    break;
                } else {
                    // Continue reading multi-line comment
                    line_idx += 1;
                    while line_idx < lines.len() {
                        let comment_line = lines[line_idx].trim();
                        if comment_line.ends_with("*/") {
                            comment.push(' ');
                            comment.push_str(comment_line.trim_end_matches("*/").trim());
                            break;
                        } else {
                            comment.push(' ');
                            comment.push_str(comment_line.trim_start_matches('*').trim());
                        }
                        line_idx += 1;
                    }
                    doc_lines.insert(0, &comment);
                    break;
                }
            } else if line.is_empty() {
                continue;
            } else {
                break;
            }
        }

        if doc_lines.is_empty() {
            None
        } else {
            Some(doc_lines.join(" ").trim().to_string())
        }
    }

    /// Calculate quality score for a documentation item
    fn calculate_item_quality(&self, item: &DocumentationItem) -> f64 {
        if let Some(ref doc) = item.documentation {
            let mut score = 0.0;
            let mut factors = 0;

            // Length factor
            if doc.len() >= self.config.min_doc_length {
                score += 0.3;
            }
            factors += 1;

            // Content quality factors
            if doc.contains("@param") || doc.contains("@parameter") {
                score += 0.2;
            }
            if doc.contains("@return") || doc.contains("@returns") {
                score += 0.2;
            }
            if doc.contains("@example") || doc.contains("```") {
                score += 0.2;
            }
            if doc.contains("@throws") || doc.contains("@error") {
                score += 0.1;
            }
            factors += 4;

            // Basic language quality (simplified)
            if self.config.enable_language_checks {
                let word_count = doc.split_whitespace().count();
                if word_count >= 5 {
                    score += 0.1;
                }
                if doc.ends_with('.') || doc.ends_with('!') || doc.ends_with('?') {
                    score += 0.1;
                }
                factors += 2;
            score / factors as f64
        } else {
            0.0
        }
    }

    /// Analyze coverage for a specific file
    fn analyze_file_coverage(&self, items: &[DocumentationItem], file_path: &Path) -> FileCoverage {
        let total_items = items.len();
        let documented_items = items.iter().filter(|item| item.documentation.is_some()).count();
        let coverage_percentage = if total_items > 0 {
            (documented_items as f64 / total_items as f64) * 100.0
        } else {
            100.0

        let quality_score = if documented_items > 0 {
            items.iter()
                .filter(|item| item.documentation.is_some())
                .map(|item| item.quality_score)
                .sum::<f64>() / documented_items as f64
        } else {
            0.0

        let missing_docs = items.iter()
            .filter(|item| item.documentation.is_none())
            .filter(|item| self.should_require_documentation(item))
            .map(|item| MissingDocumentation {
            })
            .collect();

        FileCoverage {
        }
    }

    /// Check if documentation should be required for an item
    fn should_require_documentation(&self, item: &DocumentationItem) -> bool {
        if !self.config.required_doc_items.contains(&item.item_type) {
            return false;
        match item.visibility {
        }
    }

    /// Generate a suggestion for an undocumented item
    fn generate_item_suggestion(&self, item: &DocumentationItem) -> String {
        match item.item_type {
        }
    }

    /// Calculate overall statistics
    fn calculate_overall_statistics(&mut self, items: &[DocumentationItem]) {
        self.stats.total_items = items.len();
        self.stats.documented_items = items.iter().filter(|item| item.documentation.is_some()).count();
        self.stats.undocumented_items = self.stats.total_items - self.stats.documented_items;
        self.stats.coverage_percentage = if self.stats.total_items > 0 {
            (self.stats.documented_items as f64 / self.stats.total_items as f64) * 100.0
        } else {
            100.0

        // Calculate coverage by type
        let mut type_stats: HashMap<ItemType, (usize, usize)> = HashMap::new();
        for item in items {
            let entry = type_stats.entry(item.item_type.clone()).or_insert((0, 0));
            entry.0 += 1; // total
            if item.documentation.is_some() {
                entry.1 += 1; // documented
            }
        }

        for (item_type, (total, documented)) in type_stats {
            let coverage_percentage = if total > 0 {
                (documented as f64 / total as f64) * 100.0
            } else {
                100.0

            let average_quality = items.iter()
                .filter(|item| item.item_type == item_type && item.documentation.is_some())
                .map(|item| item.quality_score)
                .sum::<f64>() / if documented > 0 { documented as f64 } else { 1.0 };

            self.stats.coverage_by_type.insert(item_type, TypeCoverage {
            });
        // Calculate quality distribution
        for item in items {
            if item.documentation.is_some() {
                match item.quality_score {
                }
            }
        }
    }

    /// Calculate overall quality metrics
    fn calculate_quality_metrics(&mut self, items: &[DocumentationItem]) {
        let documented_items: Vec<_> = items.iter().filter(|item| item.documentation.is_some()).collect();

        if documented_items.is_empty() {
            return;
        // Average length
        let total_length: usize = documented_items.iter()
            .map(|item| item.documentation.as_ref().unwrap().len())
            .sum();
        self.quality_metrics.average_length = total_length as f64 / documented_items.len() as f64;

        // Quality scores
        let total_quality: f64 = documented_items.iter().map(|item| item.quality_score).sum();
        self.quality_metrics.overall_score = total_quality / documented_items.len() as f64;

        // Completeness (documented vs total)
        self.quality_metrics.completeness_score = self.stats.documented_items as f64 / self.stats.total_items as f64;

        // Consistency (simplified - based on quality variance)
        let quality_scores: Vec<f64> = documented_items.iter().map(|item| item.quality_score).collect();
        let variance = self.calculate_variance(&quality_scores);
        self.quality_metrics.consistency_score = (1.0 - variance.min(1.0)).max(0.0);

        // Language checks (simplified implementation)
        if self.config.enable_language_checks {
            for item in &documented_items {
                if let Some(ref doc) = item.documentation {
                    // Simple spelling check (count potential misspellings)
                    self.quality_metrics.spelling_errors += self.count_potential_spelling_errors(doc);
                    // Simple grammar check (basic patterns)
                    self.quality_metrics.grammar_issues += self.count_potential_grammar_issues(doc);
                }
            }
        }
    }

    /// Calculate variance for quality scores
    fn calculate_variance(&self, scores: &[f64]) -> f64 {
        if scores.len() <= 1 {
            return 0.0;
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let variance = scores.iter()
            .map(|score| (*score - mean).powi(2))
            .sum::<f64>() / scores.len() as f64;
        variance
    /// Count potential spelling errors (simplified)
    fn count_potential_spelling_errors(&self, text: &str) -> usize {
        // Very basic implementation - count words with unusual patterns
        text.split_whitespace()
            .filter(|word| {
                let clean_word = word.chars().filter(|c| c.is_alphabetic()).collect::<String>();
                clean_word.len() > 3 && 
                clean_word.chars().filter(|c| "aeiouAEIOU".contains(*c)).count() == 0
            })
            .count()
    /// Count potential grammar issues (simplified)
    fn count_potential_grammar_issues(&self, text: &str) -> usize {
        let mut issues = 0;
        
        // Check for sentences without proper ending
        let sentences = text.split(&['.', '!', '?'][..]);
        for sentence in sentences {
            let trimmed = sentence.trim();
            if !trimmed.is_empty() && trimmed.len() > 5 {
                // Check if sentence starts with lowercase (potential issue)
                if let Some(first_char) = trimmed.chars().next() {
                    if first_char.is_lowercase() {
                        issues += 1;
                    }
                }
            }
        }

        issues
    /// Generate improvement suggestions
    fn generate_suggestions(&self, items: &[DocumentationItem]) -> Vec<DocumentationSuggestion> {
        let mut suggestions = Vec::new();

        for item in items {
            if item.documentation.is_none() && self.should_require_documentation(item) {
                suggestions.push(DocumentationSuggestion {
                });
            } else if let Some(ref doc) = item.documentation {
                if item.quality_score < self.config.min_quality_score {
                    suggestions.push(DocumentationSuggestion {
                    });
                if !doc.contains("@example") && !doc.contains("```") {
                    suggestions.push(DocumentationSuggestion {
                        example: Some(format!("```cursed\n// Example usage of {}\n```", item.name)),
                    });
                }
            }
        suggestions
    /// Generate a documentation example for an item
    fn generate_documentation_example(&self, item: &DocumentationItem) -> String {
        match item.item_type {
            ItemType::Function => {
                format!("/// Brief description of what {} does\n/// \n/// @param parameter_name Description of the parameter\n/// @return Description of what is returned\n/// \n/// @example\n/// ```cursed\n/// let result = {}();\n/// ```", item.name, item.name)
            }
            ItemType::Struct => {
                format!("/// Brief description of {} struct\n/// \n/// This struct represents...\n/// \n/// @example\n/// ```cursed\n/// let instance = {} {{\n///     // Initialize fields\n/// }};\n/// ```", item.name, item.name)
            }
            ItemType::Interface => {
                format!("/// Interface {} defines...\n/// \n/// This interface provides...\n/// \n/// @example\n/// ```cursed\n/// // Implementation example\n/// ```", item.name)
            }
            _ => {
                format!("/// Description of {}\n/// \n/// @example\n/// ```cursed\n/// // Usage example\n/// ```", item.name)
            }
        }
    /// Generate analysis summary
    fn generate_analysis_summary(&self) -> String {
        format!(
            "Documentation Coverage Analysis Summary\n\n\
            Total Items: {}\n\
            Documented Items: {} ({:.1}%)\n\
            Undocumented Items: {}\n\
            Average Quality Score: {:.2}\n\
            Files Analyzed: {}\n\n\
            Quality Distribution:\n\
            - Excellent (0.9+): {}\n\
            - Good (0.7-0.9): {}\n\
            - Fair (0.5-0.7): {}\n\
            - Poor (0.0-0.5): {}\n\n\
            Recommendations:\n\
            - {} items need documentation\n\
            - {} items need quality improvements\n\
            if self.config.require_public_docs { "public" } else { "all" }
        )
    /// Generate HTML coverage report
    pub fn generate_html_report(&self, report: &CoverageReport, output_path: &Path) -> crate::error::Result<()> {
        let html_content = self.generate_html_content(report);
        fs::write(output_path, html_content)
            .map_err(|e| CursedError::SystemError(format!("Failed to write HTML report: {}", e)))
    /// Generate markdown coverage report
    pub fn generate_markdown_report(&self, report: &CoverageReport, output_path: &Path) -> crate::error::Result<()> {
        let markdown_content = self.generate_markdown_content(report);
        fs::write(output_path, markdown_content)
            .map_err(|e| CursedError::SystemError(format!("Failed to write Markdown report: {}", e)))
    /// Generate HTML content for coverage report
    fn generate_html_content(&self, report: &CoverageReport) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Documentation Coverage Report</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 40px; line-height: 1.6; }}
        .header {{ background: #f8f9fa; padding: 20px; border-radius: 8px; margin-bottom: 30px; }}
        .stats {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 20px 0; }}
        .stat-card {{ background: white; border: 1px solid #e9ecef; border-radius: 8px; padding: 20px; text-align: center; }}
        .coverage-bar {{ background: #e9ecef; height: 20px; border-radius: 10px; overflow: hidden; margin: 10px 0; }}
        .coverage-fill {{ background: #28a745; height: 100%; transition: width 0.3s; }}
        .suggestions {{ margin-top: 30px; }}
        .suggestion {{ background: #fff3cd; border-left: 4px solid #ffc107; padding: 15px; margin: 10px 0; }}
        .high-priority {{ border-left-color: #dc3545; background: #f8d7da; }}
        .quality-distribution {{ display: flex; gap: 10px; margin: 20px 0; }}
        .quality-bar {{ flex: 1; height: 30px; display: flex; align-items: center; justify-content: center; color: white; border-radius: 4px; }}
        .excellent {{ background: #28a745; }}
        .good {{ background: #ffc107; }}
        .fair {{ background: #fd7e14; }}
        .poor {{ background: #dc3545; }}
        table {{ width: 100%; border-collapse: collapse; margin: 20px 0; }}
        th, td {{ padding: 12px; text-align: left; border-bottom: 1px solid #dee2e6; }}
        th {{ background-color: #f8f9fa; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Documentation Coverage Report</h1>
        <p>Generated on: {}</p>
        <p>{}</p>
    </div>

    <div class="stats">
        <div class="stat-card">
            <h3>Overall Coverage</h3>
            <div style="font-size: 2em; color: #28a745;">{:.1}%</div>
            <div class="coverage-bar">
                <div class="coverage-fill" style="width: {:.1}%"></div>
            </div>
            <p>{} of {} items documented</p>
        </div>
        <div class="stat-card">
            <h3>Quality Score</h3>
            <div style="font-size: 2em; color: #007bff;">{:.2}</div>
            <p>Average documentation quality</p>
        </div>
        <div class="stat-card">
            <h3>Total Items</h3>
            <div style="font-size: 2em; color: #6c757d;">{}</div>
            <p>Documentable items found</p>
        </div>
    </div>

    <h2>Quality Distribution</h2>
    <div class="quality-distribution">
        <div class="quality-bar excellent">Excellent: {}</div>
        <div class="quality-bar good">Good: {}</div>
        <div class="quality-bar fair">Fair: {}</div>
        <div class="quality-bar poor">Poor: {}</div>
    </div>

    <h2>Coverage by Type</h2>
    <table>
        <thead>
            <tr>
                <th>Type</th>
                <th>Total</th>
                <th>Documented</th>
                <th>Coverage</th>
                <th>Avg Quality</th>
            </tr>
        </thead>
        <tbody>
            {}
        </tbody>
    </table>

    <h2>Improvement Suggestions</h2>
    <div class="suggestions">
        {}
    </div>
</body>
</html>"#,
            self.generate_suggestions_html(report)
        )
    /// Generate table rows for type coverage
    fn generate_type_coverage_table_rows(&self, report: &CoverageReport) -> String {
        report.statistics.coverage_by_type
            .iter()
            .map(|(item_type, coverage)| {
                format!(
                    "<tr><td>{:?}</td><td>{}</td><td>{}</td><td>{:.1}%</td><td>{:.2}</td></tr>",
                    item_type, coverage.total, coverage.documented, coverage.coverage_percentage, coverage.average_quality
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    /// Generate HTML for suggestions
    fn generate_suggestions_html(&self, report: &CoverageReport) -> String {
        report.suggestions
            .iter()
            .take(10) // Limit to top 10 suggestions
            .map(|suggestion| {
                let priority_class = match suggestion.priority {
                format!(
                    r#"<div class="{}">
                        <strong>{:?}</strong>: {}
                        <br><small>Location: {}:{}</small>
                        {}
                    </div>"#,
                    suggestion.example.as_ref().map(|ex| format!("<pre><code>{}</code></pre>", ex)).unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    /// Generate markdown content for coverage report
    fn generate_markdown_content(&self, report: &CoverageReport) -> String {
        format!(
            r#"# Documentation Coverage Report

## Summary Statistics

- **Overall Coverage**: {:.1}% ({} of {} items documented)
- **Quality Score**: {:.2}/1.0
- **Total Items**: {}
- **Files Analyzed**: {}

## Quality Distribution

- **Excellent (0.9+)**: {}
- **Good (0.7-0.9)**: {}
- **Fair (0.5-0.7)**: {}
- **Poor (0.0-0.5)**: {}

## Coverage by Type

| Type | Total | Documented | Coverage | Avg Quality |
|------|-------|------------|----------|-------------|
## Improvement Suggestions

## Quality Metrics

- Average Length: {:.1} characters
- Spelling Errors: {}
- Grammar Issues: {}
- Consistency Score: {:.2}
- Completeness Score: {:.2}

---
            report.quality_metrics.completeness_score
        )
    /// Generate markdown table for type coverage
    fn generate_type_coverage_table_markdown(&self, report: &CoverageReport) -> String {
        report.statistics.coverage_by_type
            .iter()
            .map(|(item_type, coverage)| {
                format!(
                    item_type, coverage.total, coverage.documented, coverage.coverage_percentage, coverage.average_quality
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    /// Generate markdown for suggestions
    fn generate_suggestions_markdown(&self, report: &CoverageReport) -> String {
        report.suggestions
            .iter()
            .take(10)
            .enumerate()
            .map(|(i, suggestion)| {
                format!(
                    match suggestion.priority {
                    suggestion.example.as_ref().map(|ex| format!("   ```cursed\n   {}\n   ```", ex)).unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}
