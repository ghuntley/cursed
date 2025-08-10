//! Production Runtime Error Handling
//!
//! This module provides comprehensive error handling for production runtime operations.
//! It replaces panic! calls with proper error types and Result<T, E> returns.

use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use crate::runtime::goroutine::GoroutineId;

/// Production runtime error types
#[derive(Debug, Clone)]
pub enum ProductionError {
    /// Channel operation errors
    ChannelClosed(String),
    ChannelTimeout(Duration),
    ChannelWouldBlock(String),
    ChannelInvalidPriority(String),
    ChannelBackpressure(String),
    
    /// Goroutine operation errors
    GoroutineSpawnFailed(String),
    GoroutineSchedulerOverload(String),
    GoroutineTimeout(GoroutineId, Duration),
    GoroutineInvalidState(GoroutineId, String),
    
    /// Async operation errors
    AsyncOperationTimeout(Duration),
    AsyncOperationCancelled(String),
    AsyncInvalidState(String),
    AsyncResourceExhausted(String),
    
    /// Runtime system errors
    RuntimeShutdown(String),
    RuntimeResourceExhausted(String),
    RuntimeInvalidConfiguration(String),
    RuntimeThreadingError(String),
    
    /// Memory management errors
    MemoryAllocationFailed(usize),
    MemoryCorruption(String),
    MemoryLeakDetected(String),
    
    /// Test framework errors (for graceful test handling)
    TestExpectationFailed(String),
    TestAssertionFailed(String),
    TestTimeout(Duration),
    
    /// Generic runtime error
    Generic(String),
}

impl fmt::Display for ProductionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProductionError::ChannelClosed(msg) => write!(f, "Channel closed: {}", msg),
            ProductionError::ChannelTimeout(duration) => write!(f, "Channel operation timed out after {:?}", duration),
            ProductionError::ChannelWouldBlock(msg) => write!(f, "Channel would block: {}", msg),
            ProductionError::ChannelInvalidPriority(msg) => write!(f, "Invalid channel priority: {}", msg),
            ProductionError::ChannelBackpressure(msg) => write!(f, "Channel backpressure: {}", msg),
            
            ProductionError::GoroutineSpawnFailed(msg) => write!(f, "Failed to spawn goroutine: {}", msg),
            ProductionError::GoroutineSchedulerOverload(msg) => write!(f, "Goroutine scheduler overload: {}", msg),
            ProductionError::GoroutineTimeout(id, duration) => write!(f, "Goroutine {:?} timed out after {:?}", id, duration),
            ProductionError::GoroutineInvalidState(id, msg) => write!(f, "Goroutine {:?} in invalid state: {}", id, msg),
            
            ProductionError::AsyncOperationTimeout(duration) => write!(f, "Async operation timed out after {:?}", duration),
            ProductionError::AsyncOperationCancelled(msg) => write!(f, "Async operation cancelled: {}", msg),
            ProductionError::AsyncInvalidState(msg) => write!(f, "Async operation invalid state: {}", msg),
            ProductionError::AsyncResourceExhausted(msg) => write!(f, "Async resource exhausted: {}", msg),
            
            ProductionError::RuntimeShutdown(msg) => write!(f, "Runtime shutdown: {}", msg),
            ProductionError::RuntimeResourceExhausted(msg) => write!(f, "Runtime resource exhausted: {}", msg),
            ProductionError::RuntimeInvalidConfiguration(msg) => write!(f, "Runtime invalid configuration: {}", msg),
            ProductionError::RuntimeThreadingError(msg) => write!(f, "Runtime threading error: {}", msg),
            
            ProductionError::MemoryAllocationFailed(size) => write!(f, "Memory allocation failed for {} bytes", size),
            ProductionError::MemoryCorruption(msg) => write!(f, "Memory corruption detected: {}", msg),
            ProductionError::MemoryLeakDetected(msg) => write!(f, "Memory leak detected: {}", msg),
            
            ProductionError::TestExpectationFailed(msg) => write!(f, "Test expectation failed: {}", msg),
            ProductionError::TestAssertionFailed(msg) => write!(f, "Test assertion failed: {}", msg),
            ProductionError::TestTimeout(duration) => write!(f, "Test timed out after {:?}", duration),
            
            ProductionError::Generic(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for ProductionError {}

/// Production runtime result type
pub type ProductionResult<T> = Result<T, ProductionError>;

/// Error recovery strategy for production runtime
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Retry the operation with backoff
    Retry,
    /// Use fallback/default value
    Fallback,
    /// Continue with graceful degradation
    GracefulDegradation,
    /// Restart the failing component
    RestartComponent,
    /// Escalate to higher-level error handler
    Escalate,
    /// Shutdown gracefully
    GracefulShutdown,
}

/// Error context for production runtime
#[derive(Debug, Clone)]
pub struct ProductionErrorContext {
    pub error: ProductionError,
    pub recovery_strategy: RecoveryStrategy,
    pub retry_count: u32,
    pub max_retries: u32,
    pub component: String,
    pub operation: String,
    pub metadata: std::collections::HashMap<String, String>,
}

impl ProductionErrorContext {
    pub fn new(error: ProductionError, component: String, operation: String) -> Self {
        Self {
            error,
            recovery_strategy: RecoveryStrategy::Retry,
            retry_count: 0,
            max_retries: 3,
            component,
            operation,
            metadata: std::collections::HashMap::new(),
        }
    }
    
    pub fn with_recovery_strategy(mut self, strategy: RecoveryStrategy) -> Self {
        self.recovery_strategy = strategy;
        self
    }
    
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }
    
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }
    
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }
}

/// Production error handler trait
pub trait ProductionErrorHandler: Send + Sync {
    fn handle_error(&self, context: &ProductionErrorContext) -> ProductionResult<RecoveryStrategy>;
    fn should_retry(&self, error: &ProductionError) -> bool;
    fn get_recovery_strategy(&self, error: &ProductionError) -> RecoveryStrategy;
}

/// Default production error handler
pub struct DefaultProductionErrorHandler;

impl ProductionErrorHandler for DefaultProductionErrorHandler {
    fn handle_error(&self, context: &ProductionErrorContext) -> ProductionResult<RecoveryStrategy> {
        match &context.error {
            ProductionError::ChannelClosed(_) => Ok(RecoveryStrategy::GracefulDegradation),
            ProductionError::ChannelTimeout(_) => Ok(RecoveryStrategy::Retry),
            ProductionError::ChannelWouldBlock(_) => Ok(RecoveryStrategy::Retry),
            ProductionError::ChannelBackpressure(_) => Ok(RecoveryStrategy::Retry),
            ProductionError::ChannelInvalidPriority(_) => Ok(RecoveryStrategy::GracefulDegradation),
            
            ProductionError::GoroutineSpawnFailed(_) => Ok(RecoveryStrategy::Retry),
            ProductionError::GoroutineSchedulerOverload(_) => Ok(RecoveryStrategy::GracefulDegradation),
            ProductionError::GoroutineTimeout(_, _) => Ok(RecoveryStrategy::RestartComponent),
            ProductionError::GoroutineInvalidState(_, _) => Ok(RecoveryStrategy::RestartComponent),
            
            ProductionError::AsyncOperationTimeout(_) => Ok(RecoveryStrategy::Retry),
            ProductionError::AsyncOperationCancelled(_) => Ok(RecoveryStrategy::GracefulDegradation),
            ProductionError::AsyncInvalidState(_) => Ok(RecoveryStrategy::RestartComponent),
            ProductionError::AsyncResourceExhausted(_) => Ok(RecoveryStrategy::GracefulDegradation),
            
            ProductionError::RuntimeShutdown(_) => Ok(RecoveryStrategy::GracefulShutdown),
            ProductionError::RuntimeResourceExhausted(_) => Ok(RecoveryStrategy::GracefulDegradation),
            ProductionError::RuntimeInvalidConfiguration(_) => Ok(RecoveryStrategy::Escalate),
            ProductionError::RuntimeThreadingError(_) => Ok(RecoveryStrategy::RestartComponent),
            
            ProductionError::MemoryAllocationFailed(_) => Ok(RecoveryStrategy::GracefulDegradation),
            ProductionError::MemoryCorruption(_) => Ok(RecoveryStrategy::GracefulShutdown),
            ProductionError::MemoryLeakDetected(_) => Ok(RecoveryStrategy::GracefulDegradation),
            
            ProductionError::TestExpectationFailed(_) => Ok(RecoveryStrategy::GracefulDegradation),
            ProductionError::TestAssertionFailed(_) => Ok(RecoveryStrategy::GracefulDegradation),
            ProductionError::TestTimeout(_) => Ok(RecoveryStrategy::GracefulDegradation),
            
            ProductionError::Generic(_) => Ok(RecoveryStrategy::Retry),
        }
    }
    
    fn should_retry(&self, error: &ProductionError) -> bool {
        matches!(error, 
            ProductionError::ChannelTimeout(_) |
            ProductionError::ChannelWouldBlock(_) |
            ProductionError::ChannelBackpressure(_) |
            ProductionError::ChannelInvalidPriority(_) |
            ProductionError::GoroutineSpawnFailed(_) |
            ProductionError::AsyncOperationTimeout(_) |
            ProductionError::Generic(_)
        )
    }
    
    fn get_recovery_strategy(&self, error: &ProductionError) -> RecoveryStrategy {
        match error {
            ProductionError::ChannelClosed(_) => RecoveryStrategy::GracefulDegradation,
            ProductionError::ChannelTimeout(_) => RecoveryStrategy::Retry,
            ProductionError::ChannelWouldBlock(_) => RecoveryStrategy::Retry,
            ProductionError::ChannelBackpressure(_) => RecoveryStrategy::Retry,
            ProductionError::ChannelInvalidPriority(_) => RecoveryStrategy::GracefulDegradation,
            
            ProductionError::GoroutineSpawnFailed(_) => RecoveryStrategy::Retry,
            ProductionError::GoroutineSchedulerOverload(_) => RecoveryStrategy::GracefulDegradation,
            ProductionError::GoroutineTimeout(_, _) => RecoveryStrategy::RestartComponent,
            ProductionError::GoroutineInvalidState(_, _) => RecoveryStrategy::RestartComponent,
            
            ProductionError::AsyncOperationTimeout(_) => RecoveryStrategy::Retry,
            ProductionError::AsyncOperationCancelled(_) => RecoveryStrategy::GracefulDegradation,
            ProductionError::AsyncInvalidState(_) => RecoveryStrategy::RestartComponent,
            ProductionError::AsyncResourceExhausted(_) => RecoveryStrategy::GracefulDegradation,
            
            ProductionError::RuntimeShutdown(_) => RecoveryStrategy::GracefulShutdown,
            ProductionError::RuntimeResourceExhausted(_) => RecoveryStrategy::GracefulDegradation,
            ProductionError::RuntimeInvalidConfiguration(_) => RecoveryStrategy::Escalate,
            ProductionError::RuntimeThreadingError(_) => RecoveryStrategy::RestartComponent,
            
            ProductionError::MemoryAllocationFailed(_) => RecoveryStrategy::GracefulDegradation,
            ProductionError::MemoryCorruption(_) => RecoveryStrategy::GracefulShutdown,
            ProductionError::MemoryLeakDetected(_) => RecoveryStrategy::GracefulDegradation,
            
            ProductionError::TestExpectationFailed(_) => RecoveryStrategy::GracefulDegradation,
            ProductionError::TestAssertionFailed(_) => RecoveryStrategy::GracefulDegradation,
            ProductionError::TestTimeout(_) => RecoveryStrategy::GracefulDegradation,
            
            ProductionError::Generic(_) => RecoveryStrategy::Retry,
        }
    }
}

/// Helper macro for converting test panics to production errors
#[macro_export]
macro_rules! expect_or_error {
    ($expr:expr, $error_type:expr) => {
        match $expr {
            Ok(value) => value,
            Err(_) => return Err($error_type),
        }
    };
}

/// Helper macro for test expectations
#[macro_export]
macro_rules! test_expect {
    ($condition:expr, $message:expr) => {
        if !$condition {
            return Err(crate::runtime::production_error::ProductionError::TestExpectationFailed($message.to_string()));
        }
    };
}

/// Helper macro for test assertions
#[macro_export]
macro_rules! test_assert {
    ($condition:expr, $message:expr) => {
        if !$condition {
            return Err(crate::runtime::production_error::ProductionError::TestAssertionFailed($message.to_string()));
        }
    };
}
