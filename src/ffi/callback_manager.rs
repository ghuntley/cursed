//! Callback management for FFI operations
//!
//! This module provides safe callback management for foreign function calls,
//! including callback registration, lifetime management, and cross-language
//! callback support.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use crate::error::CursedError;
use super::{FfiValue, FunctionSignature, CallbackHandle};

/// Callback manager for FFI operations
pub struct CallbackManager {
    /// Active callbacks
    callbacks: Arc<Mutex<HashMap<usize, CallbackEntry>>>,
    
    /// Next callback ID
    next_id: AtomicUsize,
    
    /// Callback statistics
    stats: Arc<Mutex<CallbackStats>>,
}

/// Callback entry
struct CallbackEntry {
    /// Callback function
    callback: Box<dyn Fn(&[FfiValue]) -> Result<FfiValue, CursedError> + Send + Sync>,
    
    /// Function signature
    signature: FunctionSignature,
    
    /// Creation timestamp
    created_at: std::time::Instant,
    
    /// Call count
    call_count: u64,
    
    /// Total execution time
    total_time: std::time::Duration,
}

/// Callback statistics
#[derive(Debug, Clone)]
pub struct CallbackStats {
    pub total_callbacks: u64,
    pub active_callbacks: u64,
    pub total_calls: u64,
    pub average_call_time: std::time::Duration,
}

impl CallbackManager {
    /// Create new callback manager
    pub fn new() -> Self {
        Self {
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            next_id: AtomicUsize::new(1),
            stats: Arc::new(Mutex::new(CallbackStats::default())),
        }
    }
    
    /// Create a new callback
    pub fn create_callback<F>(
        &self,
        callback: F,
        signature: &FunctionSignature,
    ) -> Result<CallbackHandle, CursedError>
    where
        F: Fn(&[FfiValue]) -> Result<FfiValue, CursedError> + Send + Sync + 'static,
    {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        
        let entry = CallbackEntry {
            callback: Box::new(callback),
            signature: signature.clone(),
            created_at: std::time::Instant::now(),
            call_count: 0,
            total_time: std::time::Duration::ZERO,
        };
        
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.insert(id, entry);
        
        // Update statistics
        let mut stats = self.stats.lock().unwrap();
        stats.total_callbacks += 1;
        stats.active_callbacks += 1;
        
        Ok(CallbackHandle {
            id,
            function_ptr: std::ptr::null_mut(), // Would be set to actual function pointer
        })
    }
    
    /// Call a callback by ID
    pub fn call_callback(&self, id: usize, args: &[FfiValue]) -> Result<FfiValue, CursedError> {
        let start_time = std::time::Instant::now();
        
        let result = {
            let mut callbacks = self.callbacks.lock().unwrap();
            if let Some(entry) = callbacks.get_mut(&id) {
                let result = (entry.callback)(args);
                
                // Update entry statistics
                entry.call_count += 1;
                entry.total_time += start_time.elapsed();
                
                result
            } else {
                Err(CursedError::General(format!("Callback {} not found", id)))
            }
        };
        
        // Update global statistics
        let mut stats = self.stats.lock().unwrap();
        stats.total_calls += 1;
        let total_time = stats.average_call_time * (stats.total_calls - 1) as u32 + start_time.elapsed();
        stats.average_call_time = total_time / stats.total_calls as u32;
        
        result
    }
    
    /// Remove a callback
    pub fn remove_callback(&self, id: usize) -> Result<(), CursedError> {
        let mut callbacks = self.callbacks.lock().unwrap();
        if callbacks.remove(&id).is_some() {
            let mut stats = self.stats.lock().unwrap();
            stats.active_callbacks -= 1;
            Ok(())
        } else {
            Err(CursedError::General(format!("Callback {} not found", id)))
        }
    }
    
    /// Get callback statistics
    pub fn get_stats(&self) -> CallbackStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }
    
    /// Cleanup all callbacks
    pub fn cleanup(&self) -> Result<(), CursedError> {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.clear();
        
        let mut stats = self.stats.lock().unwrap();
        stats.active_callbacks = 0;
        
        Ok(())
    }
}

impl Default for CallbackStats {
    fn default() -> Self {
        Self {
            total_callbacks: 0,
            active_callbacks: 0,
            total_calls: 0,
            average_call_time: std::time::Duration::ZERO,
        }
    }
}

impl Default for CallbackManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{FfiType, Parameter};
    
    #[test]
    fn test_callback_creation() {
        let manager = CallbackManager::new();
        
        let signature = FunctionSignature {
            name: "test_callback".to_string(),
            return_type: FfiType::SignedInteger(32),
            parameters: vec![Parameter {
                name: "x".to_string(),
                param_type: FfiType::SignedInteger(32),
                is_const: false,
                is_nullable: false,
            }],
            is_variadic: false,
        };
        
        let callback = |args: &[FfiValue]| -> Result<FfiValue, CursedError> {
            if let [FfiValue::SignedInteger(x)] = args {
                Ok(FfiValue::SignedInteger(x * 2))
            } else {
                Err(CursedError::General("Invalid arguments".to_string()))
            }
        };
        
        let handle = manager.create_callback(callback, &signature).unwrap();
        assert_eq!(handle.id, 1);
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_callbacks, 1);
        assert_eq!(stats.active_callbacks, 1);
    }
    
    #[test]
    fn test_callback_execution() {
        let manager = CallbackManager::new();
        
        let signature = FunctionSignature {
            name: "test_callback".to_string(),
            return_type: FfiType::SignedInteger(32),
            parameters: vec![Parameter {
                name: "x".to_string(),
                param_type: FfiType::SignedInteger(32),
                is_const: false,
                is_nullable: false,
            }],
            is_variadic: false,
        };
        
        let callback = |args: &[FfiValue]| -> Result<FfiValue, CursedError> {
            if let [FfiValue::SignedInteger(x)] = args {
                Ok(FfiValue::SignedInteger(x * 2))
            } else {
                Err(CursedError::General("Invalid arguments".to_string()))
            }
        };
        
        let handle = manager.create_callback(callback, &signature).unwrap();
        
        let args = vec![FfiValue::SignedInteger(21)];
        let result = manager.call_callback(handle.id, &args).unwrap();
        
        if let FfiValue::SignedInteger(val) = result {
            assert_eq!(val, 42);
        } else {
            panic!("Expected integer result");
        }
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_calls, 1);
    }
    
    #[test]
    fn test_callback_removal() {
        let manager = CallbackManager::new();
        
        let signature = FunctionSignature {
            name: "test_callback".to_string(),
            return_type: FfiType::SignedInteger(32),
            parameters: vec![],
            is_variadic: false,
        };
        
        let callback = |_args: &[FfiValue]| -> Result<FfiValue, CursedError> {
            Ok(FfiValue::SignedInteger(42))
        };
        
        let handle = manager.create_callback(callback, &signature).unwrap();
        
        assert!(manager.remove_callback(handle.id).is_ok());
        
        let stats = manager.get_stats();
        assert_eq!(stats.active_callbacks, 0);
    }
}
