use crate::error::CursedError;
/// Signal handler for registering custom signal handlers
use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;
// use crate::stdlib::signal_boost::core::{BoostSignal, notify};
// use crate::stdlib::signal_boost::error::{SignalBoostError, SignalBoostResult, handler_exists, no_handler};

/// Configuration for SignalHandler
#[derive(Debug, Clone)]
pub struct SignalHandlerConfig {
impl Default for SignalHandlerConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Signal handler entry with priority and metadata
#[derive(Clone)]
struct HandlerEntry {
/// Enhanced signal handler for registering custom signal handlers
pub struct SignalHandler {
impl SignalHandler {
    /// Create a new SignalHandler
    pub fn new() -> Self {
        Self::with_config(SignalHandlerConfig::default())
    /// Create a new SignalHandler with custom configuration
    pub fn with_config(config: SignalHandlerConfig) -> Self {
        Self {
        }
    }
    
    /// Register a signal handler function that receives the signal
    pub fn register<F>(&mut self, signal: BoostSignal, handler: F) -> SignalBoostResult<&mut Self>
    where
    {
        self.register_with_priority(signal, 0, "anonymous", handler)
    /// Register a signal handler function that doesn't receive the signal
    pub fn register_func<F>(&mut self, signal: BoostSignal, handler: F) -> SignalBoostResult<&mut Self>
    where
    {
        let wrapper = move |_: BoostSignal| handler();
        self.register_with_priority(signal, 0, "anonymous_func", wrapper)
    /// Register a signal handler with specific priority and name
    pub fn register_with_priority<F>(&mut self, signal: BoostSignal, priority: i32, name: &str, handler: F) -> SignalBoostResult<&mut Self>
    where
    {
        let mut handlers = self.handlers.lock().unwrap();
        let entries = handlers.entry(signal).or_insert_with(Vec::new);
        
        // Check if we already have too many handlers
        if entries.len() >= self.config.max_concurrent_handlers {
            return Err(SignalBoostError::ConfigError(
                format!("Too many handlers for signal {}", signal.name())
            ));
        let entry = HandlerEntry {
        
        entries.push(entry);
        
        // Sort by priority (higher priority first)
        entries.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        if self.config.debug_enabled {
                          name, signal.name(), priority);
        Ok(self)
    /// Unregister all handlers for a signal
    pub fn unregister(&mut self, signal: BoostSignal) -> SignalBoostResult<&mut Self> {
        let mut handlers = self.handlers.lock().unwrap();
        if handlers.remove(&signal).is_some() {
            if self.config.debug_enabled {
                tracing::debug!("Unregistered all handlers for signal {}", signal.name());
            }
            Ok(self)
        } else {
            Err(no_handler(&format!("No handlers registered for signal {}", signal.name())))
        }
    }
    
    /// Unregister a specific handler by name
    pub fn unregister_named(&mut self, signal: BoostSignal, name: &str) -> SignalBoostResult<&mut Self> {
        let mut handlers = self.handlers.lock().unwrap();
        if let Some(entries) = handlers.get_mut(&signal) {
            let initial_len = entries.len();
            entries.retain(|entry| entry.name != name);
            
            if entries.len() < initial_len {
                if self.config.debug_enabled {
                    tracing::debug!("Unregistered handler '{}' for signal {}", name, signal.name());
                }
                Ok(self)
            } else {
                Err(no_handler(&format!("No handler named '{}' found for signal {}", name, signal.name())))
            }
        } else {
            Err(no_handler(&format!("No handlers registered for signal {}", signal.name())))
        }
    }
    
    /// Start handling signals
    pub fn handle(&mut self) -> SignalBoostResult<()> {
        {
            let mut running = self.running.lock().unwrap();
            if *running {
                return Err(SignalBoostError::General("Handler already running".to_string()));
            }
            *running = true;
        // Get all signals we need to monitor
        let all_signals: Vec<BoostSignal> = {
            let handlers = self.handlers.lock().unwrap();
            handlers.keys().copied().collect()
        
        if all_signals.is_empty() {
            return Err(SignalBoostError::ConfigError("No signals registered".to_string()));
        // Set up signal notification
        let (receiver, _handle) = notify(&all_signals)?;
        
        let handlers_clone = Arc::clone(&self.handlers);
        let config_clone = self.config.clone();
        let running_clone = Arc::clone(&self.running);
        
        let handle = thread::spawn(move || {
            tracing::info!("Signal handler started for {} signals", all_signals.len());
            
            while *running_clone.lock().unwrap() {
                match receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(signal) => {
                        if config_clone.debug_enabled {
                            tracing::debug!("Received signal: {}", signal);
                        // Execute all handlers for this signal
                        let handlers = handlers_clone.lock().unwrap();
                        if let Some(entries) = handlers.get(&signal) {
                            for entry in entries {
                                entry.call_count.fetch_add(1, Ordering::SeqCst);
                                
                                if config_clone.debug_enabled {
                                                  entry.name, signal.name());
                                // Execute handler with timeout
                                let handler = Arc::clone(&entry.handler);
                                let timeout_signal = signal;
                                let timeout_duration = config_clone.handler_timeout;
                                
                                thread::spawn(move || {
                                    let start_time = std::time::Instant::now();
                                    handler(timeout_signal);
                                    let elapsed = start_time.elapsed();
                                    
                                    if elapsed > timeout_duration {
                                                     elapsed.as_secs_f64(), timeout_duration.as_secs_f64());
                                    }
                                });
                            }
                        }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // Normal timeout, continue
                        continue;
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        tracing::info!("Signal receiver disconnected, stopping handler");
                        break;
                    }
                }
            tracing::info!("Signal handler stopped");
        });
        
        self.handle = Some(handle);
        Ok(())
    /// Stop handling signals
    pub fn stop(&mut self) -> SignalBoostResult<()> {
        {
            let mut running = self.running.lock().unwrap();
            *running = false;
        if let Some(handle) = self.handle.take() {
            // Give the thread a moment to stop gracefully
            thread::sleep(Duration::from_millis(100));
            
            if !handle.is_finished() {
                tracing::warn!("Signal handler thread did not stop gracefully");
            }
        }
        
        Ok(())
    /// Enable or disable debug logging
    pub fn enable_debug(&mut self, enabled: bool) -> &mut Self {
        self.config.debug_enabled = enabled;
        self
    /// Set priority for a specific signal handler
    pub fn set_priority(&mut self, signal: BoostSignal, name: &str, priority: i32) -> SignalBoostResult<&mut Self> {
        let mut handlers = self.handlers.lock().unwrap();
        if let Some(entries) = handlers.get_mut(&signal) {
            for entry in entries.iter_mut() {
                if entry.name == name {
                    entry.priority = priority;
                    
                    // Re-sort entries by priority
                    entries.sort_by(|a, b| b.priority.cmp(&a.priority));
                    
                    if self.config.debug_enabled {
                                      priority, name, signal.name());
                    }
                    return Ok(self);
                }
            }
            Err(no_handler(&format!("No handler named '{}' found for signal {}", name, signal.name())))
        } else {
            Err(no_handler(&format!("No handlers registered for signal {}", signal.name())))
        }
    }
    
    /// Get statistics for registered handlers
    pub fn get_statistics(&self) -> HandlerStatistics {
        let handlers = self.handlers.lock().unwrap();
        let mut stats = HandlerStatistics {
        
        for (signal, entries) in handlers.iter() {
            stats.total_handlers += entries.len();
            stats.signal_handlers.insert(*signal, entries.len());
            
            for entry in entries {
                let calls = entry.call_count.load(Ordering::SeqCst);
                stats.handler_calls.insert(entry.name.clone(), calls);
            }
        }
        
        stats
    /// Check if the handler is currently running
    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    /// Get the number of registered signals
    pub fn signal_count(&self) -> usize {
        self.handlers.lock().unwrap().len()
    /// Get the total number of registered handlers
    pub fn handler_count(&self) -> usize {
        let handlers = self.handlers.lock().unwrap();
        handlers.values().map(|entries| entries.len()).sum()
    }
}

impl Drop for SignalHandler {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Statistics for signal handlers
#[derive(Debug, Clone)]
pub struct HandlerStatistics {
// Global statistics
static ACTIVE_HANDLERS: AtomicUsize = AtomicUsize::new(0);

pub fn get_active_count() -> usize {
    ACTIVE_HANDLERS.load(Ordering::SeqCst)
