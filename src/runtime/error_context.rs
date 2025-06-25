// Error context management for error propagation in CURSED
//
// This module provides comprehensive error context tracking, including
// source location management, function call chains, and error propagation
// history for enhanced debugging and error reporting.

use crate::error_types::Error;
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
    pub context_registry: Arc<RwLock<ErrorContextRegistry>>,
    /// Function call stack tracker
    pub call_stack: Arc<Mutex<FunctionCallStack>>,
    /// Source location mapper for file/line resolution
    pub source_mapper: Arc<RwLock<SourceLocationMapper>>,
    /// Error chain tracker for related errors
    pub error_chains: Arc<Mutex<ErrorChainTracker>>,
    /// Configuration for context management
    pub config: ContextManagerConfig,
}

impl ErrorContextManager {
    /// Create a new error context manager
    pub fn new() -> Self {
        Self::with_config(ContextManagerConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(config: ContextManagerConfig) -> Self {
        Self {
            context_registry: Arc::new(RwLock::new(ErrorContextRegistry::new())),
            call_stack: Arc::new(Mutex::new(FunctionCallStack::new())),
            source_mapper: Arc::new(RwLock::new(SourceLocationMapper::new())),
            error_chains: Arc::new(Mutex::new(ErrorChainTracker::new())),
            config,
        }
    }

    /// Register a new error context
    #[instrument(skip(self, context))]
    pub fn register_context(&self, context: EnhancedErrorContext) -> Result<(), Error> {
        let mut registry = self.context_registry.write()
            .map_err(|_| Error::system_error("Failed to acquire context registry lock"))?;

        let context_id = registry.add_context(context);
        
        debug!(context_id = %context_id, "Registered new error context");
        Ok(context_id)
    }

    /// Push function context onto call stack
    #[instrument(skip(self))]
    pub fn push_function_context(
        &self,
        function_name: String,
        source_location: SourceLocation,
        parameters: Option<HashMap<String, String>>,
    ) -> Result<(), Error> {
        let mut call_stack = self.call_stack.lock()
            .map_err(|_| Error::system_error("Failed to acquire call stack lock"))?;

        let call_context = FunctionCallContext {
            function_name: function_name.clone(),
            source_location,
            parameters,
            entry_time: Instant::now(),
            local_variables: HashMap::new(),
        };

        call_stack.push(call_context);
        
        debug!(function = %function_name, "Pushed function context onto call stack");
        Ok(())
    }

    /// Pop function context from call stack
    #[instrument(skip(self))]
    pub fn pop_function_context(&self) -> Result<(), Error> {
        let mut call_stack = self.call_stack.lock()
            .map_err(|_| Error::system_error("Failed to acquire call stack lock"))?;

        let context = call_stack.pop();
        
        if let Some(ref ctx) = context {
            debug!(function = %ctx.function_name, "Popped function context from call stack");
        }
        
        Ok(context)
    }

    /// Create comprehensive error context from propagation error
    #[instrument(skip(self, propagation_error))]
    pub fn create_comprehensive_context<E: fmt::Debug + Clone>(
        &self,
        propagation_error: &PropagationError<E>,
        additional_info: Option<ErrorContextInfo>,
    ) -> Result<(), Error> {
        // Get current call stack
        let call_stack = self.get_current_call_stack()?;
        
        // Get source information
        let source_info = self.resolve_source_location(&propagation_error.propagation_site)?;
        
        // Create comprehensive context
        let context = EnhancedErrorContext {
            error_id: self.generate_error_id(),
            propagation_site: propagation_error.propagation_site.clone(),
            function_context: propagation_error.function_context.clone(),
            call_stack,
            source_info,
            error_type: format!("{:?}", propagation_error.inner_error),
            propagation_chain: propagation_error.full_chain(),
            timestamp: SystemTime::now(),
            additional_info,
            related_errors: Vec::new(),
        };

        Ok(context)
    }

    /// Add source file mapping for better error reporting
    #[instrument(skip(self, file_content))]
    pub fn add_source_mapping(
        &self,
        file_path: PathBuf,
        file_content: String,
    ) -> Result<(), Error> {
        let mut mapper = self.source_mapper.write()
            .map_err(|_| Error::system_error("Failed to acquire source mapper lock"))?;

        mapper.add_file(file_path, file_content);
        Ok(())
    }

    /// Get detailed error context by ID
    pub fn get_context(&self, context_id: &str) -> Result<(), Error> {
        let registry = self.context_registry.read()
            .map_err(|_| Error::system_error("Failed to acquire context registry lock"))?;

        Ok(registry.get_context(context_id))
    }

    /// Get current function call stack
    pub fn get_current_call_stack(&self) -> Result<(), Error> {
        let call_stack = self.call_stack.lock()
            .map_err(|_| Error::system_error("Failed to acquire call stack lock"))?;

        Ok(call_stack.get_stack())
    }

    /// Create error chain linking related errors
    #[instrument(skip(self))]
    pub fn create_error_chain(
        &self,
        root_context_id: String,
        related_context_ids: Vec<String>,
    ) -> Result<(), Error> {
        let mut chains = self.error_chains.lock()
            .map_err(|_| Error::system_error("Failed to acquire error chains lock"))?;

        let chain_id = chains.create_chain(root_context_id, related_context_ids);
        
        debug!(chain_id = %chain_id, "Created error chain");
        Ok(chain_id)
    }

    /// Get comprehensive error report
    #[instrument(skip(self))]
    pub fn generate_error_report(&self, context_id: &str) -> Result<(), Error> {
        let context = self.get_context(context_id)?
            .ok_or_else(|| Error::Runtime(format!("Context not found: {}", context_id)))?;

        // Get related error chains
        let chains = self.error_chains.lock()
            .map_err(|_| Error::system_error("Failed to acquire error chains lock"))?;
        
        let related_chains = chains.get_chains_containing_context(context_id);

        let report = ErrorReport {
            context,
            related_chains,
            generated_at: SystemTime::now(),
            report_id: self.generate_error_id(),
        };

        info!(report_id = %report.report_id, context_id = %context_id, "Generated error report");
        Ok(report)
    }

    /// Resolve source location to detailed information
    fn resolve_source_location(&self, location: &SourceLocation) -> Result<(), Error> {
        let mapper = self.source_mapper.read()
            .map_err(|_| Error::system_error("Failed to acquire source mapper lock"))?;

        Ok(mapper.get_source_info(location))
    }

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
    pub error_id: String,
    /// Source location where error occurred
    pub propagation_site: SourceLocation,
    /// Function context information
    pub function_context: Option<String>,
    /// Current function call stack
    pub call_stack: Vec<FunctionCallContext>,
    /// Detailed source information
    pub source_info: Option<SourceInfo>,
    /// Type of error
    pub error_type: String,
    /// Full propagation chain
    pub propagation_chain: Vec<SourceLocation>,
    /// Timestamp of error occurrence
    pub timestamp: SystemTime,
    /// Additional context information
    pub additional_info: Option<ErrorContextInfo>,
    /// Related error contexts
    pub related_errors: Vec<String>,
}

impl fmt::Display for EnhancedErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error Context [{}]:", self.error_id)?;
        writeln!(f, "  Type: {}", self.error_type)?;
        writeln!(f, "  Location: {}:{}", self.propagation_site.line, self.propagation_site.column)?;
        
        if let Some(ref func) = self.function_context {
            writeln!(f, "  Function: {}", func)?;
        }

        if let Some(ref source) = self.source_info {
            writeln!(f, "  File: {}", source.file_path.display())?;
            if let Some(ref line) = source.source_line {
                writeln!(f, "  Code: {}", line)?;
            }
        }

        if !self.call_stack.is_empty() {
            writeln!(f, "  Call Stack:")?;
            for (i, call) in self.call_stack.iter().enumerate() {
                writeln!(f, "    {}: {} at {}:{}", 
                    i, call.function_name, call.source_location.line, call.source_location.column)?;
            }
        }

        if !self.propagation_chain.is_empty() {
            writeln!(f, "  Propagation Chain: {} sites", self.propagation_chain.len())?;
        }

        Ok(())
    }
}

/// Function call context information
#[derive(Debug, Clone)]
pub struct FunctionCallContext {
    /// Name of the function
    pub function_name: String,
    /// Source location of function call
    pub source_location: SourceLocation,
    /// Function parameters (if available)
    pub parameters: Option<HashMap<String, String>>,
    /// Time when function was entered
    pub entry_time: Instant,
    /// Local variables in scope
    pub local_variables: HashMap<String, String>,
}

impl fmt::Display for FunctionCallContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}:{}", 
            self.function_name, 
            self.source_location.line, 
            self.source_location.column)?;
        
        if let Some(ref params) = self.parameters {
            if !params.is_empty() {
                write!(f, " ({})", 
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
    stack: Vec<FunctionCallContext>,
    /// Maximum stack depth
    max_depth: usize,
}

impl FunctionCallStack {
    /// Create new call stack
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            max_depth: 1000,
        }
    }

    /// Push function context
    pub fn push(&mut self, context: FunctionCallContext) {
        if self.stack.len() >= self.max_depth {
            // Remove oldest entry to prevent overflow
            self.stack.remove(0);
        }
        self.stack.push(context);
    }

    /// Pop function context
    pub fn pop(&mut self) -> Option<FunctionCallContext> {
        self.stack.pop()
    }

    /// Get current stack
    pub fn get_stack(&self) -> Vec<FunctionCallContext> {
        self.stack.clone()
    }

    /// Get stack depth
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Clear stack
    pub fn clear(&mut self) {
        self.stack.clear();
    }
}

/// Source information for enhanced error reporting
#[derive(Debug, Clone)]
pub struct SourceInfo {
    /// Path to source file
    pub file_path: PathBuf,
    /// Source line content
    pub source_line: Option<String>,
    /// Surrounding context lines
    pub context_lines: Vec<String>,
    /// Column highlighting information
    pub column_highlight: Option<(usize, usize)>,
}

/// Source location mapper for file content resolution
#[derive(Debug)]
pub struct SourceLocationMapper {
    /// Mapping from file paths to file content
    file_contents: HashMap<PathBuf, String>,
    /// Cached line mappings for quick lookup
    line_caches: HashMap<PathBuf, Vec<String>>,
}

impl SourceLocationMapper {
    /// Create new source mapper
    pub fn new() -> Self {
        Self {
            file_contents: HashMap::new(),
            line_caches: HashMap::new(),
        }
    }

    /// Add file content for mapping
    pub fn add_file(&mut self, file_path: PathBuf, content: String) {
        let lines: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
        self.line_caches.insert(file_path.clone(), lines);
        self.file_contents.insert(file_path, content);
    }

    /// Get source information for a location
    pub fn get_source_info(&self, location: &SourceLocation) -> Option<SourceInfo> {
        // Try to find the file containing this location
        for (file_path, lines) in &self.line_caches {
            if location.line > 0 && location.line <= lines.len() {
                let source_line = lines.get(location.line - 1).cloned();
                let context_lines = self.get_context_lines(lines, location.line, 3);
                
                return Some(SourceInfo {
                    file_path: file_path.clone(),
                    source_line,
                    context_lines,
                    column_highlight: Some((location.column, location.column + 1)),
                });
            }
        }
        None
    }

    /// Get context lines around a specific line
    fn get_context_lines(&self, lines: &[String], target_line: usize, context: usize) -> Vec<String> {
        let start = target_line.saturating_sub(context + 1);
        let end = std::cmp::min(target_line + context, lines.len());
        lines[start..end].to_vec()
    }
}

/// Error context registry for managing error contexts
#[derive(Debug)]
pub struct ErrorContextRegistry {
    /// Map of context ID to error context
    contexts: HashMap<String, EnhancedErrorContext>,
    /// Index by timestamp for chronological queries
    timeline_index: BTreeMap<SystemTime, String>,
    /// Maximum number of contexts to keep
    max_contexts: usize,
}

impl ErrorContextRegistry {
    /// Create new registry
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
            timeline_index: BTreeMap::new(),
            max_contexts: 10000,
        }
    }

    /// Add context and return its ID
    pub fn add_context(&mut self, context: EnhancedErrorContext) -> String {
        let context_id = context.error_id.clone();
        let timestamp = context.timestamp;

        // Clean up old contexts if needed
        if self.contexts.len() >= self.max_contexts {
            self.cleanup_old_contexts();
        }

        self.timeline_index.insert(timestamp, context_id.clone());
        self.contexts.insert(context_id.clone(), context);
        
        context_id
    }

    /// Get context by ID
    pub fn get_context(&self, context_id: &str) -> Option<EnhancedErrorContext> {
        self.contexts.get(context_id).cloned()
    }

    /// Get contexts in chronological order
    pub fn get_contexts_by_time(&self, since: SystemTime) -> Vec<EnhancedErrorContext> {
        self.timeline_index
            .range(since..)
            .filter_map(|(_, context_id)| self.contexts.get(context_id))
            .cloned()
            .collect()
    }

    /// Clean up old contexts
    fn cleanup_old_contexts(&mut self) {
        let cleanup_count = self.max_contexts / 4; // Remove 25% of contexts
        let mut to_remove = Vec::new();

        for (_, context_id) in self.timeline_index.iter().take(cleanup_count) {
            to_remove.push(context_id.clone());
        }

        for context_id in to_remove {
            if let Some(context) = self.contexts.remove(&context_id) {
                self.timeline_index.remove(&context.timestamp);
            }
        }
    }
}

/// Error chain tracker for related errors
#[derive(Debug)]
pub struct ErrorChainTracker {
    /// Map of chain ID to error chain
    chains: HashMap<String, ErrorChain>,
    /// Index from context ID to chain IDs
    context_to_chains: HashMap<String, Vec<String>>,
}

impl ErrorChainTracker {
    /// Create new chain tracker
    pub fn new() -> Self {
        Self {
            chains: HashMap::new(),
            context_to_chains: HashMap::new(),
        }
    }

    /// Create new error chain
    pub fn create_chain(&mut self, root_context: String, related_contexts: Vec<String>) -> String {
        let chain_id = format!("chain_{}", 
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos());

        let chain = ErrorChain {
            chain_id: chain_id.clone(),
            root_context: root_context.clone(),
            related_contexts: related_contexts.clone(),
            created_at: SystemTime::now(),
        };

        // Update indices
        for context_id in std::iter::once(&root_context).chain(related_contexts.iter()) {
            self.context_to_chains
                .entry(context_id.clone())
                .or_insert_with(Vec::new)
                .push(chain_id.clone());
        }

        self.chains.insert(chain_id.clone(), chain);
        chain_id
    }

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

/// Error chain linking related errors
#[derive(Debug, Clone)]
pub struct ErrorChain {
    /// Unique chain identifier
    pub chain_id: String,
    /// Root error context
    pub root_context: String,
    /// Related error contexts
    pub related_contexts: Vec<String>,
    /// When chain was created
    pub created_at: SystemTime,
}

/// Additional context information for errors
#[derive(Debug, Clone)]
pub struct ErrorContextInfo {
    /// User-provided description
    pub description: Option<String>,
    /// Associated data/state
    pub state_snapshot: Option<HashMap<String, String>>,
    /// External system information
    pub external_context: Option<HashMap<String, String>>,
    /// Recovery suggestions
    pub recovery_suggestions: Vec<String>,
}

/// Comprehensive error report
#[derive(Debug, Clone)]
pub struct ErrorReport {
    /// Primary error context
    pub context: EnhancedErrorContext,
    /// Related error chains
    pub related_chains: Vec<ErrorChain>,
    /// When report was generated
    pub generated_at: SystemTime,
    /// Unique report identifier
    pub report_id: String,
}

impl fmt::Display for ErrorReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error Report [{}]", self.report_id)?;
        writeln!(f, "Generated at: {:?}", self.generated_at)?;
        writeln!(f)?;
        writeln!(f, "{}", self.context)?;

        if !self.related_chains.is_empty() {
            writeln!(f, "Related Error Chains:")?;
            for chain in &self.related_chains {
                writeln!(f, "  Chain {}: {} related errors", 
                    chain.chain_id, chain.related_contexts.len())?;
            }
        }

        Ok(())
    }
}

/// Configuration for error context management
#[derive(Debug, Clone)]
pub struct ContextManagerConfig {
    /// Maximum number of contexts to keep
    pub max_contexts: usize,
    /// Maximum call stack depth
    pub max_call_stack_depth: usize,
    /// Whether to collect detailed source information
    pub collect_source_info: bool,
    /// Number of context lines to include
    pub context_lines: usize,
    /// Whether to enable error chaining
    pub enable_error_chaining: bool,
}

impl Default for ContextManagerConfig {
    fn default() -> Self {
        Self {
            max_contexts: 10000,
            max_call_stack_depth: 1000,
            collect_source_info: true,
            context_lines: 3,
            enable_error_chaining: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_error_context_manager_creation() {
        let manager = ErrorContextManager::new();
        let call_stack = manager.get_current_call_stack().unwrap();
        assert!(call_stack.is_empty());
    }

    #[test]
    fn test_function_call_stack() {
        let manager = ErrorContextManager::new();
        let location = SourceLocation::new(1, 5);

        manager.push_function_context(
            "test_function".to_string(),
            location,
            None,
        ).unwrap();

        let stack = manager.get_current_call_stack().unwrap();
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].function_name, "test_function");

        let popped = manager.pop_function_context().unwrap();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap().function_name, "test_function");
    }

    #[test]
    fn test_source_location_mapper() {
        let mut mapper = SourceLocationMapper::new();
        let content = "line 1\nline 2\nline 3\n".to_string();
        let path = PathBuf::from("test.csd");

        mapper.add_file(path.clone(), content);

        let location = SourceLocation::new(2, 1);
        let source_info = mapper.get_source_info(&location);
        
        assert!(source_info.is_some());
        let info = source_info.unwrap();
        assert_eq!(info.file_path, path);
        assert_eq!(info.source_line, Some("line 2".to_string()));
    }

    #[test]
    fn test_error_context_registry() {
        let mut registry = ErrorContextRegistry::new();
        
        let context = EnhancedErrorContext {
            error_id: "test_error".to_string(),
            propagation_site: SourceLocation::new(1, 5),
            function_context: None,
            call_stack: Vec::new(),
            source_info: None,
            error_type: "TestError".to_string(),
            propagation_chain: Vec::new(),
            timestamp: SystemTime::now(),
            additional_info: None,
            related_errors: Vec::new(),
        };

        let context_id = registry.add_context(context.clone());
        assert_eq!(context_id, "test_error");

        let retrieved = registry.get_context(&context_id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().error_id, context_id);
    }

    #[test]
    fn test_error_chain_tracker() {
        let mut tracker = ErrorChainTracker::new();
        
        let chain_id = tracker.create_chain(
            "root_error".to_string(),
            vec!["related_1".to_string(), "related_2".to_string()],
        );

        let chains = tracker.get_chains_containing_context("root_error");
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].chain_id, chain_id);

        let related_chains = tracker.get_chains_containing_context("related_1");
        assert_eq!(related_chains.len(), 1);
    }

    #[test]
    fn test_comprehensive_context_creation() {
        use crate::runtime::error_propagation::{PropagationError, NoneError};
        
        let manager = ErrorContextManager::new();
        
        // Create a propagation error
        let none_error = NoneError {
            message: "Test error".to_string(),
            location: SourceLocation::new(1, 5),
        };
        
        let propagation_error = PropagationError::new(
            none_error,
            SourceLocation::new(1, 5),
            Some("test_function".to_string()),
        );

        let context = manager.create_comprehensive_context(&propagation_error, None).unwrap();
        
        assert_eq!(context.propagation_site.line, 1);
        assert_eq!(context.function_context, Some("test_function".to_string()));
        assert_eq!(context.propagation_chain.len(), 1);
    }

    #[test]
    fn test_error_report_generation() {
        let manager = ErrorContextManager::new();
        
        let context = EnhancedErrorContext {
            error_id: "test_error".to_string(),
            propagation_site: SourceLocation::new(1, 5),
            function_context: Some("test_function".to_string()),
            call_stack: Vec::new(),
            source_info: None,
            error_type: "TestError".to_string(),
            propagation_chain: Vec::new(),
            timestamp: SystemTime::now(),
            additional_info: None,
            related_errors: Vec::new(),
        };

        let context_id = manager.register_context(context).unwrap();
        let report = manager.generate_error_report(&context_id).unwrap();
        
        assert_eq!(report.context.error_id, context_id);
        assert!(report.related_chains.is_empty());
    }
}
