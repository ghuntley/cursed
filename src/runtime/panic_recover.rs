//! CURSED Panic/Recover System
//!
//! This module provides a comprehensive panic/recover system for CURSED:
//! - Panic mechanism for unrecoverable errors
//! - Recover mechanism to catch and handle panics
//! - Stack unwinding with proper cleanup
//! - Error context and stack traces
//! - Integration with defer system for cleanup

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::panic::{PanicHookInfo, set_hook, take_hook, catch_unwind, AssertUnwindSafe};
use std::thread::{ThreadId, LocalKey};
use std::cell::RefCell;
use std::any::Any;
use std::fmt;

use crate::error_types::{Error, Result};
use crate::runtime::{RuntimeError, RuntimeErrorType};
use crate::runtime::goroutine::GoroutineId;
// Defer runtime will be implemented later
// use crate::runtime::defer_runtime::DeferRuntime;

/// Thread-local storage for panic state
thread_local! {
    static PANIC_STATE: RefCell<PanicState> = RefCell::new(PanicState::new());
}

/// Panic state for each thread/goroutine
pub struct PanicState {
    /// Current panic value
    pub panic_value: Option<String>,
    /// Panic message
    pub panic_message: Option<String>,
    /// Stack trace at panic
    pub stack_trace: Vec<String>,
    /// Goroutine ID where panic occurred
    pub goroutine_id: Option<GoroutineId>,
    /// Whether we're currently in a panic
    pub in_panic: bool,
    /// Whether we're currently in a recover
    pub in_recover: bool,
    /// Panic timestamp
    pub panic_timestamp: Option<Instant>,
    /// Defer handlers to execute during panic
    pub defer_handlers: Vec<Box<dyn FnOnce() + Send>>,
}

impl PanicState {
    fn new() -> Self {
        Self {
            panic_value: None,
            panic_message: None,
            stack_trace: Vec::new(),
            goroutine_id: None,
            in_panic: false,
            in_recover: false,
            panic_timestamp: None,
            defer_handlers: Vec::new(),
        }
    }
}

/// Panic information passed to handlers
#[derive(Debug, Clone)]
pub struct PanicInfo {
    /// Panic message
    pub message: String,
    /// Location where panic occurred
    pub location: Option<String>,
    /// Goroutine ID where panic occurred
    pub goroutine_id: Option<GoroutineId>,
    /// Thread ID where panic occurred
    pub thread_id: ThreadId,
    /// Stack trace at panic
    pub stack_trace: Vec<String>,
    /// Timestamp of panic
    pub timestamp: Instant,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

/// Recovery information
#[derive(Debug, Clone)]
pub struct RecoveryInfo {
    /// Whether recovery was successful
    pub recovered: bool,
    /// Panic value that was recovered
    pub panic_value: Option<String>,
    /// Stack trace at recovery
    pub stack_trace: Vec<String>,
    /// Goroutine ID where recovery occurred
    pub goroutine_id: Option<GoroutineId>,
    /// Recovery timestamp
    pub timestamp: Instant,
}

/// Panic/recover runtime system
pub struct PanicRecoverRuntime {
    /// Panic handlers
    panic_handlers: RwLock<Vec<Box<dyn Fn(&PanicInfo) + Send + Sync>>>,
    /// Recovery handlers
    recovery_handlers: RwLock<Vec<Box<dyn Fn(&RecoveryInfo) + Send + Sync>>>,
    /// Panic statistics
    panic_stats: Mutex<PanicStatistics>,
    /// Configuration
    config: PanicRecoverConfig,
    // Defer runtime integration (disabled for now)
    // defer_runtime: Arc<DeferRuntime>,
}

/// Configuration for panic/recover system
#[derive(Debug, Clone)]
pub struct PanicRecoverConfig {
    /// Enable stack trace capture
    pub capture_stack_traces: bool,
    /// Maximum stack trace depth
    pub max_stack_trace_depth: usize,
    /// Enable panic statistics
    pub enable_statistics: bool,
    /// Maximum panic history to keep
    pub max_panic_history: usize,
    /// Enable defer integration
    pub enable_defer_integration: bool,
    /// Panic log level
    pub log_level: PanicLogLevel,
}

/// Panic logging levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanicLogLevel {
    None,
    Error,
    Warn,
    Info,
    Debug,
}

impl Default for PanicRecoverConfig {
    fn default() -> Self {
        Self {
            capture_stack_traces: true,
            max_stack_trace_depth: 50,
            enable_statistics: true,
            max_panic_history: 100,
            enable_defer_integration: true,
            log_level: PanicLogLevel::Error,
        }
    }
}

/// Panic statistics
#[derive(Debug, Clone)]
pub struct PanicStatistics {
    /// Total panics
    pub total_panics: u64,
    /// Panics recovered
    pub recovered_panics: u64,
    /// Panics by goroutine
    pub panics_by_goroutine: HashMap<GoroutineId, u64>,
    /// Recent panic history
    pub recent_panics: Vec<PanicInfo>,
    /// Recovery success rate
    pub recovery_rate: f64,
    /// Average panic duration
    pub avg_panic_duration: Duration,
}

impl Default for PanicStatistics {
    fn default() -> Self {
        Self {
            total_panics: 0,
            recovered_panics: 0,
            panics_by_goroutine: HashMap::new(),
            recent_panics: Vec::new(),
            recovery_rate: 0.0,
            avg_panic_duration: Duration::from_secs(0),
        }
    }
}

impl PanicRecoverRuntime {
    /// Create a new panic/recover runtime
    pub fn new() -> Self {
        Self {
            panic_handlers: RwLock::new(Vec::new()),
            recovery_handlers: RwLock::new(Vec::new()),
            panic_stats: Mutex::new(PanicStatistics::default()),
            config: PanicRecoverConfig::default(),
            // defer_runtime,
        }
    }

    /// Initialize the panic/recover system
    pub fn initialize(&self) -> Result<()> {
        // Install custom panic hook
        set_hook(Box::new(move |panic_info| {
            eprintln!("PANIC: {:?}", panic_info);
        }));

        Ok(())
    }

    /// Register a panic handler
    pub fn register_panic_handler(&self, handler: Box<dyn Fn(&PanicInfo) + Send + Sync>) -> Result<()> {
        let mut handlers = self.panic_handlers.write().map_err(|_| {
            Error::Runtime("Failed to acquire panic handlers lock".to_string())
        })?;
        handlers.push(handler);
        Ok(())
    }

    /// Register a recovery handler
    pub fn register_recovery_handler(&self, handler: Box<dyn Fn(&RecoveryInfo) + Send + Sync>) -> Result<()> {
        let mut handlers = self.recovery_handlers.write().map_err(|_| {
            Error::Runtime("Failed to acquire recovery handlers lock".to_string())
        })?;
        handlers.push(handler);
        Ok(())
    }

    /// Handle panic from global panic hook
    fn handle_panic(&self, panic_info: &PanicHookInfo<'_>) {
        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        let location = panic_info.location().map(|loc| {
            format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
        });

        let goroutine_id = self.get_current_goroutine_id();
        let stack_trace = if self.config.capture_stack_traces {
            self.capture_stack_trace()
        } else {
            Vec::new()
        };

        let panic_info = PanicInfo {
            message: message.clone(),
            location,
            goroutine_id,
            thread_id: std::thread::current().id(),
            stack_trace: stack_trace.clone(),
            timestamp: Instant::now(),
            metadata: HashMap::new(),
        };

        // Update panic state
        PANIC_STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.panic_message = Some(message.clone());
            state.stack_trace = stack_trace;
            state.goroutine_id = goroutine_id;
            state.in_panic = true;
            state.panic_timestamp = Some(Instant::now());
        });

        // Execute defer handlers if integration is enabled
        // Disabled for now
        // if self.config.enable_defer_integration {
        //     self.execute_defer_handlers();
        // }

        // Call panic handlers
        if let Ok(handlers) = self.panic_handlers.read() {
            for handler in handlers.iter() {
                handler(&panic_info);
            }
        }

        // Update statistics
        if self.config.enable_statistics {
            self.update_panic_statistics(&panic_info);
        }

        // Log panic
        self.log_panic(&panic_info);
    }

    /// Get current goroutine ID
    fn get_current_goroutine_id(&self) -> Option<GoroutineId> {
        // This would integrate with the goroutine scheduler
        // For now, we'll use a simple implementation
        PANIC_STATE.with(|state| {
            state.borrow().goroutine_id
        })
    }

    /// Capture stack trace
    fn capture_stack_trace(&self) -> Vec<String> {
        let mut trace = Vec::new();
        
        // Use backtrace crate to capture the current stack
        let bt = backtrace::Backtrace::new();
        
        // Format the backtrace into readable strings
        for (i, frame) in bt.frames().iter().enumerate() {
            if i >= self.config.max_stack_trace_depth {
                break;
            }
            
            for symbol in frame.symbols() {
                let mut line = String::new();
                
                // Function name
                if let Some(name) = symbol.name() {
                    let name_str = name.to_string();
                    let demangled = rustc_demangle::demangle(&name_str);
                    line.push_str(&format!("{}", demangled));
                } else {
                    line.push_str("<unknown>");
                }
                
                // File and line number
                if let (Some(file), Some(line_no)) = (symbol.filename(), symbol.lineno()) {
                    line.push_str(&format!(" at {}:{}", file.display(), line_no));
                } else if let Some(file) = symbol.filename() {
                    line.push_str(&format!(" at {}", file.display()));
                }
                
                trace.push(line);
            }
        }
        
        if trace.is_empty() {
            trace.push("Stack trace unavailable".to_string());
        }
        
        trace
    }

    /// Execute defer handlers during stack unwinding
    // Disabled for now
    // fn execute_defer_handlers(&self) {
    //     PANIC_STATE.with(|state| {
    //         let mut state = state.borrow_mut();
    //         
    //         // Execute defer handlers in reverse order (LIFO)
    //         while let Some(handler) = state.defer_handlers.pop() {
    //             // Execute defer handler with panic protection
    //             std::panic::catch_unwind(AssertUnwindSafe(|| {
    //                 handler();
    //             })).unwrap_or_else(|_| {
    //                 eprintln!("Defer handler panicked during stack unwinding");
    //             });
    //         }
    //     });
    // }

    /// Update panic statistics
    fn update_panic_statistics(&self, panic_info: &PanicInfo) {
        if let Ok(mut stats) = self.panic_stats.lock() {
            stats.total_panics += 1;
            
            if let Some(goroutine_id) = panic_info.goroutine_id {
                *stats.panics_by_goroutine.entry(goroutine_id).or_insert(0) += 1;
            }
            
            // Add to recent panic history
            stats.recent_panics.push(panic_info.clone());
            if stats.recent_panics.len() > self.config.max_panic_history {
                stats.recent_panics.remove(0);
            }
            
            // Update recovery rate
            if stats.total_panics > 0 {
                stats.recovery_rate = stats.recovered_panics as f64 / stats.total_panics as f64;
            }
        }
    }

    /// Log panic information
    fn log_panic(&self, panic_info: &PanicInfo) {
        match self.config.log_level {
            PanicLogLevel::None => {}
            PanicLogLevel::Error => {
                eprintln!("PANIC: {}", panic_info.message);
            }
            PanicLogLevel::Warn => {
                eprintln!("PANIC [WARN]: {}", panic_info.message);
            }
            PanicLogLevel::Info => {
                eprintln!("PANIC [INFO]: {} at {:?}", panic_info.message, panic_info.location);
            }
            PanicLogLevel::Debug => {
                eprintln!("PANIC [DEBUG]: {:?}", panic_info);
            }
        }
    }

    /// Get panic statistics
    pub fn get_statistics(&self) -> Result<PanicStatistics> {
        let stats = self.panic_stats.lock().map_err(|_| {
            Error::Runtime("Failed to acquire panic statistics lock".to_string())
        })?;
        Ok(stats.clone())
    }
}

/// CURSED panic function - triggers a panic with message
pub fn cursed_panic(message: &str) -> ! {
    PANIC_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.panic_message = Some(message.to_string());
        state.in_panic = true;
        state.panic_timestamp = Some(Instant::now());
        
        // Execute defer handlers in reverse order (LIFO)
        while let Some(handler) = state.defer_handlers.pop() {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                handler();
            }));
        }
    });
    
    panic!("{}", message);
}

/// CURSED recover function - attempts to recover from panic
pub fn cursed_recover() -> Option<String> {
    PANIC_STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        // Can only recover if we're currently in a panic
        if !state.in_panic {
            return None;
        }
        
        // Mark as recovered
        state.in_recover = true;
        state.in_panic = false;
        
        // Return panic message
        let panic_message = state.panic_message.clone();
        
        // Clear panic state
        state.panic_message = None;
        state.panic_timestamp = None;
        
        panic_message
    })
}

/// Check if currently in panic state
pub fn is_in_panic() -> bool {
    PANIC_STATE.with(|state| {
        state.borrow().in_panic
    })
}

/// Add defer handler for cleanup during panic
pub fn add_defer_handler<F>(handler: F) 
where
    F: FnOnce() + Send + 'static,
{
    PANIC_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.defer_handlers.push(Box::new(handler));
    });
}

/// Execute a function with panic recovery
pub fn with_panic_recovery<F, R>(f: F) -> std::result::Result<R, String>
where
    F: FnOnce() -> R + std::panic::UnwindSafe,
{
    match catch_unwind(f) {
        Ok(result) => Ok(result),
        Err(panic_value) => {
            let panic_message = if let Some(s) = panic_value.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_value.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };
            
            // Update recovery statistics
            if let Some(runtime) = get_global_panic_recover_runtime() {
                runtime.update_recovery_statistics(&panic_message);
            }
            
            Err(panic_message)
        }
    }
}

impl PanicRecoverRuntime {
    /// Update recovery statistics
    fn update_recovery_statistics(&self, panic_message: &str) {
        if let Ok(mut stats) = self.panic_stats.lock() {
            stats.recovered_panics += 1;
            
            // Update recovery rate
            if stats.total_panics > 0 {
                stats.recovery_rate = stats.recovered_panics as f64 / stats.total_panics as f64;
            }
            
            // Create recovery info
            let recovery_info = RecoveryInfo {
                recovered: true,
                panic_value: Some(panic_message.to_string()),
                stack_trace: vec![], // Would be populated in real implementation
                goroutine_id: None,
                timestamp: Instant::now(),
            };
            
            // Call recovery handlers
            if let Ok(handlers) = self.recovery_handlers.read() {
                for handler in handlers.iter() {
                    handler(&recovery_info);
                }
            }
        }
    }
}

/// Global panic/recover runtime
static mut GLOBAL_PANIC_RECOVER_RUNTIME: Option<Arc<PanicRecoverRuntime>> = None;
static GLOBAL_PANIC_RECOVER_RUNTIME_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize global panic/recover runtime
pub fn initialize_global_panic_recover_runtime() -> Result<()> {
    unsafe {
        GLOBAL_PANIC_RECOVER_RUNTIME_INIT.call_once(|| {
            let runtime = Arc::new(PanicRecoverRuntime::new());
            runtime.initialize().expect("Failed to initialize panic/recover runtime");
            GLOBAL_PANIC_RECOVER_RUNTIME = Some(runtime);
        });
    }
    Ok(())
}

/// Get global panic/recover runtime
pub fn get_global_panic_recover_runtime() -> Option<Arc<PanicRecoverRuntime>> {
    unsafe {
        GLOBAL_PANIC_RECOVER_RUNTIME.clone()
    }
}

/// Test helper functions
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_panic_recovery() {
        let result = with_panic_recovery(|| {
            cursed_panic("Test panic");
        });
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Test panic");
    }

    #[test]
    fn test_recover_without_panic() {
        let recovered = cursed_recover();
        assert!(recovered.is_none());
    }

    #[test]
    fn test_defer_handler_execution() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        let result = with_panic_recovery(|| {
            add_defer_handler(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
            
            cursed_panic("Test panic with defer");
        });
        
        assert!(result.is_err());
        // Verify that the defer handler was actually executed
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_panic_state_management() {
        assert!(!is_in_panic());
        
        let result = with_panic_recovery(|| {
            cursed_panic("Test panic state");
        });
        
        assert!(result.is_err());
        assert!(!is_in_panic()); // Should be cleared after recovery
    }
}
