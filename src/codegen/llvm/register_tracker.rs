//! Register tracking to ensure consecutive numbering across all LLVM modules

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// Global register counter to ensure consistent numbering across all LLVM codegen modules
static mut GLOBAL_REGISTER_COUNTER: usize = 0;
static GLOBAL_REGISTER_MUTEX: Mutex<()> = Mutex::new(());

#[derive(Debug, Default, Clone)]
pub struct RegisterTracker {
    allocated: HashSet<usize>,
    next_expected: usize,
}

impl RegisterTracker {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new function-scoped register tracker (for WASM)
    pub fn new_function_scoped() -> Self {
        Self {
            allocated: HashSet::new(),
            next_expected: 0,
        }
    }
    
    /// Allocate next register with global synchronization
    pub fn allocate_register(&mut self) -> String {
        let _lock = GLOBAL_REGISTER_MUTEX.lock().unwrap();
        unsafe {
            // LLVM expects registers to start from %0
            let reg = GLOBAL_REGISTER_COUNTER;
            self.allocated.insert(reg);
            self.next_expected = reg + 1;
            GLOBAL_REGISTER_COUNTER += 1;
            format!("%{}", reg)
        }
    }
    
    /// Allocate next register with function scope (for WASM)
    pub fn allocate_function_register(&mut self) -> String {
        let reg = self.next_expected;
        self.allocated.insert(reg);
        self.next_expected += 1;
        format!("%{}", reg)
    }
    
    /// Get next register number without allocation
    pub fn peek_next_register(&self) -> String {
        let _lock = GLOBAL_REGISTER_MUTEX.lock().unwrap();
        unsafe {
            format!("%{}", GLOBAL_REGISTER_COUNTER)
        }
    }
    
    /// Allocate and return the next register
    pub fn next_register(&mut self) -> String {
        self.allocate_register()
    }
    
    /// Synchronize this tracker with global counter
    pub fn sync_with_global(&mut self) {
        let _lock = GLOBAL_REGISTER_MUTEX.lock().unwrap();
        unsafe {
            self.next_expected = GLOBAL_REGISTER_COUNTER;
        }
    }
    
    /// Get current counter value
    pub fn get_current_counter(&self) -> usize {
        let _lock = GLOBAL_REGISTER_MUTEX.lock().unwrap();
        unsafe { GLOBAL_REGISTER_COUNTER }
    }
    
    /// Get current function-scoped counter value
    pub fn get_function_counter(&self) -> usize {
        self.next_expected
    }
    
    /// Set current counter value
    pub fn set_counter(&mut self, value: usize) {
        let _lock = GLOBAL_REGISTER_MUTEX.lock().unwrap();
        unsafe { GLOBAL_REGISTER_COUNTER = value; }
    }
    
    /// Increment counter (for compatibility)
    pub fn increment_counter(&mut self, amount: usize) {
        let _lock = GLOBAL_REGISTER_MUTEX.lock().unwrap();
        unsafe { GLOBAL_REGISTER_COUNTER += amount; }
    }
    
    /// Set global counter (used for resetting)
    pub fn set_global_counter(value: usize) {
        let _lock = GLOBAL_REGISTER_MUTEX.lock().unwrap();
        unsafe {
            GLOBAL_REGISTER_COUNTER = value;
        }
    }
    
    /// Get current global counter value
    pub fn get_global_counter() -> usize {
        let _lock = GLOBAL_REGISTER_MUTEX.lock().unwrap();
        unsafe {
            GLOBAL_REGISTER_COUNTER
        }
    }
    
    pub fn validate(&self) -> Result<(), String> {
        for i in 0..self.next_expected {
            if !self.allocated.contains(&i) {
                return Err(format!("Missing register %{}", i));
            }
        }
        Ok(())
    }
    
    pub fn get_next_number(&self) -> usize {
        self.next_expected
    }
}
