//! Intelligent Optimization Recommendations System for CURSED
//! 
//! This module provides comprehensive code analysis and generates intelligent
//! optimization recommendations based on CURSED source code patterns and constructs.

use crate::ast::*;
use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::codegen::llvm::optimization::{OptimizationConfig, OptimizationLevel};
use crate::error::Result;
use crate::optimization::ast_analyzer::AdvancedASTAnalyzer;
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, warn, instrument};

/// Main code analysis engine that generates optimization recommendations
#[derive(Debug)]
pub struct CodeAnalysisEngine {
    /// Configuration for analysis thresholds and settings
    pub config: AnalysisConfig,
    /// Cache of analyzed patterns for efficiency
    analysis_cache: HashMap<String, Vec<AnalysisPattern>>,
}

/// Configuration for code analysis behavior
#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    /// Maximum function size before recommending decomposition
    pub max_function_size: usize,
    /// Maximum loop nesting before warning
    pub max_loop_nesting: usize,
    /// Minimum iterations before recommending loop optimization
    pub loop_optimization_threshold: usize,
    /// Maximum number of function parameters before warning
    pub max_function_parameters: usize,
    /// Enable advanced pattern analysis
    pub enable_advanced_analysis: bool,
    /// Enable memory usage analysis
    pub enable_memory_analysis: bool,
    /// Enable performance bottleneck detection
    pub enable_performance_analysis: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            max_function_size: 50,
            max_loop_nesting: 3,
            loop_optimization_threshold: 100,
            max_function_parameters: 6,
            enable_advanced_analysis: true,
            enable_memory_analysis: true,
            enable_performance_analysis: true,
        }
    }
}

/// Represents a detected code pattern that could benefit from optimization
#[derive(Debug, Clone)]
pub struct AnalysisPattern {
    /// Type of pattern detected
    pub pattern_type: PatternType,
    /// Location in source code
    pub location: SourceLocation,
    /// Severity of the optimization opportunity
    pub severity: PatternSeverity,
    /// Description of the pattern
    pub description: String,
    /// Estimated performance impact
    pub performance_impact: PerformanceImpact,
}

/// Types of code patterns that can be optimized
#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    /// Heavy nested loops that could benefit from optimization
    NestedLoops,
    /// Functions that are good candidates for inlining
    InlineCandidate,
    /// Memory allocation patterns that could be optimized
    MemoryAllocation,
    /// Complex expressions that could be simplified
    ComplexExpression,
    /// Repeated computations that could be cached
    RepeatedComputation,
    /// Large functions that should be decomposed
    LargeFunction,
    /// Deep recursion that could benefit from optimization
    DeepRecursion,
    /// Channel operations that could be optimized
    ChannelOperations,
    /// Concurrent patterns with optimization potential
    ConcurrencyPattern,
    /// String operations that could be optimized
    StringOperations,
    /// Collection operations with optimization potential
    CollectionOperations,
    /// Error handling patterns that could be optimized
    ErrorHandling,
}

/// Severity levels for optimization patterns
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PatternSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Estimated performance impact of applying optimization
#[derive(Debug, Clone)]
pub struct PerformanceImpact {
    /// Expected runtime improvement percentage
    pub runtime_improvement: f64,
    /// Expected memory usage improvement percentage  
    pub memory_improvement: f64,
    /// Expected compilation time impact percentage (negative = slower)
    pub compile_time_impact: f64,
    /// Confidence level in the estimate (0.0 to 1.0)
    pub confidence: f64,
}

/// Detailed optimization recommendation with specific actions
#[derive(Debug, Clone)]
pub struct DetailedRecommendation {
    /// Basic recommendation information
    pub category: super::OptimizationCategory,
    pub priority: super::RecommendationPriority,
    pub description: String,
    pub suggested_config: OptimizationConfig,
    /// Detailed analysis information
    pub patterns: Vec<AnalysisPattern>,
    /// Specific actions to take
    pub actions: Vec<OptimizationAction>,
    /// Expected impact
    pub expected_impact: PerformanceImpact,
    /// Code examples and suggestions
    pub code_suggestions: Vec<CodeSuggestion>,
}

/// Specific optimization action to take
#[derive(Debug, Clone)]
pub struct OptimizationAction {
    /// Type of action
    pub action_type: ActionType,
    /// Description of the action
    pub description: String,
    /// Priority of this specific action
    pub priority: ActionPriority,
    /// Configuration changes needed
    pub config_changes: Vec<ConfigChange>,
}

/// Types of optimization actions
#[derive(Debug, Clone)]
pub enum ActionType {
    EnableOptimization,
    DisableOptimization,
    ChangeOptimizationLevel,
    RefactorCode,
    AddCompilerHint,
    ChangeDataStructure,
    OptimizeAlgorithm,
    EnableFeature,
    DisableFeature,
}

/// Priority levels for specific actions
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ActionPriority {
    Immediate,
    High,
    Medium,
    Low,
    Optional,
}

/// Configuration change recommendation
#[derive(Debug, Clone)]
pub struct ConfigChange {
    pub setting: String,
    pub current_value: String,
    pub recommended_value: String,
    pub reason: String,
}

/// Code suggestion with before/after examples
#[derive(Debug, Clone)]
pub struct CodeSuggestion {
    /// Brief description of the suggestion
    pub title: String,
    /// Detailed explanation
    pub explanation: String,
    /// Original code pattern (if applicable)
    pub before_code: Option<String>,
    /// Suggested improved code
    pub after_code: Option<String>,
    /// Expected benefit
    pub benefit: String,
}

/// Source location information
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub function_name: Option<String>,
}

impl CodeAnalysisEngine {
    /// Create a new code analysis engine with default configuration
    pub fn new() -> Self {
        Self {
            config: AnalysisConfig::default(),
            analysis_cache: HashMap::new(),
        }
    }

    /// Create analysis engine with custom configuration
    pub fn with_config(config: AnalysisConfig) -> Self {
        Self {
            config,
            analysis_cache: HashMap::new(),
        }
    }

    /// Analyze CURSED source code and generate optimization recommendations
    #[instrument(skip(self, source_code))]
    pub fn analyze_code(&mut self, source_code: &str) -> Result<Vec<DetailedRecommendation>> {
        debug!("Starting code analysis for {} bytes of source", source_code.len());

        // Check cache first
        if let Some(cached_patterns) = self.analysis_cache.get(source_code) {
            debug!("Using cached analysis results");
            return Ok(self.generate_recommendations_from_patterns(cached_patterns));
        }

        // Parse the source code
        let mut lexer = Lexer::new(source_code.to_string());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

        // Analyze the AST for optimization patterns
        let patterns = self.analyze_ast(&program)?;
        
        // Cache the results
        self.analysis_cache.insert(source_code.to_string(), patterns.clone());

        // Generate detailed recommendations
        let recommendations = self.generate_recommendations_from_patterns(&patterns);
        
        info!("Generated {} optimization recommendations", recommendations.len());
        Ok(recommendations)
    }

    /// Analyze AST and detect optimization patterns
    #[instrument(skip(self, program))]
    fn analyze_ast(&self, program: &Program) -> Result<Vec<AnalysisPattern>> {
        debug!("Analyzing AST with {} statements", program.statements.len());

        let mut analyzer = AdvancedASTAnalyzer::new(self.config.clone());
        let patterns = analyzer.analyze_program(program);

        debug!("Detected {} optimization patterns", patterns.len());
        Ok(patterns)
    }

    /// Generate detailed recommendations from detected patterns
    fn generate_recommendations_from_patterns(&self, patterns: &[AnalysisPattern]) -> Vec<DetailedRecommendation> {
        let mut recommendations = Vec::new();
        
        // Group patterns by category for better recommendations
        let grouped_patterns = self.group_patterns_by_category(patterns);
        
        for (category, category_patterns) in grouped_patterns {
            let recommendation = self.create_detailed_recommendation(category, category_patterns);
            recommendations.push(recommendation);
        }

        // Sort by priority and impact
        recommendations.sort_by(|a, b| {
            b.priority.partial_cmp(&a.priority)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    b.expected_impact.runtime_improvement
                        .partial_cmp(&a.expected_impact.runtime_improvement)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        recommendations
    }

    /// Group patterns by optimization category
    fn group_patterns_by_category(&self, patterns: &[AnalysisPattern]) -> HashMap<super::OptimizationCategory, Vec<AnalysisPattern>> {
        let mut grouped = HashMap::new();
        
        for pattern in patterns {
            let category = match pattern.pattern_type {
                PatternType::NestedLoops | PatternType::InlineCandidate | 
                PatternType::ComplexExpression | PatternType::RepeatedComputation |
                PatternType::DeepRecursion | PatternType::CollectionOperations => {
                    super::OptimizationCategory::Performance
                }
                PatternType::MemoryAllocation => {
                    super::OptimizationCategory::MemoryUsage
                }
                PatternType::LargeFunction => {
                    super::OptimizationCategory::BinarySize
                }
                PatternType::ChannelOperations | PatternType::ConcurrencyPattern => {
                    super::OptimizationCategory::Performance
                }
                PatternType::StringOperations | PatternType::ErrorHandling => {
                    super::OptimizationCategory::Performance
                }
            };
            
            grouped.entry(category).or_insert_with(Vec::new).push(pattern.clone());
        }
        
        grouped
    }

    /// Create detailed recommendation for a category of patterns
    fn create_detailed_recommendation(
        &self, 
        category: super::OptimizationCategory, 
        patterns: Vec<AnalysisPattern>
    ) -> DetailedRecommendation {
        let priority = self.determine_priority(&patterns);
        let suggested_config = self.suggest_configuration(&category, &patterns);
        let actions = self.generate_actions(&patterns);
        let expected_impact = self.calculate_expected_impact(&patterns);
        let code_suggestions = self.generate_code_suggestions(&patterns);
        
        let description = match category {
            super::OptimizationCategory::Performance => {
                format!("Detected {} performance optimization opportunities. Consider enabling aggressive optimization and loop vectorization.", patterns.len())
            }
            super::OptimizationCategory::MemoryUsage => {
                format!("Found {} memory usage patterns that could be optimized. Consider enabling memory-specific optimizations.", patterns.len())
            }
            super::OptimizationCategory::CompileTime => {
                format!("Identified {} patterns affecting compile time. Consider build-specific optimizations.", patterns.len())
            }
            super::OptimizationCategory::BinarySize => {
                format!("Detected {} patterns affecting binary size. Consider size optimization settings.", patterns.len())
            }
            super::OptimizationCategory::Debugging => {
                format!("Found {} debugging-related patterns.", patterns.len())
            }
        };

        DetailedRecommendation {
            category,
            priority,
            description,
            suggested_config,
            patterns,
            actions,
            expected_impact,
            code_suggestions,
        }
    }

    /// Determine priority based on pattern severity and count
    fn determine_priority(&self, patterns: &[AnalysisPattern]) -> super::RecommendationPriority {
        let critical_count = patterns.iter().filter(|p| p.severity == PatternSeverity::Critical).count();
        let high_count = patterns.iter().filter(|p| p.severity == PatternSeverity::High).count();
        
        if critical_count > 0 {
            super::RecommendationPriority::Critical
        } else if high_count > 2 {
            super::RecommendationPriority::High
        } else if high_count > 0 || patterns.len() > 5 {
            super::RecommendationPriority::Medium
        } else {
            super::RecommendationPriority::Low
        }
    }

    /// Suggest optimization configuration based on patterns
    fn suggest_configuration(&self, category: &super::OptimizationCategory, patterns: &[AnalysisPattern]) -> OptimizationConfig {
        match category {
            super::OptimizationCategory::Performance => {
                let mut config = OptimizationConfig::new(OptimizationLevel::Aggressive);
                config.enable_vectorization = patterns.iter().any(|p| p.pattern_type == PatternType::NestedLoops);
                config.enable_function_inlining = patterns.iter().any(|p| p.pattern_type == PatternType::InlineCandidate);
                config.enable_loop_optimization = patterns.iter().any(|p| matches!(p.pattern_type, PatternType::NestedLoops | PatternType::RepeatedComputation));
                config
            }
            super::OptimizationCategory::MemoryUsage => {
                let mut config = OptimizationConfig::new(OptimizationLevel::Size);
                config.enable_memory_optimization = true;
                config.enable_dead_code_elimination = true;
                config
            }
            super::OptimizationCategory::CompileTime => {
                OptimizationConfig::new(OptimizationLevel::Fast)
            }
            super::OptimizationCategory::BinarySize => {
                let mut config = OptimizationConfig::new(OptimizationLevel::Size);
                config.enable_dead_code_elimination = true;
                config.enable_constant_folding = true;
                config
            }
            super::OptimizationCategory::Debugging => {
                OptimizationConfig::new(OptimizationLevel::Debug)
            }
        }
    }

    /// Generate specific optimization actions
    fn generate_actions(&self, patterns: &[AnalysisPattern]) -> Vec<OptimizationAction> {
        let mut actions = Vec::new();
        
        for pattern in patterns {
            match pattern.pattern_type {
                PatternType::NestedLoops => {
                    actions.push(OptimizationAction {
                        action_type: ActionType::EnableOptimization,
                        description: "Enable loop vectorization and unrolling".to_string(),
                        priority: ActionPriority::High,
                        config_changes: vec![
                            ConfigChange {
                                setting: "enable_vectorization".to_string(),
                                current_value: "false".to_string(),
                                recommended_value: "true".to_string(),
                                reason: "Nested loops can benefit significantly from vectorization".to_string(),
                            }
                        ],
                    });
                }
                PatternType::InlineCandidate => {
                    actions.push(OptimizationAction {
                        action_type: ActionType::EnableOptimization,
                        description: "Enable aggressive function inlining".to_string(),
                        priority: ActionPriority::Medium,
                        config_changes: vec![
                            ConfigChange {
                                setting: "enable_function_inlining".to_string(),
                                current_value: "false".to_string(),
                                recommended_value: "true".to_string(),
                                reason: "Small functions can be inlined for better performance".to_string(),
                            }
                        ],
                    });
                }
                PatternType::MemoryAllocation => {
                    actions.push(OptimizationAction {
                        action_type: ActionType::EnableOptimization,
                        description: "Enable memory optimization passes".to_string(),
                        priority: ActionPriority::High,
                        config_changes: vec![
                            ConfigChange {
                                setting: "enable_memory_optimization".to_string(),
                                current_value: "false".to_string(),
                                recommended_value: "true".to_string(),
                                reason: "Memory allocation patterns can be optimized".to_string(),
                            }
                        ],
                    });
                }
                PatternType::LargeFunction => {
                    actions.push(OptimizationAction {
                        action_type: ActionType::RefactorCode,
                        description: "Consider decomposing large functions".to_string(),
                        priority: ActionPriority::Medium,
                        config_changes: vec![],
                    });
                }
                _ => {
                    // Add generic optimization action
                    actions.push(OptimizationAction {
                        action_type: ActionType::EnableOptimization,
                        description: format!("Optimize {} pattern", format!("{:?}", pattern.pattern_type)),
                        priority: ActionPriority::Medium,
                        config_changes: vec![],
                    });
                }
            }
        }
        
        actions
    }

    /// Calculate expected performance impact
    fn calculate_expected_impact(&self, patterns: &[AnalysisPattern]) -> PerformanceImpact {
        let mut total_runtime_improvement = 0.0;
        let mut total_memory_improvement = 0.0;
        let mut total_compile_time_impact = 0.0;
        let mut total_confidence = 0.0;
        
        for pattern in patterns {
            total_runtime_improvement += pattern.performance_impact.runtime_improvement;
            total_memory_improvement += pattern.performance_impact.memory_improvement;
            total_compile_time_impact += pattern.performance_impact.compile_time_impact;
            total_confidence += pattern.performance_impact.confidence;
        }
        
        let count = patterns.len() as f64;
        PerformanceImpact {
            runtime_improvement: total_runtime_improvement / count,
            memory_improvement: total_memory_improvement / count,
            compile_time_impact: total_compile_time_impact / count,
            confidence: total_confidence / count,
        }
    }

    /// Generate code suggestions for patterns
    fn generate_code_suggestions(&self, patterns: &[AnalysisPattern]) -> Vec<CodeSuggestion> {
        let mut suggestions = Vec::new();
        
        for pattern in patterns {
            match pattern.pattern_type {
                PatternType::NestedLoops => {
                    suggestions.push(CodeSuggestion {
                        title: "Optimize nested loops".to_string(),
                        explanation: "Consider loop fusion, vectorization, or parallel processing for nested loops".to_string(),
                        before_code: Some("bestie (sus i = 0; i < n; i++) {\n    bestie (sus j = 0; j < m; j++) {\n        // computation\n    }\n}".to_string()),
                        after_code: Some("// Consider vectorization or parallel processing\nuse \"stdlib::parallel\";\nparallel_for(0, n, |i| {\n    parallel_for(0, m, |j| {\n        // computation\n    });\n});".to_string()),
                        benefit: "Can improve performance by 2-5x for CPU-intensive loops".to_string(),
                    });
                }
                PatternType::StringOperations => {
                    suggestions.push(CodeSuggestion {
                        title: "Optimize string operations".to_string(),
                        explanation: "Use string builders or pre-allocate capacity for multiple string operations".to_string(),
                        before_code: Some("sus result = \"\";\nbestie (sus item in items) {\n    result = result + item + \", \";\n}".to_string()),
                        after_code: Some("sus builder = StringBuilder::with_capacity(estimated_size);\nbestie (sus item in items) {\n    builder.append(item);\n    builder.append(\", \");\n}\nsus result = builder.to_string();".to_string()),
                        benefit: "Reduces memory allocations and improves string concatenation performance".to_string(),
                    });
                }
                PatternType::CollectionOperations => {
                    suggestions.push(CodeSuggestion {
                        title: "Optimize collection operations".to_string(),
                        explanation: "Pre-allocate collection capacity when size is known".to_string(),
                        before_code: Some("sus items = [];\nbestie (sus i = 0; i < 1000; i++) {\n    items.push(process_item(i));\n}".to_string()),
                        after_code: Some("sus items = Vec::with_capacity(1000);\nbestie (sus i = 0; i < 1000; i++) {\n    items.push(process_item(i));\n}".to_string()),
                        benefit: "Reduces memory reallocations and improves performance".to_string(),
                    });
                }
                _ => {
                    // Generic suggestion
                    suggestions.push(CodeSuggestion {
                        title: format!("Optimize {:?} pattern", pattern.pattern_type),
                        explanation: "Consider reviewing this code pattern for optimization opportunities".to_string(),
                        before_code: None,
                        after_code: None,
                        benefit: "May improve performance and code quality".to_string(),
                    });
                }
            }
        }
        
        suggestions
    }

    /// Clear analysis cache
    pub fn clear_cache(&mut self) {
        self.analysis_cache.clear();
    }
}

// The ASTAnalyzer has been moved to ast_analyzer.rs for better organization
// and replaced with AdvancedASTAnalyzer which provides more sophisticated analysis

impl Default for CodeAnalysisEngine {
    fn default() -> Self {
        Self::new()
    }
}
