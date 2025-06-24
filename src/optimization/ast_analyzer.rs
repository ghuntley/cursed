// Advanced AST Analysis for Optimization Recommendations
// 
// This module provides sophisticated AST analysis capabilities for detecting
// optimization patterns in CURSED source code.

use crate::ast::*;
use crate::optimization::intelligent_recommendations::*;
use std::collections::{HashMap, HashSet};
use tracing::{debug, instrument};

/// Advanced AST analyzer that implements comprehensive pattern detection
pub struct AdvancedASTAnalyzer {
    /// Analysis configuration
    config: AnalysisConfig,
    /// Detected patterns
    patterns: Vec<AnalysisPattern>,
    /// Current analysis context
    context: AnalysisContext,
    /// Function call frequency tracking
    call_frequency: HashMap<String, usize>,
    /// Variable usage tracking
    variable_usage: HashMap<String, VarUsageStats>,
    /// Loop nesting levels
    loop_stack: Vec<LoopInfo>,
}

/// Context information during analysis
#[derive(Debug, Clone)]
struct AnalysisContext {
    current_function: Option<String>,
    current_line: usize,
    current_column: usize,
    in_loop: bool,
    loop_depth: usize,
    function_complexity: usize,
    local_variables: HashSet<String>,
}

/// Information about a loop construct
#[derive(Debug, Clone)]
struct LoopInfo {
    loop_type: LoopType,
    start_location: SourceLocation,
    nesting_level: usize,
    estimated_iterations: Option<usize>,
}

/// Types of loops detected
#[derive(Debug, Clone)]
enum LoopType {
    For,
    While,
    ForEach,
}

/// Variable usage statistics
#[derive(Debug, Clone)]
struct VarUsageStats {
    reads: usize,
    writes: usize,
    first_use: SourceLocation,
    last_use: SourceLocation,
    used_in_loops: bool,
}

impl AdvancedASTAnalyzer {
    /// Create a new advanced AST analyzer
    pub fn new(config: AnalysisConfig) -> Self {
        Self {
            config,
            patterns: Vec::new(),
            context: AnalysisContext {
                current_function: None,
                current_line: 1,
                current_column: 1,
                in_loop: false,
                loop_depth: 0,
                function_complexity: 0,
                local_variables: HashSet::new(),
            },
            call_frequency: HashMap::new(),
            variable_usage: HashMap::new(),
            loop_stack: Vec::new(),
        }
    }

    /// Analyze a program and return detected patterns
    #[instrument(skip(self, program))]
    pub fn analyze_program(&mut self, program: &Program) -> Vec<AnalysisPattern> {
        debug!("Starting advanced AST analysis");
        
        self.patterns.clear();
        self.context = AnalysisContext {
            current_function: None,
            current_line: 1,
            current_column: 1,
            in_loop: false,
            loop_depth: 0,
            function_complexity: 0,
            local_variables: HashSet::new(),
        };

        // Analyze each statement in the program
        for statement in &program.statements {
            self.analyze_statement(statement.as_ref());
        }

        // Perform cross-reference analysis
        self.analyze_function_calls();
        self.analyze_variable_usage_patterns();
        self.analyze_performance_hotspots();

        debug!("Advanced analysis detected {} patterns", self.patterns.len());
        self.patterns.clone()
    }

    /// Analyze a single statement
    fn analyze_statement(&mut self, statement: &dyn Statement) {
        // Update context based on statement type
        self.context.function_complexity += 1;

        // Attempt to downcast to specific statement types for detailed analysis
        if let Some(func_stmt) = self.try_downcast_to_function(statement) {
            self.analyze_function_statement(func_stmt);
        } else if let Some(loop_stmt) = self.try_detect_loop_statement(statement) {
            self.analyze_loop_statement(loop_stmt);
        } else {
            // Generic statement analysis
            self.analyze_generic_statement(statement);
        }
    }

    /// Try to detect if a statement is a function declaration
    fn try_downcast_to_function(&self, statement: &dyn Statement) -> Option<&dyn Statement> {
        // In a real implementation, we would use proper type checking
        // For now, we'll use string representation to detect functions
        let statement_str = statement.to_string();
        if statement_str.contains("slay ") {
            Some(statement)
        } else {
            None
        }
    }

    /// Try to detect loop statements
    fn try_detect_loop_statement(&self, statement: &dyn Statement) -> Option<LoopStatement> {
        let statement_str = statement.to_string();
        
        if statement_str.contains("bestie ") {
            // Detected a for loop (bestie keyword)
            Some(LoopStatement {
                loop_type: LoopType::For,
                location: SourceLocation {
                    line: self.context.current_line,
                    column: self.context.current_column,
                    function_name: self.context.current_function.clone(),
                },
            })
        } else if statement_str.contains("periodt ") {
            // Detected a while loop (periodt keyword)
            Some(LoopStatement {
                loop_type: LoopType::While,
                location: SourceLocation {
                    line: self.context.current_line,
                    column: self.context.current_column,
                    function_name: self.context.current_function.clone(),
                },
            })
        } else {
            None
        }
    }

    /// Analyze a function statement
    fn analyze_function_statement(&mut self, statement: &dyn Statement) {
        let statement_str = statement.to_string();
        
        // Extract function name (simplified parsing)
        if let Some(func_name) = self.extract_function_name(&statement_str) {
            let old_function = self.context.current_function.clone();
            self.context.current_function = Some(func_name.clone());
            
            // Analyze function characteristics
            self.analyze_function_characteristics(&statement_str, &func_name);
            
            // Reset function context
            self.context.current_function = old_function;
            self.context.function_complexity = 0;
            self.context.local_variables.clear();
        }
    }

    /// Extract function name from statement string
    fn extract_function_name(&self, statement_str: &str) -> Option<String> {
        // Simple regex-like parsing to extract function name after "slay"
        if let Some(slay_pos) = statement_str.find("slay ") {
            let after_slay = &statement_str[slay_pos + 5..];
            if let Some(paren_pos) = after_slay.find('(') {
                let func_name = after_slay[..paren_pos].trim().to_string();
                if !func_name.is_empty() {
                    return Some(func_name);
                }
            }
        }
        None
    }

    /// Analyze function characteristics for optimization opportunities
    fn analyze_function_characteristics(&mut self, statement_str: &str, func_name: &str) {
        let lines = statement_str.split("\n").count();
        let estimated_complexity = statement_str.matches('{').count() + statement_str.matches("bestie").count() * 2;
        
        // Check for large functions
        if lines > self.config.max_function_size {
            self.patterns.push(AnalysisPattern {
                pattern_type: PatternType::LargeFunction,
                location: SourceLocation {
                    line: self.context.current_line,
                    column: self.context.current_column,
                    function_name: Some(func_name.to_string()),
                },
                severity: if lines > self.config.max_function_size * 2 {
                    PatternSeverity::High
                } else {
                    PatternSeverity::Medium
                },
                description: format!("Function '{}' has {} lines, consider decomposition (threshold: {})", 
                                   func_name, lines, self.config.max_function_size),
                performance_impact: PerformanceImpact {
                    runtime_improvement: 5.0,
                    memory_improvement: 2.0,
                    compile_time_impact: -3.0,
                    confidence: 0.7,
                },
            });
        }

        // Check for inline candidates (small, simple functions)
        if lines <= 5 && estimated_complexity <= 2 {
            self.patterns.push(AnalysisPattern {
                pattern_type: PatternType::InlineCandidate,
                location: SourceLocation {
                    line: self.context.current_line,
                    column: self.context.current_column,
                    function_name: Some(func_name.to_string()),
                },
                severity: PatternSeverity::Medium,
                description: format!("Function '{}' is a good candidate for inlining (small and simple)", func_name),
                performance_impact: PerformanceImpact {
                    runtime_improvement: 15.0,
                    memory_improvement: 0.0,
                    compile_time_impact: 2.0,
                    confidence: 0.8,
                },
            });
        }

        // Check for string operations
        if statement_str.contains(" + ") && (statement_str.contains("String") || statement_str.contains("\"")) {
            self.patterns.push(AnalysisPattern {
                pattern_type: PatternType::StringOperations,
                location: SourceLocation {
                    line: self.context.current_line,
                    column: self.context.current_column,
                    function_name: Some(func_name.to_string()),
                },
                severity: PatternSeverity::Medium,
                description: "String concatenation detected, consider using StringBuilder for better performance".to_string(),
                performance_impact: PerformanceImpact {
                    runtime_improvement: 25.0,
                    memory_improvement: 15.0,
                    compile_time_impact: 0.0,
                    confidence: 0.9,
                },
            });
        }
    }

    /// Analyze loop statement for optimization opportunities
    fn analyze_loop_statement(&mut self, loop_stmt: LoopStatement) {
        let old_in_loop = self.context.in_loop;
        let old_loop_depth = self.context.loop_depth;
        
        self.context.in_loop = true;
        self.context.loop_depth += 1;
        
        let loop_info = LoopInfo {
            loop_type: loop_stmt.loop_type.clone(),
            start_location: loop_stmt.location.clone(),
            nesting_level: self.context.loop_depth,
            estimated_iterations: None, // Could be enhanced with static analysis
        };
        
        self.loop_stack.push(loop_info);

        // Check for deeply nested loops
        if self.context.loop_depth > self.config.max_loop_nesting {
            self.patterns.push(AnalysisPattern {
                pattern_type: PatternType::NestedLoops,
                location: loop_stmt.location.clone(),
                severity: if self.context.loop_depth > self.config.max_loop_nesting * 2 {
                    PatternSeverity::Critical
                } else {
                    PatternSeverity::High
                },
                description: format!("Loop nesting level {} exceeds recommended maximum {} - consider refactoring",
                                   self.context.loop_depth, self.config.max_loop_nesting),
                performance_impact: PerformanceImpact {
                    runtime_improvement: 40.0,
                    memory_improvement: 10.0,
                    compile_time_impact: 0.0,
                    confidence: 0.85,
                },
            });
        }

        // Restore context
        self.context.in_loop = old_in_loop;
        self.context.loop_depth = old_loop_depth;
        
        if !self.loop_stack.is_empty() {
            self.loop_stack.pop();
        }
    }

    /// Analyze generic statements for patterns
    fn analyze_generic_statement(&mut self, statement: &dyn Statement) {
        let statement_str = statement.to_string();
        
        // Check for repeated computations
        if self.detect_repeated_computation(&statement_str) {
            self.patterns.push(AnalysisPattern {
                pattern_type: PatternType::RepeatedComputation,
                location: SourceLocation {
                    line: self.context.current_line,
                    column: self.context.current_column,
                    function_name: self.context.current_function.clone(),
                },
                severity: PatternSeverity::Medium,
                description: "Repeated computation detected, consider caching results".to_string(),
                performance_impact: PerformanceImpact {
                    runtime_improvement: 20.0,
                    memory_improvement: -5.0,
                    compile_time_impact: 0.0,
                    confidence: 0.7,
                },
            });
        }

        // Check for complex expressions
        if self.detect_complex_expression(&statement_str) {
            self.patterns.push(AnalysisPattern {
                pattern_type: PatternType::ComplexExpression,
                location: SourceLocation {
                    line: self.context.current_line,
                    column: self.context.current_column,
                    function_name: self.context.current_function.clone(),
                },
                severity: PatternSeverity::Low,
                description: "Complex expression detected, consider simplification or temporary variables".to_string(),
                performance_impact: PerformanceImpact {
                    runtime_improvement: 8.0,
                    memory_improvement: 0.0,
                    compile_time_impact: 1.0,
                    confidence: 0.6,
                },
            });
        }

        // Check for memory allocation patterns
        if self.detect_memory_allocation(&statement_str) {
            self.patterns.push(AnalysisPattern {
                pattern_type: PatternType::MemoryAllocation,
                location: SourceLocation {
                    line: self.context.current_line,
                    column: self.context.current_column,
                    function_name: self.context.current_function.clone(),
                },
                severity: PatternSeverity::Medium,
                description: "Memory allocation in loop or frequent context, consider pre-allocation".to_string(),
                performance_impact: PerformanceImpact {
                    runtime_improvement: 30.0,
                    memory_improvement: 20.0,
                    compile_time_impact: 0.0,
                    confidence: 0.8,
                },
            });
        }
    }

    /// Detect repeated computations in a statement
    fn detect_repeated_computation(&self, statement_str: &str) -> bool {
        // Simple heuristic: look for identical sub-expressions
        let expressions: Vec<&str> = statement_str.split(&['+', '-', '*', '/', '=', '(', ')']).collect();
        let mut expr_counts = HashMap::new();
        
        for expr in expressions {
            let expr = expr.trim();
            if expr.len() > 3 && !expr.chars().all(|c| c.is_whitespace()) {
                *expr_counts.entry(expr).or_insert(0) += 1;
            }
        }
        
        expr_counts.values().any(|&count| count > 1)
    }

    /// Detect complex expressions
    fn detect_complex_expression(&self, statement_str: &str) -> bool {
        // Heuristic: count operators and nesting
        let operator_count = statement_str.matches(&['+', '-', '*', '/', '%', '&', '|', '^']).count();
        let paren_count = statement_str.matches('(').count();
        
        operator_count > 5 || paren_count > 3
    }

    /// Detect memory allocation patterns
    fn detect_memory_allocation(&self, statement_str: &str) -> bool {
        let allocation_keywords = ["new ", "Vec::", "HashMap::", "String::", ".push(", ".insert("];
        
        allocation_keywords.iter().any(|keyword| statement_str.contains(keyword)) && self.context.in_loop
    }

    /// Analyze function call patterns
    fn analyze_function_calls(&mut self) {
        // Analyze call frequency for inlining recommendations
        for (func_name, frequency) in &self.call_frequency {
            if *frequency > 10 {
                self.patterns.push(AnalysisPattern {
                    pattern_type: PatternType::InlineCandidate,
                    location: SourceLocation {
                        line: 0,
                        column: 0,
                        function_name: Some(func_name.clone()),
                    },
                    severity: PatternSeverity::High,
                    description: format!("Function '{}' called {} times, good candidate for inlining", func_name, frequency),
                    performance_impact: PerformanceImpact {
                        runtime_improvement: 12.0,
                        memory_improvement: 0.0,
                        compile_time_impact: 3.0,
                        confidence: 0.9,
                    },
                });
            }
        }
    }

    /// Analyze variable usage patterns
    fn analyze_variable_usage_patterns(&mut self) {
        for (var_name, usage) in &self.variable_usage {
            // Check for variables that are read much more than written
            if usage.reads > usage.writes * 5 && usage.used_in_loops {
                self.patterns.push(AnalysisPattern {
                    pattern_type: PatternType::RepeatedComputation,
                    location: usage.first_use.clone(),
                    severity: PatternSeverity::Medium,
                    description: format!("Variable '{}' read {} times but written {} times in loops, consider caching", 
                                       var_name, usage.reads, usage.writes),
                    performance_impact: PerformanceImpact {
                        runtime_improvement: 15.0,
                        memory_improvement: -2.0,
                        compile_time_impact: 0.0,
                        confidence: 0.75,
                    },
                });
            }
        }
    }

    /// Analyze performance hotspots
    fn analyze_performance_hotspots(&mut self) {
        // Combine multiple patterns to identify performance hotspots
        let nested_loops: Vec<_> = self.patterns.iter()
            .filter(|p| p.pattern_type == PatternType::NestedLoops)
            .collect();
        
        let string_ops: Vec<_> = self.patterns.iter()
            .filter(|p| p.pattern_type == PatternType::StringOperations)
            .collect();

        // If we have nested loops with string operations, that's a critical hotspot
        if !nested_loops.is_empty() && !string_ops.is_empty() {
            self.patterns.push(AnalysisPattern {
                pattern_type: PatternType::ComplexExpression,
                location: SourceLocation {
                    line: 0,
                    column: 0,
                    function_name: None,
                },
                severity: PatternSeverity::Critical,
                description: "Performance hotspot detected: nested loops with string operations".to_string(),
                performance_impact: PerformanceImpact {
                    runtime_improvement: 60.0,
                    memory_improvement: 30.0,
                    compile_time_impact: 0.0,
                    confidence: 0.95,
                },
            });
        }
    }
}

/// Simplified loop statement representation for analysis
#[derive(Debug, Clone)]
struct LoopStatement {
    loop_type: LoopType,
    location: SourceLocation,
}

impl Default for AnalysisContext {
    fn default() -> Self {
        Self {
            current_function: None,
            current_line: 1,
            current_column: 1,
            in_loop: false,
            loop_depth: 0,
            function_complexity: 0,
            local_variables: HashSet::new(),
        }
    }
}
