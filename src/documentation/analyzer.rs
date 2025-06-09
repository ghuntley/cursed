//! Documentation analyzer for CURSED code quality and completeness
//!
//! This module provides the DocumentationAnalyzer that validates documentation
//! completeness, accuracy, and cross-reference integrity for CURSED programs.

use crate::documentation::{DocumentationError, DocumentationResult, ExtractionResult, DocumentationItem};
use crate::error::SourceLocation;
use std::collections::{HashMap, HashSet};
use tracing::{instrument, debug, warn, info};

/// Analysis result containing validation issues and coverage information
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Coverage report showing documentation completeness
    pub coverage: CoverageReport,
    /// List of validation issues found
    pub issues: Vec<ValidationIssue>,
    /// Cross-reference validation results
    pub link_validation: LinkValidationResult,
    /// Example code validation results
    pub example_validation: ExampleValidationResult,
}

/// Documentation coverage report
#[derive(Debug, Clone)]
pub struct CoverageReport {
    /// Total number of functions
    pub total_functions: usize,
    /// Number of documented functions
    pub documented_functions: usize,
    /// Function documentation coverage percentage
    pub function_coverage: f64,
    /// Total number of types (structs, interfaces)
    pub total_types: usize,
    /// Number of documented types
    pub documented_types: usize,
    /// Type documentation coverage percentage
    pub type_coverage: f64,
    /// Total number of public fields
    pub total_public_fields: usize,
    /// Number of documented public fields
    pub documented_public_fields: usize,
    /// Field documentation coverage percentage
    pub field_coverage: f64,
    /// Overall documentation coverage percentage
    pub overall_coverage: f64,
}

/// Documentation validation issue
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Type of issue
    pub issue_type: IssueType,
    /// Location where the issue was found
    pub location: SourceLocation,
    /// Description of the issue
    pub message: String,
    /// Severity level
    pub severity: Severity,
    /// Suggested fix, if available
    pub suggestion: Option<String>,
}

/// Type of validation issue
#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    /// Missing documentation
    MissingDocumentation,
    /// Parameter mismatch between docs and code
    ParameterMismatch,
    /// Return type mismatch
    ReturnTypeMismatch,
    /// Broken cross-reference
    BrokenReference,
    /// Invalid example code
    InvalidExample,
    /// Spelling or grammar issue
    LanguageIssue,
    /// Inconsistent formatting
    FormattingIssue,
}

/// Issue severity level
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Link validation results
#[derive(Debug, Clone)]
pub struct LinkValidationResult {
    /// Total number of links checked
    pub total_links: usize,
    /// Number of valid links
    pub valid_links: usize,
    /// Number of broken links
    pub broken_links: usize,
    /// Details of broken links
    pub broken_link_details: Vec<BrokenLinkInfo>,
}

/// Information about a broken link
#[derive(Debug, Clone)]
pub struct BrokenLinkInfo {
    /// Location of the broken link
    pub location: SourceLocation,
    /// The broken reference text
    pub reference: String,
    /// Suggested alternatives, if any
    pub suggestions: Vec<String>,
}

/// Example code validation results
#[derive(Debug, Clone)]
pub struct ExampleValidationResult {
    /// Total number of examples found
    pub total_examples: usize,
    /// Number of syntactically valid examples
    pub valid_examples: usize,
    /// Number of invalid examples
    pub invalid_examples: usize,
    /// Details of validation errors
    pub validation_errors: Vec<ExampleValidationError>,
}

/// Example validation error details
#[derive(Debug, Clone)]
pub struct ExampleValidationError {
    /// Location of the example
    pub location: SourceLocation,
    /// The example code that failed
    pub code: String,
    /// Error message from validation
    pub error: String,
}

/// Documentation analyzer for quality and completeness validation
pub struct DocumentationAnalyzer {
    /// Minimum coverage thresholds
    coverage_thresholds: CoverageThresholds,
    /// Whether to check example code syntax
    validate_examples: bool,
    /// Whether to perform spell checking
    spell_check: bool,
}

/// Coverage thresholds for quality gates
#[derive(Debug, Clone)]
pub struct CoverageThresholds {
    /// Minimum function documentation coverage (0.0-1.0)
    pub function_coverage: f64,
    /// Minimum type documentation coverage (0.0-1.0)
    pub type_coverage: f64,
    /// Minimum field documentation coverage (0.0-1.0)
    pub field_coverage: f64,
    /// Minimum overall coverage (0.0-1.0)
    pub overall_coverage: f64,
}

impl Default for CoverageThresholds {
    fn default() -> Self {
        Self {
            function_coverage: 0.8,  // 80% function coverage
            type_coverage: 0.9,      // 90% type coverage
            field_coverage: 0.7,     // 70% field coverage
            overall_coverage: 0.8,   // 80% overall coverage
        }
    }
}

impl DocumentationAnalyzer {
    /// Create a new documentation analyzer with default settings
    pub fn new() -> Self {
        Self {
            coverage_thresholds: CoverageThresholds::default(),
            validate_examples: true,
            spell_check: false, // Disabled by default due to complexity
        }
    }

    /// Access to coverage thresholds for testing
    pub fn coverage_thresholds(&self) -> &CoverageThresholds {
        &self.coverage_thresholds
    }

    /// Check if example validation is enabled
    pub fn validate_examples(&self) -> bool {
        self.validate_examples
    }

    /// Create analyzer with custom coverage thresholds
    pub fn with_thresholds(thresholds: CoverageThresholds) -> Self {
        Self {
            coverage_thresholds: thresholds,
            validate_examples: true,
            spell_check: false,
        }
    }

    /// Set whether to validate example code syntax
    pub fn set_example_validation(&mut self, enabled: bool) -> &mut Self {
        self.validate_examples = enabled;
        self
    }

    /// Perform comprehensive documentation analysis
    #[instrument(skip(self, extraction_result))]
    pub fn analyze(&self, extraction_result: &ExtractionResult) -> DocumentationResult<AnalysisResult> {
        info!("Starting documentation analysis");

        let coverage = self.calculate_coverage(extraction_result);
        let mut issues = self.validate_completeness(extraction_result, &coverage);
        
        // Validate parameter consistency
        issues.extend(self.validate_parameters(extraction_result)?);
        
        // Validate cross-references
        let link_validation = self.validate_links(extraction_result)?;
        issues.extend(self.link_issues_to_validation_issues(&link_validation));

        // Validate example code if enabled
        let example_validation = if self.validate_examples {
            self.validate_examples_impl(extraction_result)?
        } else {
            ExampleValidationResult {
                total_examples: 0,
                valid_examples: 0,
                invalid_examples: 0,
                validation_errors: Vec::new(),
            }
        };
        issues.extend(self.example_issues_to_validation_issues(&example_validation));

        // Add coverage-based issues
        issues.extend(self.check_coverage_thresholds(&coverage));

        info!(
            "Analysis complete: {} issues found, {:.1}% coverage",
            issues.len(),
            coverage.overall_coverage * 100.0
        );

        Ok(AnalysisResult {
            coverage,
            issues,
            link_validation,
            example_validation,
        })
    }

    /// Calculate documentation coverage metrics
    #[instrument(skip(self, extraction_result))]
    pub fn calculate_coverage(&self, extraction_result: &ExtractionResult) -> CoverageReport {
        let stats = &extraction_result.stats;
        
        let function_coverage = if stats.total_functions > 0 {
            stats.documented_functions as f64 / stats.total_functions as f64
        } else {
            1.0
        };

        let type_coverage = if stats.total_types > 0 {
            stats.documented_types as f64 / stats.total_types as f64
        } else {
            1.0
        };

        let field_coverage = if stats.total_fields > 0 {
            stats.documented_fields as f64 / stats.total_fields as f64
        } else {
            1.0
        };

        // Calculate overall coverage as weighted average
        let total_items = stats.total_functions + stats.total_types + stats.total_fields;
        let documented_items = stats.documented_functions + stats.documented_types + stats.documented_fields;
        
        let overall_coverage = if total_items > 0 {
            documented_items as f64 / total_items as f64
        } else {
            1.0
        };

        CoverageReport {
            total_functions: stats.total_functions,
            documented_functions: stats.documented_functions,
            function_coverage,
            total_types: stats.total_types,
            documented_types: stats.documented_types,
            type_coverage,
            total_public_fields: stats.total_fields,
            documented_public_fields: stats.documented_fields,
            field_coverage,
            overall_coverage,
        }
    }

    /// Validate documentation completeness
    pub fn validate_completeness(&self, extraction_result: &ExtractionResult, coverage: &CoverageReport) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Find undocumented items
        for item in &extraction_result.items {
            if item.documentation.is_none() {
                issues.push(ValidationIssue {
                    issue_type: IssueType::MissingDocumentation,
                    location: item.location.clone(),
                    message: format!("{} '{}' is missing documentation", 
                        self.item_type_name(&item.item_type), item.name),
                    severity: self.severity_for_missing_docs(&item.item_type),
                    suggestion: Some(format!("Add documentation comment above {}", item.name)),
                });
            }
        }

        issues
    }

    /// Validate parameter documentation consistency
    #[instrument(skip(self, extraction_result))]
    fn validate_parameters(&self, extraction_result: &ExtractionResult) -> DocumentationResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        for item in &extraction_result.items {
            if let Some(doc) = &item.documentation {
                // Extract @param tags from documentation
                let doc_params = self.extract_param_tags(doc);
                
                // Check that all function parameters are documented
                for param in &item.parameters {
                    if !doc_params.contains_key(&param.name) {
                        issues.push(ValidationIssue {
                            issue_type: IssueType::ParameterMismatch,
                            location: item.location.clone(),
                            message: format!("Parameter '{}' is not documented in {}", param.name, item.name),
                            severity: Severity::Warning,
                            suggestion: Some(format!("Add @param {} description", param.name)),
                        });
                    }
                }

                // Check that documented parameters exist in function
                let actual_params: HashSet<_> = item.parameters.iter()
                    .map(|p| p.name.clone())
                    .collect();
                    
                for doc_param in doc_params.keys() {
                    if !actual_params.contains(doc_param) {
                        issues.push(ValidationIssue {
                            issue_type: IssueType::ParameterMismatch,
                            location: item.location.clone(),
                            message: format!("Documented parameter '{}' does not exist in function {}", doc_param, item.name),
                            severity: Severity::Error,
                            suggestion: Some(format!("Remove @param {} or add parameter to function", doc_param)),
                        });
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Validate cross-references in documentation
    #[instrument(skip(self, extraction_result))]
    fn validate_links(&self, extraction_result: &ExtractionResult) -> DocumentationResult<LinkValidationResult> {
        let mut total_links = 0;
        let mut valid_links = 0;
        let mut broken_link_details = Vec::new();

        // Build symbol lookup table
        let symbol_names: HashSet<_> = extraction_result.symbols.keys().cloned().collect();

        for item in &extraction_result.items {
            for reference in &item.references {
                total_links += 1;
                
                if symbol_names.contains(reference) {
                    valid_links += 1;
                } else {
                    // Look for similar names for suggestions
                    let suggestions = self.find_similar_symbols(reference, &symbol_names);
                    
                    broken_link_details.push(BrokenLinkInfo {
                        location: item.location.clone(),
                        reference: reference.clone(),
                        suggestions,
                    });
                }
            }
        }

        Ok(LinkValidationResult {
            total_links,
            valid_links,
            broken_links: total_links - valid_links,
            broken_link_details,
        })
    }

    /// Validate example code in documentation
    #[instrument(skip(self, extraction_result))]
    fn validate_examples_impl(&self, extraction_result: &ExtractionResult) -> DocumentationResult<ExampleValidationResult> {
        let mut total_examples = 0;
        let mut valid_examples = 0;
        let mut validation_errors = Vec::new();

        for item in &extraction_result.items {
            if let Some(doc) = &item.documentation {
                let examples = self.extract_code_examples(doc);
                total_examples += examples.len();

                for example in examples {
                    match self.validate_example_syntax(&example) {
                        Ok(_) => valid_examples += 1,
                        Err(error) => {
                            validation_errors.push(ExampleValidationError {
                                location: item.location.clone(),
                                code: example,
                                error: error.to_string(),
                            });
                        }
                    }
                }
            }
        }

        Ok(ExampleValidationResult {
            total_examples,
            valid_examples,
            invalid_examples: total_examples - valid_examples,
            validation_errors,
        })
    }

    /// Extract @param tags from documentation
    pub fn extract_param_tags(&self, documentation: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        
        for line in documentation.lines() {
            if let Some(param_line) = line.trim().strip_prefix("@param") {
                if let Some((name, description)) = param_line.trim().split_once(' ') {
                    params.insert(name.to_string(), description.to_string());
                }
            }
        }
        
        params
    }

    /// Extract code examples from documentation
    pub fn extract_code_examples(&self, documentation: &str) -> Vec<String> {
        let mut examples = Vec::new();
        let mut in_code_block = false;
        let mut current_example = String::new();

        for line in documentation.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("```") {
                if in_code_block {
                    // End of code block
                    if !current_example.trim().is_empty() {
                        examples.push(current_example.trim().to_string());
                    }
                    current_example.clear();
                    in_code_block = false;
                } else {
                    // Start of code block
                    in_code_block = true;
                }
            } else if in_code_block {
                current_example.push_str(line);
                current_example.push('\n');
            }
        }

        examples
    }

    /// Validate example code syntax using CURSED parser
    fn validate_example_syntax(&self, code: &str) -> Result<(), Box<dyn std::error::Error>> {
        // For now, just do basic syntax validation
        // TODO: Implement proper CURSED syntax validation
        
        // Basic checks for common syntax issues
        if code.trim().is_empty() {
            return Err("Empty code block".into());
        }
        
        // Check for balanced braces
        let mut brace_count = 0;
        for char in code.chars() {
            match char {
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                _ => {}
            }
        }
        
        if brace_count != 0 {
            return Err("Unbalanced braces".into());
        }
        
        Ok(())
    }

    /// Find similar symbol names for broken link suggestions
    pub fn find_similar_symbols(&self, reference: &str, symbols: &HashSet<String>) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Simple similarity check - find symbols with similar names
        for symbol in symbols {
            if self.string_similarity(reference, symbol) > 0.6 {
                suggestions.push(symbol.clone());
            }
        }
        
        // Limit to 3 suggestions
        suggestions.sort();
        suggestions.truncate(3);
        suggestions
    }

    /// Calculate string similarity (simplified Levenshtein distance)
    pub fn string_similarity(&self, s1: &str, s2: &str) -> f64 {
        let len1 = s1.len();
        let len2 = s2.len();
        
        if len1 == 0 || len2 == 0 {
            return 0.0;
        }
        
        let max_len = len1.max(len2);
        let common_chars = s1.chars()
            .filter(|&c| s2.contains(c))
            .count();
            
        common_chars as f64 / max_len as f64
    }

    /// Convert link validation issues to validation issues
    fn link_issues_to_validation_issues(&self, link_validation: &LinkValidationResult) -> Vec<ValidationIssue> {
        link_validation.broken_link_details.iter()
            .map(|broken_link| ValidationIssue {
                issue_type: IssueType::BrokenReference,
                location: broken_link.location.clone(),
                message: format!("Broken reference: '{}'", broken_link.reference),
                severity: Severity::Warning,
                suggestion: if broken_link.suggestions.is_empty() {
                    None
                } else {
                    Some(format!("Did you mean: {}?", broken_link.suggestions.join(", ")))
                },
            })
            .collect()
    }

    /// Convert example validation issues to validation issues  
    fn example_issues_to_validation_issues(&self, example_validation: &ExampleValidationResult) -> Vec<ValidationIssue> {
        example_validation.validation_errors.iter()
            .map(|error| ValidationIssue {
                issue_type: IssueType::InvalidExample,
                location: error.location.clone(),
                message: format!("Invalid example code: {}", error.error),
                severity: Severity::Error,
                suggestion: Some("Fix the syntax error in the example code".to_string()),
            })
            .collect()
    }

    /// Check coverage against thresholds and generate issues
    fn check_coverage_thresholds(&self, coverage: &CoverageReport) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        let location = SourceLocation::default();

        if coverage.function_coverage < self.coverage_thresholds.function_coverage {
            issues.push(ValidationIssue {
                issue_type: IssueType::MissingDocumentation,
                location: location.clone(),
                message: format!(
                    "Function documentation coverage {:.1}% is below threshold {:.1}%",
                    coverage.function_coverage * 100.0,
                    self.coverage_thresholds.function_coverage * 100.0
                ),
                severity: Severity::Warning,
                suggestion: Some("Add documentation to more functions".to_string()),
            });
        }

        if coverage.type_coverage < self.coverage_thresholds.type_coverage {
            issues.push(ValidationIssue {
                issue_type: IssueType::MissingDocumentation,
                location: location.clone(),
                message: format!(
                    "Type documentation coverage {:.1}% is below threshold {:.1}%",
                    coverage.type_coverage * 100.0,
                    self.coverage_thresholds.type_coverage * 100.0
                ),
                severity: Severity::Warning,
                suggestion: Some("Add documentation to more types".to_string()),
            });
        }

        issues
    }

    /// Get human-readable name for item type
    pub fn item_type_name(&self, item_type: &crate::documentation::extractor::ItemType) -> &'static str {
        match item_type {
            crate::documentation::extractor::ItemType::Function => "Function",
            crate::documentation::extractor::ItemType::Struct => "Struct",
            crate::documentation::extractor::ItemType::Interface => "Interface",
            crate::documentation::extractor::ItemType::Method => "Method",
            crate::documentation::extractor::ItemType::Field => "Field",
            crate::documentation::extractor::ItemType::Parameter => "Parameter",
            crate::documentation::extractor::ItemType::Constant => "Constant",
            crate::documentation::extractor::ItemType::Variable => "Variable",
            crate::documentation::extractor::ItemType::Module => "Module",
        }
    }

    /// Get severity level for missing documentation based on item type
    pub fn severity_for_missing_docs(&self, item_type: &crate::documentation::extractor::ItemType) -> Severity {
        match item_type {
            crate::documentation::extractor::ItemType::Function => Severity::Warning,
            crate::documentation::extractor::ItemType::Struct => Severity::Warning,
            crate::documentation::extractor::ItemType::Interface => Severity::Warning,
            crate::documentation::extractor::ItemType::Method => Severity::Info,
            crate::documentation::extractor::ItemType::Field => Severity::Info,
            _ => Severity::Info,
        }
    }
}

impl Default for DocumentationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
