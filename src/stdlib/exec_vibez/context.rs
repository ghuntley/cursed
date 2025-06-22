/// Context types for exec_vibez - timeout and cancellation support
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// VibeContext provides timeout and cancellation functionality for commands
#[derive(Debug, Clone)]
pub struct VibeContext {
    /// Deadline for the context
    deadline: Option<Instant>,
    /// Cancellation signal
    cancelled: Arc<RwLock<bool>>,
    /// Parent context if any
    parent: Option<Box<VibeContext>>,
    /// Context values for passing data
    values: Arc<RwLock<std::collections::HashMap<String, String>>>,
}

impl VibeContext {
    /// Create a new background context (never times out or cancels)
    pub fn background() -> Self {
        Self {
            deadline: None,
            cancelled: Arc::new(RwLock::new(false)),
            parent: None,
            values: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    /// Create a context with timeout from this context
    pub fn with_timeout(self, timeout: Duration) -> (Self, CancelFunc) {
        let deadline = Some(Instant::now() + timeout);
        let cancelled = Arc::new(RwLock::new(false));
        let cancelled_clone = cancelled.clone();
        
        let ctx = Self {
            deadline,
            cancelled,
            parent: Some(Box::new(self)),
            values: Arc::new(RwLock::new(std::collections::HashMap::new())),
        };
        
        let cancel_func = CancelFunc {
            cancelled: cancelled_clone,
        };
        
        (ctx, cancel_func)
    }

    /// Create a context with timeout (simplified version for Cmd)
    pub fn with_timeout_simple(timeout: Duration) -> Self {
        Self {
            deadline: Some(Instant::now() + timeout),
            cancelled: Arc::new(RwLock::new(false)),
            parent: None,
            values: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Set timeout for existing context
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.deadline = Some(Instant::now() + timeout);
    }
    
    /// Create a context with deadline from this context
    pub fn with_deadline(self, deadline: Instant) -> (Self, CancelFunc) {
        let cancelled = Arc::new(RwLock::new(false));
        let cancelled_clone = cancelled.clone();
        
        let ctx = Self {
            deadline: Some(deadline),
            cancelled,
            parent: Some(Box::new(self)),
            values: Arc::new(RwLock::new(std::collections::HashMap::new())),
        };
        
        let cancel_func = CancelFunc {
            cancelled: cancelled_clone,
        };
        
        (ctx, cancel_func)
    }
    
    /// Create a cancellable context from this context
    pub fn with_cancel(self) -> (Self, CancelFunc) {
        let cancelled = Arc::new(RwLock::new(false));
        let cancelled_clone = cancelled.clone();
        
        let ctx = Self {
            deadline: self.deadline,
            cancelled,
            parent: Some(Box::new(self)),
            values: Arc::new(RwLock::new(std::collections::HashMap::new())),
        };
        
        let cancel_func = CancelFunc {
            cancelled: cancelled_clone,
        };
        
        (ctx, cancel_func)
    }
    
    /// Set a value in the context
    pub fn with_value(self, key: String, value: String) -> Self {
        if let Ok(mut values) = self.values.write() {
            values.insert(key, value);
        }
        self
    }
    
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
        }
        
        None
    }
    
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
    }
    
    /// Get the error if context is done
    pub fn err(&self) -> Option<ContextError> {
        if !self.done() {
            return None;
        }
        
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
        }
        
        None
    }
    
    /// Get the deadline if any
    pub fn deadline(&self) -> Option<Instant> {
        self.deadline.or_else(|| {
            self.parent.as_ref().and_then(|p| p.deadline())
        })
    }
    
    /// Wait for context to be done or return immediately if already done
    pub fn wait_done(&self) -> ContextError {
        while !self.done() {
            std::thread::sleep(Duration::from_millis(1));
        }
        
        self.err().unwrap_or(ContextError::Cancelled)
    }
    
    /// Check if context will be done within the given duration
    pub fn will_timeout(&self, within: Duration) -> bool {
        if let Some(deadline) = self.deadline() {
            Instant::now() + within >= deadline
        } else {
            false
        }
    }
}

/// Cancel function for contexts
#[derive(Debug)]
pub struct CancelFunc {
    cancelled: Arc<RwLock<bool>>,
}

impl CancelFunc {
    /// Cancel the context
    pub fn cancel(&self) {
        if let Ok(mut cancelled) = self.cancelled.write() {
            *cancelled = true;
        }
    }
}

/// Context error types
#[derive(Debug, Clone, PartialEq)]
pub enum ContextError {
    /// Context was cancelled
    Cancelled,
    /// Context deadline exceeded
    DeadlineExceeded,
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextError::Cancelled => write!(f, "context cancelled"),
            ContextError::DeadlineExceeded => write!(f, "context deadline exceeded"),
        }
    }
}

impl std::error::Error for ContextError {}

/// Convenience functions following Go-style API

/// Create a background context
pub fn Background() -> VibeContext {
    VibeContext::background()
}

/// Create a TODO context (placeholder for development)
pub fn TODO() -> VibeContext {
    VibeContext::background()
}

/// Create a context with timeout
pub fn WithTimeout(parent: VibeContext, timeout: Duration) -> (VibeContext, CancelFunc) {
    parent.with_timeout(timeout)
}

/// Create a context with deadline
pub fn WithDeadline(parent: VibeContext, deadline: Instant) -> (VibeContext, CancelFunc) {
    parent.with_deadline(deadline)
}

/// Create a cancellable context
pub fn WithCancel(parent: VibeContext) -> (VibeContext, CancelFunc) {
    parent.with_cancel()
}

/// Create a context with value
pub fn WithValue(parent: VibeContext, key: String, value: String) -> VibeContext {
    parent.with_value(key, value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_background_context() {
        let ctx = Background();
        assert!(!ctx.done());
        assert!(ctx.err().is_none());
        assert!(ctx.deadline().is_none());
    }
    
    #[test]
    fn test_context_with_timeout() {
        let parent = Background();
        let (ctx, _cancel) = WithTimeout(parent, Duration::from_millis(50));
        
        assert!(!ctx.done());
        
        // Wait for timeout
        std::thread::sleep(Duration::from_millis(100));
        
        assert!(ctx.done());
        assert_eq!(ctx.err(), Some(ContextError::DeadlineExceeded));
    }
    
    #[test]
    fn test_context_with_cancel() {
        let parent = Background();
        let (ctx, cancel) = WithCancel(parent);
        
        assert!(!ctx.done());
        
        cancel.cancel();
        
        assert!(ctx.done());
        assert_eq!(ctx.err(), Some(ContextError::Cancelled));
    }
    
    #[test]
    fn test_context_with_deadline() {
        let parent = Background();
        let deadline = Instant::now() + Duration::from_millis(50);
        let (ctx, _cancel) = WithDeadline(parent, deadline);
        
        assert!(!ctx.done());
        assert_eq!(ctx.deadline(), Some(deadline));
        
        // Wait for deadline
        std::thread::sleep(Duration::from_millis(100));
        
        assert!(ctx.done());
        assert_eq!(ctx.err(), Some(ContextError::DeadlineExceeded));
    }
    
    #[test]
    fn test_context_with_value() {
        let parent = Background();
        let ctx = WithValue(parent, "key1".to_string(), "value1".to_string());
        
        assert_eq!(ctx.value("key1"), Some("value1".to_string()));
        assert_eq!(ctx.value("nonexistent"), None);
    }
    
    #[test]
    fn test_context_inheritance() {
        let parent = Background();
        let ctx1 = WithValue(parent, "key1".to_string(), "value1".to_string());
        let (ctx2, _cancel) = WithCancel(ctx1);
        let ctx3 = WithValue(ctx2, "key2".to_string(), "value2".to_string());
        
        // Should inherit values from parent contexts
        assert_eq!(ctx3.value("key1"), Some("value1".to_string()));
        assert_eq!(ctx3.value("key2"), Some("value2".to_string()));
    }
    
    #[test]
    fn test_context_timeout_inheritance() {
        let parent = Background();
        let (ctx1, _cancel1) = WithTimeout(parent, Duration::from_millis(100));
        let (ctx2, cancel2) = WithCancel(ctx1);
        
        // Cancel child context
        cancel2.cancel();
        
        // Child should be done due to cancellation
        assert!(ctx2.done());
        assert_eq!(ctx2.err(), Some(ContextError::Cancelled));
    }
    
    #[test]
    fn test_will_timeout() {
        let parent = Background();
        let (ctx, _cancel) = WithTimeout(parent, Duration::from_millis(100));
        
        assert!(ctx.will_timeout(Duration::from_millis(200)));
        assert!(!ctx.will_timeout(Duration::from_millis(50)));
    }
    
    #[test]
    fn test_context_error_display() {
        let cancelled = ContextError::Cancelled;
        let deadline = ContextError::DeadlineExceeded;
        
        assert_eq!(format!("{}", cancelled), "context cancelled");
        assert_eq!(format!("{}", deadline), "context deadline exceeded");
    }
}


#[derive(Debug, Clone)]
pub struct ProcessContext {
    pub environment: std::collections::HashMap<String, String>,
    pub working_dir: Option<std::path::PathBuf>,
    pub timeout: Option<std::time::Duration>,
}

impl ProcessContext {
    pub fn new() -> Self {
        Self {
            environment: std::collections::HashMap::new(),
            working_dir: None,
            timeout: None,
        }
    }
}

impl Default for ProcessContext {
    fn default() -> Self {
        Self::new()
    }
}
