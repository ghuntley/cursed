use crate::error::CursedError;
/// Context types for exec_vibez - timeout and cancellation support
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// VibeContext provides timeout and cancellation functionality for commands
#[derive(Debug, Clone)]
pub struct VibeContext {
    /// Deadline for the context
    /// Cancellation signal
    /// Parent context if any
    /// Context values for passing data
impl VibeContext {
    /// Create a new background context (never times out or cancels)
    pub fn background() -> Self {
        Self {
        }
    }
    
    /// Create a context with timeout from this context
    pub fn with_timeout(self, timeout: Duration) -> (Self, CancelFunc) {
        let deadline = Some(Instant::now() + timeout);
        let cancelled = Arc::new(RwLock::new(false));
        let cancelled_clone = cancelled.clone();
        
        let ctx = Self {
        
        let cancel_func = CancelFunc {
        
        (ctx, cancel_func)
    /// Create a context with timeout (simplified version for Cmd)
    pub fn with_timeout_simple(timeout: Duration) -> Self {
        Self {
        }
    }

    /// Set timeout for existing context
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.deadline = Some(Instant::now() + timeout);
    /// Create a context with deadline from this context
    pub fn with_deadline(self, deadline: Instant) -> (Self, CancelFunc) {
        let cancelled = Arc::new(RwLock::new(false));
        let cancelled_clone = cancelled.clone();
        
        let ctx = Self {
        
        let cancel_func = CancelFunc {
        
        (ctx, cancel_func)
    /// Create a cancellable context from this context
    pub fn with_cancel(self) -> (Self, CancelFunc) {
        let cancelled = Arc::new(RwLock::new(false));
        let cancelled_clone = cancelled.clone();
        
        let ctx = Self {
        
        let cancel_func = CancelFunc {
        
        (ctx, cancel_func)
    /// Set a value in the context
    pub fn with_value(self, key: String, value: String) -> Self {
        if let Ok(mut values) = self.values.write() {
            values.insert(key, value);
        }
        self
    /// Get a value from the context
    pub fn value(&self, key: &str) -> Option<String> {
        // Check current context
        if let Ok(values) = self.values.read() {
            if let Some(value) = values.get(key) {
                return Some(value.clone());
            }
        }
        
        // Check parent context
        if let Some(ref parent) = self.parent {
            return parent.value(key);
        None
    /// Check if context is done (cancelled or timed out)
    pub fn done(&self) -> bool {
        // Check if explicitly cancelled
        if let Ok(cancelled) = self.cancelled.read() {
            if *cancelled {
                return true;
            }
        }
        
        // Check if deadline exceeded
        if let Some(deadline) = self.deadline {
            if Instant::now() > deadline {
                return true;
            }
        }
        
        // Check parent context
        if let Some(ref parent) = self.parent {
            if parent.done() {
                return true;
            }
        }
        
        false
    /// Get the error if context is done
    pub fn err(&self) -> Option<ContextError> {
        if !self.done() {
            return None;
        // Check if explicitly cancelled
        if let Ok(cancelled) = self.cancelled.read() {
            if *cancelled {
                return Some(ContextError::Cancelled);
            }
        }
        
        // Check if deadline exceeded
        if let Some(deadline) = self.deadline {
            if Instant::now() > deadline {
                return Some(ContextError::DeadlineExceeded);
            }
        }
        
        // Check parent context
        if let Some(ref parent) = self.parent {
            return parent.err();
        None
    /// Get the deadline if any
    pub fn deadline(&self) -> Option<Instant> {
        self.deadline.or_else(|| {
            self.parent.as_ref().and_then(|p| p.deadline())
        })
    /// Wait for context to be done or return immediately if already done
    pub fn wait_done(&self) -> ContextError {
        while !self.done() {
            std::thread::sleep(Duration::from_millis(1));
        self.err().unwrap_or(ContextError::Cancelled)
    /// Check if context will be done within the given duration
    pub fn will_timeout(&self, within: Duration) -> bool {
        if let Some(deadline) = self.deadline() {
            Instant::now() + within >= deadline
        } else {
            false
        }
    }
/// Cancel function for contexts
#[derive(Debug)]
pub struct CancelFunc {
impl CancelFunc {
    /// Cancel the context
    pub fn cancel(&self) {
        if let Ok(mut cancelled) = self.cancelled.write() {
            *cancelled = true;
        }
    }
/// Context error types
#[derive(Debug, Clone, PartialEq)]
pub enum ContextError {
    /// Context was cancelled
    /// Context deadline exceeded
// impl std::fmt::Display for ContextError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ContextError::Cancelled => write!(f, "context cancelled"),
//             ContextError::DeadlineExceeded => write!(f, "context deadline exceeded"),
//         }
//     }
// }

// impl std::error::CursedError for ContextError {}
// 
/// Convenience functions following Go-style API

/// Create a background context
pub fn Background() -> VibeContext {
    VibeContext::background()
/// Create a TODO context (placeholder for development)
pub fn TODO() -> VibeContext {
    VibeContext::background()
/// Create a context with timeout
pub fn WithTimeout(parent: VibeContext, timeout: Duration) -> (VibeContext, CancelFunc) {
    parent.with_timeout(timeout)
/// Create a context with deadline
pub fn WithDeadline(parent: VibeContext, deadline: Instant) -> (VibeContext, CancelFunc) {
    parent.with_deadline(deadline)
/// Create a cancellable context
pub fn WithCancel(parent: VibeContext) -> (VibeContext, CancelFunc) {
    parent.with_cancel()
/// Create a context with value
pub fn WithValue(parent: VibeContext, key: String, value: String) -> VibeContext {
    parent.with_value(key, value)

impl ProcessContext {
    pub fn new() -> Self {
        Self {
        }
    }
impl Default for ProcessContext {
    fn default() -> Self {
        Self::new()
    }
}
