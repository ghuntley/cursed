/// Documentation Quality System
/// 
/// Provides comprehensive documentation quality analysis including linting,
/// consistency checks, grammar validation, and best practices enforcement.

use crate::error::{Error, SourceLocation};
use crate::docs::generator::{ExtractedDocumentation, DocumentationItem};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;

/// Documentation quality analyzer
#[derive(Debug)]
pub struct DocumentationQualityAnalyzer {
    /// Configuration for quality analysis
    config: QualityConfig,
    /// Grammar and spelling checker
    language_checker: LanguageChecker,
    /// Style consistency analyzer
    style_analyzer: StyleAnalyzer,
    /// Best practices checker
    best_practices_checker: BestPracticesChecker,
    /// Quality metrics calculator
    metrics_calculator: QualityMetricsCalculator,
}

/// Configuration for quality analysis
#[derive(Debug, Clone)]
pub struct QualityConfig {
    /// Enable grammar checking
    pub enable_grammar_check: bool,
    /// Enable spelling checking
    pub enable_spelling_check: bool,
    /// Enable style consistency checking
    pub enable_style_check: bool,
    /// Enable best practices checking
    pub enable_best_practices_check: bool,
    /// Minimum quality score threshold
    pub min_quality_score: f64,
    /// Language settings
    pub language_settings: LanguageSettings,
    /// Style preferences
    pub style_preferences: StylePreferences,
    /// Custom rules
    pub custom_rules: Vec<CustomRule>,
    /// Quality reporting options
    pub reporting_options: ReportingOptions,
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            enable_grammar_check: true,
            enable_spelling_check: true,
            enable_style_check: true,
            enable_best_practices_check: true,
            min_quality_score: 0.7,
            language_settings: LanguageSettings::default(),
            style_preferences: StylePreferences::default(),
            custom_rules: Vec::new(),
            reporting_options: ReportingOptions::default(),
        }
    }
}

/// Language settings for grammar and spelling
#[derive(Debug, Clone)]
pub struct LanguageSettings {
    /// Primary language code (e.g., "en-US")
    pub primary_language: String,
    /// Enable technical terminology checking
    pub enable_technical_terms: bool,
    /// Custom dictionary words
    pub custom_dictionary: HashSet<String>,
    /// Ignored words/patterns
    pub ignored_patterns: Vec<String>,
    /// Language-specific rules
    pub language_rules: Vec<LanguageRule>,
}

impl Default for LanguageSettings {
    fn default() -> Self {
        let mut custom_dict = HashSet::new();
        // Add CURSED-specific terms
        custom_dict.insert("CURSED".to_string());
        custom_dict.insert("slay".to_string());
        custom_dict.insert("yolo".to_string());
        custom_dict.insert("periodt".to_string());
        custom_dict.insert("sus".to_string());
        custom_dict.insert("facts".to_string());
        custom_dict.insert("lowkey".to_string());
        custom_dict.insert("highkey".to_string());
        custom_dict.insert("bestie".to_string());
        custom_dict.insert("flex".to_string());

        Self {
            primary_language: "en-US".to_string(),
            enable_technical_terms: true,
            custom_dictionary: custom_dict,
            ignored_patterns: vec!["\\b[A-Z]{2,}\\b".to_string()], // Ignore acronyms
            language_rules: Vec::new(),
        }
    }
}

/// Language-specific rules
#[derive(Debug, Clone)]
pub struct LanguageRule {
    pub rule_type: LanguageRuleType,
    pub pattern: String,
    pub suggestion: String,
    pub severity: Severity,
}

/// Types of language rules
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageRuleType {
    Grammar,
    Spelling,
    Style,
    Terminology,
    Punctuation,
}

/// Style preferences
#[derive(Debug, Clone)]
pub struct StylePreferences {
    /// Preferred documentation format
    pub documentation_format: DocumentationFormat,
    /// Heading style preferences
    pub heading_style: HeadingStyle,
    /// Code block preferences
    pub code_block_style: CodeBlockStyle,
    /// Link style preferences
    pub link_style: LinkStyle,
    /// Punctuation preferences
    pub punctuation_style: PunctuationStyle,
}

impl Default for StylePreferences {
    fn default() -> Self {
        Self {
            documentation_format: DocumentationFormat::JSDoc,
            heading_style: HeadingStyle::TitleCase,
            code_block_style: CodeBlockStyle::Fenced,
            link_style: LinkStyle::Inline,
            punctuation_style: PunctuationStyle::Standard,
        }
    }
}

/// Documentation format styles
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentationFormat {
    JSDoc,
    Rustdoc,
    Sphinx,
    Markdown,
    Custom(String),
}

/// Heading style preferences
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeadingStyle {
    TitleCase,
    SentenceCase,
    Lowercase,
    Uppercase,
}

/// Code block style preferences
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeBlockStyle {
    Fenced,
    Indented,
    Mixed,
}

/// Link style preferences
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkStyle {
    Inline,
    Reference,
    Mixed,
}

/// Punctuation style preferences
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PunctuationStyle {
    Standard,
    Oxford,
    Technical,
}

/// Custom quality rules
#[derive(Debug, Clone)]
pub struct CustomRule {
    pub name: String,
    pub description: String,
    pub pattern: String,
    pub replacement: Option<String>,
    pub severity: Severity,
    pub applies_to: RuleScope,
}

/// Rule severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Suggestion,
}

/// Scope where rules apply
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleScope {
    All,
    Descriptions,
    Examples,
    Parameters,
    Comments,
}

/// Reporting options
#[derive(Debug, Clone)]
pub struct ReportingOptions {
    /// Include detailed explanations
    pub include_explanations: bool,
    /// Include fix suggestions
    pub include_suggestions: bool,
    /// Group issues by type
    pub group_by_type: bool,
    /// Show context around issues
    pub show_context: bool,
    /// Maximum context lines
    pub context_lines: usize,
}

impl Default for ReportingOptions {
    fn default() -> Self {
        Self {
            include_explanations: true,
            include_suggestions: true,
            group_by_type: true,
            show_context: true,
            context_lines: 3,
        }
    }
}

/// Grammar and spelling checker
#[derive(Debug, Default)]
pub struct LanguageChecker {
    /// Grammar rules
    grammar_rules: Vec<GrammarRule>,
    /// Spelling dictionary
    spelling_dictionary: HashSet<String>,
    /// Common misspellings
    common_misspellings: HashMap<String, String>,
    /// Technical terms
    technical_terms: HashSet<String>,
}

/// Grammar rule
#[derive(Debug, Clone)]
pub struct GrammarRule {
    pub name: String,
    pub pattern: Regex,
    pub message: String,
    pub suggestion: Option<String>,
    pub severity: Severity,
}

/// Style consistency analyzer
#[derive(Debug, Default)]
pub struct StyleAnalyzer {
    /// Style patterns
    style_patterns: Vec<StylePattern>,
    /// Consistency checks
    consistency_checks: Vec<ConsistencyCheck>,
    /// Style metrics
    style_metrics: StyleMetrics,
}

/// Style pattern
#[derive(Debug, Clone)]
pub struct StylePattern {
    pub name: String,
    pub pattern: Regex,
    pub expected_style: String,
    pub severity: Severity,
}

/// Consistency check
#[derive(Debug, Clone)]
pub struct ConsistencyCheck {
    pub check_type: ConsistencyType,
    pub pattern: Regex,
    pub message: String,
    pub severity: Severity,
}

/// Types of consistency checks
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsistencyType {
    Terminology,
    Formatting,
    Structure,
    Capitalization,
    Punctuation,
}

/// Style metrics
#[derive(Debug, Clone, Default)]
pub struct StyleMetrics {
    pub heading_consistency: f64,
    pub terminology_consistency: f64,
    pub formatting_consistency: f64,
    pub overall_consistency: f64,
}

/// Best practices checker
#[derive(Debug, Default)]
pub struct BestPracticesChecker {
    /// Best practice rules
    best_practice_rules: Vec<BestPracticeRule>,
    /// Documentation guidelines
    guidelines: Vec<Guideline>,
}

/// Best practice rule
#[derive(Debug, Clone)]
pub struct BestPracticeRule {
    pub name: String,
    pub description: String,
    pub check_function: BestPracticeCheck,
    pub severity: Severity,
    pub category: BestPracticeCategory,
}

/// Best practice check function type
#[derive(Debug, Clone)]
pub enum BestPracticeCheck {
    MinLength(usize),
    MaxLength(usize),
    RequiredSections(Vec<String>),
    RequiredParameters,
    RequiredExamples,
    NoEmptyDescriptions,
    ProperCapitalization,
    EndWithPeriod,
    Custom(String), // Custom rule identifier
}

/// Best practice categories
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BestPracticeCategory {
    Structure,
    Content,
    Formatting,
    Completeness,
    Clarity,
    Maintainability,
}

/// Documentation guideline
#[derive(Debug, Clone)]
pub struct Guideline {
    pub title: String,
    pub description: String,
    pub examples: Vec<String>,
    pub category: BestPracticeCategory,
}

/// Quality metrics calculator
#[derive(Debug, Default)]
pub struct QualityMetricsCalculator {
    /// Metric weights
    metric_weights: MetricWeights,
    /// Quality thresholds
    quality_thresholds: QualityThresholds,
}

/// Weights for different quality metrics
#[derive(Debug, Clone)]
pub struct MetricWeights {
    pub grammar_weight: f64,
    pub spelling_weight: f64,
    pub style_weight: f64,
    pub completeness_weight: f64,
    pub clarity_weight: f64,
    pub consistency_weight: f64,
}

impl Default for MetricWeights {
    fn default() -> Self {
        Self {
            grammar_weight: 0.2,
            spelling_weight: 0.15,
            style_weight: 0.2,
            completeness_weight: 0.25,
            clarity_weight: 0.1,
            consistency_weight: 0.1,
        }
    }
}

/// Quality thresholds for different ratings
#[derive(Debug, Clone)]
pub struct QualityThresholds {
    pub excellent_threshold: f64,
    pub good_threshold: f64,
    pub fair_threshold: f64,
    pub poor_threshold: f64,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            excellent_threshold: 0.9,
            good_threshold: 0.7,
            fair_threshold: 0.5,
            poor_threshold: 0.3,
        }
    }
}

/// Quality analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAnalysisResult {
    /// Overall quality score
    pub overall_score: f64,
    /// Quality rating
    pub quality_rating: QualityRating,
    /// Detailed scores by category
    pub category_scores: CategoryScores,
    /// Quality issues found
    pub issues: Vec<QualityIssue>,
    /// Quality metrics
    pub metrics: QualityMetrics,
    /// Improvement suggestions
    pub suggestions: Vec<ImprovementSuggestion>,
    /// Analysis summary
    pub summary: QualitySummary,
}

/// Quality rating levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityRating {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

/// Scores by category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryScores {
    pub grammar_score: f64,
    pub spelling_score: f64,
    pub style_score: f64,
    pub completeness_score: f64,
    pub clarity_score: f64,
    pub consistency_score: f64,
}

/// Quality issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    /// Issue type
    pub issue_type: IssueType,
    /// Severity level
    pub severity: Severity,
    /// Location in documentation
    pub location: SourceLocation,
    /// Issue message
    pub message: String,
    /// Context around the issue
    pub context: Option<String>,
    /// Suggested fix
    pub suggested_fix: Option<String>,
    /// Rule that triggered this issue
    pub rule_name: Option<String>,
    /// Additional details
    pub details: HashMap<String, String>,
}

/// Types of quality issues
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueType {
    Grammar,
    Spelling,
    Style,
    BestPractice,
    Consistency,
    Completeness,
    Clarity,
    Structure,
}

/// Quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Total issues count
    pub total_issues: usize,
    /// Issues by severity
    pub issues_by_severity: HashMap<Severity, usize>,
    /// Issues by type
    pub issues_by_type: HashMap<IssueType, usize>,
    /// Documentation coverage metrics
    pub coverage_metrics: CoverageMetrics,
    /// Readability metrics
    pub readability_metrics: ReadabilityMetrics,
    /// Consistency metrics
    pub consistency_metrics: ConsistencyMetrics,
}

/// Coverage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageMetrics {
    pub documented_items: usize,
    pub total_items: usize,
    pub coverage_percentage: f64,
    pub missing_descriptions: usize,
    pub missing_parameters: usize,
    pub missing_examples: usize,
}

/// Readability metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadabilityMetrics {
    pub average_sentence_length: f64,
    pub average_word_length: f64,
    pub reading_level: ReadingLevel,
    pub complexity_score: f64,
}

/// Reading difficulty levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadingLevel {
    Elementary,
    MiddleSchool,
    HighSchool,
    College,
    Graduate,
}

/// Consistency metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyMetrics {
    pub terminology_consistency: f64,
    pub formatting_consistency: f64,
    pub style_consistency: f64,
    pub structure_consistency: f64,
}

/// Improvement suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementSuggestion {
    /// Suggestion type
    pub suggestion_type: SuggestionType,
    /// Priority level
    pub priority: Priority,
    /// Description of the improvement
    pub description: String,
    /// Affected items
    pub affected_items: Vec<String>,
    /// Expected impact
    pub expected_impact: ImpactLevel,
    /// Implementation effort
    pub implementation_effort: EffortLevel,
}

/// Types of improvement suggestions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuggestionType {
    FixGrammar,
    FixSpelling,
    ImproveStyle,
    AddMissingContent,
    ImproveConsistency,
    SimplifyLanguage,
    AddExamples,
    StructureImprovement,
}

/// Priority levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

/// Impact levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactLevel {
    High,
    Medium,
    Low,
}

/// Effort levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

/// Quality analysis summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitySummary {
    pub analysis_date: String,
    pub total_documents_analyzed: usize,
    pub overall_assessment: String,
    pub key_strengths: Vec<String>,
    pub key_weaknesses: Vec<String>,
    pub top_recommendations: Vec<String>,
    pub quality_trend: QualityTrend,
}

/// Quality trend information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityTrend {
    pub current_score: f64,
    pub previous_score: Option<f64>,
    pub trend_direction: TrendDirection,
    pub change_percentage: f64,
}

/// Trend directions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Declining,
    Stable,
    Unknown,
}

impl DocumentationQualityAnalyzer {
    /// Create a new documentation quality analyzer
    pub fn new(config: QualityConfig) -> Self {
        let mut analyzer = Self {
            config,
            language_checker: LanguageChecker::default(),
            style_analyzer: StyleAnalyzer::default(),
            best_practices_checker: BestPracticesChecker::default(),
            metrics_calculator: QualityMetricsCalculator::default(),
        };

        analyzer.initialize_checkers();
        analyzer
    }

    /// Initialize all checkers with default rules
    fn initialize_checkers(&mut self) {
        self.initialize_language_checker();
        self.initialize_style_analyzer();
        self.initialize_best_practices_checker();
    }

    /// Initialize the language checker with grammar and spelling rules
    fn initialize_language_checker(&mut self) {
        // Add basic grammar rules
        self.language_checker.grammar_rules.push(GrammarRule {
            name: "double_spaces".to_string(),
            pattern: Regex::new(r"  +").unwrap(),
            message: "Multiple consecutive spaces found".to_string(),
            suggestion: Some("Use single spaces".to_string()),
            severity: Severity::Warning,
        });

        self.language_checker.grammar_rules.push(GrammarRule {
            name: "sentence_start_lowercase".to_string(),
            pattern: Regex::new(r"(?:^|\.)\s*[a-z]").unwrap(),
            message: "Sentence should start with a capital letter".to_string(),
            suggestion: Some("Capitalize the first letter".to_string()),
            severity: Severity::Warning,
        });

        // Add common technical terms to dictionary
        let mut technical_terms = HashSet::new();
        technical_terms.insert("API".to_string());
        technical_terms.insert("HTTP".to_string());
        technical_terms.insert("JSON".to_string());
        technical_terms.insert("URL".to_string());
        technical_terms.insert("LLVM".to_string());
        technical_terms.insert("AST".to_string());
        technical_terms.insert("CURSED".to_string());

        self.language_checker.technical_terms = technical_terms;

        // Add common misspellings
        self.language_checker.common_misspellings.insert("recieve".to_string(), "receive".to_string());
        self.language_checker.common_misspellings.insert("seperate".to_string(), "separate".to_string());
        self.language_checker.common_misspellings.insert("occured".to_string(), "occurred".to_string());
        self.language_checker.common_misspellings.insert("definately".to_string(), "definitely".to_string());
    }

    /// Initialize the style analyzer
    fn initialize_style_analyzer(&mut self) {
        // Add style patterns
        self.style_analyzer.style_patterns.push(StylePattern {
            name: "heading_capitalization".to_string(),
            pattern: Regex::new(r"^#{1,6}\s+[a-z]").unwrap(),
            expected_style: "Title Case for headings".to_string(),
            severity: Severity::Info,
        });

        // Add consistency checks
        self.style_analyzer.consistency_checks.push(ConsistencyCheck {
            check_type: ConsistencyType::Terminology,
            pattern: Regex::new(r"\b(function|func|fn)\b").unwrap(),
            message: "Inconsistent function terminology".to_string(),
            severity: Severity::Info,
        });
    }

    /// Initialize the best practices checker
    fn initialize_best_practices_checker(&mut self) {
        self.best_practices_checker.best_practice_rules.push(BestPracticeRule {
            name: "minimum_description_length".to_string(),
            description: "Descriptions should be at least 10 characters long".to_string(),
            check_function: BestPracticeCheck::MinLength(10),
            severity: Severity::Warning,
            category: BestPracticeCategory::Completeness,
        });

        self.best_practices_checker.best_practice_rules.push(BestPracticeRule {
            name: "end_with_period".to_string(),
            description: "Descriptions should end with proper punctuation".to_string(),
            check_function: BestPracticeCheck::EndWithPeriod,
            severity: Severity::Info,
            category: BestPracticeCategory::Formatting,
        });

        self.best_practices_checker.best_practice_rules.push(BestPracticeRule {
            name: "required_parameters".to_string(),
            description: "Functions with parameters should document them".to_string(),
            check_function: BestPracticeCheck::RequiredParameters,
            severity: Severity::Warning,
            category: BestPracticeCategory::Completeness,
        });
    }

    /// Analyze documentation quality
    pub fn analyze_quality(&mut self, documentation: &ExtractedDocumentation) -> Result<(), Error> {
        let mut issues = Vec::new();

        // Run all quality checks
        if self.config.enable_grammar_check {
            issues.extend(self.check_grammar(documentation)?);
        }

        if self.config.enable_spelling_check {
            issues.extend(self.check_spelling(documentation)?);
        }

        if self.config.enable_style_check {
            issues.extend(self.check_style(documentation)?);
        }

        if self.config.enable_best_practices_check {
            issues.extend(self.check_best_practices(documentation)?);
        }

        // Calculate quality metrics
        let metrics = self.calculate_quality_metrics(documentation, &issues);

        // Calculate category scores
        let category_scores = self.calculate_category_scores(&issues, &metrics);

        // Calculate overall score
        let overall_score = self.calculate_overall_score(&category_scores);

        // Determine quality rating
        let quality_rating = self.determine_quality_rating(overall_score);

        // Generate improvement suggestions
        let suggestions = self.generate_improvement_suggestions(&issues, &metrics);

        // Generate summary
        let summary = self.generate_quality_summary(&issues, &metrics, overall_score);

        Ok(QualityAnalysisResult {
            overall_score,
            quality_rating,
            category_scores,
            issues,
            metrics,
            suggestions,
            summary,
        })
    }

    /// Check grammar in documentation
    fn check_grammar(&self, documentation: &ExtractedDocumentation) -> Result<(), Error> {
        let mut issues = Vec::new();

        for item in &documentation.items {
            for rule in &self.language_checker.grammar_rules {
                let text = &item.description;
                for mat in rule.pattern.find_iter(text) {
                    let context = self.extract_context(text, mat.start(), mat.end());
                    
                    issues.push(QualityIssue {
                        issue_type: IssueType::Grammar,
                        severity: rule.severity.clone(),
                        location: item.source_info.clone(),
                        message: rule.message.clone(),
                        context: Some(context),
                        suggested_fix: rule.suggestion.clone(),
                        rule_name: Some(rule.name.clone()),
                        details: HashMap::new(),
                    });
                }
            }
        }

        Ok(issues)
    }

    /// Check spelling in documentation
    fn check_spelling(&self, documentation: &ExtractedDocumentation) -> Result<(), Error> {
        let mut issues = Vec::new();

        for item in &documentation.items {
            let words = self.extract_words(&item.description);
            
            for word in words {
                // Skip if word is in technical terms or custom dictionary
                if self.config.language_settings.custom_dictionary.contains(&word) ||
                   self.language_checker.technical_terms.contains(&word) {
                    continue;
                }

                // Check for common misspellings
                if let Some(correction) = self.language_checker.common_misspellings.get(&word) {
                    issues.push(QualityIssue {
                        issue_type: IssueType::Spelling,
                        severity: Severity::Warning,
                        location: item.source_info.clone(),
                        message: format!("Possible misspelling: '{}'", word),
                        context: Some(item.description.clone()),
                        suggested_fix: Some(format!("Did you mean '{}'?", correction)),
                        rule_name: Some("common_misspelling".to_string()),
                        details: HashMap::new(),
                    });
                }

                // Simple spell check (very basic implementation)
                if !self.is_likely_correct_spelling(&word) {
                    issues.push(QualityIssue {
                        issue_type: IssueType::Spelling,
                        severity: Severity::Info,
                        location: item.source_info.clone(),
                        message: format!("Possible spelling issue: '{}'", word),
                        context: Some(item.description.clone()),
                        suggested_fix: None,
                        rule_name: Some("spell_check".to_string()),
                        details: HashMap::new(),
                    });
                }
            }
        }

        Ok(issues)
    }

    /// Check style consistency
    fn check_style(&self, documentation: &ExtractedDocumentation) -> Result<(), Error> {
        let mut issues = Vec::new();

        for item in &documentation.items {
            // Check style patterns
            for pattern in &self.style_analyzer.style_patterns {
                if pattern.pattern.is_match(&item.description) {
                    issues.push(QualityIssue {
                        issue_type: IssueType::Style,
                        severity: pattern.severity.clone(),
                        location: item.source_info.clone(),
                        message: format!("Style issue: {}", pattern.expected_style),
                        context: Some(item.description.clone()),
                        suggested_fix: None,
                        rule_name: Some(pattern.name.clone()),
                        details: HashMap::new(),
                    });
                }
            }

            // Check consistency
            for check in &self.style_analyzer.consistency_checks {
                if check.pattern.is_match(&item.description) {
                    issues.push(QualityIssue {
                        issue_type: IssueType::Consistency,
                        severity: check.severity.clone(),
                        location: item.source_info.clone(),
                        message: check.message.clone(),
                        context: Some(item.description.clone()),
                        suggested_fix: None,
                        rule_name: Some(format!("{:?}_consistency", check.check_type)),
                        details: HashMap::new(),
                    });
                }
            }
        }

        Ok(issues)
    }

    /// Check best practices
    fn check_best_practices(&self, documentation: &ExtractedDocumentation) -> Result<(), Error> {
        let mut issues = Vec::new();

        for item in &documentation.items {
            for rule in &self.best_practices_checker.best_practice_rules {
                if let Some(issue) = self.check_best_practice_rule(item, rule) {
                    issues.push(issue);
                }
            }
        }

        Ok(issues)
    }

    /// Check a specific best practice rule
    fn check_best_practice_rule(&self, item: &DocumentationItem, rule: &BestPracticeRule) -> Option<QualityIssue> {
        match &rule.check_function {
            BestPracticeCheck::MinLength(min_len) => {
                if item.description.len() < *min_len {
                    Some(QualityIssue {
                        issue_type: IssueType::BestPractice,
                        severity: rule.severity.clone(),
                        location: item.source_info.clone(),
                        message: format!("Description too short (minimum {} characters)", min_len),
                        context: Some(item.description.clone()),
                        suggested_fix: Some("Add more descriptive content".to_string()),
                        rule_name: Some(rule.name.clone()),
                        details: HashMap::new(),
                    })
                } else {
                    None
                }
            }
            BestPracticeCheck::EndWithPeriod => {
                if !item.description.trim().ends_with('.') && !item.description.trim().ends_with('!') && !item.description.trim().ends_with('?') {
                    Some(QualityIssue {
                        issue_type: IssueType::BestPractice,
                        severity: rule.severity.clone(),
                        location: item.source_info.clone(),
                        message: "Description should end with proper punctuation".to_string(),
                        context: Some(item.description.clone()),
                        suggested_fix: Some("Add a period at the end".to_string()),
                        rule_name: Some(rule.name.clone()),
                        details: HashMap::new(),
                    })
                } else {
                    None
                }
            }
            BestPracticeCheck::RequiredParameters => {
                if !item.parameters.is_empty() && item.description.is_empty() {
                    Some(QualityIssue {
                        issue_type: IssueType::BestPractice,
                        severity: rule.severity.clone(),
                        location: item.source_info.clone(),
                        message: "Functions with parameters should document them".to_string(),
                        context: None,
                        suggested_fix: Some("Add parameter documentation".to_string()),
                        rule_name: Some(rule.name.clone()),
                        details: HashMap::new(),
                    })
                } else {
                    None
                }
            }
            BestPracticeCheck::NoEmptyDescriptions => {
                if item.description.trim().is_empty() {
                    Some(QualityIssue {
                        issue_type: IssueType::Completeness,
                        severity: rule.severity.clone(),
                        location: item.source_info.clone(),
                        message: "Item is missing a description".to_string(),
                        context: None,
                        suggested_fix: Some("Add a meaningful description".to_string()),
                        rule_name: Some(rule.name.clone()),
                        details: HashMap::new(),
                    })
                } else {
                    None
                }
            }
            _ => None, // Other checks would be implemented similarly
        }
    }

    /// Calculate quality metrics
    fn calculate_quality_metrics(&self, documentation: &ExtractedDocumentation, issues: &[QualityIssue]) -> QualityMetrics {
        let total_issues = issues.len();
        
        let mut issues_by_severity = HashMap::new();
        let mut issues_by_type = HashMap::new();

        for issue in issues {
            *issues_by_severity.entry(issue.severity.clone()).or_insert(0) += 1;
            *issues_by_type.entry(issue.issue_type.clone()).or_insert(0) += 1;
        }

        let coverage_metrics = self.calculate_coverage_metrics(documentation);
        let readability_metrics = self.calculate_readability_metrics(documentation);
        let consistency_metrics = self.calculate_consistency_metrics(documentation, issues);

        QualityMetrics {
            total_issues,
            issues_by_severity,
            issues_by_type,
            coverage_metrics,
            readability_metrics,
            consistency_metrics,
        }
    }

    /// Calculate coverage metrics
    fn calculate_coverage_metrics(&self, documentation: &ExtractedDocumentation) -> CoverageMetrics {
        let total_items = documentation.items.len();
        let documented_items = documentation.items.iter()
            .filter(|item| !item.description.trim().is_empty())
            .count();

        let missing_descriptions = total_items - documented_items;
        let missing_parameters = documentation.items.iter()
            .filter(|item| !item.parameters.is_empty() && item.parameters.iter().all(|p| p.description.trim().is_empty()))
            .count();
        let missing_examples = documentation.items.iter()
            .filter(|item| item.examples.is_empty())
            .count();

        let coverage_percentage = if total_items > 0 {
            (documented_items as f64 / total_items as f64) * 100.0
        } else {
            100.0
        };

        CoverageMetrics {
            documented_items,
            total_items,
            coverage_percentage,
            missing_descriptions,
            missing_parameters,
            missing_examples,
        }
    }

    /// Calculate readability metrics
    fn calculate_readability_metrics(&self, documentation: &ExtractedDocumentation) -> ReadabilityMetrics {
        let mut total_sentences = 0;
        let mut total_words = 0;
        let mut total_word_length = 0;

        for item in &documentation.items {
            let sentences = self.count_sentences(&item.description);
            let words = self.extract_words(&item.description);
            
            total_sentences += sentences;
            total_words += words.len();
            total_word_length += words.iter().map(|w| w.len()).sum::<usize>();
        }

        let average_sentence_length = if total_sentences > 0 {
            total_words as f64 / total_sentences as f64
        } else {
            0.0
        };

        let average_word_length = if total_words > 0 {
            total_word_length as f64 / total_words as f64
        } else {
            0.0
        };

        let reading_level = self.determine_reading_level(average_sentence_length, average_word_length);
        let complexity_score = self.calculate_complexity_score(average_sentence_length, average_word_length);

        ReadabilityMetrics {
            average_sentence_length,
            average_word_length,
            reading_level,
            complexity_score,
        }
    }

    /// Calculate consistency metrics
    fn calculate_consistency_metrics(&self, documentation: &ExtractedDocumentation, issues: &[QualityIssue]) -> ConsistencyMetrics {
        let total_items = documentation.items.len() as f64;
        
        let terminology_issues = issues.iter()
            .filter(|issue| issue.issue_type == IssueType::Consistency && 
                           issue.rule_name.as_ref().map_or(false, |name| name.contains("terminology")))
            .count() as f64;

        let formatting_issues = issues.iter()
            .filter(|issue| issue.issue_type == IssueType::Style)
            .count() as f64;

        let terminology_consistency = if total_items > 0.0 {
            ((total_items - terminology_issues) / total_items).max(0.0)
        } else {
            1.0
        };

        let formatting_consistency = if total_items > 0.0 {
            ((total_items - formatting_issues) / total_items).max(0.0)
        } else {
            1.0
        };

        let style_consistency = (terminology_consistency + formatting_consistency) / 2.0;
        let structure_consistency = 0.8; // Placeholder - would analyze structural patterns

        ConsistencyMetrics {
            terminology_consistency,
            formatting_consistency,
            style_consistency,
            structure_consistency,
        }
    }

    /// Calculate category scores
    fn calculate_category_scores(&self, issues: &[QualityIssue], metrics: &QualityMetrics) -> CategoryScores {
        let total_items = metrics.coverage_metrics.total_items as f64;

        let grammar_issues = issues.iter().filter(|i| i.issue_type == IssueType::Grammar).count() as f64;
        let spelling_issues = issues.iter().filter(|i| i.issue_type == IssueType::Spelling).count() as f64;
        let style_issues = issues.iter().filter(|i| i.issue_type == IssueType::Style).count() as f64;

        let grammar_score = if total_items > 0.0 {
            ((total_items - grammar_issues) / total_items).max(0.0)
        } else {
            1.0
        };

        let spelling_score = if total_items > 0.0 {
            ((total_items - spelling_issues) / total_items).max(0.0)
        } else {
            1.0
        };

        let style_score = if total_items > 0.0 {
            ((total_items - style_issues) / total_items).max(0.0)
        } else {
            1.0
        };

        let completeness_score = metrics.coverage_metrics.coverage_percentage / 100.0;
        let clarity_score = self.calculate_clarity_score(&metrics.readability_metrics);
        let consistency_score = metrics.consistency_metrics.style_consistency;

        CategoryScores {
            grammar_score,
            spelling_score,
            style_score,
            completeness_score,
            clarity_score,
            consistency_score,
        }
    }

    /// Calculate overall quality score
    fn calculate_overall_score(&self, scores: &CategoryScores) -> f64 {
        let weights = &self.metrics_calculator.metric_weights;
        
        scores.grammar_score * weights.grammar_weight +
        scores.spelling_score * weights.spelling_weight +
        scores.style_score * weights.style_weight +
        scores.completeness_score * weights.completeness_weight +
        scores.clarity_score * weights.clarity_weight +
        scores.consistency_score * weights.consistency_weight
    }

    /// Determine quality rating from score
    fn determine_quality_rating(&self, score: f64) -> QualityRating {
        let thresholds = &self.metrics_calculator.quality_thresholds;
        
        if score >= thresholds.excellent_threshold {
            QualityRating::Excellent
        } else if score >= thresholds.good_threshold {
            QualityRating::Good
        } else if score >= thresholds.fair_threshold {
            QualityRating::Fair
        } else if score >= thresholds.poor_threshold {
            QualityRating::Poor
        } else {
            QualityRating::Critical
        }
    }

    /// Generate improvement suggestions
    fn generate_improvement_suggestions(&self, issues: &[QualityIssue], metrics: &QualityMetrics) -> Vec<ImprovementSuggestion> {
        let mut suggestions = Vec::new();

        // Grammar improvements
        let grammar_issues = issues.iter().filter(|i| i.issue_type == IssueType::Grammar).count();
        if grammar_issues > 0 {
            suggestions.push(ImprovementSuggestion {
                suggestion_type: SuggestionType::FixGrammar,
                priority: if grammar_issues > 10 { Priority::High } else { Priority::Medium },
                description: format!("Fix {} grammar issues to improve readability", grammar_issues),
                affected_items: vec![format!("{} items", grammar_issues)],
                expected_impact: if grammar_issues > 10 { ImpactLevel::High } else { ImpactLevel::Medium },
                implementation_effort: EffortLevel::Low,
            });
        }

        // Spelling improvements
        let spelling_issues = issues.iter().filter(|i| i.issue_type == IssueType::Spelling).count();
        if spelling_issues > 0 {
            suggestions.push(ImprovementSuggestion {
                suggestion_type: SuggestionType::FixSpelling,
                priority: Priority::Medium,
                description: format!("Fix {} potential spelling issues", spelling_issues),
                affected_items: vec![format!("{} items", spelling_issues)],
                expected_impact: ImpactLevel::Medium,
                implementation_effort: EffortLevel::Low,
            });
        }

        // Completeness improvements
        if metrics.coverage_metrics.missing_descriptions > 0 {
            suggestions.push(ImprovementSuggestion {
                suggestion_type: SuggestionType::AddMissingContent,
                priority: Priority::High,
                description: format!("Add descriptions to {} undocumented items", metrics.coverage_metrics.missing_descriptions),
                affected_items: vec![format!("{} items", metrics.coverage_metrics.missing_descriptions)],
                expected_impact: ImpactLevel::High,
                implementation_effort: EffortLevel::Medium,
            });
        }

        // Style consistency improvements
        let style_issues = issues.iter().filter(|i| i.issue_type == IssueType::Style || i.issue_type == IssueType::Consistency).count();
        if style_issues > 0 {
            suggestions.push(ImprovementSuggestion {
                suggestion_type: SuggestionType::ImproveConsistency,
                priority: Priority::Low,
                description: format!("Improve style consistency by addressing {} issues", style_issues),
                affected_items: vec![format!("{} items", style_issues)],
                expected_impact: ImpactLevel::Medium,
                implementation_effort: EffortLevel::Low,
            });
        }

        suggestions
    }

    /// Generate quality summary
    fn generate_quality_summary(&self, issues: &[QualityIssue], metrics: &QualityMetrics, overall_score: f64) -> QualitySummary {
        let mut key_strengths = Vec::new();
        let mut key_weaknesses = Vec::new();
        let mut top_recommendations = Vec::new();

        // Analyze strengths
        if metrics.coverage_metrics.coverage_percentage > 80.0 {
            key_strengths.push("Good documentation coverage".to_string());
        }
        if issues.iter().filter(|i| i.issue_type == IssueType::Grammar).count() < 5 {
            key_strengths.push("Few grammar issues".to_string());
        }
        if metrics.consistency_metrics.style_consistency > 0.8 {
            key_strengths.push("Consistent formatting style".to_string());
        }

        // Analyze weaknesses
        if metrics.coverage_metrics.missing_descriptions > 10 {
            key_weaknesses.push("Many items lack descriptions".to_string());
        }
        if issues.iter().filter(|i| i.severity == Severity::Error).count() > 0 {
            key_weaknesses.push("Critical issues present".to_string());
        }
        if metrics.readability_metrics.complexity_score > 0.8 {
            key_weaknesses.push("Complex language may hinder understanding".to_string());
        }

        // Top recommendations
        top_recommendations.push("Focus on adding missing descriptions first".to_string());
        if issues.len() > 20 {
            top_recommendations.push("Prioritize fixing high-severity issues".to_string());
        }
        top_recommendations.push("Consider using automated formatting tools".to_string());

        let overall_assessment = match self.determine_quality_rating(overall_score) {
            QualityRating::Excellent => "Documentation quality is excellent with minimal issues",
            QualityRating::Good => "Documentation quality is good with some areas for improvement",
            QualityRating::Fair => "Documentation quality is fair and would benefit from focused improvements",
            QualityRating::Poor => "Documentation quality needs significant improvement",
            QualityRating::Critical => "Documentation quality requires immediate attention",
        };

        QualitySummary {
            analysis_date: chrono::Utc::now().to_rfc3339(),
            total_documents_analyzed: metrics.coverage_metrics.total_items,
            overall_assessment: overall_assessment.to_string(),
            key_strengths,
            key_weaknesses,
            top_recommendations,
            quality_trend: QualityTrend {
                current_score: overall_score,
                previous_score: None, // Would be populated from historical data
                trend_direction: TrendDirection::Unknown,
                change_percentage: 0.0,
            },
        }
    }

    /// Generate quality report
    pub fn generate_quality_report(&self, result: &QualityAnalysisResult, output_path: &Path) -> Result<(), Error> {
        let report_content = self.generate_html_quality_report(result);
        fs::write(output_path, report_content)
            .map_err(|e| Error::SystemError(format!("Failed to write quality report: {}", e)))
    }

    /// Generate HTML quality report
    fn generate_html_quality_report(&self, result: &QualityAnalysisResult) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Documentation Quality Report</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 40px; line-height: 1.6; }}
        .header {{ background: #f8f9fa; padding: 20px; border-radius: 8px; margin-bottom: 30px; }}
        .quality-score {{ text-align: center; margin: 20px 0; }}
        .score-circle {{ width: 120px; height: 120px; border-radius: 50%; display: inline-flex; align-items: center; justify-content: center; font-size: 24px; font-weight: bold; color: white; margin: 10px; }}
        .excellent {{ background: #28a745; }}
        .good {{ background: #ffc107; }}
        .fair {{ background: #fd7e14; }}
        .poor {{ background: #dc3545; }}
        .critical {{ background: #6f42c1; }}
        .metrics-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 20px 0; }}
        .metric-card {{ background: white; border: 1px solid #e9ecef; border-radius: 8px; padding: 20px; }}
        .metric-score {{ font-size: 2em; font-weight: bold; color: #007bff; }}
        .issues-section {{ margin: 30px 0; }}
        .issue {{ background: #f8f9fa; border-left: 4px solid #007bff; padding: 15px; margin: 10px 0; }}
        .issue.error {{ border-left-color: #dc3545; }}
        .issue.warning {{ border-left-color: #ffc107; }}
        .issue.info {{ border-left-color: #17a2b8; }}
        .suggestions {{ background: #d4edda; border: 1px solid #c3e6cb; border-radius: 5px; padding: 15px; margin: 10px 0; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Documentation Quality Report</h1>
        <p>Comprehensive analysis of documentation quality for CURSED project</p>
        <p>Generated: {}</p>
    </div>

    <div class="quality-score">
        <h2>Overall Quality Score</h2>
        <div class="score-circle {}">
            {:.1}%
        </div>
        <h3>{:?}</h3>
    </div>

    <div class="metrics-grid">
        <div class="metric-card">
            <div class="metric-score">{:.1}%</div>
            <div>Grammar Score</div>
        </div>
        <div class="metric-card">
            <div class="metric-score">{:.1}%</div>
            <div>Spelling Score</div>
        </div>
        <div class="metric-card">
            <div class="metric-score">{:.1}%</div>
            <div>Style Score</div>
        </div>
        <div class="metric-card">
            <div class="metric-score">{:.1}%</div>
            <div>Completeness Score</div>
        </div>
        <div class="metric-card">
            <div class="metric-score">{:.1}%</div>
            <div>Clarity Score</div>
        </div>
        <div class="metric-card">
            <div class="metric-score">{:.1}%</div>
            <div>Consistency Score</div>
        </div>
    </div>

    <div class="issues-section">
        <h2>Quality Issues ({} total)</h2>
        {}
    </div>

    <div class="suggestions">
        <h2>Improvement Suggestions</h2>
        {}
    </div>

    <div>
        <h2>Summary</h2>
        <p><strong>Overall Assessment:</strong> {}</p>
        <h3>Key Strengths:</h3>
        <ul>{}</ul>
        <h3>Key Weaknesses:</h3>
        <ul>{}</ul>
        <h3>Top Recommendations:</h3>
        <ul>{}</ul>
    </div>
</body>
</html>"#,
            result.summary.analysis_date,
            match result.quality_rating {
                QualityRating::Excellent => "excellent",
                QualityRating::Good => "good",
                QualityRating::Fair => "fair",
                QualityRating::Poor => "poor",
                QualityRating::Critical => "critical",
            },
            result.overall_score * 100.0,
            result.quality_rating,
            result.category_scores.grammar_score * 100.0,
            result.category_scores.spelling_score * 100.0,
            result.category_scores.style_score * 100.0,
            result.category_scores.completeness_score * 100.0,
            result.category_scores.clarity_score * 100.0,
            result.category_scores.consistency_score * 100.0,
            result.issues.len(),
            self.generate_issues_html(&result.issues),
            self.generate_suggestions_html(&result.suggestions),
            result.summary.overall_assessment,
            result.summary.key_strengths.iter().map(|s| format!("<li>{}</li>", s)).collect::<Vec<_>>().join(""),
            result.summary.key_weaknesses.iter().map(|s| format!("<li>{}</li>", s)).collect::<Vec<_>>().join(""),
            result.summary.top_recommendations.iter().map(|s| format!("<li>{}</li>", s)).collect::<Vec<_>>().join("")
        )
    }

    /// Generate HTML for issues
    fn generate_issues_html(&self, issues: &[QualityIssue]) -> String {
        issues.iter()
            .take(20) // Limit to first 20 issues
            .map(|issue| {
                let severity_class = match issue.severity {
                    Severity::Error => "error",
                    Severity::Warning => "warning",
                    _ => "info",
                };
                
                format!(
                    r#"<div class="issue {}">
                        <strong>{:?}</strong>: {}
                        <br><small>Location: {}:{}</small>
                        {}
                    </div>"#,
                    severity_class,
                    issue.issue_type,
                    issue.message,
                    issue.location.file.as_ref().map(|f| f.display().to_string()).unwrap_or_else(|| "unknown".to_string()),
                    issue.location.line,
                    issue.suggested_fix.as_ref().map(|fix| format!("<br><em>Suggestion: {}</em>", fix)).unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Generate HTML for suggestions
    fn generate_suggestions_html(&self, suggestions: &[ImprovementSuggestion]) -> String {
        suggestions.iter()
            .map(|suggestion| {
                format!(
                    r#"<div>
                        <strong>{:?}</strong> (Priority: {:?}): {}
                        <br><small>Expected Impact: {:?} | Effort: {:?}</small>
                    </div>"#,
                    suggestion.suggestion_type,
                    suggestion.priority,
                    suggestion.description,
                    suggestion.expected_impact,
                    suggestion.implementation_effort
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    // Helper methods

    /// Extract context around a text position
    fn extract_context(&self, text: &str, start: usize, end: usize) -> String {
        let context_start = if start >= 20 { start - 20 } else { 0 };
        let context_end = if end + 20 < text.len() { end + 20 } else { text.len() };
        text[context_start..context_end].to_string()
    }

    /// Extract words from text
    fn extract_words(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase())
            .filter(|word| !word.is_empty())
            .collect()
    }

    /// Simple spelling check
    fn is_likely_correct_spelling(&self, word: &str) -> bool {
        // Very basic heuristic - check if word has reasonable letter patterns
        word.len() > 1 && 
        word.chars().any(|c| "aeiouAEIOU".contains(c)) &&
        !word.chars().any(|c| c.is_numeric())
    }

    /// Count sentences in text
    fn count_sentences(&self, text: &str) -> usize {
        text.matches(&['.', '!', '?'][..]).count().max(1)
    }

    /// Determine reading level
    fn determine_reading_level(&self, avg_sentence_length: f64, avg_word_length: f64) -> ReadingLevel {
        // Simplified Flesch reading ease calculation
        let complexity = avg_sentence_length + (avg_word_length * 2.0);
        
        if complexity < 8.0 {
            ReadingLevel::Elementary
        } else if complexity < 12.0 {
            ReadingLevel::MiddleSchool
        } else if complexity < 16.0 {
            ReadingLevel::HighSchool
        } else if complexity < 20.0 {
            ReadingLevel::College
        } else {
            ReadingLevel::Graduate
        }
    }

    /// Calculate complexity score
    fn calculate_complexity_score(&self, avg_sentence_length: f64, avg_word_length: f64) -> f64 {
        ((avg_sentence_length + avg_word_length) / 20.0).min(1.0)
    }

    /// Calculate clarity score
    fn calculate_clarity_score(&self, readability: &ReadabilityMetrics) -> f64 {
        // Score based on readability - simpler text gets higher score
        match readability.reading_level {
            ReadingLevel::Elementary => 1.0,
            ReadingLevel::MiddleSchool => 0.9,
            ReadingLevel::HighSchool => 0.8,
            ReadingLevel::College => 0.6,
            ReadingLevel::Graduate => 0.4,
        }
    }
}
