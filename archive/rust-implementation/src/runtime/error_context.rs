//! Error context and recovery information for CURSED runtime
//!
//! Provides detailed error context, recovery information, and error analysis
//! capabilities for the CURSED runtime system.

use crate::error_types::{Error, Result as CursedResult};
use crate::runtime::stack_trace::{StackTrace, StackFrame};
use crate::runtime::error_propagation::{ErrorSeverity, RecoveryHint, RecoveryAction};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

/// Comprehensive error context with recovery information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Unique error ID for tracking
    pub error_id: String,
    /// Primary error
    pub primary_error: Error,
    /// Stack trace at error occurrence
    pub stack_trace: Option<StackTrace>,
    /// Error chain (nested/related errors)
    pub error_chain: Vec<ErrorLink>,
    /// Contextual data at time of error
    pub context_data: HashMap<String, ContextValue>,
    /// Recovery suggestions
    pub recovery_suggestions: Vec<RecoverySuggestion>,
    /// Error severity and classification
    pub classification: ErrorClassification,
    /// Timing information
    pub timing: ErrorTiming,
    /// Environment information
    pub environment: ErrorEnvironment,
    /// User-provided annotations
    pub annotations: Vec<String>,
}

/// Link in the error chain
#[derive(Debug, Clone)]
pub struct ErrorLink {
    /// The error in the chain
    pub error: Error,
    /// Relationship to previous error
    pub relationship: ErrorRelationship,
    /// Context at this point in the chain
    pub context: HashMap<String, ContextValue>,
    /// Timestamp when this error was added
    pub timestamp: Instant,
}

/// Relationship between errors in a chain
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorRelationship {
    /// This error caused the next error
    Caused,
    /// This error is related to the next error
    Related,
    /// This error occurred while handling the next error
    HandlingError,
    /// This error is a retry of the next error
    Retry,
    /// This error is a fallback from the next error
    Fallback,
}

impl fmt::Display for ErrorRelationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorRelationship::Caused => write!(f, "caused"),
            ErrorRelationship::Related => write!(f, "related to"),
            ErrorRelationship::HandlingError => write!(f, "occurred while handling"),
            ErrorRelationship::Retry => write!(f, "retry of"),
            ErrorRelationship::Fallback => write!(f, "fallback from"),
        }
    }
}

/// Context value that can hold different types of data
#[derive(Debug, Clone)]
pub enum ContextValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<ContextValue>),
    Object(HashMap<String, ContextValue>),
    Binary(Vec<u8>),
}

impl fmt::Display for ContextValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextValue::String(s) => write!(f, "\"{}\"", s),
            ContextValue::Integer(i) => write!(f, "{}", i),
            ContextValue::Float(fl) => write!(f, "{}", fl),
            ContextValue::Boolean(b) => write!(f, "{}", b),
            ContextValue::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            ContextValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "\"{}\": {}", key, value)?;
                }
                write!(f, "}}")
            }
            ContextValue::Binary(data) => write!(f, "<binary: {} bytes>", data.len()),
        }
    }
}

/// Detailed recovery suggestion
#[derive(Debug, Clone)]
pub struct RecoverySuggestion {
    /// Suggestion ID for tracking
    pub id: String,
    /// Human-readable description
    pub description: String,
    /// Recovery action type
    pub action: RecoveryAction,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Estimated cost of recovery (time, resources)
    pub cost: RecoveryCost,
    /// Prerequisites for this recovery
    pub prerequisites: Vec<String>,
    /// Expected outcome
    pub expected_outcome: String,
    /// Code example (if applicable)
    pub code_example: Option<String>,
}

/// Cost estimation for recovery actions
#[derive(Debug, Clone)]
pub struct RecoveryCost {
    /// Time cost in milliseconds
    pub time_ms: Option<u64>,
    /// Memory cost in bytes
    pub memory_bytes: Option<usize>,
    /// Network operations required
    pub network_ops: Option<usize>,
    /// Disk operations required
    pub disk_ops: Option<usize>,
    /// Overall cost level
    pub level: CostLevel,
}

/// Cost level categories
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CostLevel {
    Free,      // No significant cost
    Low,       // Minimal cost
    Medium,    // Moderate cost
    High,      // Significant cost
    Expensive, // Very high cost
}

impl fmt::Display for CostLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CostLevel::Free => write!(f, "free"),
            CostLevel::Low => write!(f, "low"),
            CostLevel::Medium => write!(f, "medium"),
            CostLevel::High => write!(f, "high"),
            CostLevel::Expensive => write!(f, "expensive"),
        }
    }
}

/// Error classification information
#[derive(Debug, Clone)]
pub struct ErrorClassification {
    /// Error severity
    pub severity: ErrorSeverity,
    /// Error category
    pub category: ErrorCategory,
    /// Whether error is transient (might succeed on retry)
    pub is_transient: bool,
    /// Whether error is recoverable
    pub is_recoverable: bool,
    /// Whether error indicates a bug
    pub indicates_bug: bool,
    /// Confidence in classification (0.0 to 1.0)
    pub confidence: f64,
}

/// Error category for classification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Logic error in the program
    Logic,
    /// Resource unavailable (memory, disk, network)
    Resource,
    /// Configuration or setup error
    Configuration,
    /// Input/output error
    IO,
    /// Network-related error
    Network,
    /// Security or permission error
    Security,
    /// Data format or validation error
    Data,
    /// Timeout or timing error
    Timeout,
    /// Concurrency or thread safety error
    Concurrency,
    /// External dependency error
    Dependency,
    /// Unknown or unclassified error
    Unknown,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCategory::Logic => write!(f, "logic"),
            ErrorCategory::Resource => write!(f, "resource"),
            ErrorCategory::Configuration => write!(f, "configuration"),
            ErrorCategory::IO => write!(f, "I/O"),
            ErrorCategory::Network => write!(f, "network"),
            ErrorCategory::Security => write!(f, "security"),
            ErrorCategory::Data => write!(f, "data"),
            ErrorCategory::Timeout => write!(f, "timeout"),
            ErrorCategory::Concurrency => write!(f, "concurrency"),
            ErrorCategory::Dependency => write!(f, "dependency"),
            ErrorCategory::Unknown => write!(f, "unknown"),
        }
    }
}

/// Timing information for error occurrence
#[derive(Debug, Clone)]
pub struct ErrorTiming {
    /// When the error occurred
    pub occurred_at: Instant,
    /// Duration of the operation that failed
    pub operation_duration: Option<Duration>,
    /// Time since program start
    pub uptime: Duration,
    /// Time since last similar error
    pub time_since_similar: Option<Duration>,
}

/// Environment information at error time
#[derive(Debug, Clone)]
pub struct ErrorEnvironment {
    /// Thread ID where error occurred
    pub thread_id: Option<usize>,
    /// Goroutine ID (if applicable)
    pub goroutine_id: Option<usize>,
    /// Function name where error originated
    pub function_name: Option<String>,
    /// File and line number
    pub source_location: Option<(String, usize)>,
    /// Memory usage at error time
    pub memory_usage: Option<usize>,
    /// CPU usage at error time
    pub cpu_usage: Option<f64>,
    /// Number of active goroutines
    pub active_goroutines: Option<usize>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(error: Error) -> Self {
        let now = Instant::now();
        let error_id = Self::generate_error_id();
        
        Self {
            error_id,
            primary_error: error,
            stack_trace: None,
            error_chain: Vec::new(),
            context_data: HashMap::new(),
            recovery_suggestions: Vec::new(),
            classification: ErrorClassification {
                severity: ErrorSeverity::Error,
                category: ErrorCategory::Unknown,
                is_transient: false,
                is_recoverable: true,
                indicates_bug: false,
                confidence: 0.5,
            },
            timing: ErrorTiming {
                occurred_at: now,
                operation_duration: None,
                uptime: now.elapsed(),
                time_since_similar: None,
            },
            environment: ErrorEnvironment {
                thread_id: None,
                goroutine_id: None,
                function_name: None,
                source_location: None,
                memory_usage: None,
                cpu_usage: None,
                active_goroutines: None,
            },
            annotations: Vec::new(),
        }
    }

    /// Create error context with stack trace
    pub fn with_stack_trace(error: Error, stack_trace: StackTrace) -> Self {
        let mut context = Self::new(error);
        context.stack_trace = Some(stack_trace);
        context
    }

    /// Generate a unique error ID
    fn generate_error_id() -> String {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        Instant::now().hash(&mut hasher);
        std::thread::current().id().hash(&mut hasher);
        format!("err_{:x}", hasher.finish())
    }

    /// Add context data
    pub fn add_context<K: Into<String>>(&mut self, key: K, value: ContextValue) {
        self.context_data.insert(key.into(), value);
    }

    /// Add string context
    pub fn add_string_context<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.context_data.insert(key.into(), ContextValue::String(value.into()));
    }

    /// Add error to chain
    pub fn add_to_chain(&mut self, error: Error, relationship: ErrorRelationship) {
        let link = ErrorLink {
            error,
            relationship,
            context: HashMap::new(),
            timestamp: Instant::now(),
        };
        self.error_chain.push(link);
    }

    /// Add recovery suggestion
    pub fn add_recovery_suggestion(&mut self, suggestion: RecoverySuggestion) {
        self.recovery_suggestions.push(suggestion);
    }

    /// Set error classification
    pub fn set_classification(&mut self, classification: ErrorClassification) {
        self.classification = classification;
    }

    /// Set environment information
    pub fn set_environment(&mut self, environment: ErrorEnvironment) {
        self.environment = environment;
    }

    /// Add annotation
    pub fn add_annotation<S: Into<String>>(&mut self, annotation: S) {
        self.annotations.push(annotation.into());
    }

    /// Get best recovery suggestion
    pub fn best_recovery_suggestion(&self) -> Option<&RecoverySuggestion> {
        self.recovery_suggestions
            .iter()
            .max_by(|a, b| {
                a.confidence.partial_cmp(&b.confidence)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| b.cost.level.cmp(&a.cost.level))
            })
    }

    /// Get recovery suggestions by action type
    pub fn suggestions_by_action(&self, action: &RecoveryAction) -> Vec<&RecoverySuggestion> {
        self.recovery_suggestions
            .iter()
            .filter(|s| &s.action == action)
            .collect()
    }

    /// Check if error matches a pattern
    pub fn matches_pattern(&self, pattern: &ErrorPattern) -> bool {
        pattern.matches(self)
    }

    /// Get full error report
    pub fn full_report(&self) -> String {
        let mut report = String::new();
        
        // Header
        report.push_str(&format!("Error Report [ID: {}]\\n", self.error_id));
        report.push_str(&format!("Occurred: {:?}\\n", self.timing.occurred_at));
        report.push_str(&format!("Severity: {} | Category: {}\\n", 
            self.classification.severity, self.classification.category));
        report.push_str("\\n");
        
        // Primary error
        report.push_str(&format!("Primary Error: {}\\n", self.primary_error));
        
        // Error chain
        if !self.error_chain.is_empty() {
            report.push_str("\\nError Chain:\\n");
            for (i, link) in self.error_chain.iter().enumerate() {
                report.push_str(&format!("  {}: {} -> {}\\n", 
                    i + 1, link.relationship, link.error));
            }
        }
        
        // Context data
        if !self.context_data.is_empty() {
            report.push_str("\\nContext Data:\\n");
            for (key, value) in &self.context_data {
                report.push_str(&format!("  {}: {}\\n", key, value));
            }
        }
        
        // Stack trace
        if let Some(ref trace) = self.stack_trace {
            report.push_str("\\nStack Trace:\\n");
            report.push_str(&trace.format_trace());
        }
        
        // Recovery suggestions
        if !self.recovery_suggestions.is_empty() {
            report.push_str("\\nRecovery Suggestions:\\n");
            for (i, suggestion) in self.recovery_suggestions.iter().enumerate() {
                report.push_str(&format!("  {}: {} (confidence: {:.1}%, cost: {})\\n", 
                    i + 1, suggestion.description, suggestion.confidence * 100.0, suggestion.cost.level));
            }
        }
        
        // Environment
        report.push_str("\\nEnvironment:\\n");
        if let Some(thread_id) = self.environment.thread_id {
            report.push_str(&format!("  Thread ID: {}\\n", thread_id));
        }
        if let Some(ref func_name) = self.environment.function_name {
            report.push_str(&format!("  Function: {}\\n", func_name));
        }
        if let Some((ref file, line)) = self.environment.source_location {
            report.push_str(&format!("  Location: {}:{}\\n", file, line));
        }
        
        // Annotations
        if !self.annotations.is_empty() {
            report.push_str("\\nAnnotations:\\n");
            for annotation in &self.annotations {
                report.push_str(&format!("  - {}\\n", annotation));
            }
        }
        
        report
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} - {}", 
            self.classification.severity, 
            self.error_id, 
            self.primary_error)
    }
}

/// Pattern for matching errors
#[derive(Debug, Clone)]
pub struct ErrorPattern {
    /// Error message pattern
    pub message_pattern: Option<String>,
    /// Error category
    pub category: Option<ErrorCategory>,
    /// Error severity
    pub severity: Option<ErrorSeverity>,
    /// Function name pattern
    pub function_pattern: Option<String>,
    /// File name pattern
    pub file_pattern: Option<String>,
}

impl ErrorPattern {
    /// Create a new error pattern
    pub fn new() -> Self {
        Self {
            message_pattern: None,
            category: None,
            severity: None,
            function_pattern: None,
            file_pattern: None,
        }
    }

    /// Set message pattern
    pub fn with_message_pattern<S: Into<String>>(mut self, pattern: S) -> Self {
        self.message_pattern = Some(pattern.into());
        self
    }

    /// Set category
    pub fn with_category(mut self, category: ErrorCategory) -> Self {
        self.category = Some(category);
        self
    }

    /// Set severity
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// Check if pattern matches error context
    pub fn matches(&self, context: &ErrorContext) -> bool {
        // Check message pattern
        if let Some(ref pattern) = self.message_pattern {
            let error_msg = format!("{}", context.primary_error);
            if !error_msg.contains(pattern) {
                return false;
            }
        }
        
        // Check category
        if let Some(ref category) = self.category {
            if &context.classification.category != category {
                return false;
            }
        }
        
        // Check severity
        if let Some(ref severity) = self.severity {
            if &context.classification.severity != severity {
                return false;
            }
        }
        
        // Check function pattern
        if let Some(ref pattern) = self.function_pattern {
            if let Some(ref func_name) = context.environment.function_name {
                if !func_name.contains(pattern) {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        // Check file pattern
        if let Some(ref pattern) = self.file_pattern {
            if let Some((ref file, _)) = context.environment.source_location {
                if !file.contains(pattern) {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        true
    }
}

impl Default for ErrorPattern {
    fn default() -> Self {
        Self::new()
    }
}

/// Error context manager
pub struct ErrorContextManager {
    /// Active error contexts
    active_contexts: Mutex<HashMap<String, ErrorContext>>,
    /// Context history
    context_history: Mutex<Vec<ErrorContext>>,
    /// Maximum history size
    max_history_size: usize,
    /// Analysis patterns
    patterns: Mutex<Vec<ErrorPattern>>,
}

impl ErrorContextManager {
    /// Create a new error context manager
    pub fn new() -> Self {
        Self {
            active_contexts: Mutex::new(HashMap::new()),
            context_history: Mutex::new(Vec::new()),
            max_history_size: 1000,
            patterns: Mutex::new(Vec::new()),
        }
    }

    /// Register an error context
    pub fn register_context(&self, context: ErrorContext) -> String {
        let error_id = context.error_id.clone();
        
        {
            let mut active = self.active_contexts.lock().unwrap();
            active.insert(error_id.clone(), context);
        }
        
        error_id
    }

    /// Get error context by ID
    pub fn get_context(&self, error_id: &str) -> Option<ErrorContext> {
        let active = self.active_contexts.lock().unwrap();
        active.get(error_id).cloned()
    }

    /// Archive error context
    pub fn archive_context(&self, error_id: &str) -> bool {
        let context = {
            let mut active = self.active_contexts.lock().unwrap();
            active.remove(error_id)
        };
        
        if let Some(context) = context {
            let mut history = self.context_history.lock().unwrap();
            if history.len() >= self.max_history_size {
                history.remove(0); // Remove oldest
            }
            history.push(context);
            true
        } else {
            false
        }
    }

    /// Find contexts matching pattern
    pub fn find_contexts(&self, pattern: &ErrorPattern) -> Vec<ErrorContext> {
        let active = self.active_contexts.lock().unwrap();
        active.values()
            .filter(|ctx| pattern.matches(ctx))
            .cloned()
            .collect()
    }

    /// Get error statistics
    pub fn get_statistics(&self) -> ErrorStatistics {
        let active = self.active_contexts.lock().unwrap();
        let history = self.context_history.lock().unwrap();
        
        let mut stats = ErrorStatistics::default();
        stats.active_contexts = active.len();
        stats.total_contexts = active.len() + history.len();
        
        // Analyze categories
        for context in active.values().chain(history.iter()) {
            *stats.by_category.entry(context.classification.category.clone()).or_insert(0) += 1;
            *stats.by_severity.entry(context.classification.severity.clone()).or_insert(0) += 1;
        }
        
        stats
    }

    /// Clear all contexts
    pub fn clear_all(&self) {
        self.active_contexts.lock().unwrap().clear();
        self.context_history.lock().unwrap().clear();
    }
}

impl Default for ErrorContextManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Error statistics
#[derive(Debug, Default, Clone)]
pub struct ErrorStatistics {
    pub active_contexts: usize,
    pub total_contexts: usize,
    pub by_category: HashMap<ErrorCategory, usize>,
    pub by_severity: HashMap<ErrorSeverity, usize>,
}

/// Global error context manager
static GLOBAL_ERROR_CONTEXT_MANAGER: std::sync::LazyLock<ErrorContextManager> = 
    std::sync::LazyLock::new(|| ErrorContextManager::new());

/// Get the global error context manager
pub fn get_global_error_context_manager() -> &'static ErrorContextManager {
    &GLOBAL_ERROR_CONTEXT_MANAGER
}

/// Utility functions for error context operations
pub mod utils {
    use super::*;

    /// Create a basic recovery suggestion
    pub fn create_basic_recovery_suggestion(
        description: &str,
        action: RecoveryAction,
        confidence: f64,
    ) -> RecoverySuggestion {
        RecoverySuggestion {
            id: format!("recovery_{}", rand::random::<u32>()),
            description: description.to_string(),
            action,
            confidence,
            cost: RecoveryCost {
                time_ms: None,
                memory_bytes: None,
                network_ops: None,
                disk_ops: None,
                level: CostLevel::Low,
            },
            prerequisites: Vec::new(),
            expected_outcome: "Error resolution".to_string(),
            code_example: None,
        }
    }

    /// Classify error automatically
    pub fn auto_classify_error(error: &Error) -> ErrorClassification {
        let (category, is_transient, indicates_bug) = match error {
            Error::Io(_) => (ErrorCategory::IO, true, false),
            Error::Runtime(msg) => {
                if msg.contains("timeout") {
                    (ErrorCategory::Timeout, true, false)
                } else if msg.contains("memory") {
                    (ErrorCategory::Resource, true, false)
                } else if msg.contains("network") {
                    (ErrorCategory::Network, true, false)
                } else {
                    (ErrorCategory::Logic, false, true)
                }
            }
            Error::Parse(_) => (ErrorCategory::Data, false, false),
            Error::TypeCheck(_) => (ErrorCategory::Logic, false, true),
            Error::Compile(_) => (ErrorCategory::Logic, false, true),
            Error::Import(_) => (ErrorCategory::Configuration, false, false),
            Error::Package(_) => (ErrorCategory::Dependency, true, false),
            Error::Memory(_) => (ErrorCategory::Resource, true, false),
            _ => (ErrorCategory::Unknown, false, false),
        };

        ErrorClassification {
            severity: ErrorSeverity::Error,
            category,
            is_transient,
            is_recoverable: is_transient || !indicates_bug,
            indicates_bug,
            confidence: 0.7,
        }
    }

    /// Create context value from string
    pub fn context_value_from_str(value: &str) -> ContextValue {
        // Try to parse as different types
        if let Ok(i) = value.parse::<i64>() {
            ContextValue::Integer(i)
        } else if let Ok(f) = value.parse::<f64>() {
            ContextValue::Float(f)
        } else if let Ok(b) = value.parse::<bool>() {
            ContextValue::Boolean(b)
        } else {
            ContextValue::String(value.to_string())
        }
    }
}

/// Public function that was likely being used by external code
pub fn get_minimal_result() -> CursedResult<String> {
    Ok("CURSED error context system initialized".to_string())
}
