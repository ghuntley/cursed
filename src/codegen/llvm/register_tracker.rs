//! Register tracking to ensure consecutive numbering across all LLVM modules

use std::collections::HashSet;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};

/// Global register counter to ensure consistent numbering across all LLVM codegen modules
static GLOBAL_REGISTER_COUNTER: AtomicUsize = AtomicUsize::new(0);

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
        // Use purely global tracking to avoid instance reset issues
        Self::allocate_global_register()
    }
    
    /// Allocate register using only global state (stateless)
    pub fn allocate_global_register() -> String {
        let reg = GLOBAL_REGISTER_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("%{}", reg)
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
        format!("%{}", GLOBAL_REGISTER_COUNTER.load(Ordering::SeqCst))
    }
    
    /// Allocate and return the next register
    pub fn next_register(&mut self) -> String {
        self.allocate_register()
    }
    
    /// Synchronize this tracker with global counter
    pub fn sync_with_global(&mut self) {
        self.next_expected = GLOBAL_REGISTER_COUNTER.load(Ordering::SeqCst);
    }
    
    /// Get current counter value
    pub fn get_current_counter(&self) -> usize {
        GLOBAL_REGISTER_COUNTER.load(Ordering::SeqCst)
    }
    
    /// Get current function-scoped counter value
    pub fn get_function_counter(&self) -> usize {
        self.next_expected
    }
    
    /// Set current counter value
    pub fn set_counter(&mut self, value: usize) {
        GLOBAL_REGISTER_COUNTER.store(value, Ordering::SeqCst);
    }
    
    /// Increment counter (for compatibility)
    pub fn increment_counter(&mut self, amount: usize) {
        GLOBAL_REGISTER_COUNTER.fetch_add(amount, Ordering::SeqCst);
    }
    
    /// Set global counter (used for resetting)
    pub fn set_global_counter(value: usize) {
        GLOBAL_REGISTER_COUNTER.store(value, Ordering::SeqCst);
    }
    
    /// Get current global counter value
    pub fn get_global_counter() -> usize {
        GLOBAL_REGISTER_COUNTER.load(Ordering::SeqCst)
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
