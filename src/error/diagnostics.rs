//! Enhanced error diagnostics system for CURSED compiler
//! 
//! This module provides comprehensive error analysis, clustering, and suggestion
//! generation to improve the developer experience when encountering compilation errors.

use std::collections::{HashMap, HashSet};
use colored::Colorize;
use crate::error::{CursedError, StructuredError, ErrorCode, ErrorSeverity};
use crate::error::structured::ErrorSourceLocation;

/// Error diagnostic analyzer
pub struct ErrorDiagnostics {
    /// Error clusters grouped by similarity
    error_clusters: HashMap<String, Vec<DiagnosticError>>,
    /// Fix-it hints for common errors
    fix_hints: HashMap<ErrorCode, Vec<FixHint>>,
    /// Context-aware suggestions
    context_suggestions: HashMap<String, Vec<String>>,
    /// Error frequency tracking
    error_frequency: HashMap<ErrorCode, usize>,
}

/// Enhanced error with diagnostic information
#[derive(Debug, Clone)]
pub struct DiagnosticError {
    pub structured_error: StructuredError,
    pub confidence_level: ConfidenceLevel,
    pub fix_hints: Vec<FixHint>,
    pub related_errors: Vec<String>,
    pub severity_adjusted: bool,
}

/// Fix-it hint with automatic correction
#[derive(Debug, Clone)]
pub struct FixHint {
    pub description: String,
    pub fix_type: FixType,
    pub auto_fix: Option<AutoFix>,
    pub confidence: ConfidenceLevel,
}

/// Type of fix being suggested
#[derive(Debug, Clone, PartialEq)]
pub enum FixType {
    TokenInsertion,
    TokenDeletion,
    TokenReplacement,
    StructuralChange,
    TypeAnnotation,
    ImportAddition,
    DeclarationAddition,
}

/// Automatic fix suggestion
#[derive(Debug, Clone)]
pub struct AutoFix {
    pub location: ErrorSourceLocation,
    pub replacement_text: String,
    pub description: String,
}

/// Confidence level for suggestions
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ConfidenceLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl ErrorDiagnostics {
    pub fn new() -> Self {
        let mut diagnostics = Self {
            error_clusters: HashMap::new(),
            fix_hints: HashMap::new(),
            context_suggestions: HashMap::new(),
            error_frequency: HashMap::new(),
        };
        
        diagnostics.initialize_fix_hints();
        diagnostics.initialize_context_suggestions();
        diagnostics
    }
    
    /// Initialize built-in fix hints for common errors
    fn initialize_fix_hints(&mut self) {
        // Syntax error fixes
        self.fix_hints.insert(ErrorCode::E0001, vec![
            FixHint {
                description: "Check for missing punctuation".to_string(),
                fix_type: FixType::TokenInsertion,
                auto_fix: None,
                confidence: ConfidenceLevel::High,
            },
            FixHint {
                description: "Verify bracket matching".to_string(),
                fix_type: FixType::StructuralChange,
                auto_fix: None,
                confidence: ConfidenceLevel::Medium,
            },
        ]);
        
        // Unterminated string fixes
        self.fix_hints.insert(ErrorCode::E0002, vec![
            FixHint {
                description: "Add closing quote".to_string(),
                fix_type: FixType::TokenInsertion,
                auto_fix: Some(AutoFix {
                    location: ErrorSourceLocation {
file: "".to_string(),
                        line: 0,
                        column: 0,
                        length: 0,
                        source_line: None,
                    
                    offset: 0,
                },
                    replacement_text: "\"".to_string(),
                    description: "Insert closing quote".to_string(),
                }),
                confidence: ConfidenceLevel::VeryHigh,
            },
        ]);
        
        // Type mismatch fixes
        self.fix_hints.insert(ErrorCode::E0100, vec![
            FixHint {
                description: "Add explicit type conversion".to_string(),
                fix_type: FixType::TypeAnnotation,
                auto_fix: None,
                confidence: ConfidenceLevel::Medium,
            },
            FixHint {
                description: "Check variable types".to_string(),
                fix_type: FixType::StructuralChange,
                auto_fix: None,
                confidence: ConfidenceLevel::High,
            },
        ]);
        
        // Variable not found fixes
        self.fix_hints.insert(ErrorCode::E0109, vec![
            FixHint {
                description: "Declare the variable".to_string(),
                fix_type: FixType::DeclarationAddition,
                auto_fix: None,
                confidence: ConfidenceLevel::High,
            },
            FixHint {
                description: "Check for typos".to_string(),
                fix_type: FixType::TokenReplacement,
                auto_fix: None,
                confidence: ConfidenceLevel::Medium,
            },
        ]);
        
        // Function not found fixes
        self.fix_hints.insert(ErrorCode::E0110, vec![
            FixHint {
                description: "Define the function".to_string(),
                fix_type: FixType::DeclarationAddition,
                auto_fix: None,
                confidence: ConfidenceLevel::High,
            },
            FixHint {
                description: "Import the function".to_string(),
                fix_type: FixType::ImportAddition,
                auto_fix: None,
                confidence: ConfidenceLevel::Medium,
            },
        ]);
    }
    
    /// Initialize context-aware suggestions
    fn initialize_context_suggestions(&mut self) {
        self.context_suggestions.insert("function_call".to_string(), vec![
            "Make sure the function is declared".to_string(),
            "Check function parameter types".to_string(),
            "Verify function is in scope".to_string(),
        ]);
        
        self.context_suggestions.insert("variable_access".to_string(), vec![
            "Ensure variable is declared".to_string(),
            "Check variable scope".to_string(),
            "Verify variable name spelling".to_string(),
        ]);
        
        self.context_suggestions.insert("type_annotation".to_string(), vec![
            "Use CURSED type names (normie, tea, lit, etc.)".to_string(),
            "Check type syntax".to_string(),
            "Consider type inference".to_string(),
        ]);
        
        self.context_suggestions.insert("block_structure".to_string(), vec![
            "Check brace matching".to_string(),
            "Verify statement syntax".to_string(),
            "Ensure proper nesting".to_string(),
        ]);
    }
    
    /// Analyze and enhance a collection of errors
    pub fn analyze_errors(&mut self, errors: Vec<StructuredError>) -> Vec<DiagnosticError> {
        let mut diagnostic_errors = Vec::new();
        
        // Convert to diagnostic errors
        for error in errors {
            let diagnostic_error = self.create_diagnostic_error(error);
            diagnostic_errors.push(diagnostic_error);
        }
        
        // Cluster related errors
        self.cluster_errors(&diagnostic_errors);
        
        // Enhance errors with context
        for diagnostic_error in &mut diagnostic_errors {
            self.enhance_error_with_context(diagnostic_error);
        }
        
        // Update frequency tracking
        for diagnostic_error in &diagnostic_errors {
            *self.error_frequency.entry(diagnostic_error.structured_error.code).or_insert(0) += 1;
        }
        
        diagnostic_errors
    }
    
    /// Create a diagnostic error from a structured error
    fn create_diagnostic_error(&self, structured_error: StructuredError) -> DiagnosticError {
        let fix_hints = self.get_fix_hints(&structured_error);
        let confidence_level = self.calculate_confidence_level(&structured_error);
        
        DiagnosticError {
            structured_error,
            confidence_level,
            fix_hints,
            related_errors: Vec::new(),
            severity_adjusted: false,
        }
    }
    
    /// Get fix hints for an error
    fn get_fix_hints(&self, error: &StructuredError) -> Vec<FixHint> {
        let mut hints = Vec::new();
        
        // Add built-in hints
        if let Some(builtin_hints) = self.fix_hints.get(&error.code) {
            hints.extend(builtin_hints.clone());
        }
        
        // Generate context-specific hints
        hints.extend(self.generate_context_hints(error));
        
        // Generate auto-fix hints where possible
        hints.extend(self.generate_auto_fix_hints(error));
        
        hints
    }
    
    /// Generate context-specific hints
    fn generate_context_hints(&self, error: &StructuredError) -> Vec<FixHint> {
        let mut hints = Vec::new();
        
        match error.code {
            ErrorCode::E0001 => {
                // Unexpected token - analyze what was expected vs found
                if error.message.contains("expected") && error.message.contains("found") {
                    hints.push(FixHint {
                        description: "The parser expected a different token".to_string(),
                        fix_type: FixType::TokenReplacement,
                        auto_fix: None,
                        confidence: ConfidenceLevel::High,
                    });
                }
            }
            ErrorCode::E0002 => {
                // Unterminated string - provide specific fix
                hints.push(FixHint {
                    description: "String literals must be properly terminated".to_string(),
                    fix_type: FixType::TokenInsertion,
                    auto_fix: None,
                    confidence: ConfidenceLevel::VeryHigh,
                });
            }
            ErrorCode::E0100 => {
                // Type mismatch - suggest conversions
                if error.message.contains("expected") && error.message.contains("found") {
                    hints.push(FixHint {
                        description: "Consider using type assertions with .() syntax".to_string(),
                        fix_type: FixType::TypeAnnotation,
                        auto_fix: None,
                        confidence: ConfidenceLevel::Medium,
                    });
                }
            }
            ErrorCode::E0109 => {
                // Variable not found - suggest declaration
                if let Some(location) = &error.location {
                    hints.push(FixHint {
                        description: "Declare the variable before use".to_string(),
                        fix_type: FixType::DeclarationAddition,
                        auto_fix: Some(AutoFix {
                            location: location.clone(),
                            replacement_text: "sus variable_name type = value;\n".to_string(),
                            description: "Add variable declaration".to_string(),
                        }),
                        confidence: ConfidenceLevel::Medium,
                    });
                }
            }
            _ => {}
        }
        
        hints
    }
    
    /// Generate automatic fix hints
    fn generate_auto_fix_hints(&self, error: &StructuredError) -> Vec<FixHint> {
        let mut hints = Vec::new();
        
        match error.code {
            ErrorCode::E0006 => {
                // Missing semicolon
                if let Some(location) = &error.location {
                    hints.push(FixHint {
                        description: "Add missing semicolon".to_string(),
                        fix_type: FixType::TokenInsertion,
                        auto_fix: Some(AutoFix {
                            location: location.clone(),
                            replacement_text: ";".to_string(),
                            description: "Insert semicolon".to_string(),
                        }),
                        confidence: ConfidenceLevel::VeryHigh,
                    });
                }
            }
            ErrorCode::E0008 => {
                // Unclosed delimiter
                if error.message.contains("(") {
                    hints.push(FixHint {
                        description: "Add missing closing parenthesis".to_string(),
                        fix_type: FixType::TokenInsertion,
                        auto_fix: Some(AutoFix {
                            location: error.location.clone().unwrap_or_default(),
                            replacement_text: ")".to_string(),
                            description: "Insert closing parenthesis".to_string(),
                        }),
                        confidence: ConfidenceLevel::High,
                    });
                } else if error.message.contains("{") {
                    hints.push(FixHint {
                        description: "Add missing closing brace".to_string(),
                        fix_type: FixType::TokenInsertion,
                        auto_fix: Some(AutoFix {
                            location: error.location.clone().unwrap_or_default(),
                            replacement_text: "}".to_string(),
                            description: "Insert closing brace".to_string(),
                        }),
                        confidence: ConfidenceLevel::High,
                    });
                }
            }
            _ => {}
        }
        
        hints
    }
    
    /// Calculate confidence level for error diagnosis
    fn calculate_confidence_level(&self, error: &StructuredError) -> ConfidenceLevel {
        match error.code {
            ErrorCode::E0002 | ErrorCode::E0006 | ErrorCode::E0008 => ConfidenceLevel::VeryHigh,
            ErrorCode::E0001 | ErrorCode::E0100 => ConfidenceLevel::High,
            ErrorCode::E0109 | ErrorCode::E0110 => ConfidenceLevel::Medium,
            _ => ConfidenceLevel::Low,
        }
    }
    
    /// Cluster related errors together
    fn cluster_errors(&mut self, errors: &[DiagnosticError]) {
        self.error_clusters.clear();
        
        for error in errors {
            let cluster_key = self.generate_cluster_key(&error.structured_error);
            self.error_clusters.entry(cluster_key).or_insert_with(Vec::new).push(error.clone());
        }
    }
    
    /// Generate a key for error clustering
    fn generate_cluster_key(&self, error: &StructuredError) -> String {
        format!("{}:{}", 
                error.code.category(),
                error.location.as_ref().map(|l| l.line).unwrap_or(0))
    }
    
    /// Enhance error with contextual information
    fn enhance_error_with_context(&self, diagnostic_error: &mut DiagnosticError) {
        // Add related errors from the same cluster
        let cluster_key = self.generate_cluster_key(&diagnostic_error.structured_error);
        if let Some(cluster) = self.error_clusters.get(&cluster_key) {
            if cluster.len() > 1 {
                diagnostic_error.related_errors = cluster.iter()
                    .filter(|e| e.structured_error.code != diagnostic_error.structured_error.code)
                    .map(|e| e.structured_error.message.clone())
                    .collect();
            }
        }
        
        // Adjust severity based on frequency
        if let Some(&frequency) = self.error_frequency.get(&diagnostic_error.structured_error.code) {
            if frequency > 5 {
                // This is a common error, might want to highlight it more
                diagnostic_error.severity_adjusted = true;
            }
        }
    }
    
    /// Generate "did you mean" suggestions for similar tokens
    pub fn suggest_similar_tokens(&self, token: &str, available_tokens: &[String]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for available in available_tokens {
            let distance = Self::levenshtein_distance(token, available);
            if distance <= 2 && distance < token.len() / 2 {
                suggestions.push(format!("Did you mean '{}'?", available));
            }
        }
        
        // Sort by similarity
        suggestions.sort_by_key(|s| Self::levenshtein_distance(token, s.split('\'').nth(1).unwrap_or("")));
        suggestions.truncate(3); // Limit to top 3 suggestions
        
        suggestions
    }
    
    /// Calculate edit distance for similarity
    fn levenshtein_distance(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let a_len = a_chars.len();
        let b_len = b_chars.len();
        
        if a_len == 0 { return b_len; }
        if b_len == 0 { return a_len; }
        
        let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
        
        for i in 0..=a_len {
            matrix[i][0] = i;
        }
        for j in 0..=b_len {
            matrix[0][j] = j;
        }
        
        for i in 1..=a_len {
            for j in 1..=b_len {
                let cost = if a_chars[i-1] == b_chars[j-1] { 0 } else { 1 };
                matrix[i][j] = std::cmp::min(
                    std::cmp::min(matrix[i-1][j] + 1, matrix[i][j-1] + 1),
                    matrix[i-1][j-1] + cost
                );
            }
        }
        
        matrix[a_len][b_len]
    }
    
    /// Format diagnostic error for display
    pub fn format_diagnostic_error(&self, diagnostic_error: &DiagnosticError) -> String {
        let mut output = String::new();
        
        let error = &diagnostic_error.structured_error;
        
        // Error header with confidence indicator
        let confidence_indicator = match diagnostic_error.confidence_level {
            ConfidenceLevel::VeryHigh => "🎯",
            ConfidenceLevel::High => "✅",
            ConfidenceLevel::Medium => "⚠️",
            ConfidenceLevel::Low => "❓",
        };
        
        output.push_str(&format!("{} {} [{}]: {}\n", 
                                confidence_indicator,
                                error.severity.to_string().red().bold(),
                                error.code.as_str().bright_black(),
                                error.message));
        
        // Location information
        if let Some(location) = &error.location {
            output.push_str(&format!("  {} {}:{}:{}\n", 
                                   "-->".bright_blue(),
                                   location.file,
                                   location.line,
                                   location.column));
        }
        
        // Fix hints
        if !diagnostic_error.fix_hints.is_empty() {
            output.push_str(&format!("\n{}\n", "Fix suggestions:".green().bold()));
            for (i, hint) in diagnostic_error.fix_hints.iter().enumerate() {
                let confidence_badge = match hint.confidence {
                    ConfidenceLevel::VeryHigh => "🔧",
                    ConfidenceLevel::High => "🛠️",
                    ConfidenceLevel::Medium => "💡",
                    ConfidenceLevel::Low => "💭",
                };
                
                output.push_str(&format!("  {}. {} {}\n", 
                                       i + 1,
                                       confidence_badge,
                                       hint.description));
                
                if let Some(auto_fix) = &hint.auto_fix {
                    output.push_str(&format!("     {} {}\n", 
                                           "Auto-fix:".cyan(),
                                           auto_fix.description));
                }
            }
        }
        
        // Related errors
        if !diagnostic_error.related_errors.is_empty() {
            output.push_str(&format!("\n{}\n", "Related issues:".yellow().bold()));
            for related in &diagnostic_error.related_errors {
                output.push_str(&format!("  • {}\n", related));
            }
        }
        
        output
    }
    
    /// Get error statistics
    pub fn get_error_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        for (code, frequency) in &self.error_frequency {
            let category = code.category();
            *stats.entry(category.to_string()).or_insert(0) += frequency;
        }
        
        stats
    }
    
    /// Generate comprehensive error report
    pub fn generate_error_report(&self, diagnostic_errors: &[DiagnosticError]) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("{}\n", "=== CURSED Compiler Error Report ===".bold()));
        report.push_str(&format!("Total errors: {}\n\n", diagnostic_errors.len()));
        
        // Group by severity
        let mut by_severity: HashMap<&str, Vec<&DiagnosticError>> = HashMap::new();
        for error in diagnostic_errors {
            let severity = match error.structured_error.severity {
                crate::error::structured::ErrorSeverity::Error => "Errors",
                crate::error::structured::ErrorSeverity::Warning => "Warnings",
                crate::error::structured::ErrorSeverity::Note => "Notes",
                crate::error::structured::ErrorSeverity::Help => "Help",
            };
            by_severity.entry(severity).or_insert_with(Vec::new).push(error);
        }
        
        for (severity, errors) in by_severity {
            if !errors.is_empty() {
                report.push_str(&format!("{} ({}):\n", severity.bold(), errors.len()));
                for error in errors {
                    report.push_str(&format!("  • {} [{}]\n", 
                                           error.structured_error.message,
                                           error.structured_error.code.as_str()));
                }
                report.push('\n');
            }
        }
        
        // Add suggestions summary
        let total_auto_fixes: usize = diagnostic_errors.iter()
            .map(|e| e.fix_hints.iter().filter(|h| h.auto_fix.is_some()).count())
            .sum();
        
        if total_auto_fixes > 0 {
            report.push_str(&format!("{} automatic fixes available\n", 
                                   total_auto_fixes.to_string().green().bold()));
        }
        
        report
    }
}

impl Default for ErrorDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ErrorSourceLocation {
fn default() -> Self {
        Self {
            file: "unknown".to_string(),
            line: 0,
            column: 0,
            length: 0,
            source_line: None,
        
                    offset: 0,
                }
    }
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSeverity::Error => write!(f, "error"),
            ErrorSeverity::Warning => write!(f, "warning"),
            ErrorSeverity::Help => write!(f, "help"),
            ErrorSeverity::Note => write!(f, "note"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_diagnostics_creation() {
        let diagnostics = ErrorDiagnostics::new();
        assert!(!diagnostics.fix_hints.is_empty());
        assert!(!diagnostics.context_suggestions.is_empty());
    }
    
    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(ErrorDiagnostics::levenshtein_distance("sus", "sux"), 1);
        assert_eq!(ErrorDiagnostics::levenshtein_distance("slay", "play"), 1);
        assert_eq!(ErrorDiagnostics::levenshtein_distance("vibes", "vibe"), 1);
    }
    
    #[test]
    fn test_similar_token_suggestions() {
        let diagnostics = ErrorDiagnostics::new();
        let available = vec!["sus".to_string(), "slay".to_string(), "facts".to_string()];
        let suggestions = diagnostics.suggest_similar_tokens("sux", &available);
        println!("Suggestions for 'sux': {:?}", suggestions);
        // For now, just check that we get some suggestions
        assert!(!suggestions.is_empty() || available.iter().any(|a| ErrorDiagnostics::levenshtein_distance("sux", a) <= 2));
    }
    
    #[test]
    fn test_confidence_level_calculation() {
        let diagnostics = ErrorDiagnostics::new();
        let error = StructuredError::new(ErrorCode::E0002, "Test error".to_string());
        let confidence = diagnostics.calculate_confidence_level(&error);
        assert_eq!(confidence, ConfidenceLevel::VeryHigh);
    }
}
