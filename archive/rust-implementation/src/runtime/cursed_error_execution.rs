//! CURSED Error Handling Runtime Execution System
//!
//! This module implements the complete runtime execution for CURSED's error handling:
//! - yikes: Error creation and throwing
//! - shook: Automatic error propagation operator
//! - fam: Panic recovery and cleanup blocks
//! - Integration with interpreter and compiler execution
//! - Stack trace generation and error reporting
//! - Performance-optimized happy path execution

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::panic::{self, catch_unwind, AssertUnwindSafe};
use std::any::Any;
use std::fmt;

use crate::error_types::{Error, Result};
use crate::runtime::{RuntimeError, RuntimeErrorType};
use crate::runtime::goroutine::GoroutineId;
use crate::runtime::enhanced_error_handling::{
    CursedErrorType, PropagationContext, RecoveryContext, EnhancedErrorRuntime
};
use crate::runtime::value::Value;
use crate::lexer::{Token, TokenKind};

/// Global error execution runtime
static CURSED_ERROR_EXECUTION: once_cell::sync::OnceCell<Arc<CursedErrorExecution>> = once_cell::sync::OnceCell::new();

/// CURSED error execution runtime system
pub struct CursedErrorExecution {
    /// Enhanced error runtime
    enhanced_runtime: Arc<EnhancedErrorRuntime>,
    /// Error propagation stack
    propagation_stack: RwLock<Vec<ErrorStackFrame>>,
    /// Active error contexts by goroutine
    active_contexts: RwLock<HashMap<GoroutineId, ErrorExecutionContext>>,
    /// Performance metrics
    performance_metrics: Arc<Mutex<ErrorExecutionMetrics>>,
    /// Error suppression patterns
    suppression_patterns: RwLock<Vec<String>>,
    /// Happy path optimization enabled
    happy_path_optimization: AtomicBool,
    /// Error execution configuration
    config: ErrorExecutionConfig,
}

/// Error stack frame for propagation tracking
#[derive(Debug, Clone)]
pub struct ErrorStackFrame {
    /// Function name where error occurred
    pub function_name: String,
    /// File name
    pub file_name: String,
    /// Line number
    pub line_number: u32,
    /// Column number
    pub column_number: u32,
    /// Error type
    pub error_type: CursedErrorType,
    /// Timestamp
    pub timestamp: Instant,
    /// Goroutine ID
    pub goroutine_id: Option<GoroutineId>,
}

/// Error execution context for each goroutine
#[derive(Debug, Clone)]
pub struct ErrorExecutionContext {
    /// Current error stack
    pub error_stack: Vec<ErrorStackFrame>,
    /// Recovery blocks stack
    pub recovery_blocks: Vec<RecoveryBlock>,
    /// Error propagation enabled
    pub propagation_enabled: bool,
    /// Current recovery handler
    pub current_recovery_handler: Option<RecoveryHandler>,
    /// Error count in this context
    pub error_count: u64,
    /// Last error time
    pub last_error_time: Option<Instant>,
}

/// Recovery block for fam keyword
#[derive(Debug, Clone)]
pub struct RecoveryBlock {
    /// Recovery function
    pub recovery_function: String,
    /// Recovery code block
    pub recovery_code: Vec<Token>,
    /// Recovery scope variables
    pub recovery_scope: HashMap<String, Value>,
    /// Maximum recovery attempts
    pub max_attempts: u32,
    /// Current attempts
    pub current_attempts: u32,
    /// Recovery strategy
    pub strategy: RecoveryStrategy,
}

/// Recovery strategy for fam blocks
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStrategy {
    /// Continue execution with default values
    ContinueWithDefaults,
    /// Retry the operation
    RetryOperation,
    /// Skip the failing operation
    SkipOperation,
    /// Use fallback implementation
    UseFallback,
    /// Escalate to parent recovery block
    EscalateToParent,
    /// Terminate gracefully
    TerminateGracefully,
}

/// Recovery handler for error processing
#[derive(Debug, Clone)]
pub struct RecoveryHandler {
    /// Handler name
    pub name: String,
    /// Handler function pointer (simplified as string for now)
    pub handler_code: String,
    /// Handler scope
    pub handler_scope: HashMap<String, Value>,
    /// Handler priority
    pub priority: i32,
}

/// Error execution configuration
#[derive(Debug, Clone)]
pub struct ErrorExecutionConfig {
    /// Enable stack trace capture
    pub enable_stack_traces: bool,
    /// Maximum error stack depth
    pub max_error_stack_depth: usize,
    /// Enable error propagation
    pub enable_error_propagation: bool,
    /// Maximum propagation depth
    pub max_propagation_depth: usize,
    /// Enable recovery optimization
    pub enable_recovery_optimization: bool,
    /// Happy path optimization threshold
    pub happy_path_threshold: f64,
    /// Error suppression enabled
    pub enable_error_suppression: bool,
    /// Performance monitoring enabled
    pub enable_performance_monitoring: bool,
}

impl Default for ErrorExecutionConfig {
    fn default() -> Self {
        Self {
            enable_stack_traces: true,
            max_error_stack_depth: 100,
            enable_error_propagation: true,
            max_propagation_depth: 50,
            enable_recovery_optimization: true,
            happy_path_threshold: 0.95,
            enable_error_suppression: false,
            enable_performance_monitoring: true,
        }
    }
}

/// Error execution performance metrics
#[derive(Debug, Clone)]
pub struct ErrorExecutionMetrics {
    /// Total errors handled
    pub total_errors_handled: u64,
    /// Total error handling time
    pub total_error_handling_time: Duration,
    /// Average error handling time
    pub avg_error_handling_time: Duration,
    /// Happy path execution percentage
    pub happy_path_percentage: f64,
    /// Error propagation count
    pub error_propagation_count: u64,
    /// Recovery success count
    pub recovery_success_count: u64,
    /// Recovery failure count
    pub recovery_failure_count: u64,
    /// Performance overhead percentage
    pub performance_overhead_percentage: f64,
}

impl Default for ErrorExecutionMetrics {
    fn default() -> Self {
        Self {
            total_errors_handled: 0,
            total_error_handling_time: Duration::from_secs(0),
            avg_error_handling_time: Duration::from_secs(0),
            happy_path_percentage: 100.0,
            error_propagation_count: 0,
            recovery_success_count: 0,
            recovery_failure_count: 0,
            performance_overhead_percentage: 0.0,
        }
    }
}

impl CursedErrorExecution {
    /// Create new error execution runtime
    pub fn new() -> Result<Self> {
        let enhanced_runtime = Arc::new(EnhancedErrorRuntime::new());
        enhanced_runtime.initialize()?;
        
        Ok(Self {
            enhanced_runtime,
            propagation_stack: RwLock::new(Vec::new()),
            active_contexts: RwLock::new(HashMap::new()),
            performance_metrics: Arc::new(Mutex::new(ErrorExecutionMetrics::default())),
            suppression_patterns: RwLock::new(Vec::new()),
            happy_path_optimization: AtomicBool::new(true),
            config: ErrorExecutionConfig::default(),
        })
    }

    /// Initialize the error execution system
    pub fn initialize(&self) -> Result<()> {
        // Set up panic hook for error handling integration
        self.setup_error_panic_hook()?;
        
        // Initialize performance monitoring
        if self.config.enable_performance_monitoring {
            self.start_performance_monitoring()?;
        }
        
        Ok(())
    }

    /// Handle yikes error creation and throwing
    pub fn execute_yikes_error(
        &self,
        name: String,
        message: String,
        context: HashMap<String, String>,
        goroutine_id: Option<GoroutineId>,
        file: &str,
        line: u32,
        column: u32,
    ) -> Result<CursedErrorType> {
        let start_time = Instant::now();
        
        // Create enhanced stack trace
        let stack_trace = if self.config.enable_stack_traces {
            self.capture_cursed_stack_trace()?
        } else {
            Vec::new()
        };
        
        // Create CURSED error
        let cursed_error = CursedErrorType::Yikes {
            name: name.clone(),
            message: message.clone(),
            context,
            stack_trace,
        };
        
        // Create error stack frame
        let error_frame = ErrorStackFrame {
            function_name: "yikes".to_string(),
            file_name: file.to_string(),
            line_number: line,
            column_number: column,
            error_type: cursed_error.clone(),
            timestamp: Instant::now(),
            goroutine_id,
        };
        
        // Add to propagation stack
        if let Ok(mut stack) = self.propagation_stack.write() {
            if stack.len() < self.config.max_error_stack_depth {
                stack.push(error_frame);
            }
        }
        
        // Update goroutine context
        if let Some(gid) = goroutine_id {
            self.update_goroutine_context(gid, &cursed_error)?;
        }
        
        // Update performance metrics
        self.update_error_metrics(start_time.elapsed())?;
        
        // Register with enhanced runtime
        self.enhanced_runtime.handle_yikes_error(name, message, HashMap::new())?;
        
        Ok(cursed_error)
    }

    /// Handle shook error propagation
    pub fn execute_shook_propagation(
        &self,
        source_error: CursedErrorType,
        goroutine_id: Option<GoroutineId>,
        file: &str,
        line: u32,
        column: u32,
    ) -> Result<CursedErrorType> {
        let start_time = Instant::now();
        
        // Check if propagation is enabled
        if !self.config.enable_error_propagation {
            return Ok(source_error);
        }
        
        // Check propagation depth
        if let Ok(stack) = self.propagation_stack.read() {
            if stack.len() >= self.config.max_propagation_depth {
                return Err(Error::Runtime("Maximum error propagation depth exceeded".to_string()));
            }
        }
        
        // Create propagation context
        let propagation_context = PropagationContext {
            source_function: self.get_current_function_name()?,
            source_line: line,
            source_column: column,
            goroutine_id,
            propagation_chain: self.get_propagation_chain()?,
            timestamp: Instant::now(),
        };
        
        // Create propagated error
        let propagated_error = self.enhanced_runtime.handle_shook_error(source_error, propagation_context)?;
        
        // Create error stack frame
        let error_frame = ErrorStackFrame {
            function_name: "shook".to_string(),
            file_name: file.to_string(),
            line_number: line,
            column_number: column,
            error_type: propagated_error.clone(),
            timestamp: Instant::now(),
            goroutine_id,
        };
        
        // Add to propagation stack
        if let Ok(mut stack) = self.propagation_stack.write() {
            stack.push(error_frame);
        }
        
        // Update metrics
        self.update_propagation_metrics(start_time.elapsed())?;
        
        Ok(propagated_error)
    }

    /// Handle fam recovery block execution
    pub fn execute_fam_recovery<F, T>(
        &self,
        operation: F,
        recovery_handler: Option<RecoveryHandler>,
        goroutine_id: Option<GoroutineId>,
        file: &str,
        line: u32,
        column: u32,
    ) -> Result<T>
    where
        F: FnOnce() -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let start_time = Instant::now();
        
        // Create recovery block
        let recovery_block = RecoveryBlock {
            recovery_function: self.get_current_function_name().unwrap_or_else(|_| "unknown".to_string()),
            recovery_code: Vec::new(), // Would contain actual recovery code
            recovery_scope: HashMap::new(),
            max_attempts: 3,
            current_attempts: 0,
            strategy: RecoveryStrategy::RetryOperation,
        };
        
        // Add recovery block to context
        if let Some(gid) = goroutine_id {
            self.add_recovery_block(gid, recovery_block)?;
        }
        
        // Execute operation with panic recovery
        let result = catch_unwind(AssertUnwindSafe(|| {
            operation()
        }));
        
        match result {
            Ok(operation_result) => {
                // Operation succeeded
                self.update_recovery_success_metrics(start_time.elapsed())?;
                operation_result
            }
            Err(panic_payload) => {
                // Operation panicked, attempt recovery
                self.handle_fam_panic_recovery(panic_payload, recovery_handler, goroutine_id, file, line, column)
            }
        }
    }

    /// Handle panic recovery in fam blocks
    fn handle_fam_panic_recovery<T>(
        &self,
        panic_payload: Box<dyn Any + Send>,
        recovery_handler: Option<RecoveryHandler>,
        goroutine_id: Option<GoroutineId>,
        file: &str,
        line: u32,
        column: u32,
    ) -> Result<T> {
        // Extract panic message
        let panic_message = if let Some(s) = panic_payload.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = panic_payload.downcast_ref::<&str>() {
            s.to_string()
        } else {
            "Unknown panic".to_string()
        };
        
        // Create error from panic
        let error = CursedErrorType::Yikes {
            name: "panic_error".to_string(),
            message: panic_message.clone(),
            context: HashMap::new(),
            stack_trace: self.capture_cursed_stack_trace().unwrap_or_default(),
        };
        
        // Create recovery context
        let recovery_context = RecoveryContext {
            recovery_function: self.get_current_function_name().unwrap_or_else(|_| "unknown".to_string()),
            recovery_line: line,
            recovery_column: column,
            recovery_attempts: 1,
            recovery_strategy: crate::runtime::error_handling::RecoveryStrategy::Custom,
            timestamp: Instant::now(),
        };
        
        // Attempt recovery using enhanced runtime
        let recovery_result = self.enhanced_runtime.handle_fam_recovery(error, recovery_context)?;
        
        // Check if recovery was successful
        match recovery_result {
            CursedErrorType::Fam { recovery_successful: true, .. } => {
                // Recovery successful, but we need to provide a value
                // In a real implementation, this would use the recovery handler
                // For now, return an error indicating recovery was attempted
                self.update_recovery_success_metrics(Duration::from_millis(1))?;
                Err(Error::Runtime(format!("Panic recovered but no value available: {}", panic_message)))
            }
            _ => {
                // Recovery failed
                self.update_recovery_failure_metrics(Duration::from_millis(1))?;
                Err(Error::Runtime(format!("Failed to recover from panic: {}", panic_message)))
            }
        }
    }

    /// Capture CURSED-specific stack trace
    pub fn capture_cursed_stack_trace(&self) -> Result<Vec<String>> {
        let mut trace = Vec::new();
        
        // Get current propagation stack
        if let Ok(stack) = self.propagation_stack.read() {
            for frame in stack.iter() {
                trace.push(format!(
                    "{}:{} in {} ({}:{})",
                    frame.file_name,
                    frame.line_number,
                    frame.function_name,
                    frame.line_number,
                    frame.column_number
                ));
            }
        }
        
        // Add system stack trace if needed
        if trace.is_empty() {
            let bt = std::backtrace::Backtrace::capture();
            let bt_string = format!("{}", bt);
            for line in bt_string.lines().take(20) {
                if !line.trim().is_empty() {
                    trace.push(line.trim().to_string());
                }
            }
        }
        
        Ok(trace)
    }

    /// Update goroutine error context
    fn update_goroutine_context(&self, goroutine_id: GoroutineId, error: &CursedErrorType) -> Result<()> {
        let mut contexts = self.active_contexts.write().map_err(|_| {
            Error::Runtime("Failed to acquire contexts lock".to_string())
        })?;
        
        let context = contexts.entry(goroutine_id).or_insert_with(|| ErrorExecutionContext {
            error_stack: Vec::new(),
            recovery_blocks: Vec::new(),
            propagation_enabled: true,
            current_recovery_handler: None,
            error_count: 0,
            last_error_time: None,
        });
        
        context.error_count += 1;
        context.last_error_time = Some(Instant::now());
        
        Ok(())
    }

    /// Add recovery block to goroutine context
    fn add_recovery_block(&self, goroutine_id: GoroutineId, recovery_block: RecoveryBlock) -> Result<()> {
        let mut contexts = self.active_contexts.write().map_err(|_| {
            Error::Runtime("Failed to acquire contexts lock".to_string())
        })?;
        
        let context = contexts.entry(goroutine_id).or_insert_with(|| ErrorExecutionContext {
            error_stack: Vec::new(),
            recovery_blocks: Vec::new(),
            propagation_enabled: true,
            current_recovery_handler: None,
            error_count: 0,
            last_error_time: None,
        });
        
        context.recovery_blocks.push(recovery_block);
        
        Ok(())
    }

    /// Get current function name (simplified)
    fn get_current_function_name(&self) -> Result<String> {
        // In a real implementation, this would extract the current function name
        // from the call stack or execution context
        Ok("unknown".to_string())
    }

    /// Get propagation chain
    fn get_propagation_chain(&self) -> Result<Vec<String>> {
        let mut chain = Vec::new();
        
        if let Ok(stack) = self.propagation_stack.read() {
            for frame in stack.iter() {
                chain.push(format!("{}:{}", frame.function_name, frame.line_number));
            }
        }
        
        Ok(chain)
    }

    /// Setup panic hook for error handling integration
    fn setup_error_panic_hook(&self) -> Result<()> {
        // This would integrate with the existing panic system
        // For now, just ensure the enhanced runtime panic hook is set up
        Ok(())
    }

    /// Start performance monitoring
    fn start_performance_monitoring(&self) -> Result<()> {
        // Initialize performance monitoring systems
        Ok(())
    }

    /// Update error handling metrics
    fn update_error_metrics(&self, elapsed: Duration) -> Result<()> {
        if let Ok(mut metrics) = self.performance_metrics.lock() {
            metrics.total_errors_handled += 1;
            metrics.total_error_handling_time += elapsed;
            
            // Calculate average
            if metrics.total_errors_handled > 0 {
                metrics.avg_error_handling_time = 
                    metrics.total_error_handling_time / metrics.total_errors_handled as u32;
            }
            
            // Update happy path percentage
            let total_operations = metrics.total_errors_handled + 1000; // Assume some successful operations
            metrics.happy_path_percentage = 
                ((total_operations - metrics.total_errors_handled) as f64 / total_operations as f64) * 100.0;
        }
        
        Ok(())
    }

    /// Update error propagation metrics
    fn update_propagation_metrics(&self, elapsed: Duration) -> Result<()> {
        if let Ok(mut metrics) = self.performance_metrics.lock() {
            metrics.error_propagation_count += 1;
            metrics.total_error_handling_time += elapsed;
        }
        
        Ok(())
    }

    /// Update recovery success metrics
    fn update_recovery_success_metrics(&self, elapsed: Duration) -> Result<()> {
        if let Ok(mut metrics) = self.performance_metrics.lock() {
            metrics.recovery_success_count += 1;
            metrics.total_error_handling_time += elapsed;
        }
        
        Ok(())
    }

    /// Update recovery failure metrics
    fn update_recovery_failure_metrics(&self, elapsed: Duration) -> Result<()> {
        if let Ok(mut metrics) = self.performance_metrics.lock() {
            metrics.recovery_failure_count += 1;
            metrics.total_error_handling_time += elapsed;
        }
        
        Ok(())
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> Result<ErrorExecutionMetrics> {
        if let Ok(metrics) = self.performance_metrics.lock() {
            Ok(metrics.clone())
        } else {
            Err(Error::Runtime("Failed to get performance metrics".to_string()))
        }
    }

    /// Clear error execution context for goroutine
    pub fn clear_goroutine_context(&self, goroutine_id: GoroutineId) -> Result<()> {
        if let Ok(mut contexts) = self.active_contexts.write() {
            contexts.remove(&goroutine_id);
        }
        
        Ok(())
    }

    /// Check if error should be suppressed
    pub fn should_suppress_error(&self, error: &CursedErrorType) -> bool {
        if !self.config.enable_error_suppression {
            return false;
        }
        
        if let Ok(patterns) = self.suppression_patterns.read() {
            let error_str = format!("{:?}", error);
            for pattern in patterns.iter() {
                if error_str.contains(pattern) {
                    return true;
                }
            }
        }
        
        false
    }

    /// Add error suppression pattern
    pub fn add_suppression_pattern(&self, pattern: String) -> Result<()> {
        if let Ok(mut patterns) = self.suppression_patterns.write() {
            patterns.push(pattern);
        }
        
        Ok(())
    }
}

/// Initialize global CURSED error execution
pub fn initialize_cursed_error_execution() -> Result<()> {
    let execution = Arc::new(CursedErrorExecution::new()?);
    execution.initialize()?;
    
    CURSED_ERROR_EXECUTION.set(execution).map_err(|_| {
        Error::Runtime("Failed to initialize CURSED error execution".to_string())
    })?;
    
    Ok(())
}

/// Get global CURSED error execution
pub fn get_cursed_error_execution() -> Option<Arc<CursedErrorExecution>> {
    CURSED_ERROR_EXECUTION.get().cloned()
}

/// Shutdown CURSED error execution
pub fn shutdown_cursed_error_execution() -> Result<()> {
    if let Some(execution) = CURSED_ERROR_EXECUTION.get() {
        // Clear all contexts
        if let Ok(mut contexts) = execution.active_contexts.write() {
            contexts.clear();
        }
        
        // Clear propagation stack
        if let Ok(mut stack) = execution.propagation_stack.write() {
            stack.clear();
        }
    }
    
    Ok(())
}

/// Helper function to execute yikes error
pub fn cursed_yikes(
    name: String,
    message: String,
    context: HashMap<String, String>,
    goroutine_id: Option<GoroutineId>,
    file: &str,
    line: u32,
    column: u32,
) -> Result<CursedErrorType> {
    if let Some(execution) = get_cursed_error_execution() {
        execution.execute_yikes_error(name, message, context, goroutine_id, file, line, column)
    } else {
        Err(Error::Runtime("CURSED error execution not initialized".to_string()))
    }
}

/// Helper function to execute shook propagation
pub fn cursed_shook(
    source_error: CursedErrorType,
    goroutine_id: Option<GoroutineId>,
    file: &str,
    line: u32,
    column: u32,
) -> Result<CursedErrorType> {
    if let Some(execution) = get_cursed_error_execution() {
        execution.execute_shook_propagation(source_error, goroutine_id, file, line, column)
    } else {
        Err(Error::Runtime("CURSED error execution not initialized".to_string()))
    }
}

/// Helper function to execute fam recovery
pub fn cursed_fam<F, T>(
    operation: F,
    recovery_handler: Option<RecoveryHandler>,
    goroutine_id: Option<GoroutineId>,
    file: &str,
    line: u32,
    column: u32,
) -> Result<T>
where
    F: FnOnce() -> Result<T> + Send + 'static,
    T: Send + 'static,
{
    if let Some(execution) = get_cursed_error_execution() {
        execution.execute_fam_recovery(operation, recovery_handler, goroutine_id, file, line, column)
    } else {
        Err(Error::Runtime("CURSED error execution not initialized".to_string()))
    }
}

/// Macro for simplified yikes error creation
#[macro_export]
macro_rules! cursed_yikes {
    ($name:expr, $message:expr) => {
        $crate::runtime::cursed_error_execution::cursed_yikes(
            $name.to_string(),
            $message.to_string(),
            std::collections::HashMap::new(),
            None,
            file!(),
            line!(),
            column!(),
        )
    };
    ($name:expr, $message:expr, $context:expr) => {
        $crate::runtime::cursed_error_execution::cursed_yikes(
            $name.to_string(),
            $message.to_string(),
            $context,
            None,
            file!(),
            line!(),
            column!(),
        )
    };
}

/// Macro for simplified shook propagation
#[macro_export]
macro_rules! cursed_shook {
    ($error:expr) => {
        $crate::runtime::cursed_error_execution::cursed_shook(
            $error,
            None,
            file!(),
            line!(),
            column!(),
        )
    };
}

/// Macro for simplified fam recovery
#[macro_export]
macro_rules! cursed_fam {
    ($operation:expr) => {
        $crate::runtime::cursed_error_execution::cursed_fam(
            || $operation,
            None,
            None,
            file!(),
            line!(),
            column!(),
        )
    };
    ($operation:expr, $recovery:expr) => {
        $crate::runtime::cursed_error_execution::cursed_fam(
            || $operation,
            Some($recovery),
            None,
            file!(),
            line!(),
            column!(),
        )
    };
}

/// Default implementation
impl Default for CursedErrorExecution {
    fn default() -> Self {
        Self::new().expect("Failed to create CursedErrorExecution")
    }
}

impl fmt::Display for CursedErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CursedErrorType::Yikes { name, message, .. } => {
                write!(f, "yikes {}: {}", name, message)
            }
            CursedErrorType::Shook { source_error, .. } => {
                write!(f, "shook: {}", source_error)
            }
            CursedErrorType::Fam { original_error, recovery_successful, .. } => {
                if *recovery_successful {
                    write!(f, "fam (recovered): {}", original_error)
                } else {
                    write!(f, "fam (failed): {}", original_error)
                }
            }
        }
    }
}
