// CursedError context management for error propagation in CURSED
//
// This module provides comprehensive error context tracking, including
// source location management, function call chains, and error propagation
// history for enhanced debugging and error reporting.

use crate::error::CursedError;
use crate::runtime::error_propagation::{ErrorPropagationContext, PropagationError};

use std::collections::{HashMap, BTreeMap, VecDeque};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, instrument, warn};

/// Enhanced error context manager for comprehensive error tracking
#[derive(Debug)]
pub struct ErrorContextManager {
    /// Global error context registry
    /// Function call stack tracker
    /// Source location mapper for file/line resolution
    /// CursedError chain tracker for related errors
    /// Configuration for context management
impl ErrorContextManager {
    /// Create a new error context manager
    pub fn new() -> Self {
        Self::with_config(ContextManagerConfig::default())
    /// Create with custom configuration
    pub fn with_config(config: ContextManagerConfig) -> Self {
        Self {
        }
    }

    /// Register a new error context
    #[instrument(skip(self, context))]
    pub fn register_context(&self, context: EnhancedErrorContext) -> crate::error::Result<()> {
        let mut registry = self.context_registry.write()
            .map_err(|_| CursedError::system_error("Failed to acquire context registry lock"))?;

        let context_id = registry.add_context(context);
        
        debug!(context_id = %context_id, "Registered new error context");
        Ok(context_id)
    /// Push function context onto call stack
    #[instrument(skip(self))]
    pub fn push_function_context(
    ) -> crate::error::Result<()> {
        let mut call_stack = self.call_stack.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire call stack lock"))?;

        let call_context = FunctionCallContext {

        call_stack.push(call_context);
        
        debug!(function = %function_name, "Pushed function context onto call stack");
        Ok(())
    /// Pop function context from call stack
    #[instrument(skip(self))]
    pub fn pop_function_context(&self) -> crate::error::Result<()> {
        let mut call_stack = self.call_stack.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire call stack lock"))?;

        let context = call_stack.pop();
        
        if let Some(ref ctx) = context {
            debug!(function = %ctx.function_name, "Popped function context from call stack");
        Ok(context)
    /// Create comprehensive error context from propagation error
    #[instrument(skip(self, propagation_error))]
    pub fn create_comprehensive_context<E: fmt::Debug + Clone>(
    ) -> crate::error::Result<()> {
        // Get current call stack
        let call_stack = self.get_current_call_stack()?;
        
        // Get source information
        let source_info = self.resolve_source_location(&propagation_error.propagation_site)?;
        
        // Create comprehensive context
        let context = EnhancedErrorContext {

        Ok(context)
    /// Add source file mapping for better error reporting
    #[instrument(skip(self, file_content))]
    pub fn add_source_mapping(
    ) -> crate::error::Result<()> {
        let mut mapper = self.source_mapper.write()
            .map_err(|_| CursedError::system_error("Failed to acquire source mapper lock"))?;

        mapper.add_file(file_path, file_content);
        Ok(())
    /// Get detailed error context by ID
    pub fn get_context(&self, context_id: &str) -> crate::error::Result<()> {
        let registry = self.context_registry.read()
            .map_err(|_| CursedError::system_error("Failed to acquire context registry lock"))?;

        Ok(registry.get_context(context_id))
    /// Get current function call stack
    pub fn get_current_call_stack(&self) -> crate::error::Result<()> {
        let call_stack = self.call_stack.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire call stack lock"))?;

        Ok(call_stack.get_stack())
    /// Create error chain linking related errors
    #[instrument(skip(self))]
    pub fn create_error_chain(
    ) -> crate::error::Result<()> {
        let mut chains = self.error_chains.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire error chains lock"))?;

        let chain_id = chains.create_chain(root_context_id, related_context_ids);
        
        debug!(chain_id = %chain_id, "Created error chain");
        Ok(chain_id)
    /// Get comprehensive error report
    #[instrument(skip(self))]
    pub fn generate_error_report(&self, context_id: &str) -> crate::error::Result<()> {
        let context = self.get_context(context_id)?
            .ok_or_else(|| CursedError::Runtime(format!("Context not found: {}", context_id)))?;

        // Get related error chains
        let chains = self.error_chains.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire error chains lock"))?;
        
        let related_chains = chains.get_chains_containing_context(context_id);

        let report = ErrorReport {

        info!(report_id = %report.report_id, context_id = %context_id, "Generated error report");
        Ok(report)
    /// Resolve source location to detailed information
    fn resolve_source_location(&self, location: &SourceLocation) -> crate::error::Result<()> {
        let mapper = self.source_mapper.read()
            .map_err(|_| CursedError::system_error("Failed to acquire source mapper lock"))?;

        Ok(mapper.get_source_info(location))
    /// Generate unique error ID
    fn generate_error_id(&self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        format!("err_{}", timestamp)
    }
}

/// Enhanced error context with comprehensive information
#[derive(Debug, Clone)]
pub struct EnhancedErrorContext {
    /// Unique identifier for this error context
    /// Source location where error occurred
    /// Function context information
    /// Current function call stack
    /// Detailed source information
    /// Type of error
    /// Full propagation chain
    /// Timestamp of error occurrence
    /// Additional context information
    /// Related error contexts
// impl fmt::Display for EnhancedErrorContext {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "CursedError Context [{}]:", self.error_id)?;
//         writeln!(f, "  Type: {}", self.error_type)?;
//         writeln!(f, "  Location: {}:{}", self.propagation_site.line, self.propagation_site.column)?;
//         
//         if let Some(ref func) = self.function_context {
//             writeln!(f, "  Function: {}", func)?;
//         }
// 
//         if let Some(ref source) = self.source_info {
//             writeln!(f, "  File: {}", source.file_path.display())?;
//             if let Some(ref line) = source.source_line {
//                 writeln!(f, "  Code: {}", line)?;
//             }
//         }
// 
//         if !self.call_stack.is_empty() {
//             writeln!(f, "  Call Stack:")?;
//             for (i, call) in self.call_stack.iter().enumerate() {
//                 writeln!(f, "    {}: {} at {}:{}", 
//                     i, call.function_name, call.source_location.line, call.source_location.column)?;
//             }
//         }
// 
//         if !self.propagation_chain.is_empty() {
//             writeln!(f, "  Propagation Chain: {} sites", self.propagation_chain.len())?;
//         }
// 
//         Ok(())
//     }
// }

/// Function call context information
#[derive(Debug, Clone)]
pub struct FunctionCallContext {
    /// Name of the function
    /// Source location of function call
    /// Function parameters (if available)
    /// Time when function was entered
    /// Local variables in scope
impl fmt::Display for FunctionCallContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.source_location.column)?;
        
        if let Some(ref params) = self.parameters {
            if !params.is_empty() {
                    params.iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect::<Vec<_>>()
                        .join(", "))?;
            }
        }
        
        Ok(())
    }
}

/// Function call stack manager
#[derive(Debug)]
pub struct FunctionCallStack {
    /// Stack of function calls
    /// Maximum stack depth
impl FunctionCallStack {
    /// Create new call stack
    pub fn new() -> Self {
        Self {
        }
    }

    /// Push function context
    pub fn push(&mut self, context: FunctionCallContext) {
        if self.stack.len() >= self.max_depth {
            // Remove oldest entry to prevent overflow
            self.stack.remove(0);
        }
        self.stack.push(context);
    /// Pop function context
    pub fn pop(&mut self) -> Option<FunctionCallContext> {
        self.stack.pop()
    /// Get current stack
    pub fn get_stack(&self) -> Vec<FunctionCallContext> {
        self.stack.clone()
    /// Get stack depth
    pub fn depth(&self) -> usize {
        self.stack.len()
    /// Clear stack
    pub fn clear(&mut self) {
        self.stack.clear();
    }
}

/// Source information for enhanced error reporting
#[derive(Debug, Clone)]
pub struct SourceInfo {
    /// Path to source file
    /// Source line content
    /// Surrounding context lines
    /// Column highlighting information
/// Source location mapper for file content resolution
#[derive(Debug)]
pub struct SourceLocationMapper {
    /// Mapping from file paths to file content
    /// Cached line mappings for quick lookup
impl SourceLocationMapper {
    /// Create new source mapper
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add file content for mapping
    pub fn add_file(&mut self, file_path: PathBuf, content: String) {
        let lines: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
        self.line_caches.insert(file_path.clone(), lines);
        self.file_contents.insert(file_path, content);
    /// Get source information for a location
    pub fn get_source_info(&self, location: &SourceLocation) -> Option<SourceInfo> {
        // Try to find the file containing this location
        for (file_path, lines) in &self.line_caches {
            if location.line > 0 && location.line <= lines.len() {
                let source_line = lines.get(location.line - 1).cloned();
                let context_lines = self.get_context_lines(lines, location.line, 3);
                
                return Some(SourceInfo {
                });
            }
        }
        None
    /// Get context lines around a specific line
    fn get_context_lines(&self, lines: &[String], target_line: usize, context: usize) -> Vec<String> {
        let start = target_line.saturating_sub(context + 1);
        let end = std::cmp::min(target_line + context, lines.len());
        lines[start..end].to_vec()
    }
}

/// CursedError context registry for managing error contexts
#[derive(Debug)]
pub struct ErrorContextRegistry {
    /// Map of context ID to error context
    /// Index by timestamp for chronological queries
    /// Maximum number of contexts to keep
impl ErrorContextRegistry {
    /// Create new registry
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add context and return its ID
    pub fn add_context(&mut self, context: EnhancedErrorContext) -> String {
        let context_id = context.error_id.clone();
        let timestamp = context.timestamp;

        // Clean up old contexts if needed
        if self.contexts.len() >= self.max_contexts {
            self.cleanup_old_contexts();
        self.timeline_index.insert(timestamp, context_id.clone());
        self.contexts.insert(context_id.clone(), context);
        
        context_id
    /// Get context by ID
    pub fn get_context(&self, context_id: &str) -> Option<EnhancedErrorContext> {
        self.contexts.get(context_id).cloned()
    /// Get contexts in chronological order
    pub fn get_contexts_by_time(&self, since: SystemTime) -> Vec<EnhancedErrorContext> {
        self.timeline_index
            .range(since..)
            .filter_map(|(_, context_id)| self.contexts.get(context_id))
            .cloned()
            .collect()
    /// Clean up old contexts
    fn cleanup_old_contexts(&mut self) {
        let cleanup_count = self.max_contexts / 4; // Remove 25% of contexts
        let mut to_remove = Vec::new();

        for (_, context_id) in self.timeline_index.iter().take(cleanup_count) {
            to_remove.push(context_id.clone());
        for context_id in to_remove {
            if let Some(context) = self.contexts.remove(&context_id) {
                self.timeline_index.remove(&context.timestamp);
            }
        }
    }
}

/// CursedError chain tracker for related errors
#[derive(Debug)]
pub struct ErrorChainTracker {
    /// Map of chain ID to error chain
    /// Index from context ID to chain IDs
impl ErrorChainTracker {
    /// Create new chain tracker
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create new error chain
    pub fn create_chain(&mut self, root_context: String, related_contexts: Vec<String>) -> String {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos());

        let chain = ErrorChain {

        // Update indices
        for context_id in std::iter::once(&root_context).chain(related_contexts.iter()) {
            self.context_to_chains
                .entry(context_id.clone())
                .or_insert_with(Vec::new)
                .push(chain_id.clone());
        self.chains.insert(chain_id.clone(), chain);
        chain_id
    /// Get chains containing a specific context
    pub fn get_chains_containing_context(&self, context_id: &str) -> Vec<ErrorChain> {
        self.context_to_chains
            .get(context_id)
            .map(|chain_ids| {
                chain_ids.iter()
                    .filter_map(|id| self.chains.get(id))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// CursedError chain linking related errors
#[derive(Debug, Clone)]
pub struct ErrorChain {
    /// Unique chain identifier
    /// Root error context
    /// Related error contexts
    /// When chain was created
/// Additional context information for errors
#[derive(Debug, Clone)]
pub struct ErrorContextInfo {
    /// User-provided description
    /// Associated data/state
    /// External system information
    /// Recovery suggestions
/// Comprehensive error report
#[derive(Debug, Clone)]
pub struct ErrorReport {
    /// Primary error context
    /// Related error chains
    /// When report was generated
    /// Unique report identifier
// impl fmt::Display for ErrorReport {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "CursedError Report [{}]", self.report_id)?;
//         writeln!(f, "Generated at: {:?}", self.generated_at)?;
//         writeln!(f)?;
//         writeln!(f, "{}", self.context)?;
// 
//         if !self.related_chains.is_empty() {
//             writeln!(f, "Related CursedError Chains:")?;
//             for chain in &self.related_chains {
//                 writeln!(f, "  Chain {}: {} related errors", 
//                     chain.chain_id, chain.related_contexts.len())?;
//             }
//         }
// 
//         Ok(())
//     }
// }

/// Configuration for error context management
#[derive(Debug, Clone)]
pub struct ContextManagerConfig {
    /// Maximum number of contexts to keep
    /// Maximum call stack depth
    /// Whether to collect detailed source information
    /// Number of context lines to include
    /// Whether to enable error chaining
impl Default for ContextManagerConfig {
    fn default() -> Self {
        Self {
        }
    }
