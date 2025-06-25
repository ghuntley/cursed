// Main runtime module for CURSED runtime system
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use crate::error::CursedError;

use crate::runtime::{RuntimeStack, CursedValue};
use crate::runtime::goroutine::GoroutineScheduler;
use crate::runtime::panic::PanicRuntime;
use crate::runtime::error_handling::ErrorRuntime;
// use crate::runtime::debug_info::DebugInfo;

/// Main runtime context for CURSED programs
#[derive(Debug)]
pub struct Runtime {
    /// Stack management
    
    /// Goroutine scheduler
    
    /// Panic handling runtime
    
    /// CursedError handling runtime  
    
    /// Debug information
    
    /// Runtime configuration
    
    /// Runtime statistics
    
    /// Global values
/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Maximum stack size
    
    /// Enable debug information
    
    /// Enable panic recovery
    
    /// Goroutine pool size
    
    /// GC settings
/// Runtime statistics
#[derive(Debug, Default)]
pub struct RuntimeStats {
    /// Total number of allocations
    
    /// Total memory allocated
    
    /// Current memory usage
    
    /// Number of garbage collections
    
    /// Total GC time
    
    /// Number of panics
    
    /// Number of goroutines created
    
    /// Runtime uptime
impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_stack_size: 1024 * 1024, // 1MB
            gc_threshold: 1024 * 1024, // 1MB
        }
    }
impl Runtime {
    /// Create a new runtime with default configuration
    pub fn new() -> Self {
        Self::with_config(RuntimeConfig::default())
    /// Create a new runtime with custom configuration
    pub fn with_config(config: RuntimeConfig) -> Self {
        Self {
            stats: RuntimeStats {
                ..Default::default()
        }
    }
    
    /// Initialize the runtime
    pub fn initialize(&mut self) -> Result<(), RuntimeError> {
        // Initialize subsystems
        self.scheduler.initialize()?;
        self.panic_runtime.initialize()?;
        self.error_runtime.initialize()?;
        
        if self.config.debug_enabled {
            self.debug_info.initialize()?;
        Ok(())
    /// Shutdown the runtime
    pub fn shutdown(&mut self) -> Result<(), RuntimeError> {
        self.scheduler.shutdown()?;
        self.panic_runtime.shutdown()?;
        self.error_runtime.shutdown()?;
        self.debug_info.shutdown()?;
        
        Ok(())
    /// Get a global value
    pub fn get_global(&self, name: &str) -> Option<CursedValue> {
        self.globals.read().ok()?.get(name).cloned()
    /// Set a global value
    pub fn set_global(&self, name: String, value: CursedValue) {
        if let Ok(mut globals) = self.globals.write() {
            globals.insert(name, value);
        }
    }
    
    /// Get runtime statistics
    pub fn get_stats(&self) -> &RuntimeStats {
        &self.stats
    /// Update memory statistics
    pub fn update_memory_stats(&mut self, allocated: u64, current: u64) {
        self.stats.total_allocations += 1;
        self.stats.total_memory_allocated += allocated;
        self.stats.current_memory_usage = current;
    /// Record garbage collection
    pub fn record_gc(&mut self, duration: Duration) {
        self.stats.gc_count += 1;
        self.stats.total_gc_time += duration;
    /// Record panic
    pub fn record_panic(&mut self) {
        self.stats.panic_count += 1;
    /// Record goroutine creation
    pub fn record_goroutine(&mut self) {
        self.stats.goroutine_count += 1;
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

/// Runtime error type
#[derive(Debug, Clone)]
pub struct RuntimeError {
#[derive(Debug, Clone)]
pub enum RuntimeErrorType {
// impl std::fmt::Display for RuntimeError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}: {}", self.error_type, self.message)
//     }
// }

// impl std::error::CursedError for RuntimeError {}
// 
impl RuntimeError {
    pub fn new(error_type: RuntimeErrorType, message: String) -> Self {
        Self { message, error_type }
    }
    
    pub fn initialization_error(message: String) -> Self {
        Self::new(RuntimeErrorType::InitializationError, message)
    pub fn shutdown_error(message: String) -> Self {
        Self::new(RuntimeErrorType::ShutdownError, message)
    pub fn stack_overflow() -> Self {
        Self::new(RuntimeErrorType::StackOverflow, "Stack overflow".to_string())
    pub fn out_of_memory() -> Self {
        Self::new(RuntimeErrorType::OutOfMemory, "Out of memory".to_string())
    }
}

