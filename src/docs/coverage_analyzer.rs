/// Documentation Coverage Analysis System
/// 
/// Provides comprehensive analysis of documentation coverage across CURSED source files,
/// including missing documentation detection, quality metrics, and improvement suggestions.

use crate::error::{Error, SourceLocation};
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
    config: CoverageConfig,
    /// Collected coverage statistics
    stats: CoverageStatistics,
    /// Quality metrics
    quality_metrics: QualityMetrics,
}

/// Configuration for coverage analysis
#[derive(Debug, Clone)]
pub struct CoverageConfig {
    /// Minimum documentation length threshold
    pub min_doc_length: usize,
    /// Require documentation for public items
    pub require_public_docs: bool,
    /// Require documentation for private items
    pub require_private_docs: bool,
    /// Minimum quality score threshold
    pub min_quality_score: f64,
    /// Enable spelling/grammar checks
    pub enable_language_checks: bool,
    /// Files to exclude from analysis
    pub excluded_files: HashSet<PathBuf>,
    /// Item types to require documentation
    pub required_doc_items: HashSet<ItemType>,
}

impl Default for CoverageConfig {
    fn default() -> Self {
        let mut required_items = HashSet::new();
        required_items.insert(ItemType::Function);
        required_items.insert(ItemType::Struct);
        required_items.insert(ItemType::Interface);
        required_items.insert(ItemType::Module);
        
        Self {
            min_doc_length: 10,
            require_public_docs: true,
            require_private_docs: false,
            min_quality_score: 0.7,
            enable_language_checks: true,
            excluded_files: HashSet::new(),
            required_doc_items: required_items,
        }
    }
}

/// Types of documentable items
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    Function,
    Struct,
    Interface,
    Variable,
    Constant,
    Module,
    Enum,
    Type,
    Macro,
    Field,
    Method,
}

/// Documentation coverage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageStatistics {
    /// Total items found
    pub total_items: usize,
    /// Items with documentation
    pub documented_items: usize,
    /// Items missing documentation
    pub undocumented_items: usize,
    /// Coverage percentage
    pub coverage_percentage: f64,
    /// Coverage by item type
    pub coverage_by_type: HashMap<ItemType, TypeCoverage>,
    /// Coverage by file
    pub coverage_by_file: HashMap<PathBuf, FileCoverage>,
    /// Quality distribution
    pub quality_distribution: QualityDistribution,
}

impl Default for CoverageStatistics {
    fn default() -> Self {
        Self {
            total_items: 0,
            documented_items: 0,
            undocumented_items: 0,
            coverage_percentage: 0.0,
            coverage_by_type: HashMap::new(),
            coverage_by_file: HashMap::new(),
            quality_distribution: QualityDistribution::default(),
        }
    }
}

/// Coverage statistics for a specific item type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeCoverage {
    pub total: usize,
    pub documented: usize,
    pub coverage_percentage: f64,
    pub average_quality: f64,
}

/// Coverage statistics for a specific file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    pub file_path: PathBuf,
    pub total_items: usize,
    pub documented_items: usize,
    pub coverage_percentage: f64,
    pub quality_score: f64,
    pub missing_docs: Vec<MissingDocumentation>,
}

/// Missing documentation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingDocumentation {
    pub item_name: String,
    pub item_type: ItemType,
    pub location: SourceLocation,
    pub visibility: Visibility,
    pub suggestion: String,
}

/// Item visibility
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

/// Quality metrics for documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Average documentation length
    pub average_length: f64,
    /// Spelling errors found
    pub spelling_errors: usize,
    /// Grammar issues found
    pub grammar_issues: usize,
    /// Consistency score (0.0 - 1.0)
    pub consistency_score: f64,
    /// Completeness score (0.0 - 1.0)
    pub completeness_score: f64,
    /// Overall quality score (0.0 - 1.0)
    pub overall_score: f64,
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
            average_length: 0.0,
            spelling_errors: 0,
            grammar_issues: 0,
            consistency_score: 1.0,
            completeness_score: 0.0,
            overall_score: 0.0,
        }
    }
}

/// Quality score distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityDistribution {
    pub excellent: usize,   // 0.9 - 1.0
    pub good: usize,        // 0.7 - 0.9
    pub fair: usize,        // 0.5 - 0.7
    pub poor: usize,        // 0.0 - 0.5
}

impl Default for QualityDistribution {
    fn default() -> Self {
        Self {
            excellent: 0,
            good: 0,
            fair: 0,
            poor: 0,
        }
    }
}

/// Documentation item information
#[derive(Debug, Clone)]
pub struct DocumentationItem {
    pub name: String,
    pub item_type: ItemType,
    pub location: SourceLocation,
    pub visibility: Visibility,
    pub documentation: Option<String>,
    pub quality_score: f64,
}

/// Coverage analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
    pub statistics: CoverageStatistics,
    pub quality_metrics: QualityMetrics,
    pub suggestions: Vec<DocumentationSuggestion>,
    pub analysis_summary: String,
    pub generated_at: String,
}

/// Documentation improvement suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationSuggestion {
    pub item_name: String,
    pub location: SourceLocation,
    pub suggestion_type: SuggestionType,
    pub description: String,
    pub example: Option<String>,
    pub priority: Priority,
}

/// Types of documentation suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    MissingDocumentation,
    ImproveQuality,
    AddExamples,
    FixSpelling,
    FixGrammar,
    ImproveConsistency,
    AddParameters,
    AddReturnInfo,
    AddErrorInfo,
}

/// Suggestion priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl CoverageAnalyzer {
    /// Create a new coverage analyzer
    pub fn new(config: CoverageConfig) -> Self {
        Self {
            config,
            stats: CoverageStatistics::default(),
            quality_metrics: QualityMetrics::default(),
        }
    }

    /// Analyze documentation coverage for a set of files
    pub fn analyze_files(&mut self, files: &[PathBuf]) -> Result<(), Error> {
        self.stats = CoverageStatistics::default();
        self.quality_metrics = QualityMetrics::default();

        let mut all_items = Vec::new();
        let mut file_coverages = HashMap::new();

        for file_path in files {
            if self.config.excluded_files.contains(file_path) {
                continue;
            }

            let file_content = fs::read_to_string(file_path)
                .map_err(|e| Error::SystemError(format!("Failed to read file {}: {}", file_path.display(), e)))?;

            let items = self.extract_items_from_file(&file_content, file_path)?;
            let file_coverage = self.analyze_file_coverage(&items, file_path);
            
            file_coverages.insert(file_path.clone(), file_coverage);
            all_items.extend(items);
        }

        self.stats.coverage_by_file = file_coverages;
        self.calculate_overall_statistics(&all_items);
        self.calculate_quality_metrics(&all_items);

        let suggestions = self.generate_suggestions(&all_items);
        let analysis_summary = self.generate_analysis_summary();

        Ok(CoverageReport {
            statistics: self.stats.clone(),
            quality_metrics: self.quality_metrics.clone(),
            suggestions,
            analysis_summary,
            generated_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Extract documentable items from a file
    fn extract_items_from_file(&self, content: &str, file_path: &Path) -> Result<(), Error> {
        let mut items = Vec::new();

        // Parse the file to get AST
        let mut lexer = Lexer::new(content.to_string());
        let tokens = lexer.tokenize()
            .map_err(|e| Error::SystemError(format!("Failed to tokenize file {}: {:?}", file_path.display(), e)))?;

        let mut parser = Parser::new(tokens);
        let program = parser.parse()
            .map_err(|e| Error::SystemError(format!("Failed to parse file {}: {:?}", file_path.display(), e)))?;

        // Extract items from AST
        self.extract_items_from_program(&program, &mut items);

        // Extract documentation for each item
        for item in &mut items {
            item.documentation = self.extract_documentation_for_item(content, &item.location);
            item.quality_score = self.calculate_item_quality(&item);
        }

        Ok(items)
    }

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
                    name: name.clone(),
                    item_type: ItemType::Function,
                    location: location.clone(),
                    visibility: Visibility::Public, // Default assumption
                    documentation: None,
                    quality_score: 0.0,
                });
            }
            Statement::StructDeclaration { name, location, .. } => {
                items.push(DocumentationItem {
                    name: name.clone(),
                    item_type: ItemType::Struct,
                    location: location.clone(),
                    visibility: Visibility::Public,
                    documentation: None,
                    quality_score: 0.0,
                });
            }
            Statement::InterfaceDeclaration { name, location, .. } => {
                items.push(DocumentationItem {
                    name: name.clone(),
                    item_type: ItemType::Interface,
                    location: location.clone(),
                    visibility: Visibility::Public,
                    documentation: None,
                    quality_score: 0.0,
                });
            }
            Statement::VariableDeclaration { name, location, .. } => {
                items.push(DocumentationItem {
                    name: name.clone(),
                    item_type: ItemType::Variable,
                    location: location.clone(),
                    visibility: Visibility::Private, // Default assumption
                    documentation: None,
                    quality_score: 0.0,
                });
            }
            _ => {
                // Handle other statement types as needed
            }
        }
    }

    /// Extract documentation for a specific item
    fn extract_documentation_for_item(&self, content: &str, location: &SourceLocation) -> Option<String> {
        let lines: Vec<&str> = content.split("\n").collect();
        
        if location.line == 0 || location.line > lines.len() {
            return None;
        }

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
            }

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
        };

        let quality_score = if documented_items > 0 {
            items.iter()
                .filter(|item| item.documentation.is_some())
                .map(|item| item.quality_score)
                .sum::<f64>() / documented_items as f64
        } else {
            0.0
        };

        let missing_docs = items.iter()
            .filter(|item| item.documentation.is_none())
            .filter(|item| self.should_require_documentation(item))
            .map(|item| MissingDocumentation {
                item_name: item.name.clone(),
                item_type: item.item_type.clone(),
                location: item.location.clone(),
                visibility: item.visibility.clone(),
                suggestion: self.generate_item_suggestion(item),
            })
            .collect();

        FileCoverage {
            file_path: file_path.to_path_buf(),
            total_items,
            documented_items,
            coverage_percentage,
            quality_score,
            missing_docs,
        }
    }

    /// Check if documentation should be required for an item
    fn should_require_documentation(&self, item: &DocumentationItem) -> bool {
        if !self.config.required_doc_items.contains(&item.item_type) {
            return false;
        }

        match item.visibility {
            Visibility::Public => self.config.require_public_docs,
            Visibility::Private | Visibility::Protected => self.config.require_private_docs,
        }
    }

    /// Generate a suggestion for an undocumented item
    fn generate_item_suggestion(&self, item: &DocumentationItem) -> String {
        match item.item_type {
            ItemType::Function => format!("Add documentation describing what the function '{}' does, its parameters, and return value", item.name),
            ItemType::Struct => format!("Add documentation describing the purpose and usage of struct '{}'", item.name),
            ItemType::Interface => format!("Add documentation describing the interface '{}' and its contract", item.name),
            ItemType::Variable => format!("Add documentation describing the purpose of variable '{}'", item.name),
            ItemType::Constant => format!("Add documentation describing the constant '{}' and its value", item.name),
            ItemType::Module => format!("Add documentation describing the module '{}' and its functionality", item.name),
            _ => format!("Add documentation for {}", item.name),
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
        };

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
            };

            let average_quality = items.iter()
                .filter(|item| item.item_type == item_type && item.documentation.is_some())
                .map(|item| item.quality_score)
                .sum::<f64>() / if documented > 0 { documented as f64 } else { 1.0 };

            self.stats.coverage_by_type.insert(item_type, TypeCoverage {
                total,
                documented,
                coverage_percentage,
                average_quality,
            });
        }

        // Calculate quality distribution
        for item in items {
            if item.documentation.is_some() {
                match item.quality_score {
                    score if score >= 0.9 => self.stats.quality_distribution.excellent += 1,
                    score if score >= 0.7 => self.stats.quality_distribution.good += 1,
                    score if score >= 0.5 => self.stats.quality_distribution.fair += 1,
                    _ => self.stats.quality_distribution.poor += 1,
                }
            }
        }
    }

    /// Calculate overall quality metrics
    fn calculate_quality_metrics(&mut self, items: &[DocumentationItem]) {
        let documented_items: Vec<_> = items.iter().filter(|item| item.documentation.is_some()).collect();

        if documented_items.is_empty() {
            return;
        }

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
        }

        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let variance = scores.iter()
            .map(|score| (*score - mean).powi(2))
            .sum::<f64>() / scores.len() as f64;
        variance
    }

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
    }

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
    }

    /// Generate improvement suggestions
    fn generate_suggestions(&self, items: &[DocumentationItem]) -> Vec<DocumentationSuggestion> {
        let mut suggestions = Vec::new();

        for item in items {
            if item.documentation.is_none() && self.should_require_documentation(item) {
                suggestions.push(DocumentationSuggestion {
                    item_name: item.name.clone(),
                    location: item.location.clone(),
                    suggestion_type: SuggestionType::MissingDocumentation,
                    description: self.generate_item_suggestion(item),
                    example: Some(self.generate_documentation_example(item)),
                    priority: if item.visibility == Visibility::Public { Priority::High } else { Priority::Medium },
                });
            } else if let Some(ref doc) = item.documentation {
                if item.quality_score < self.config.min_quality_score {
                    suggestions.push(DocumentationSuggestion {
                        item_name: item.name.clone(),
                        location: item.location.clone(),
                        suggestion_type: SuggestionType::ImproveQuality,
                        description: format!("Improve documentation quality for '{}' (current score: {:.2})", item.name, item.quality_score),
                        example: None,
                        priority: Priority::Medium,
                    });
                }

                if !doc.contains("@example") && !doc.contains("```") {
                    suggestions.push(DocumentationSuggestion {
                        item_name: item.name.clone(),
                        location: item.location.clone(),
                        suggestion_type: SuggestionType::AddExamples,
                        description: format!("Add usage examples to documentation for '{}'", item.name),
                        example: Some(format!("```cursed\n// Example usage of {}\n```", item.name)),
                        priority: Priority::Low,
                    });
                }
            }
        }

        suggestions
    }

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
            - Focus on {} visibility items first",
            self.stats.total_items,
            self.stats.documented_items,
            self.stats.coverage_percentage,
            self.stats.undocumented_items,
            self.quality_metrics.overall_score,
            self.stats.coverage_by_file.len(),
            self.stats.quality_distribution.excellent,
            self.stats.quality_distribution.good,
            self.stats.quality_distribution.fair,
            self.stats.quality_distribution.poor,
            self.stats.undocumented_items,
            self.stats.quality_distribution.poor + self.stats.quality_distribution.fair,
            if self.config.require_public_docs { "public" } else { "all" }
        )
    }

    /// Generate HTML coverage report
    pub fn generate_html_report(&self, report: &CoverageReport, output_path: &Path) -> Result<(), Error> {
        let html_content = self.generate_html_content(report);
        fs::write(output_path, html_content)
            .map_err(|e| Error::SystemError(format!("Failed to write HTML report: {}", e)))
    }

    /// Generate markdown coverage report
    pub fn generate_markdown_report(&self, report: &CoverageReport, output_path: &Path) -> Result<(), Error> {
        let markdown_content = self.generate_markdown_content(report);
        fs::write(output_path, markdown_content)
            .map_err(|e| Error::SystemError(format!("Failed to write Markdown report: {}", e)))
    }

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
            report.generated_at,
            report.analysis_summary.replace('\n', "<br>"),
            report.statistics.coverage_percentage,
            report.statistics.coverage_percentage,
            report.statistics.documented_items,
            report.statistics.total_items,
            report.quality_metrics.overall_score,
            report.statistics.total_items,
            report.statistics.quality_distribution.excellent,
            report.statistics.quality_distribution.good,
            report.statistics.quality_distribution.fair,
            report.statistics.quality_distribution.poor,
            self.generate_type_coverage_table_rows(report),
            self.generate_suggestions_html(report)
        )
    }

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
    }

    /// Generate HTML for suggestions
    fn generate_suggestions_html(&self, report: &CoverageReport) -> String {
        report.suggestions
            .iter()
            .take(10) // Limit to top 10 suggestions
            .map(|suggestion| {
                let priority_class = match suggestion.priority {
                    Priority::High => "suggestion high-priority",
                    _ => "suggestion",
                };
                format!(
                    r#"<div class="{}">
                        <strong>{:?}</strong>: {}
                        <br><small>Location: {}:{}</small>
                        {}
                    </div>"#,
                    priority_class,
                    suggestion.suggestion_type,
                    suggestion.description,
                    suggestion.location.file.as_ref().map(|f| f.display().to_string()).unwrap_or_else(|| "unknown".to_string()),
                    suggestion.location.line,
                    suggestion.example.as_ref().map(|ex| format!("<pre><code>{}</code></pre>", ex)).unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Generate markdown content for coverage report
    fn generate_markdown_content(&self, report: &CoverageReport) -> String {
        format!(
            r#"# Documentation Coverage Report

Generated on: {}

{}

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
{}

## Improvement Suggestions

{}

## Quality Metrics

- Average Length: {:.1} characters
- Spelling Errors: {}
- Grammar Issues: {}
- Consistency Score: {:.2}
- Completeness Score: {:.2}

---
*Generated by CURSED Documentation Coverage Analyzer*"#,
            report.generated_at,
            report.analysis_summary,
            report.statistics.coverage_percentage,
            report.statistics.documented_items,
            report.statistics.total_items,
            report.quality_metrics.overall_score,
            report.statistics.total_items,
            report.statistics.coverage_by_file.len(),
            report.statistics.quality_distribution.excellent,
            report.statistics.quality_distribution.good,
            report.statistics.quality_distribution.fair,
            report.statistics.quality_distribution.poor,
            self.generate_type_coverage_table_markdown(report),
            self.generate_suggestions_markdown(report),
            report.quality_metrics.average_length,
            report.quality_metrics.spelling_errors,
            report.quality_metrics.grammar_issues,
            report.quality_metrics.consistency_score,
            report.quality_metrics.completeness_score
        )
    }

    /// Generate markdown table for type coverage
    fn generate_type_coverage_table_markdown(&self, report: &CoverageReport) -> String {
        report.statistics.coverage_by_type
            .iter()
            .map(|(item_type, coverage)| {
                format!(
                    "| {:?} | {} | {} | {:.1}% | {:.2} |",
                    item_type, coverage.total, coverage.documented, coverage.coverage_percentage, coverage.average_quality
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Generate markdown for suggestions
    fn generate_suggestions_markdown(&self, report: &CoverageReport) -> String {
        report.suggestions
            .iter()
            .take(10)
            .enumerate()
            .map(|(i, suggestion)| {
                format!(
                    "{}. **{:?}** ({}): {}\n   - Location: {}:{}\n   {}",
                    i + 1,
                    suggestion.suggestion_type,
                    match suggestion.priority {
                        Priority::High => "High Priority",
                        Priority::Medium => "Medium Priority",
                        Priority::Low => "Low Priority",
                    },
                    suggestion.description,
                    suggestion.location.file.as_ref().map(|f| f.display().to_string()).unwrap_or_else(|| "unknown".to_string()),
                    suggestion.location.line,
                    suggestion.example.as_ref().map(|ex| format!("   ```cursed\n   {}\n   ```", ex)).unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}
