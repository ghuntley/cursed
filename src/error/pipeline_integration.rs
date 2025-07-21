//! Compilation pipeline error integration for CURSED compiler
//! 
//! This module provides seamless integration of error handling throughout
//! the compilation pipeline with enhanced context propagation and recovery.

use std::collections::HashMap;
use crate::error::{CursedError, StructuredError, ErrorCode, ErrorDiagnostics, DiagnosticError};
use crate::lexer::Token;
use crate::ast::Program;

/// Compilation pipeline error manager
pub struct PipelineErrorManager {
    /// Error diagnostics system
    diagnostics: ErrorDiagnostics,
    /// Error context stack for nested operations
    context_stack: Vec<CompilationContext>,
    /// Accumulated errors from all pipeline stages
    accumulated_errors: Vec<DiagnosticError>,
    /// Warning buffer
    accumulated_warnings: Vec<DiagnosticError>,
    /// Pipeline stage performance metrics
    stage_metrics: HashMap<PipelineStage, StageMetrics>,
    /// Error recovery state
    recovery_state: RecoveryState,
}

/// Compilation context for error tracking
#[derive(Debug, Clone)]
pub struct CompilationContext {
    pub stage: PipelineStage,
    pub file_path: String,
    pub current_line: usize,
    pub current_function: Option<String>,
    pub current_scope: usize,
    pub additional_context: HashMap<String, String>,
}

/// Pipeline stages for error tracking
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipelineStage {
    Lexing,
    Parsing,
    TypeChecking,
    SemanticAnalysis,
    CodeGeneration,
    Optimization,
    Linking,
}

/// Stage performance metrics
#[derive(Debug, Clone)]
pub struct StageMetrics {
    pub errors_count: usize,
    pub warnings_count: usize,
    pub processing_time: std::time::Duration,
    pub recovery_attempts: usize,
    pub successful_recoveries: usize,
}

/// Error recovery state
#[derive(Debug, Clone)]
pub struct RecoveryState {
    pub enabled: bool,
    pub max_errors_per_stage: usize,
    pub max_total_errors: usize,
    pub continue_after_errors: bool,
    pub recovery_strategies: HashMap<PipelineStage, RecoveryStrategy>,
}

/// Recovery strategy for different pipeline stages
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// Skip problematic tokens and continue
    SkipAndContinue,
    /// Insert fallback values and continue
    InsertFallback,
    /// Stop processing this unit but continue with others
    SkipUnit,
    /// Stop entire pipeline
    StopPipeline,
    /// Custom recovery with specific actions
    Custom(Vec<RecoveryAction>),
}

/// Specific recovery actions
#[derive(Debug, Clone)]
pub enum RecoveryAction {
    InsertToken(String),
    RemoveToken,
    ReplaceToken(String),
    CreateFallbackAST,
    InferMissingType,
    SkipToNextStatement,
    SkipToNextFunction,
}

/// Compilation result with comprehensive error information
pub struct CompilationResult<T> {
    pub result: Option<T>,
    pub errors: Vec<DiagnosticError>,
    pub warnings: Vec<DiagnosticError>,
    pub stage_metrics: HashMap<PipelineStage, StageMetrics>,
    pub recovery_successful: bool,
    pub compilation_successful: bool,
}

impl PipelineErrorManager {
    pub fn new() -> Self {
        let mut recovery_strategies = HashMap::new();
        
        // Default recovery strategies for each stage
        recovery_strategies.insert(PipelineStage::Lexing, RecoveryStrategy::SkipAndContinue);
        recovery_strategies.insert(PipelineStage::Parsing, RecoveryStrategy::Custom(vec![
            RecoveryAction::SkipToNextStatement,
            RecoveryAction::CreateFallbackAST,
        ]));
        recovery_strategies.insert(PipelineStage::TypeChecking, RecoveryStrategy::InsertFallback);
        recovery_strategies.insert(PipelineStage::SemanticAnalysis, RecoveryStrategy::SkipUnit);
        recovery_strategies.insert(PipelineStage::CodeGeneration, RecoveryStrategy::StopPipeline);
        recovery_strategies.insert(PipelineStage::Optimization, RecoveryStrategy::SkipAndContinue);
        recovery_strategies.insert(PipelineStage::Linking, RecoveryStrategy::StopPipeline);
        
        Self {
            diagnostics: ErrorDiagnostics::new(),
            context_stack: Vec::new(),
            accumulated_errors: Vec::new(),
            accumulated_warnings: Vec::new(),
            stage_metrics: HashMap::new(),
            recovery_state: RecoveryState {
                enabled: true,
                max_errors_per_stage: 50,
                max_total_errors: 200,
                continue_after_errors: true,
                recovery_strategies,
            },
        }
    }
    
    /// Push a new compilation context
    pub fn push_context(&mut self, context: CompilationContext) {
        self.context_stack.push(context);
    }
    
    /// Pop the current compilation context
    pub fn pop_context(&mut self) -> Option<CompilationContext> {
        self.context_stack.pop()
    }
    
    /// Get the current compilation context
    pub fn current_context(&self) -> Option<&CompilationContext> {
        self.context_stack.last()
    }
    
    /// Handle error with full pipeline context
    pub fn handle_error(&mut self, error: CursedError) -> Result<RecoveryAction, CursedError> {
        let context = self.current_context();
        let stage = context.map(|c| c.stage.clone()).unwrap_or(PipelineStage::Parsing);
        
        // Convert to structured error with context
        let mut structured_error = self.convert_to_structured_error(error, context);
        
        // Enhance with diagnostics
        let diagnostic_errors = self.diagnostics.analyze_errors(vec![structured_error]);
        let diagnostic_error = diagnostic_errors.into_iter().next().unwrap_or_else(|| {
            DiagnosticError {
                structured_error: StructuredError::new(ErrorCode::E0001, "Unknown error".to_string()),
                confidence_level: crate::error::ConfidenceLevel::Low,
                fix_hints: Vec::new(),
                related_errors: Vec::new(),
                severity_adjusted: false,
            }
        });
        
        // Update stage metrics
        self.update_stage_metrics(&stage, true, false);
        
        // Determine recovery action
        let recovery_action = self.determine_recovery_action(&stage, &diagnostic_error);
        
        // Store error based on severity
        match diagnostic_error.structured_error.severity {
            crate::error::structured::ErrorSeverity::Error => {
                self.accumulated_errors.push(diagnostic_error);
            }
            crate::error::structured::ErrorSeverity::Warning => {
                self.accumulated_warnings.push(diagnostic_error);
            }
            _ => {
                // Info and help messages don't stop compilation
                self.accumulated_warnings.push(diagnostic_error);
            }
        }
        
        // Check if we should continue compilation
        if self.should_stop_compilation() {
            return Err(CursedError::CompilerError("Too many errors".to_string()));
        }
        
        Ok(recovery_action)
    }
    
    /// Handle warning with context
    pub fn handle_warning(&mut self, warning: CursedError) {
        let context = self.current_context();
        let stage = context.map(|c| c.stage.clone()).unwrap_or(PipelineStage::Parsing);
        
        let mut structured_error = self.convert_to_structured_error(warning, context);
        structured_error.severity = crate::error::structured::ErrorSeverity::Warning;
        
        let diagnostic_errors = self.diagnostics.analyze_errors(vec![structured_error]);
        if let Some(diagnostic_error) = diagnostic_errors.into_iter().next() {
            self.accumulated_warnings.push(diagnostic_error);
        }
        
        self.update_stage_metrics(&stage, false, true);
    }
    
    /// Convert CursedError to StructuredError with context
    fn convert_to_structured_error(&self, error: CursedError, context: Option<&CompilationContext>) -> StructuredError {
        let error_code = match &error {
            CursedError::SyntaxError(_) => ErrorCode::E0001,
            CursedError::TypeError(_) => ErrorCode::E0100,
            CursedError::RuntimeError(_) => ErrorCode::E0300,
            CursedError::CompilerError(_) => ErrorCode::E0200,
            CursedError::Parse(_) => ErrorCode::E0001,
            _ => ErrorCode::E0001,
        };
        
        let mut structured_error = StructuredError::new(error_code, error.to_string());
        
        // Add context information
        if let Some(ctx) = context {
            let location = crate::error::structured::ErrorSourceLocation {
file: ctx.file_path.clone(),
                line: ctx.current_line,
                column: 0,
                length: 1,
                source_line: None,
            
                    offset: 0,
                };
            structured_error = structured_error.with_location(location);
            
            // Add contextual information
            let mut context_info = vec![
                format!("In {} stage", format!("{:?}", ctx.stage).to_lowercase()),
            ];
            
            if let Some(function) = &ctx.current_function {
                context_info.push(format!("In function '{}'", function));
            }
            
            context_info.push(format!("Scope level: {}", ctx.current_scope));
            
            for (key, value) in &ctx.additional_context {
                context_info.push(format!("{}: {}", key, value));
            }
            
            structured_error = structured_error.with_context(context_info);
        }
        
        structured_error
    }
    
    /// Determine the appropriate recovery action
    fn determine_recovery_action(&mut self, stage: &PipelineStage, error: &DiagnosticError) -> RecoveryAction {
        if !self.recovery_state.enabled {
            return RecoveryAction::SkipToNextStatement;
        }
        
        let strategy = self.recovery_state.recovery_strategies
            .get(stage)
            .cloned()
            .unwrap_or(RecoveryStrategy::SkipAndContinue);
        
        match strategy {
            RecoveryStrategy::SkipAndContinue => RecoveryAction::SkipToNextStatement,
            RecoveryStrategy::InsertFallback => {
                // Choose appropriate fallback based on error type
                match error.structured_error.code {
                    ErrorCode::E0100 => RecoveryAction::InferMissingType,
                    ErrorCode::E0109 => RecoveryAction::CreateFallbackAST,
                    _ => RecoveryAction::SkipToNextStatement,
                }
            }
            RecoveryStrategy::SkipUnit => RecoveryAction::SkipToNextFunction,
            RecoveryStrategy::StopPipeline => RecoveryAction::SkipToNextStatement, // Handled by caller
            RecoveryStrategy::Custom(actions) => {
                // Use the first applicable action
                actions.into_iter().next().unwrap_or(RecoveryAction::SkipToNextStatement)
            }
        }
    }
    
    /// Update metrics for a pipeline stage
    fn update_stage_metrics(&mut self, stage: &PipelineStage, is_error: bool, is_warning: bool) {
        let metrics = self.stage_metrics.entry(stage.clone()).or_insert_with(|| StageMetrics {
            errors_count: 0,
            warnings_count: 0,
            processing_time: std::time::Duration::from_secs(0),
            recovery_attempts: 0,
            successful_recoveries: 0,
        });
        
        if is_error {
            metrics.errors_count += 1;
        }
        if is_warning {
            metrics.warnings_count += 1;
        }
    }
    
    /// Check if compilation should stop
    fn should_stop_compilation(&self) -> bool {
        let total_errors = self.accumulated_errors.len();
        
        if total_errors >= self.recovery_state.max_total_errors {
            return true;
        }
        
        if !self.recovery_state.continue_after_errors && total_errors > 0 {
            return true;
        }
        
        // Check per-stage limits
        for (stage, metrics) in &self.stage_metrics {
            if metrics.errors_count >= self.recovery_state.max_errors_per_stage {
                eprintln!("Too many errors in {:?} stage", stage);
                return true;
            }
        }
        
        false
    }
    
    /// Execute a recovery action
    pub fn execute_recovery_action(&mut self, action: RecoveryAction) -> Result<(), CursedError> {
        if let Some(metrics) = self.stage_metrics.get_mut(&self.current_context().map(|c| c.stage.clone()).unwrap_or(PipelineStage::Parsing)) {
            metrics.recovery_attempts += 1;
        }
        
        match action {
            RecoveryAction::InsertToken(_) | 
            RecoveryAction::RemoveToken | 
            RecoveryAction::ReplaceToken(_) |
            RecoveryAction::CreateFallbackAST |
            RecoveryAction::InferMissingType |
            RecoveryAction::SkipToNextStatement |
            RecoveryAction::SkipToNextFunction => {
                // Mark recovery as successful
                if let Some(metrics) = self.stage_metrics.get_mut(&self.current_context().map(|c| c.stage.clone()).unwrap_or(PipelineStage::Parsing)) {
                    metrics.successful_recoveries += 1;
                }
                Ok(())
            }
        }
    }
    
    /// Create a compilation result
    pub fn create_result<T>(&self, result: Option<T>) -> CompilationResult<T> {
        let compilation_successful = result.is_some() && self.accumulated_errors.is_empty();
        let recovery_successful = self.stage_metrics.values()
            .all(|m| m.recovery_attempts == 0 || m.successful_recoveries > 0);
        
        CompilationResult {
            result,
            errors: self.accumulated_errors.clone(),
            warnings: self.accumulated_warnings.clone(),
            stage_metrics: self.stage_metrics.clone(),
            recovery_successful,
            compilation_successful,
        }
    }
    
    /// Get error summary
    pub fn get_error_summary(&self) -> String {
        let error_count = self.accumulated_errors.len();
        let warning_count = self.accumulated_warnings.len();
        
        let mut summary = format!("Compilation finished with {} error(s) and {} warning(s)\n", 
                                 error_count, warning_count);
        
        if error_count > 0 {
            summary.push_str("\nErrors by stage:\n");
            for (stage, metrics) in &self.stage_metrics {
                if metrics.errors_count > 0 {
                    summary.push_str(&format!("  {:?}: {} error(s)\n", stage, metrics.errors_count));
                }
            }
        }
        
        if warning_count > 0 {
            summary.push_str("\nWarnings by stage:\n");
            for (stage, metrics) in &self.stage_metrics {
                if metrics.warnings_count > 0 {
                    summary.push_str(&format!("  {:?}: {} warning(s)\n", stage, metrics.warnings_count));
                }
            }
        }
        
        // Recovery statistics
        let total_recoveries: usize = self.stage_metrics.values()
            .map(|m| m.successful_recoveries)
            .sum();
        
        if total_recoveries > 0 {
            summary.push_str(&format!("\nSuccessful error recoveries: {}\n", total_recoveries));
        }
        
        summary
    }
    
    /// Clear accumulated errors and warnings
    pub fn clear(&mut self) {
        self.accumulated_errors.clear();
        self.accumulated_warnings.clear();
        self.stage_metrics.clear();
        self.context_stack.clear();
    }
    
    /// Enable or disable error recovery
    pub fn set_recovery_enabled(&mut self, enabled: bool) {
        self.recovery_state.enabled = enabled;
    }
    
    /// Set maximum errors per stage
    pub fn set_max_errors_per_stage(&mut self, max_errors: usize) {
        self.recovery_state.max_errors_per_stage = max_errors;
    }
    
    /// Set maximum total errors
    pub fn set_max_total_errors(&mut self, max_errors: usize) {
        self.recovery_state.max_total_errors = max_errors;
    }
    
    /// Get detailed error report
    pub fn get_detailed_report(&self) -> String {
        let mut report = self.diagnostics.generate_error_report(&self.accumulated_errors);
        
        if !self.accumulated_warnings.is_empty() {
            report.push_str("\n=== Warnings ===\n");
            for warning in &self.accumulated_warnings {
                report.push_str(&self.diagnostics.format_diagnostic_error(warning));
                report.push('\n');
            }
        }
        
        report.push_str(&self.get_error_summary());
        
        report
    }
}

impl Default for PipelineErrorManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StageMetrics {
    fn default() -> Self {
        Self {
            errors_count: 0,
            warnings_count: 0,
            processing_time: std::time::Duration::from_secs(0),
            recovery_attempts: 0,
            successful_recoveries: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pipeline_error_manager_creation() {
        let manager = PipelineErrorManager::new();
        assert!(manager.recovery_state.enabled);
        assert_eq!(manager.accumulated_errors.len(), 0);
    }
    
    #[test]
    fn test_context_management() {
        let mut manager = PipelineErrorManager::new();
        
        let context = CompilationContext {
            stage: PipelineStage::Parsing,
            file_path: "test.csd".to_string(),
            current_line: 10,
            current_function: Some("main".to_string()),
            current_scope: 0,
            additional_context: HashMap::new(),
        };
        
        manager.push_context(context.clone());
        assert_eq!(manager.current_context().unwrap().stage, PipelineStage::Parsing);
        
        let popped = manager.pop_context();
        assert!(popped.is_some());
        assert!(manager.current_context().is_none());
    }
    
    #[test]
    fn test_error_handling() {
        let mut manager = PipelineErrorManager::new();
        
        let context = CompilationContext {
            stage: PipelineStage::Parsing,
            file_path: "test.csd".to_string(),
            current_line: 5,
            current_function: None,
            current_scope: 0,
            additional_context: HashMap::new(),
        };
        
        manager.push_context(context);
        
        let error = CursedError::SyntaxError("Test error".to_string());
        let result = manager.handle_error(error);
        
        assert!(result.is_ok());
        assert_eq!(manager.accumulated_errors.len(), 1);
    }
    
    #[test]
    fn test_compilation_result() {
        let manager = PipelineErrorManager::new();
        let result: CompilationResult<String> = manager.create_result(Some("success".to_string()));
        
        assert!(result.compilation_successful);
        assert!(result.result.is_some());
        assert_eq!(result.errors.len(), 0);
    }
}
